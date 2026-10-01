use kutha_common::{
    Event, EventId, Op, SupportPolarity, TermDictionary, TermId, TransactionTime, ValidTime,
    KUTHA_ACTION, KUTHA_ADMISSION, KUTHA_ALLOWS, KUTHA_POLICY, KUTHA_RULE,
};
use rustc_hash::FxHashMap;
use sha2::{Digest, Sha256};
use smallvec::SmallVec;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};

/// Ingest lookup leases: Fx hasher (trusted EventId/TermId; Samyama-style).
type IngestMap<K, V> = FxHashMap<K, V>;
/// Typical relatedTo mint: one fact per claim/event — inline without heap (Pogocache-ish).
type FactIdxList = SmallVec<[usize; 1]>;

/// Integration-test examine counter (not fingerprint / JSON). `cfg(test)` is off for `tests/*.rs`.
#[derive(Debug, Default)]
struct ExamineCounter(AtomicU64);

impl Clone for ExamineCounter {
    fn clone(&self) -> Self {
        Self(AtomicU64::new(self.0.load(Ordering::Relaxed)))
    }
}

impl ExamineCounter {
    fn reset(&self) {
        self.0.store(0, Ordering::Relaxed);
    }

    fn get(&self) -> u64 {
        self.0.load(Ordering::Relaxed)
    }

