//! One write path: propose → admit → commit → receipt (D-01).
use crate::fold::Fact;
use crate::quantum::{PersistedQuantumOutcome, QuantumOutcome, Runtime, RuntimeError, disposition};
use crate::receipt::{QuantumReceipt, digest_to_hex};
use kutha_common::{
    Event, EventId, KUTHA_ACTION, KUTHA_ADMISSION, KUTHA_ALLOWS, KUTHA_POLICY, KUTHA_RULE, Op,
    OutcomeDisposition, RuleAst, WallTime, blob_digest_matches, policy_version_hash,
};

/// Rebuild a prefix Assert Event that the sealed-prefix rewrite dropped.
fn event_from_assert_fact(fact: &Fact) -> Event {
    let claim = if fact.claim_id == fact.event_id {
        None
    } else {
        Some(fact.claim_id)
    };
    let op = Op::Assert {
        subject: fact.subject,
        relation: fact.relation,
        object: fact.object(),
        valid_from: fact.valid_from,
        valid_to: fact.valid_to,
        claim,
        delivery_key: fact.delivery_key.clone(),
        polarity: fact.polarity,
    };
    Event {
        id: fact.event_id,
        ingested_at: fact.ingested_at,
        op,
        object_ids: vec![fact.subject, fact.relation, fact.object()],
    }
}

/// How the shared pipeline finishes a committed quantum (D-01 / D-07).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ReceiptMode {
    /// User `emit`: cascade + persist `Op::QuantumOutcome` on the log.
    User,
    /// Dictionary constructors: admit+commit only (named oracles keep +1/+2 log growth).
    Dictionary,
    /// Justification / action: admit+commit + buffer receipt (no extra log QO).
    Typed,
}

impl Runtime {
    pub(crate) fn rebuild_log_indexes(&mut self) {
        self.by_event_id.clear();
        self.by_delivery_key.clear();
        self.derived_by_cause.clear();
        self.cause_of_behavior.clear();
        self.live_behaviors.clear();
        self.behaviors_by_premise_rel.clear();
        self.premise_rels_of_behavior.clear();
        self.behaviors_by_premise_fact.clear();
        self.premise_facts_of_behavior.clear();
        self.compacted_behavior_by_id.clear();
        self.compacted_assert_by_id.clear();
        let n = self.log.len();
        self.by_event_id.reserve(n);
        self.by_delivery_key.reserve(n);
        for (i, e) in self.log.as_slice().iter().enumerate() {
            self.by_event_id.insert(e.id, i);
            if let Op::Assert {
                relation,
                delivery_key: Some(k),
                ..
            } = &e.op
            {
                if !k.is_empty() {
                    self.by_delivery_key.insert((*relation, k.clone()), e.id);
                }
            }
        }
        for e in &self.fold.compacted_behaviors {
            self.compacted_behavior_by_id.insert(e.id, e.clone());
        }
        // Prefix Asserts were dropped from the rewritten log. Restored facts
        // still carry delivery_key + event_id; fill leases the log walk missed
        // (or_insert: tail log Asserts stay last-wins).
        for fact in self.fold.facts() {
            if let Some(k) = fact.delivery_key.as_ref() {
                if !k.is_empty() {
                    self.by_delivery_key
                        .entry((fact.relation, k.clone()))
                        .or_insert(fact.event_id);
                }
            }
            if self.by_event_id.contains_key(&fact.event_id)
                || self.compacted_behavior_by_id.contains_key(&fact.event_id)
            {
                continue;
            }
            self.compacted_assert_by_id
                .insert(fact.event_id, event_from_assert_fact(fact));
        }
        // Only live Behavior heads belong in retract/unindex leases. Dead rows
        // stay on the log (SoT) or in compacted_behavior_by_id but must not
        // reappear after open/rebuild (M110). Prefix compacted records first,
        // then log (tail). Index every minting row even when DET-02 reused the
        // head EventId for a second caused_by (same as append_indexed).
        let compacted = self.fold.compacted_behaviors.clone();
        for e in &compacted {
            self.index_live_behavior_event(e);
        }
        for i in 0..self.log.len() {
            let e = self.log.as_slice()[i].clone();
            self.index_live_behavior_event(&e);
        }
    }

