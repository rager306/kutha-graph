//! One write path: propose → admit → commit → receipt (D-01).
use crate::quantum::{disposition, PersistedQuantumOutcome, QuantumOutcome, Runtime, RuntimeError};
use crate::receipt::{digest_to_hex, QuantumReceipt};
use kutha_common::{
    blob_digest_matches, policy_version_hash, Event, Op, OutcomeDisposition, RuleAst, WallTime,
    KUTHA_ACTION, KUTHA_ADMISSION, KUTHA_ALLOWS, KUTHA_POLICY, KUTHA_RULE,
};

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
        let n = self.log.len();
        self.by_event_id.reserve(n);
        self.by_delivery_key.reserve(n);
        for (i, e) in self.log.as_slice().iter().enumerate() {
            self.by_event_id.insert(e.id, i);
            if let Op::Assert {
                delivery_key: Some(k),
                ..
            } = &e.op
            {
                if !k.is_empty() {
                    self.by_delivery_key.insert(k.clone(), e.id);
                }
            }
        }
    }

    pub(crate) fn append_indexed(&mut self, event: Event) {
        let pos = self.log.len();
        self.by_event_id.insert(event.id, pos);
        if let Op::Assert {
            delivery_key: Some(k),
            ..
        } = &event.op
        {
            if !k.is_empty() {
                self.by_delivery_key.insert(k.clone(), event.id);
            }
        }
        self.log.append(event);
    }

    pub(crate) fn event_by_id(&self, id: kutha_common::EventId) -> Option<&Event> {
        self.ingest_examine.bump();
        let pos = *self.by_event_id.get(&id)?;
        self.log.as_slice().get(pos)
    }

    pub(crate) fn next_tt(&mut self) -> u64 {
        let t = self.next_tt;
        self.next_tt += 1;
        t
    }

    pub(crate) fn commit_event(&mut self, event: Event) {
        self.fold.apply(&event);
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
        if matches!(op, Op::QuantumOutcome { .. } | Op::JustificationCite { .. }) {
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
        if let Op::Retract { event_id } = op {
            self.ingest_examine.bump();
            if self.fold.fact_index_by_event_id(*event_id).is_none() {
                return Err(RuntimeError::UnknownFact {
                    event_id: *event_id,
                });
            }
        } else if let Op::Correct { event_id, .. } | Op::CorrectInterval { event_id, .. } = op {
            self.ingest_examine.bump();
            if self.fold.fact_index_by_event_id(*event_id).is_none() {
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
            let idx = self
                .fold
                .fact_index_by_event_id(*event_id)
                .expect("UnknownFact gate already ran");
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
        let first = Event::new(op, self.next_tt());
        for follow in self.rule_follow_ons(&first) {
            self.admit(&follow.op, ReceiptMode::User)?;
        }
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
            let follow = self.rule_follow_ons(&event);
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