    fn bump(&self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

fn nil_claim() -> EventId {
    EventId::nil()
}

/// Half-open intersection of `[a_from, a_to)` and `[b_from, b_to)`. `None` end is +∞.
/// Empty when `from` is not strictly less than `to` after substituting infinity (D-C7).
pub(crate) fn vt_intersect(
    a_from: ValidTime,
    a_to: Option<ValidTime>,
    b_from: ValidTime,
    b_to: Option<ValidTime>,
) -> Option<(ValidTime, Option<ValidTime>)> {
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
fn leftover_halves(
    fact_from: ValidTime,
    fact_to: Option<ValidTime>,
    inter_from: ValidTime,
    inter_to: Option<ValidTime>,
) -> (
    Option<(ValidTime, Option<ValidTime>)>,
    Option<(ValidTime, Option<ValidTime>)>,
) {
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
    pub valid_from: ValidTime,
    pub valid_to: Option<ValidTime>,
    pub ingested_at: TransactionTime,
    pub invalidated_at: Option<TransactionTime>,
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

    pub fn is_live_at(&self, tt: TransactionTime, vt: ValidTime) -> bool {
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
}

/// How fold secondary index leases are maintained on the write path.
///
/// - [`FoldIndexMode::Eager`]: warm maps on every fact push (accept write-cost; default).
/// - [`FoldIndexMode::Lazy`]: thin write — append facts only; rebuild or scan on demand.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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
enum LiveFactFilter {
    Product,
    Dictionary,
    All,
}

impl LiveFactFilter {
    fn keep(self, fold: &GraphFold, fact: &Fact) -> bool {
        let dict = fold.is_dictionary_rel(fact.relation);
        match self {
            Self::Product => !dict,
            Self::Dictionary => dict,
            Self::All => true,
        }
    }
}

/// Deterministic fold of the log. Droppable picture, not SoT.
/// Hot maps are leases: skip-serialized and rebuilt from `facts` (D-02).
#[derive(Clone, Debug, serde::Serialize)]
pub struct GraphFold {
    facts: Vec<Fact>,
    next_seq: u64,
    /// Hot-path examine count. Not SoT; skipped in snapshot JSON.
    #[serde(skip)]
    hot_examine: ExamineCounter,
    /// Ingest index policy (not fingerprint / JSON).
    #[serde(skip)]
    index_mode: FoldIndexMode,
    /// Whether secondary maps match `facts` (Lazy may leave this false).
    #[serde(skip)]
    hot_maps_warm: bool,
    /// Fact indices grouped by `valid_from` so a cut can skip future-start decoys.
    #[serde(skip)]
    vt_by_from: BTreeMap<ValidTime, Vec<usize>>,
    /// Fact indices grouped by portable `claim_id`.
    #[serde(skip)]
    claim_facts: IngestMap<EventId, FactIdxList>,
    /// Fact indices grouped by minting `event_id` (ingest lookup lease).
    #[serde(skip)]
    fact_by_event: IngestMap<EventId, FactIdxList>,
    /// Live (subject, relation, object) triples for residual cascade checks.
    #[serde(skip)]
    live_triples: IngestMap<(TermId, TermId, TermId), usize>,
    /// Interned `kutha:*` relation TermIds (D-05). Rebuilt from the dictionary; not fingerprint.
    #[serde(skip)]
    dictionary_rels: HashSet<TermId>,
    #[serde(skip)]
    rel_allows: Option<TermId>,
    #[serde(skip)]
    rel_rule: Option<TermId>,
    #[serde(skip)]
    rel_policy: Option<TermId>,
    #[serde(skip)]
    rel_admission: Option<TermId>,
    #[serde(skip)]
    rel_action: Option<TermId>,
    /// Content-addressed bodies from `Op::StoreBlob` (store-once). Not fingerprint input.
    #[serde(skip)]
    blobs: HashMap<String, Vec<u8>>,
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
            facts: Vec<Fact>,
            next_seq: u64,
        }
        let de = GraphFoldDe::deserialize(deserializer)?;
        let mut fold = GraphFold::with_index_mode(FoldIndexMode::from_env());
        fold.facts = de.facts;
        fold.next_seq = de.next_seq;
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
            index_mode,
            hot_maps_warm: true,
            vt_by_from: BTreeMap::new(),
            claim_facts: IngestMap::default(),
            fact_by_event: IngestMap::default(),
            live_triples: IngestMap::default(),
            dictionary_rels: HashSet::new(),
            rel_allows: None,
            rel_rule: None,
            rel_policy: None,
            rel_admission: None,
            rel_action: None,
            blobs: HashMap::new(),
        }
    }

    /// Bind reserved `kutha:*` relation TermIds from the live dictionary (D-01 / D-05).
    pub fn bind_reserved(&mut self, dict: &TermDictionary) {
        self.dictionary_rels.clear();
        for (i, s) in dict.strings().iter().enumerate() {
            if s.starts_with("kutha:") {
                self.dictionary_rels.insert(i as TermId);
            }
        }
        self.rel_allows = dict.id(KUTHA_ALLOWS);
        self.rel_rule = dict.id(KUTHA_RULE);
        self.rel_policy = dict.id(KUTHA_POLICY);
        self.rel_admission = dict.id(KUTHA_ADMISSION);
        self.rel_action = dict.id(KUTHA_ACTION);
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

    /// Hint capacity for ingest index leases (bulk load / bench). Not SoT.
    pub(crate) fn reserve_ingest_indexes(&mut self, expected_facts: usize) {
        if expected_facts == 0 {
            return;
        }
        self.claim_facts.reserve(expected_facts);
        self.fact_by_event.reserve(expected_facts);
        self.live_triples.reserve(expected_facts);
    }

    fn index_slot(&mut self, idx: usize) {
        let fact = &self.facts[idx];
        self.vt_by_from
            .entry(fact.valid_from)
            .or_insert_with(|| Vec::with_capacity(4))
            .push(idx);
        // Plain relatedTo: one fact per minting event / claim id — SmallVec inline.
        self.claim_facts
            .entry(fact.claim_id)
            .or_default()
            .push(idx);
        self.fact_by_event
            .entry(fact.event_id)
            .or_default()
            .push(idx);
        if fact.invalidated_at.is_none() {
            self.live_triples
                .insert((fact.subject, fact.relation, fact.object()), idx);
        }
    }

    fn clear_hot_maps(&mut self) {
        self.vt_by_from.clear();
        self.claim_facts.clear();
        self.fact_by_event.clear();
        self.live_triples.clear();
    }

    fn rebuild_hot_maps(&mut self) {
        self.clear_hot_maps();
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

    /// Live facts at an explicit cut. Uses VT lease when warm; otherwise scans `facts`.
    /// Product picture: `kutha:` dictionary relations are excluded (D-05).
    pub(crate) fn live_facts_at(&self, tt: TransactionTime, vt: ValidTime) -> Vec<&Fact> {
        self.collect_live_facts_at(tt, vt, LiveFactFilter::Product)
    }

    /// All live facts including dictionary relations (fingerprint/replay walks `facts` directly).
    fn all_live_facts_at(&self, tt: TransactionTime, vt: ValidTime) -> Vec<&Fact> {
        self.collect_live_facts_at(tt, vt, LiveFactFilter::All)
    }

    fn dictionary_live_facts_at(&self, tt: TransactionTime, vt: ValidTime) -> Vec<&Fact> {
        self.collect_live_facts_at(tt, vt, LiveFactFilter::Dictionary)
    }

    fn collect_live_facts_at(
        &self,
        tt: TransactionTime,
        vt: ValidTime,
        filter: LiveFactFilter,
    ) -> Vec<&Fact> {
        let keep = |f: &Fact| filter.keep(self, f);
        if !self.hot_maps_warm {
            return self
                .facts
                .iter()
                .filter(|f| {
                    self.hot_examine.bump();
                    f.is_live_at(tt, vt) && keep(f)
                })
                .collect();
        }
        self.vt_by_from
            .range(..=vt)
            .flat_map(|(_, idxs)| idxs.iter().copied())
            .filter_map(|idx| {
                self.hot_examine.bump();
                let fact = &self.facts[idx];
                (fact.is_live_at(tt, vt) && keep(fact)).then_some(fact)
            })
            .collect()
    }

    pub fn live_count(&self, tt: TransactionTime, vt: ValidTime) -> usize {
        self.live_at(tt, vt).len()
    }

    /// Live interned triples at an explicit transaction-time × valid-time cut.
    /// There is no “now” default (FF5 / MemStrata).
    pub fn live_at(&self, tt: TransactionTime, vt: ValidTime) -> Vec<(TermId, TermId, TermId)> {
        self.live_facts_at(tt, vt)
            .into_iter()
            .map(|f| (f.subject, f.relation, f.object()))
            .collect()
    }

    /// AS OF valid-time on the current transaction picture (`tt = MAX`).
    /// Product picture: dictionary (`kutha:`) relations are excluded (D-05).
    pub fn as_of(&self, vt: ValidTime) -> Vec<(TermId, TermId, TermId)> {
        self.live_at(u64::MAX, vt)
    }

    /// Dictionary entries at a valid-time cut (kutha: relations only).
    pub fn dictionary_as_of(&self, vt: ValidTime) -> Vec<(TermId, TermId, TermId)> {
        self.dictionary_live_at(u64::MAX, vt)
    }

    pub fn dictionary_live_at(
        &self,
        tt: TransactionTime,
        vt: ValidTime,
    ) -> Vec<(TermId, TermId, TermId)> {
        self.dictionary_live_facts_at(tt, vt)
            .into_iter()
            .map(|f| (f.subject, f.relation, f.object()))
            .collect()
    }

    pub fn fingerprint(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        for f in &self.facts {
            h.update(f.seq.to_le_bytes());
            h.update(f.subject.to_le_bytes());
            h.update(f.relation.to_le_bytes());
            h.update(f.object.to_le_bytes());
            h.update(f.valid_from.to_le_bytes());
            h.update(f.valid_to.unwrap_or(u64::MAX).to_le_bytes());
            h.update(f.ingested_at.to_le_bytes());
            h.update(f.invalidated_at.unwrap_or(u64::MAX).to_le_bytes());
            h.update(f.claim_id.as_bytes());
            h.update(f.event_id.as_bytes());
            match &f.delivery_key {
                Some(k) => {
                    h.update(b"dkey");
                    h.update((k.len() as u64).to_le_bytes());
                    h.update(k.as_bytes());
                }
                None => h.update(b"no-dkey"),
            }
            h.update(match f.polarity {
                None => b"pol-none".as_slice(),
                Some(SupportPolarity::Positive) => b"pol-pos",
                Some(SupportPolarity::Negative) => b"pol-neg",
            });
        }
        h.finalize().into()
    }

    /// First Fact minted (or residual-replaced) by `event_id`. Prefer a live row.
    pub(crate) fn fact_index_by_event_id(&self, event_id: EventId) -> Option<usize> {
        if !self.hot_maps_warm {
            return self
                .facts
                .iter()
                .enumerate()
                .filter(|(_, f)| {
                    self.hot_examine.bump();
                    f.event_id == event_id
                })
                .map(|(i, _)| i)
                .find(|&i| self.facts[i].invalidated_at.is_none())
                .or_else(|| {
                    self.facts
                        .iter()
                        .rposition(|f| f.event_id == event_id)
                });
        }
        let idxs = self.fact_by_event.get(&event_id)?;
        idxs.iter()
            .copied()
            .find(|&i| self.facts[i].invalidated_at.is_none())
            .or_else(|| idxs.last().copied())
    }

    /// Whether any Fact (live or not) carries this claim id.
    pub(crate) fn claim_known(&self, claim: EventId) -> bool {
        if !self.hot_maps_warm {
            return self.facts.iter().any(|f| {
                self.hot_examine.bump();
                f.claim_id == claim
            });
        }
        self.claim_facts.contains_key(&claim)
    }

    /// Live triple presence for residual cascade checks (knows → knownBy).
    pub(crate) fn has_live_triple(
        &self,
        subject: TermId,
        relation: TermId,
        object: TermId,
    ) -> bool {
        if !self.hot_maps_warm {
            return self.facts.iter().any(|f| {
                self.hot_examine.bump();
                f.invalidated_at.is_none()
                    && f.subject == subject
                    && f.relation == relation
                    && f.object() == object
            });
        }
        match self.live_triples.get(&(subject, relation, object)) {
            Some(&idx) => self.facts[idx].invalidated_at.is_none(),
            None => false,
        }
    }

    /// Mark a fact slot invalidated and drop it from the live-triple lease.
    fn invalidate_fact_at(&mut self, idx: usize, at: TransactionTime) {
        self.facts[idx].invalidated_at = Some(at);
        let fact = &self.facts[idx];
        let key = (fact.subject, fact.relation, fact.object());
        if self.live_triples.get(&key) == Some(&idx) {
            self.live_triples.remove(&key);
        }
    }

    /// Live Facts that support `claim` at an explicit cut (product facts only, D-05).
    pub fn live_supports(&self, claim: EventId, tt: TransactionTime, vt: ValidTime) -> Vec<&Fact> {
        if !self.hot_maps_warm {
            return self
                .facts
                .iter()
                .filter(|f| {
                    self.hot_examine.bump();
                    f.claim_id == claim
                        && f.is_live_at(tt, vt)
                        && !self.is_dictionary_rel(f.relation)
                })
                .collect();
        }
        let Some(idxs) = self.claim_facts.get(&claim) else {
            return Vec::new();
        };
        idxs.iter()
            .filter_map(|&idx| {
                self.hot_examine.bump();
                let fact = &self.facts[idx];
                (fact.is_live_at(tt, vt) && !self.is_dictionary_rel(fact.relation)).then_some(fact)
            })
            .collect()
    }

    pub fn live_support_count(&self, claim: EventId, tt: TransactionTime, vt: ValidTime) -> usize {
        self.live_supports(claim, tt, vt).len()
    }

    pub fn claim_supported_at(&self, claim: EventId, tt: TransactionTime, vt: ValidTime) -> bool {
        self.live_support_count(claim, tt, vt) > 0
    }

    /// Fold-cut allowlist: a live `kutha:allows` fact whose delivery_key is `name`.
    pub fn relation_allowed_at(&self, name: &str, tt: TransactionTime, vt: ValidTime) -> bool {
        let Some(rel) = self.rel_allows else {
            return false;
        };
        self.all_live_facts_at(tt, vt)
            .into_iter()
            .any(|f| f.relation == rel && f.delivery_key.as_deref() == Some(name))
    }

    /// Fold-cut rule registry: a live `kutha:rule` fact whose delivery_key is `pin`.
    pub fn rule_hash_live_at(&self, pin: &str, tt: TransactionTime, vt: ValidTime) -> bool {
        let Some(rel) = self.rel_rule else {
            return false;
        };
        self.all_live_facts_at(tt, vt)
            .into_iter()
            .any(|f| f.relation == rel && f.delivery_key.as_deref() == Some(pin))
    }

    /// Fold-cut policy pin: a live `kutha:policy` fact whose delivery_key is `pin`.
    pub fn policy_hash_live_at(&self, pin: &str, tt: TransactionTime, vt: ValidTime) -> bool {
        let Some(rel) = self.rel_policy else {
            return false;
        };
        self.all_live_facts_at(tt, vt)
            .into_iter()
            .any(|f| f.relation == rel && f.delivery_key.as_deref() == Some(pin))
    }

    /// Latest ingested live policy hash at `(tt, vt)`.
    pub fn live_policy_pin_at(&self, tt: TransactionTime, vt: ValidTime) -> Option<String> {
        let rel = self.rel_policy?;
        self.all_live_facts_at(tt, vt)
            .into_iter()
            .filter(|f| f.relation == rel)
            .max_by_key(|f| f.ingested_at)
            .and_then(|f| f.delivery_key.clone())
    }

    /// Latest ingested live admission status for `justification_id` at `(tt, vt)`.
    pub fn admission_status_at(
        &self,
        justification_id: &str,
        tt: TransactionTime,
        vt: ValidTime,
    ) -> Option<bool> {
        self.live_admission_fact(justification_id, tt, vt)
            .map(|f| f.polarity == Some(SupportPolarity::Positive))
    }

    pub(crate) fn live_admission_at(
        &self,
        justification_id: &str,
        tt: TransactionTime,
        vt: ValidTime,
    ) -> Option<(bool, String)> {
        let f = self.live_admission_fact(justification_id, tt, vt)?;
        let admitted = f.polarity == Some(SupportPolarity::Positive);
        let policy = f.delivery_key.clone().unwrap_or_default();
        Some((admitted, policy))
    }

    fn live_admission_fact(
        &self,
        justification_id: &str,
        tt: TransactionTime,
        vt: ValidTime,
    ) -> Option<&Fact> {
        let rel = self.rel_admission?;
        if let Ok(eid) = justification_id.parse::<EventId>() {
            if let Some(idx) = self.fact_index_by_event_id(eid) {
                let f = &self.facts[idx];
                if f.relation == rel && f.is_live_at(tt, vt) {
                    return Some(f);
                }
            }
        }
        self.all_live_facts_at(tt, vt)
            .into_iter()
            .filter(|f| f.relation == rel && f.delivery_key.as_deref() == Some(justification_id))
            .max_by_key(|f| f.ingested_at)
    }

    pub(crate) fn live_action_fact(
        &self,
        justification_id: &str,
        tt: TransactionTime,
        vt: ValidTime,
    ) -> Option<&Fact> {
        let rel = self.rel_action?;
        self.all_live_facts_at(tt, vt)
            .into_iter()
            .filter(|f| f.relation == rel && f.delivery_key.as_deref() == Some(justification_id))
            .max_by_key(|f| f.ingested_at)
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
                if let Some(idx) = self.fact_index_by_event_id(*event_id) {
                    if self.facts[idx].invalidated_at.is_none() {
                        self.invalidate_fact_at(idx, event.ingested_at);
                    }
                }
            }
            Op::Correct {
                event_id,
                object,
                valid_from,
                valid_to,
            } => {
                if let Some(idx) = self.fact_index_by_event_id(*event_id) {
                    if self.facts[idx].invalidated_at.is_none() {
                        let s = self.facts[idx].subject;
                        let r = self.facts[idx].relation;
                        let claim_id = self.facts[idx].claim_id;
                        let old_object = self.facts[idx].object();
                        let old_polarity = self.facts[idx].polarity;
                        self.invalidate_fact_at(idx, event.ingested_at);
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
                            delivery_key: None,
                            polarity: polarity_for_row(old_polarity, old_object, *object),
                        });
                    }
                }
            }
            Op::CorrectInterval {
                event_id,
                object,
                patch_from,
                patch_to,
            } => {
                let Some(idx) = self.fact_index_by_event_id(*event_id) else {
                    return;
                };
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
                self.invalidate_fact_at(idx, event.ingested_at);

                let mut push_row = |obj: TermId, vf: ValidTime, vt: Option<ValidTime>| {
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
                        delivery_key: None,
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
            Op::Define { .. } | Op::QuantumOutcome { .. } | Op::JustificationCite { .. } => {}
        }
    }

    pub fn replay(events: &[Event]) -> Self {
        let mut fold = Self::default();
        for e in events {
            fold.apply(e);
        }
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