    fn index_live_behavior_event(&mut self, e: &Event) {
        let Op::Behavior {
            caused_by,
            rule_version,
            ..
        } = &e.op
        else {
            return;
        };
        if !self.behavior_fact_live(e.id) {
            return;
        }
        self.record_derived_by_cause(e.id, *caused_by);
        self.record_cause_of_behavior(e.id, *caused_by);
        self.live_behaviors.insert(e.id);
        self.index_behavior_premise_rels(e.id, rule_version);
        self.index_behavior_premise_facts(e);
    }

    fn index_behavior_premise_rels(&mut self, behavior_id: EventId, rule_version: &str) {
        let Some(bytes) = self.fold.blobs.get(rule_version) else {
            return;
        };
        let Some(RuleAst::Horn { premises, .. }) = RuleAst::from_canonical(bytes) else {
            return;
        };
        let mut seen: Vec<kutha_common::TermId> = Vec::new();
        for p in &premises {
            let Some(rel) = self.dict.id(&p.relation) else {
                continue;
            };
            if seen.iter().any(|r| *r == rel) {
                continue;
            }
            seen.push(rel);
            self.behaviors_by_premise_rel
                .entry(rel)
                .or_default()
                .push(behavior_id);
            self.premise_rels_of_behavior
                .entry(behavior_id)
                .or_default()
                .push(rel);
        }
    }

    fn index_behavior_premise_facts(&mut self, e: &Event) {
        let ids = self.supporting_premise_ids_for_behavior(e);
        self.index_behavior_premise_fact_ids(e.id, ids);
    }

    fn index_behavior_premise_fact_ids(&mut self, behavior_id: EventId, ids: Vec<EventId>) {
        if ids.is_empty() {
            return;
        }
        self.premise_facts_of_behavior
            .entry(behavior_id)
            .or_default()
            .extend(ids.iter().copied());
        for pid in ids {
            self.behaviors_by_premise_fact
                .entry(pid)
                .or_default()
                .push(behavior_id);
        }
    }

    fn record_derived_by_cause(&mut self, id: EventId, caused_by: EventId) {
        let kids = self.derived_by_cause.entry(caused_by).or_default();
        if !kids.contains(&id) {
            kids.push(id);
        }
    }

    fn record_cause_of_behavior(&mut self, id: EventId, caused_by: EventId) {
        let causes = self.cause_of_behavior.entry(id).or_default();
        if !causes.contains(&caused_by) {
            causes.push(caused_by);
        }
    }

    fn unindex_behavior(&mut self, id: EventId) {
        let tracked = self.cause_of_behavior.contains_key(&id)
            || self.premise_rels_of_behavior.contains_key(&id)
            || self.premise_facts_of_behavior.contains_key(&id);
        if !tracked {
            return;
        }
        if self.live_behaviors.remove(&id) {
            self.unindex_live_examine.bump();
        }
        // Reverse pointers are authoritative (M104/M105). Missing keys mean the
        // id was never in that lease — do not fall back to a full-bucket scan.
        // Walk every stored cause: DET-02 can share one EventId across Horn paths.
        if let Some(causes) = self.cause_of_behavior.remove(&id) {
            for cause in causes {
                self.unindex_cause_examine.bump();
                if let Some(kids) = self.derived_by_cause.get_mut(&cause) {
                    kids.retain(|x| *x != id);
                    if kids.is_empty() {
                        self.derived_by_cause.remove(&cause);
                    }
                }
            }
        }
        if let Some(rels) = self.premise_rels_of_behavior.remove(&id) {
            for rel in rels {
                self.unindex_premise_examine.bump();
                if let Some(kids) = self.behaviors_by_premise_rel.get_mut(&rel) {
                    kids.retain(|x| *x != id);
                    if kids.is_empty() {
                        self.behaviors_by_premise_rel.remove(&rel);
                    }
                }
            }
        }
        if let Some(facts) = self.premise_facts_of_behavior.remove(&id) {
            for pid in facts {
                if let Some(kids) = self.behaviors_by_premise_fact.get_mut(&pid) {
                    kids.retain(|x| *x != id);
                    if kids.is_empty() {
                        self.behaviors_by_premise_fact.remove(&pid);
                    }
                }
            }
        }
    }

