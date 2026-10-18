use kutha_common::{
    Event, EventId, Op, RuleAst, SupportPolarity, TermDictionary, TermId, TransactionTime,
    ValidTime, KUTHA_ACTION, KUTHA_ADMISSION, KUTHA_ALLOWS, KUTHA_POLICY, KUTHA_RULE,
};
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};

/// Ingest lookup leases: Fx hasher (trusted EventId/TermId; Samyama-style).
type IngestMap<K, V> = FxHashMap<K, V>;
/// Typical relatedTo mint: one fact per claim/event — inline without heap (Pogocache-ish).
type FactIdxList = SmallVec<[usize; 1]>;

/// Integration-test examine counter (not fingerprint / JSON). `cfg(test)` is off for `tests/*.rs`.
#[derive(Debug, Default)]
pub(crate) struct ExamineCounter(AtomicU64);

impl Clone for ExamineCounter {
    fn clone(&self) -> Self {
        Self(AtomicU64::new(self.0.load(Ordering::Relaxed)))
    }
}

impl ExamineCounter {
    pub(crate) fn reset(&self) {
        self.0.store(0, Ordering::Relaxed);
    }

    pub(crate) fn get(&self) -> u64 {
        self.0.load(Ordering::Relaxed)
    }

    pub(crate) fn bump(&self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

fn nil_claim() -> EventId {
    EventId::nil()
}

/// Half-open intersection of `[a_from, a_to)` and `[b_from, b_to)`. `None` end is +∞.
/// Empty when `from` is not strictly less than `to` after substituting infinity (D-C7).
pub(crate) fn vt_intersect(
    a_from: u64,
    a_to: Option<u64>,
    b_from: u64,
    b_to: Option<u64>,
) -> Option<(u64, Option<u64>)> {
    let from = a_from.max(b_from);
    let a_end = a_to.unwrap_or(u64::MAX);
    let b_end = b_to.unwrap_or(u64::MAX);
    let to_raw = a_end.min(b_end);
    if from >= to_raw {
        return None;
    }
    let to = if to_raw == u64::MAX {
        None
    } else {
        Some(to_raw)
    };
    Some((from, to))
}

/// Leftover halves of fact VT minus a non-empty intersection (prefix, suffix). Skip empty halves.
pub(crate) fn leftover_halves(
    fact_from: u64,
    fact_to: Option<u64>,
    inter_from: u64,
    inter_to: Option<u64>,
) -> (Option<(u64, Option<u64>)>, Option<(u64, Option<u64>)>) {
    let prefix = if fact_from < inter_from {
        Some((fact_from, Some(inter_from)))
    } else {
        None
    };
    let suffix = match inter_to {
        None => None,
        Some(i_to) => {
            let fact_end = fact_to.unwrap_or(u64::MAX);
            if i_to < fact_end {
                Some((i_to, fact_to))
            } else {
                None
            }
        }
    };
    (prefix, suffix)
}

fn flipped_polarity(p: SupportPolarity) -> SupportPolarity {
    match p {
        SupportPolarity::Positive => SupportPolarity::Negative,
        SupportPolarity::Negative => SupportPolarity::Positive,
    }
}

/// Residuals copy polarity. An object-changing replacement row flips when old polarity is Some.
fn polarity_for_row(
    old: Option<SupportPolarity>,
    old_object: TermId,
    new_object: TermId,
) -> Option<SupportPolarity> {
    if new_object == old_object {
        old
    } else {
        old.map(flipped_polarity)
    }
}

/// One asserted (or behavior-asserted) fact in the fold. Losers stay on retract (ADR-013).
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Fact {
    pub seq: u64,
    pub subject: TermId,
    pub relation: TermId,
    object: TermId,
    pub valid_from: u64,
    pub valid_to: Option<u64>,
    pub ingested_at: u64,
    pub invalidated_at: Option<u64>,
    /// Portable claim / proposition identity (ADR-011, ING-02). Multiple Facts may share one
    /// `claim_id` as independent supports. Not the support slot and not `delivery_key`.
    #[serde(default = "nil_claim")]
    pub claim_id: EventId,
    /// Minting Event.id — the support slot (REF-01, ING-02). Retract/Correct look up this field, not fold-local seq.
    #[serde(default = "nil_claim")]
    pub event_id: EventId,
    /// Durable retry key copied from `Op::Assert` (ING-01 / D-01). Not claim identity.
    #[serde(default)]
    pub delivery_key: Option<String>,
    /// Stored support polarity copied from `Op::Assert` (ING-03). `None` is in neither conflict bucket.
    #[serde(default)]
    pub polarity: Option<SupportPolarity>,
}

impl Fact {
    pub fn object(&self) -> TermId {
        self.object
    }

    pub fn is_live_at(&self, tt: impl Into<TransactionTime>, vt: impl Into<ValidTime>) -> bool {
        let tt = tt.into().as_u64();
        let vt = vt.into().as_u64();
        if self.ingested_at > tt {
            return false;
        }
        if self.invalidated_at.is_some_and(|inv| inv <= tt) {
            return false;
        }
        if vt < self.valid_from {
            return false;
        }
        if self.valid_to.is_some_and(|to| vt >= to) {
            return false;
        }
        true
    }

    /// Transaction-time liveness only (no `valid_from` / `valid_to`).
    ///
    /// Used by `as_known_on` so closed VT intervals stay visible at a knowledge cut.
    pub fn is_known_at(&self, tt: impl Into<TransactionTime>) -> bool {
        let tt = tt.into().as_u64();
        if self.ingested_at > tt {
            return false;
        }
        if self.invalidated_at.is_some_and(|inv| inv <= tt) {
            return false;
        }
        true
    }
}

/// How fold secondary index leases are maintained on the write path.
///
/// - [`FoldIndexMode::Eager`]: warm maps on every fact push (accept write-cost; default).
/// - [`FoldIndexMode::Lazy`]: thin write — append facts only; rebuild or scan on demand.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FoldIndexMode {
    /// O(1) lookups; higher ingest wall time (D-06 tradeoff).
    #[default]
    Eager,
    /// Thin write / lazy lease (Helix/Samyama-style deferred secondary work).
    Lazy,
}

impl FoldIndexMode {
    /// Reads `KUTHA_FOLD_INDEX_MODE` from the process env after `apply_dotenv`
    /// (repo-root `.env`; template `.env.example`).
    ///
    /// - `lazy` | `thin` → [`FoldIndexMode::Lazy`]
    /// - `eager` | unset | other → [`FoldIndexMode::Eager`]
    pub fn from_env() -> Self {
        crate::allow::apply_dotenv();
        match std::env::var("KUTHA_FOLD_INDEX_MODE")
            .unwrap_or_default()
            .to_ascii_lowercase()
            .as_str()
        {
            "lazy" | "thin" => Self::Lazy,
            _ => Self::Eager,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum LiveFactFilter {
    Product,
    Dictionary,
    All,
}

impl LiveFactFilter {
    pub(crate) fn keep(self, fold: &GraphFold, fact: &Fact) -> bool {
        let dict = fold.is_dictionary_rel(fact.relation);
        match self {
            Self::Product => !dict,
            Self::Dictionary => dict,
            Self::All => true,
        }
    }
}

/// One live Horn rule, keyed from each distinct premise relation. Droppable.
#[derive(Clone, Debug)]
pub(crate) struct HornLeaseSlot {
    pub(crate) fact_idx: usize,
    pub(crate) pin: String,
    pub(crate) ast: RuleAst,
}

/// Deterministic fold of the log. Droppable picture, not SoT.
/// Hot maps are leases: skip-serialized and rebuilt from `facts` (D-02).
#[derive(Clone, Debug, serde::Serialize)]
pub struct GraphFold {
    pub(crate) facts: Vec<Fact>,
    pub(crate) next_seq: u64,
    /// Hot-path examine count. Not SoT; skipped in snapshot JSON.
    #[serde(skip)]
    pub(crate) hot_examine: ExamineCounter,
    /// Horn-eval row visits (`live_horns` / `search_at`). Not SoT; skipped in snapshot JSON.
    #[serde(skip)]
    pub(crate) horn_examine: ExamineCounter,
    /// Ingest index policy. Pinned in snapshot JSON (REPL-04); not fingerprint input.
    pub(crate) index_mode: FoldIndexMode,
    /// Whether secondary maps match `facts` (Lazy may leave this false).
    #[serde(skip)]
    pub(crate) hot_maps_warm: bool,
    /// Fact indices grouped by `valid_from` so a cut can skip future-start decoys.
    #[serde(skip)]
    pub(crate) vt_by_from: BTreeMap<u64, Vec<usize>>,
    /// Fact indices grouped by portable `claim_id`.
    #[serde(skip)]
    pub(crate) claim_facts: IngestMap<EventId, FactIdxList>,
    /// Fact indices grouped by minting `event_id` (ingest lookup lease).
    #[serde(skip)]
    pub(crate) fact_by_event: IngestMap<EventId, FactIdxList>,
    /// Live (subject, relation, object) → fact indices (co-live VT residuals).
    #[serde(skip)]
    pub(crate) live_triples: IngestMap<(TermId, TermId, TermId), FactIdxList>,
    /// `kutha:allows` fact indices by delivery_key (product relation name). ADR-040
    /// droppable lease, rebuilt from `facts`. Not fingerprint / JSON. Maintained on
    /// every fact push even under [`FoldIndexMode::Lazy`] so admit stays O(allows);
    /// product maps (`vt_by_from`, `claim_facts`, `fact_by_event`, `live_triples`)
    /// still follow Eager/Lazy.
    #[serde(skip)]
    pub(crate) allows_by_name: IngestMap<String, FactIdxList>,
    /// Horn rule slots keyed by premise relation name. Droppable lease, rebuilt
    /// from `kutha:rule` facts + blobs; not fingerprint / JSON. Maintained on
    /// every rule push even under [`FoldIndexMode::Lazy`] (like `allows_by_name`).
    #[serde(skip)]
    pub(crate) horns_by_premise: IngestMap<String, Vec<HornLeaseSlot>>,
    /// Fact indices grouped by relation (Horn probe lease). Product hot map:
    /// follows Eager/Lazy with `vt_by_from`.
    #[serde(skip)]
    pub(crate) facts_by_relation: IngestMap<TermId, FactIdxList>,
    /// `kutha:rule` fact indices by delivery_key (pin). Droppable; Lazy-maintained.
    #[serde(skip)]
    pub(crate) rules_by_pin: IngestMap<String, FactIdxList>,
    /// `kutha:policy` fact indices by delivery_key (pin). Droppable; Lazy-maintained.
    #[serde(skip)]
    pub(crate) policies_by_pin: IngestMap<String, FactIdxList>,
    /// Policy fact indexes, oldest first. Election walks newest-first and stops
    /// at the first row live at the cut (M105).
    pub(crate) policy_by_ingest: Vec<usize>,
    /// True after a rebuild or bind so a miss is "no rule uses this premise",
    /// not "lease never built" (fallback scan).
    #[serde(skip)]
    pub(crate) horn_lease_ready: bool,
    /// Interned `kutha:*` relation TermIds (D-05). Rebuilt from the dictionary; not fingerprint.
    #[serde(skip)]
    pub(crate) dictionary_rels: HashSet<TermId>,
    #[serde(skip)]
    pub(crate) rel_allows: Option<TermId>,
    #[serde(skip)]
    pub(crate) rel_rule: Option<TermId>,
    #[serde(skip)]
    pub(crate) rel_policy: Option<TermId>,
    #[serde(skip)]
    pub(crate) rel_admission: Option<TermId>,
    #[serde(skip)]
    pub(crate) rel_action: Option<TermId>,
    /// Content-addressed bodies from `Op::StoreBlob` (store-once). Not fingerprint input.
    #[serde(skip)]
    pub(crate) blobs: HashMap<String, Vec<u8>>,
    /// Wall date → transaction time (`Event.ingested_at`) from `Op::TtAnchor`. Not fingerprint.
    pub(crate) tt_anchors: BTreeMap<u64, u64>,
    /// True when `tt_anchors` was present in snapshot JSON (legacy files omit the key).
    #[serde(skip)]
    pub(crate) tt_anchors_pinned: bool,
}

impl Default for GraphFold {
    fn default() -> Self {
        Self::with_index_mode(FoldIndexMode::from_env())
    }
}

impl<'de> serde::Deserialize<'de> for GraphFold {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct GraphFoldDe {
            pub(crate) facts: Vec<Fact>,
            pub(crate) next_seq: u64,
            #[serde(default)]
            pub(crate) tt_anchors: Option<BTreeMap<u64, u64>>,
            #[serde(default)]
            pub(crate) index_mode: FoldIndexMode,
        }
        let de = GraphFoldDe::deserialize(deserializer)?;
        let mut fold = GraphFold::with_index_mode(de.index_mode);
        fold.facts = de.facts;
        fold.next_seq = de.next_seq;
        if let Some(anchors) = de.tt_anchors {
            fold.tt_anchors = anchors;
            fold.tt_anchors_pinned = true;
        }
        match fold.index_mode {
            FoldIndexMode::Eager => {
                fold.rebuild_hot_maps();
            }
            FoldIndexMode::Lazy => {
                // Thin hydrate: leave secondary leases cold until ensure / indexed op.
                fold.hot_maps_warm = fold.facts.is_empty();
            }
        }
        Ok(fold)
    }
}

impl GraphFold {
    pub fn with_index_mode(index_mode: FoldIndexMode) -> Self {
        Self {
            facts: Vec::new(),
            next_seq: 0,
            hot_examine: ExamineCounter::default(),
            horn_examine: ExamineCounter::default(),
            index_mode,
            hot_maps_warm: true,
            vt_by_from: BTreeMap::new(),
            claim_facts: IngestMap::default(),
            fact_by_event: IngestMap::default(),
            live_triples: IngestMap::default(),
            allows_by_name: IngestMap::default(),
            horns_by_premise: IngestMap::default(),
            facts_by_relation: IngestMap::default(),
            rules_by_pin: IngestMap::default(),
            policies_by_pin: IngestMap::default(),
            policy_by_ingest: Vec::new(),
            horn_lease_ready: false,
            dictionary_rels: HashSet::new(),
            rel_allows: None,
            rel_rule: None,
            rel_policy: None,
            rel_admission: None,
            rel_action: None,
            blobs: HashMap::new(),
            tt_anchors: BTreeMap::new(),
            tt_anchors_pinned: false,
        }
    }

    /// Log `Op::TtAnchor` walls and `ingested_at` (not hashed into the fact fingerprint).
    pub(crate) fn tt_anchors_from_events(events: &[Event]) -> BTreeMap<u64, u64> {
        let mut out = BTreeMap::new();
        for e in events {
            if let Op::TtAnchor { wall } = &e.op {
                out.insert(*wall, e.ingested_at);
            }
        }
        out
    }

    /// Bind reserved `kutha:*` relation TermIds from the live dictionary (D-01 / D-05).
    pub fn bind_reserved(&mut self, dict: &TermDictionary) {
        self.dictionary_rels.clear();
        for (i, s) in dict.strings().iter().enumerate() {
            if s.starts_with("kutha:") {
                self.dictionary_rels.insert(i as TermId);
            }
        }
        let prev_allows = self.rel_allows;
        let prev_rule = self.rel_rule;
        let prev_policy = self.rel_policy;
        self.rel_allows = dict.id(KUTHA_ALLOWS);
        self.rel_rule = dict.id(KUTHA_RULE);
        self.rel_policy = dict.id(KUTHA_POLICY);
        self.rel_admission = dict.id(KUTHA_ADMISSION);
        self.rel_action = dict.id(KUTHA_ACTION);
        // Deserialize / replay may index before reserved ids exist. Rebuild the
        // admit lease once `kutha:allows` is bound; skip when the id is unchanged.
        if self.rel_allows != prev_allows {
            self.rebuild_allows_lease();
        }
        if self.rel_rule != prev_rule {
            self.rebuild_horn_lease();
            self.rebuild_rule_pin_lease();
        }
        if self.rel_policy != prev_policy {
            self.rebuild_policy_pin_lease();
        }
    }

    pub(crate) fn is_dictionary_rel(&self, rel: TermId) -> bool {
        self.dictionary_rels.contains(&rel)
    }

    pub(crate) fn store_blob_once(&mut self, digest: &str, bytes: &[u8]) {
        if self.blobs.contains_key(digest) {
            return;
        }
        self.blobs.insert(digest.to_string(), bytes.to_vec());
    }

    pub(crate) fn rebuild_blobs(&mut self, events: &[Event]) {
        self.blobs.clear();
        for e in events {
            if let Op::StoreBlob { digest, bytes } = &e.op {
                self.store_blob_once(digest, bytes);
            }
        }
    }

    pub(crate) fn rebuild_tt_anchors(&mut self, events: &[Event]) {
        self.tt_anchors.clear();
        for e in events {
            if let Op::TtAnchor { wall } = &e.op {
                self.tt_anchors.insert(*wall, e.ingested_at);
            }
        }
    }

    pub fn index_mode(&self) -> FoldIndexMode {
        self.index_mode
    }

    pub fn set_index_mode(&mut self, mode: FoldIndexMode) {
        if self.index_mode == mode {
            return;
        }
        self.index_mode = mode;
        match mode {
            FoldIndexMode::Eager => self.ensure_hot_maps(),
            FoldIndexMode::Lazy => {
                self.clear_hot_maps();
                self.hot_maps_warm = self.facts.is_empty();
            }
        }
    }

    pub fn facts(&self) -> &[Fact] {
        &self.facts
    }

    /// Reset the hot-path examine counter (integration tests; not fingerprint input).
    pub fn reset_hot_examine_count(&self) {
        self.hot_examine.reset();
    }

    /// Facts (or index slots) considered for liveness on the last hot reads since reset.
    pub fn hot_examine_count(&self) -> u64 {
        self.hot_examine.get()
    }

    /// Reset Horn-eval row visit counter (integration tests; not fingerprint input).
    #[doc(hidden)]
    pub fn reset_horn_examine_count(&self) {
        self.horn_examine.reset();
    }

    /// `live_horns` / `search_at` rows visited since reset (integration tests).
    #[doc(hidden)]
    pub fn horn_examine_count(&self) -> u64 {
        self.horn_examine.get()
    }

    /// Hint capacity for ingest index leases (bulk load / bench). Not SoT.
    pub(crate) fn reserve_ingest_indexes(&mut self, expected_facts: usize) {
        if expected_facts == 0 {
            return;
        }
        self.claim_facts.reserve(expected_facts);
        self.fact_by_event.reserve(expected_facts);
        self.live_triples.reserve(expected_facts);
        self.facts_by_relation.reserve(expected_facts);
    }

    fn index_slot(&mut self, idx: usize) {
        let fact = &self.facts[idx];
        self.vt_by_from
            .entry(fact.valid_from)
            .or_insert_with(|| Vec::with_capacity(4))
            .push(idx);
        // Plain relatedTo: one fact per minting event / claim id — SmallVec inline.
        self.claim_facts.entry(fact.claim_id).or_default().push(idx);
        self.fact_by_event
            .entry(fact.event_id)
            .or_default()
            .push(idx);
        if fact.invalidated_at.is_none() {
            self.live_triples
                .entry((fact.subject, fact.relation, fact.object()))
                .or_default()
                .push(idx);
        }
        let rel = fact.relation;
        self.facts_by_relation.entry(rel).or_default().push(idx);
    }

    fn index_allows_slot(&mut self, idx: usize) {
        let Some(rel) = self.rel_allows else {
            return;
        };
        let fact = &self.facts[idx];
        if fact.relation != rel {
            return;
        }
        let Some(name) = fact.delivery_key.as_ref() else {
            return;
        };
        self.allows_by_name
            .entry(name.clone())
            .or_default()
            .push(idx);
    }

    fn rebuild_allows_lease(&mut self) {
        self.allows_by_name.clear();
        if self.rel_allows.is_none() {
            return;
        }
        for idx in 0..self.facts.len() {
            self.index_allows_slot(idx);
        }
    }

    fn index_horn_slot(&mut self, idx: usize) {
        let Some(rel) = self.rel_rule else {
            return;
        };
        let fact = &self.facts[idx];
        if fact.relation != rel || fact.invalidated_at.is_some() {
            return;
        }
        let Some(pin) = fact.delivery_key.clone() else {
            return;
        };
        let Some(bytes) = self.blobs.get(&pin) else {
            return;
        };
        let Some(ast) = RuleAst::from_canonical(bytes) else {
            return;
        };
        let RuleAst::Horn { premises, .. } = &ast else {
            return;
        };
        let mut names = Vec::new();
        for p in premises {
            if !names.iter().any(|n| n == &p.relation) {
                names.push(p.relation.clone());
            }
        }
        for name in names {
            self.horns_by_premise
                .entry(name)
                .or_default()
                .push(HornLeaseSlot {
                    fact_idx: idx,
                    pin: pin.clone(),
                    ast: ast.clone(),
                });
        }
    }

    pub(crate) fn unindex_horn_slot(&mut self, idx: usize) {
        self.horns_by_premise.retain(|_, slots| {
            slots.retain(|s| s.fact_idx != idx);
            !slots.is_empty()
        });
    }

    fn rebuild_horn_lease(&mut self) {
        self.horns_by_premise.clear();
        self.horn_lease_ready = self.rel_rule.is_some();
        if self.rel_rule.is_none() {
            return;
        }
        for idx in 0..self.facts.len() {
            self.index_horn_slot(idx);
        }
    }

    fn index_rule_pin_slot(&mut self, idx: usize) {
        let Some(rel) = self.rel_rule else {
            return;
        };
        let fact = &self.facts[idx];
        if fact.relation != rel {
            return;
        }
        let Some(pin) = fact.delivery_key.as_ref() else {
            return;
        };
        self.rules_by_pin.entry(pin.clone()).or_default().push(idx);
    }

    fn rebuild_rule_pin_lease(&mut self) {
        self.rules_by_pin.clear();
        if self.rel_rule.is_none() {
            return;
        }
        for idx in 0..self.facts.len() {
            self.index_rule_pin_slot(idx);
        }
    }

    fn index_policy_pin_slot(&mut self, idx: usize) {
        let Some(rel) = self.rel_policy else {
            return;
        };
        let fact = &self.facts[idx];
        if fact.relation != rel {
            return;
        }
        let Some(pin) = fact.delivery_key.as_ref() else {
            return;
        };
        self.policies_by_pin
            .entry(pin.clone())
            .or_default()
            .push(idx);
        self.policy_by_ingest.push(idx);
    }

    fn rebuild_policy_pin_lease(&mut self) {
        self.policies_by_pin.clear();
        self.policy_by_ingest.clear();
        if self.rel_policy.is_none() {
            return;
        }
        for idx in 0..self.facts.len() {
            self.index_policy_pin_slot(idx);
        }
    }

    fn clear_hot_maps(&mut self) {
        self.vt_by_from.clear();
        self.claim_facts.clear();
        self.fact_by_event.clear();
        self.live_triples.clear();
        self.facts_by_relation.clear();
    }

    fn rebuild_hot_maps(&mut self) {
        self.clear_hot_maps();
        self.rebuild_allows_lease();
        self.rebuild_horn_lease();
        self.rebuild_rule_pin_lease();
        self.rebuild_policy_pin_lease();
        self.reserve_ingest_indexes(self.facts.len());
        for idx in 0..self.facts.len() {
            self.index_slot(idx);
        }
        self.hot_maps_warm = true;
    }

    /// Rebuild secondary leases if stale (Lazy) or missing after hydrate.
    pub fn ensure_hot_maps(&mut self) {
        if self.hot_maps_warm || self.facts.is_empty() {
            return;
        }
        self.rebuild_hot_maps();
    }

    fn push_fact(&mut self, fact: Fact) {
        let idx = self.facts.len();
        self.facts.push(fact);
        self.index_allows_slot(idx);
        self.index_horn_slot(idx);
        self.index_rule_pin_slot(idx);
        self.index_policy_pin_slot(idx);
        match self.index_mode {
            FoldIndexMode::Eager => {
                self.index_slot(idx);
                self.hot_maps_warm = true;
            }
            FoldIndexMode::Lazy => {
                self.hot_maps_warm = false;
            }
        }
    }

    pub fn apply(&mut self, event: &Event) {
        // Eager keeps leases warm; Lazy only warms when the op needs indexed lookups.
        let needs_indexes = matches!(
            &event.op,
            Op::Retract { .. }
                | Op::Correct { .. }
                | Op::CorrectInterval { .. }
                | Op::Behavior { .. }
        );
        if self.index_mode == FoldIndexMode::Eager || needs_indexes {
            self.ensure_hot_maps();
        }
        match &event.op {
            Op::Assert {
                subject,
                relation,
                object,
                valid_from,
                valid_to,
                claim,
                delivery_key,
                polarity,
            } => {
                let seq = self.next_seq;
                self.next_seq += 1;
                let claim_id = claim.unwrap_or(event.id);
                self.push_fact(Fact {
                    seq,
                    subject: *subject,
                    relation: *relation,
                    object: *object,
                    valid_from: *valid_from,
                    valid_to: *valid_to,
                    ingested_at: event.ingested_at,
                    invalidated_at: None,
                    claim_id,
                    event_id: event.id,
                    delivery_key: delivery_key.clone(),
                    polarity: *polarity,
                });
            }
            Op::Behavior {
                subject,
                relation,
                object,
                valid_from,
                valid_to,
                ..
            } => {
                let seq = self.next_seq;
                self.next_seq += 1;
                self.push_fact(Fact {
                    seq,
                    subject: *subject,
                    relation: *relation,
                    object: *object,
                    valid_from: *valid_from,
                    valid_to: *valid_to,
                    ingested_at: event.ingested_at,
                    invalidated_at: None,
                    claim_id: event.id,
                    event_id: event.id,
                    delivery_key: None,
                    polarity: None,
                });
            }
            Op::Retract { event_id } => {
                // Invalidate every live residual minted by this event (CorrectInterval
                // may leave prefix/patch/suffix rows sharing one event_id).
                let live_idxs = self.live_fact_indices_by_event_id(*event_id);
                for idx in live_idxs {
                    self.invalidate_fact_at(idx, event.ingested_at);
                }
            }
            Op::Correct {
                event_id,
                object,
                valid_from,
                valid_to,
            } => {
                // G-A02: multi-residual Correct invalidates every live row under
                // the id and mints one replacement (claim-level whole correct).
                let live_idxs = self.live_fact_indices_by_event_id(*event_id);
                if live_idxs.is_empty() {
                    return;
                }
                let head = live_idxs[0];
                let s = self.facts[head].subject;
                let r = self.facts[head].relation;
                let claim_id = self.facts[head].claim_id;
                let old_object = self.facts[head].object();
                let old_polarity = self.facts[head].polarity;
                let prior_key = self.facts[head].delivery_key.clone();
                for idx in live_idxs {
                    self.invalidate_fact_at(idx, event.ingested_at);
                }
                let seq = self.next_seq;
                self.next_seq += 1;
                self.push_fact(Fact {
                    seq,
                    subject: s,
                    relation: r,
                    object: *object,
                    valid_from: *valid_from,
                    valid_to: *valid_to,
                    ingested_at: event.ingested_at,
                    invalidated_at: None,
                    claim_id,
                    event_id: event.id,
                    delivery_key: prior_key,
                    polarity: polarity_for_row(old_polarity, old_object, *object),
                });
            }
            Op::CorrectInterval {
                event_id,
                object,
                patch_from,
                patch_to,
            } => {
                // Pipeline rejects multi-residual CI; fold still requires one live.
                let live = self.live_fact_indices_by_event_id(*event_id);
                if live.len() != 1 {
                    return;
                }
                let idx = live[0];
                if self.facts[idx].invalidated_at.is_some() {
                    return;
                }
                let old = &self.facts[idx];
                let Some((inter_from, inter_to)) =
                    vt_intersect(old.valid_from, old.valid_to, *patch_from, *patch_to)
                else {
                    return;
                };
                let (prefix, suffix) =
                    leftover_halves(old.valid_from, old.valid_to, inter_from, inter_to);
                let s = old.subject;
                let r = old.relation;
                let claim_id = old.claim_id;
                let old_object = old.object();
                let old_polarity = old.polarity;
                let prior_key = old.delivery_key.clone();
                self.invalidate_fact_at(idx, event.ingested_at);

                let mut push_row = |obj: TermId, vf: u64, vt: Option<u64>| {
                    let seq = self.next_seq;
                    self.next_seq += 1;
                    self.push_fact(Fact {
                        seq,
                        subject: s,
                        relation: r,
                        object: obj,
                        valid_from: vf,
                        valid_to: vt,
                        ingested_at: event.ingested_at,
                        invalidated_at: None,
                        claim_id,
                        event_id: event.id,
                        delivery_key: prior_key.clone(),
                        polarity: polarity_for_row(old_polarity, old_object, obj),
                    });
                };
                if let Some((vf, vt)) = prefix {
                    push_row(old_object, vf, vt);
                }
                push_row(*object, inter_from, inter_to);
                if let Some((vf, vt)) = suffix {
                    push_row(old_object, vf, vt);
                }
            }
            Op::StoreBlob { digest, bytes } => {
                self.store_blob_once(digest, bytes);
            }
            Op::TtAnchor { wall } => {
                self.tt_anchors.insert(*wall, event.ingested_at);
            }
            Op::Define { .. } | Op::QuantumOutcome { .. } | Op::JustificationCite { .. } => {}
        }
    }

    /// Rebuild fold facts from `events` without binding reserved `kutha:*` ids.
    ///
    /// Fingerprint-safe (fingerprint walks `facts` directly). Product queries
    /// (`as_of` / `known_at`) need [`Self::replay_with_dict`] or a later
    /// [`Self::bind_reserved`] — otherwise dictionary_rels stay empty and
    /// genesis allows leak into the product picture (CE review P1 #10).
    pub fn replay(events: &[Event]) -> Self {
        let mut fold = Self::default();
        for e in events {
            fold.apply(e);
        }
        fold
    }

    /// Replay then bind reserved relations from the live term dictionary.
    pub fn replay_with_dict(events: &[Event], dict: &TermDictionary) -> Self {
        let mut fold = Self::replay(events);
        fold.bind_reserved(dict);
        fold
    }

    /// Test-only: mutate the picture without a log event (must fail replay).
    #[cfg(test)]
    pub(crate) fn tamper_invalidate_first(&mut self) {
        if !self.facts.is_empty() {
            self.invalidate_fact_at(0, 999);
        }
    }
}