    pub(crate) fn append_indexed(&mut self, event: Event) {
        let pos = self.log.len();
        self.by_event_id.insert(event.id, pos);
        // Every append path reaches here, including `intern`'s `Op::Define`,
        // so the visible-log lease stays a faithful mirror.
        self.log_ids.insert(event.id);
        if let Op::Assert {
            relation,
            delivery_key: Some(k),
            ..
        } = &event.op
        {
            if !k.is_empty() {
                self.by_delivery_key
                    .insert((*relation, k.clone()), event.id);
            }
        }
        match &event.op {
            Op::Behavior {
                caused_by,
                rule_version,
                ..
            } => {
                self.record_derived_by_cause(event.id, *caused_by);
                self.record_cause_of_behavior(event.id, *caused_by);
                self.live_behaviors.insert(event.id);
                self.index_behavior_premise_rels(event.id, rule_version);
                self.index_behavior_premise_facts(&event);
            }
            Op::Retract { event_id }
            | Op::Correct { event_id, .. }
            | Op::CorrectInterval { event_id, .. } => {
                let rid = *event_id;
                self.unindex_behavior(rid);
            }
            _ => {}
        }
        self.log.append(event);
    }

    pub(crate) fn event_by_id(&self, id: kutha_common::EventId) -> Option<&Event> {
        self.ingest_examine.bump();
        if let Some(pos) = self.by_event_id.get(&id) {
            return self.log.as_slice().get(*pos);
        }
        self.compacted_behavior_by_id
            .get(&id)
            .or_else(|| self.compacted_assert_by_id.get(&id))
    }

    pub(crate) fn next_tt(&mut self) -> u64 {
        let t = self.next_tt;
        self.next_tt += 1;
        t
    }

    pub(crate) fn commit_event(&mut self, event: Event) {
        self.fold.apply(&event);
        // Keep the droppable operator index in step with the log so admission
        // sees the same names replay will.
        match &event.op {
            Op::EnrichmentProposal { cache_key, .. } => {
                self.enrichment_cache_keys.insert(cache_key.clone());
            }
            Op::PackInstall { .. } | Op::PackActivate { .. } | Op::PackRetire { .. } => {
                let _ = self.lifecycle.replay(&event.op);
                if let Op::PackRetire { pack_id, .. } = &event.op {
                    self.operators.retire(pack_id);
                } else if let Op::PackInstall {
                    pack_id,
                    manifest_digest: Some(_),
                    ..
                } = &event.op
                {
                    if let Ok(names) =
                        crate::pack_manifest::declared_operators_for(&self.fold.blobs, &event.op)
                    {
                        let _ = self.operators.declare(pack_id, names);
                    }
                }
            }
            _ => {}
        }
        self.append_indexed(event);
    }

    fn outcome_op(row: &PersistedQuantumOutcome) -> Op {
        Op::QuantumOutcome {
            quantum_id: row.quantum_id.clone(),
            disposition: row.disposition,
            aborted_on_budget: row.aborted_on_budget,
            events_in_quantum: row.events_in_quantum,
            event_ids: row.event_ids.clone(),
            receipt_digest_hex: row.receipt_digest_hex.clone(),
            resume_of: row.resume_of.clone(),
        }
    }

    fn reject_forged_meta(&self, op: &Op) -> Result<(), RuntimeError> {
        if matches!(
            op,
            Op::QuantumOutcome { .. }
                | Op::JustificationCite { .. }
                | Op::Checkpoint { .. }
                | Op::PackInstall { .. }
                | Op::PackActivate { .. }
                | Op::PackRetire { .. }
                | Op::EnrichmentProposal { .. }
        ) {
            return Err(RuntimeError::MetaOpRejected);
        }
        if let Op::Assert { relation, .. } = op {
            if self
                .dict
                .lookup(*relation)
                .is_some_and(|n| n == KUTHA_ADMISSION || n == KUTHA_ACTION)
            {
                return Err(RuntimeError::MetaOpRejected);
            }
        }
        Ok(())
    }

    fn gate_user_op(&self, op: &Op) -> Result<(), RuntimeError> {
        // G-A05: Retract/Correct require at least one *live* residual. Dead ids
        // are UnknownFact (fail-closed), not a silent fold no-op.
        if let Op::Retract { event_id } = op {
            self.ingest_examine.bump();
            if self
                .fold
                .live_fact_indices_by_event_id(*event_id)
                .is_empty()
            {
                return Err(RuntimeError::UnknownFact {
                    event_id: *event_id,
                });
            }
        } else if let Op::Correct { event_id, .. } | Op::CorrectInterval { event_id, .. } = op {
            self.ingest_examine.bump();
            if self
                .fold
                .live_fact_indices_by_event_id(*event_id)
                .is_empty()
            {
                return Err(RuntimeError::UnknownFact {
                    event_id: *event_id,
                });
            }
        }
        if let Op::CorrectInterval {
            event_id,
            patch_from,
            patch_to,
            ..
        } = op
        {
            self.ingest_examine.bump();
            // G-A02: interval patch targets exactly one live VT window.
            let live = self.fold.live_fact_indices_by_event_id(*event_id);
            if live.len() != 1 {
                return Err(RuntimeError::IntervalPatchRejected {
                    event_id: *event_id,
                });
            }
            let idx = live[0];
            let fact = &self.fold.facts()[idx];
            if fact.invalidated_at.is_some()
                || patch_to.is_some_and(|t| *patch_from >= t)
                || crate::fold::vt_intersect(fact.valid_from, fact.valid_to, *patch_from, *patch_to)
                    .is_none()
            {
                return Err(RuntimeError::IntervalPatchRejected {
                    event_id: *event_id,
                });
            }
        }
        if let Op::Behavior {
            rule_version,
            valid_from,
            ..
        } = op
        {
            let tt = self.next_tt.saturating_sub(1);
            if !self.fold.rule_hash_live_at(rule_version, tt, *valid_from) {
                return Err(RuntimeError::UnknownRuleVersion {
                    pin: rule_version.clone(),
                });
            }
        }
        Ok(())
    }

    fn buffer_receipt(
        &mut self,
        receipt: &crate::receipt::QuantumReceipt,
        events_in_quantum: usize,
        aborted: bool,
    ) {
        let receipt_digest_hex = digest_to_hex(&receipt.digest);
        self.outcomes.push(PersistedQuantumOutcome {
            quantum_id: receipt_digest_hex.clone(),
            disposition: disposition(aborted, events_in_quantum),
            aborted_on_budget: aborted,
            events_in_quantum,
            event_ids: receipt.event_ids.clone(),
            receipt_digest_hex,
            resume_of: None,
        });
    }

    fn persist_outcome_event(&mut self) {
        let row = self.outcomes.last().expect("receipt just buffered").clone();
        let event = Event::new(Self::outcome_op(&row), self.next_tt());
        self.commit_event(event);
    }

    /// Shared write path. `emit` is the thin public wrapper (MetaOpRejected stays there).
    pub(crate) fn pipeline(&mut self, op: Op) -> Result<QuantumOutcome, RuntimeError> {
        self.pipeline_kind(op, ReceiptMode::User)
    }

    pub(crate) fn pipeline_kind(
        &mut self,
        op: Op,
        mode: ReceiptMode,
    ) -> Result<QuantumOutcome, RuntimeError> {
        if mode == ReceiptMode::User {
            self.gate_user_op(&op)?;
        }
        self.admit(&op, mode)?;
        if mode == ReceiptMode::User && matches!(&op, Op::StoreBlob { .. } | Op::TtAnchor { .. }) {
            return self.pipeline_kind(op, ReceiptMode::Dictionary);
        }
        if mode == ReceiptMode::User {
            self.admit_claim(&op)?;
            if let Some(retry) = self.delivery_key_retry(&op)? {
                return Ok(retry);
            }
            return self.commit_user_cascade(op);
        }
        if let Some(retry) = self.delivery_key_retry(&op)? {
            return Ok(retry);
        }
        let event = Event::new(op, self.next_tt());
        self.pipeline_events(vec![event], mode)
    }

    /// Commit already-built events (justification subject patch) through the same receipt.
    pub(crate) fn pipeline_events(
        &mut self,
        events: Vec<Event>,
        mode: ReceiptMode,
    ) -> Result<QuantumOutcome, RuntimeError> {
        for e in &events {
            self.admit(&e.op, mode)?;
        }
        let mut ids = Vec::new();
        let mut digests = Vec::new();
        for event in events {
            ids.push(event.id);
            digests.push(event.digest_bytes());
            self.commit_event(event);
        }
        let receipt = QuantumReceipt::from_events(ids, &digests, false);
        let events_in_quantum = receipt.event_ids.len();
        match mode {
            ReceiptMode::User => {
                self.buffer_receipt(&receipt, events_in_quantum, false);
                self.persist_outcome_event();
            }
            ReceiptMode::Typed => self.buffer_receipt(&receipt, events_in_quantum, false),
            ReceiptMode::Dictionary => {}
        }
        Ok(QuantumOutcome {
            receipt,
            events_in_quantum,
        })
    }

    fn commit_user_cascade(&mut self, op: Op) -> Result<QuantumOutcome, RuntimeError> {
        let prefix = self.log.len();
        let next_tt = self.next_tt;
        match self.commit_user_cascade_apply(op) {
            Ok(outcome) => Ok(outcome),
            Err(err) => {
                self.rollback_user_cascade(prefix, next_tt);
                Err(err)
            }
        }
    }

    /// Replay the pre-cascade log prefix so a later-hop `rule_follow_ons` / `admit`
    /// error cannot leave a live Assert without `QuantumOutcome`.
    fn rollback_user_cascade(&mut self, prefix: usize, next_tt: u64) {
        if self.log.len() != prefix {
            *self = self
                .fork_at(prefix)
                .expect("rollback prefix was log.len() before the failed cascade");
        }
        self.next_tt = next_tt;
    }

    fn commit_user_cascade_apply(&mut self, op: Op) -> Result<QuantumOutcome, RuntimeError> {
        let first = Event::new(op, self.next_tt());
        let mut ids = Vec::new();
        let mut digests = Vec::new();
        let mut aborted = false;
        let mut pending = vec![first];
        let mut used = 0usize;

        while let Some(event) = pending.pop() {
            if used >= self.max_cascade {
                aborted = true;
                break;
            }
            used += 1;
            ids.push(event.id);
            digests.push(event.digest_bytes());
            let follow = self.rule_follow_ons(&event)?;
            for e in &follow {
                self.admit(&e.op, ReceiptMode::User)?;
            }
            self.commit_event(event);
            for e in follow {
                pending.push(e);
            }
        }

        let receipt = QuantumReceipt::from_events(ids, &digests, aborted);
        self.buffer_receipt(&receipt, used, aborted);
        self.persist_outcome_event();
        Ok(QuantumOutcome {
            receipt,
            events_in_quantum: used,
        })
    }

    fn admit_store_blob(&self, digest: &str, bytes: &[u8]) -> Result<(), RuntimeError> {
        if !blob_digest_matches(digest, bytes) {
            return Err(RuntimeError::UnknownRuleVersion {
                pin: digest.to_string(),
            });
        }
        Ok(())
    }

    /// Return replay-derived D021 lifecycle lease (droppable; not graph facts).
    pub fn pack_lifecycle(&self) -> Vec<crate::pack_lifecycle::PackLifecycle> {
        crate::pack_lifecycle::derive_pack_lifecycle(self.log.as_slice())
    }

    /// Live owned operator names in the droppable index.
    ///
    /// A retired pack releases its names, so it contributes zero. Contrast
    /// [`Self::pack_operators`], which returns audit rows including retired
    /// packs. The two answers differ by design; do not swap them.
    pub fn operator_count(&self) -> usize {
        self.operators.len()
    }

    /// Return the replay-derived C1b operator registry. No operator executes.
    ///
    /// Audit view: retired packs keep their rows with `state = Retired`, so
    /// this count includes them. Contrast [`Self::operator_count`], which is
    /// the live name set.
    pub fn pack_operators(&self) -> Result<Vec<crate::PackOperator>, crate::PackManifestError> {
        crate::pack_registry::derive_pack_operators(self.log.as_slice(), &self.fold.blobs)
    }

    /// Return the replay-derived C5 enrichment ledger. No model is called.
    pub fn enrichment_ledger(&self) -> Vec<crate::EnrichmentRecord> {
        crate::enrichment::derive_enrichment_ledger(self.log.as_slice())
    }

    /// Record an ADR-052 enrichment proposal for an active pack.
    ///
    /// The payload is stored as a content-addressed `StoreBlob`; the proposal
    /// cites it. Nothing is executed and no kernel fact changes.
    pub fn emit_enrichment_proposal(
        &mut self,
        pack_id: impl Into<String>,
        pack_version: impl Into<String>,
        source_event_ids: Vec<kutha_common::EventId>,
        model: impl Into<String>,
        prompt: impl Into<String>,
        cache_key: impl Into<String>,
        payload: Vec<u8>,
    ) -> Result<QuantumOutcome, RuntimeError> {
        if payload.is_empty() {
            return Err(RuntimeError::EnrichmentRejected {
                reason: "enrichment payload is empty".to_string(),
            });
        }
        let digest = kutha_common::sha256_hex(&payload);
        let proposal = Op::EnrichmentProposal {
            pack_id: pack_id.into(),
            pack_version: pack_version.into(),
            source_event_ids,
            model: model.into(),
            prompt: prompt.into(),
            cache_key: cache_key.into(),
            blob_digest: digest.clone(),
        };
        // Validate before the blob append so a rejected proposal leaves no
        // orphan `StoreBlob` behind, using the same runtime leases admission
        // uses — no log scan. Admission re-checks with the blob present.
        {
            let pack_id = match &proposal {
                Op::EnrichmentProposal { pack_id, .. } => pack_id.clone(),
                _ => unreachable!(),
            };
            let state = self
                .lifecycle
                .state(&pack_id)
                .map(|state| (state, self.lifecycle.version(&pack_id).unwrap_or("").to_string()));
            let mut source_known = |id: kutha_common::EventId| self.log_ids.contains(&id);
            crate::enrichment::check_proposal(
                &proposal,
                state,
                &mut source_known,
                Some(&payload),
                &self.enrichment_cache_keys,
            )
            .map_err(|err| RuntimeError::EnrichmentRejected {
                reason: err.to_string(),
            })?;
        }
        self.pipeline_kind(
            Op::StoreBlob {
                digest: digest.clone(),
                bytes: payload,
            },
            ReceiptMode::Dictionary,
        )?;
        self.pipeline_kind(proposal, ReceiptMode::Dictionary)
    }

    /// Record D021 pack install (not a graph Fact).
    ///
    /// `manifest` is optional declaration metadata. When present its canonical
    /// bytes are stored as a `StoreBlob` first, then the install pins the
    /// digest. Nothing in the manifest is executed.
    pub fn emit_pack_install(
        &mut self,
        pack_id: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> Result<QuantumOutcome, RuntimeError> {
        self.emit_pack_install_with_manifest(pack_id, name, version, None)
    }

    /// Record D021 pack install with an optional [`crate::PackManifest`].
    pub fn emit_pack_install_with_manifest(
        &mut self,
        pack_id: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
        manifest: Option<crate::PackManifest>,
    ) -> Result<QuantumOutcome, RuntimeError> {
        let pack_id = pack_id.into();
        let name = name.into();
        let version = version.into();
        let manifest_digest = match manifest {
            Some(manifest) => {
                manifest
                    .validate_shape()
                    .map_err(|err| RuntimeError::PackLifecycleRejected {
                        pack_id: pack_id.clone(),
                        reason: format!("manifest: {err}"),
                    })?;
                manifest
                    .validate_identity(&pack_id, &name, &version)
                    .map_err(|err| RuntimeError::PackLifecycleRejected {
                        pack_id: pack_id.clone(),
                        reason: format!("manifest: {err}"),
                    })?;
                let bytes = manifest.canonical_bytes();
                let digest = kutha_common::sha256_hex(&bytes);
                self.pipeline_kind(
                    Op::StoreBlob {
                        digest: digest.clone(),
                        bytes,
                    },
                    ReceiptMode::Dictionary,
                )?;
                Some(digest)
            }
            None => None,
        };
        self.pipeline_kind(
            Op::PackInstall {
                pack_id,
                name,
                version,
                manifest_digest,
            },
            ReceiptMode::Dictionary,
        )
    }

    /// Record D021 pack activation (not a graph Fact).
    pub fn emit_pack_activate(
        &mut self,
        pack_id: impl Into<String>,
        version: impl Into<String>,
    ) -> Result<QuantumOutcome, RuntimeError> {
        self.pipeline_kind(
            Op::PackActivate {
                pack_id: pack_id.into(),
                version: version.into(),
            },
            ReceiptMode::Dictionary,
        )
    }

    /// Record D021 pack retirement (not a graph Fact).
    pub fn emit_pack_retire(
        &mut self,
        pack_id: impl Into<String>,
        version: impl Into<String>,
    ) -> Result<QuantumOutcome, RuntimeError> {
        self.pipeline_kind(
            Op::PackRetire {
                pack_id: pack_id.into(),
                version: version.into(),
            },
            ReceiptMode::Dictionary,
        )
    }

    /// Record a wall-date → transaction-time anchor on the log (not a graph Fact).
    pub fn emit_tt_anchor(
        &mut self,
        wall: impl Into<WallTime>,
    ) -> Result<QuantumOutcome, RuntimeError> {
        self.pipeline_kind(
            Op::TtAnchor {
                wall: wall.into().as_u64(),
            },
            ReceiptMode::Dictionary,
        )
    }

    /// Retract a dictionary / reserved fact (constructor path). User `emit(Retract)` of
    /// `kutha:*` is `MetaOpRejected` (M020 S04).
    pub fn emit_retract_dict(&mut self, event_id: EventId) -> Result<QuantumOutcome, RuntimeError> {
        self.pipeline_kind(Op::Retract { event_id }, ReceiptMode::Dictionary)
    }

    /// Allow a product relation name as a `kutha:allows` fact (D-01).
    pub fn emit_allow(
        &mut self,
        name: impl Into<String>,
        valid_from: u64,
        valid_to: Option<u64>,
    ) -> Result<QuantumOutcome, RuntimeError> {
        let name = name.into();
        if name.is_empty() {
            return Err(RuntimeError::UnknownRelation {
                name: String::new(),
            });
        }
        let subject = self.intern_silent(&name);
        let relation = self.intern_silent(KUTHA_ALLOWS);
        self.pipeline_kind(
            Op::Assert {
                subject,
                relation,
                object: subject,
                valid_from,
                valid_to,
                claim: None,
                delivery_key: Some(name),
                polarity: None,
            },
            ReceiptMode::Dictionary,
        )
    }

    /// Store a named rule body once, then assert `kutha:rule` carrying its canonical hash.
    pub fn emit_rule(
        &mut self,
        definition: impl Into<String>,
        valid_from: u64,
        valid_to: Option<u64>,
    ) -> Result<QuantumOutcome, RuntimeError> {
        let definition = definition.into();
        let Some(ast) = RuleAst::named(&definition) else {
            return Err(RuntimeError::UnknownRuleVersion { pin: definition });
        };
        self.emit_rule_ast(&ast, valid_from, valid_to)
    }

    /// Store a canonical [`RuleAst`] body once, then assert `kutha:rule` (RULE-01).
    pub fn emit_rule_ast(
        &mut self,
        ast: &RuleAst,
        valid_from: u64,
        valid_to: Option<u64>,
    ) -> Result<QuantumOutcome, RuntimeError> {
        let bytes = ast.encode();
        if RuleAst::from_canonical(&bytes).as_ref() != Some(ast) {
            return Err(RuntimeError::UnknownRuleVersion { pin: ast.hash() });
        }
        let digest = ast.hash();
        self.admit_store_blob(&digest, &bytes)?;
        self.pipeline_kind(
            Op::StoreBlob {
                digest: digest.clone(),
                bytes,
            },
            ReceiptMode::Dictionary,
        )?;
        let subject = self.intern_silent(&digest);
        let relation = self.intern_silent(KUTHA_RULE);
        self.pipeline_kind(
            Op::Assert {
                subject,
                relation,
                object: subject,
                valid_from,
                valid_to,
                claim: None,
                delivery_key: Some(digest),
                polarity: None,
            },
            ReceiptMode::Dictionary,
        )
    }

    /// Store a policy body once, then assert `kutha:policy` carrying `policy_version_hash` (D-02).
    pub fn emit_policy(
        &mut self,
        definition: impl Into<String>,
        valid_from: u64,
        valid_to: Option<u64>,
    ) -> Result<QuantumOutcome, RuntimeError> {
        let definition = definition.into();
        if definition.is_empty() {
            return Err(RuntimeError::UnknownPolicyVersion { pin: String::new() });
        }
        let digest = policy_version_hash(&definition);
        self.admit_store_blob(&digest, definition.as_bytes())?;
        self.pipeline_kind(
            Op::StoreBlob {
                digest: digest.clone(),
                bytes: definition.into_bytes(),
            },
            ReceiptMode::Dictionary,
        )?;
        let subject = self.intern_silent(&digest);
        let relation = self.intern_silent(KUTHA_POLICY);
        self.pipeline_kind(
            Op::Assert {
                subject,
                relation,
                object: subject,
                valid_from,
                valid_to,
                claim: None,
                delivery_key: Some(digest),
                polarity: None,
            },
            ReceiptMode::Dictionary,
        )
    }

    pub fn record_resume(&mut self, quantum_id: &str) -> Result<(), RuntimeError> {
        if self
            .outcomes
            .iter()
            .any(|r| r.resume_of.as_deref() == Some(quantum_id))
        {
            return Err(RuntimeError::DuplicateResume {
                resume_of: quantum_id.to_string(),
            });
        }
        self.outcomes.push(PersistedQuantumOutcome {
            quantum_id: format!("resume:{quantum_id}"),
            disposition: OutcomeDisposition::Resume,
            aborted_on_budget: false,
            events_in_quantum: 0,
            event_ids: Vec::new(),
            receipt_digest_hex: String::new(),
            resume_of: Some(quantum_id.to_string()),
        });
        self.persist_outcome_event();
        Ok(())
    }

    /// Admit a user op, append, fold, and apply Horn follow-ons until idle or budget.
    pub fn emit(&mut self, op: Op) -> Result<QuantumOutcome, RuntimeError> {
        self.reject_forged_meta(&op)?;
        self.pipeline(op)
    }
}
