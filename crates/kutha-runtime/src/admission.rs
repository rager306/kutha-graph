//! Admit and delivery-key gates for the write pipeline (D-01 / T-27-02).
use crate::pipeline::ReceiptMode;
use crate::quantum::{
    ConflictReport, Justification, QuantumOutcome, Runtime, RuntimeError, op_relation,
};
use crate::receipt::QuantumReceipt;
use kutha_common::{
    EventId, KUTHA_RESERVED_RELS, Op, TermId, TransactionTime, ValidTime, blob_digest_matches,
};

impl Runtime {
    pub fn check_admission(&self, justification_id: &str) -> Result<(), RuntimeError> {
        let row = self
            .cite_from_log(justification_id)
            .ok_or(RuntimeError::AdmissionDenied {
                justification_id: justification_id.into(),
                reason: "unknown_justification",
            })?;
        self.check_admission_row(&row)
    }

    pub(crate) fn check_admission_row(&self, row: &Justification) -> Result<(), RuntimeError> {
        for event_id in &row.source_event_ids {
            self.ingest_examine.bump();
            // CorrectInterval may mint several live residuals under one event_id.
            // A single representative (fact_index_by_event_id) can miss patch/suffix VT.
            let live = self
                .fold
                .live_fact_indices_by_event_id(*event_id)
                .iter()
                .any(|&i| self.fold.facts()[i].is_live_at(u64::MAX, row.vt));
            if !live {
                return Err(RuntimeError::AdmissionDenied {
                    justification_id: row.justification_id.clone(),
                    reason: "stale_support",
                });
            }
        }
        if !self.derivation_eligible_at(row.target_claim, row.tt, row.vt) {
            return Err(RuntimeError::AdmissionDenied {
                justification_id: row.justification_id.clone(),
                reason: "ineligible",
            });
        }
        let pin = match self.event_by_id(row.target_claim) {
            Some(e) => match &e.op {
                Op::Behavior { rule_version, .. } => rule_version.as_str(),
                _ => "",
            },
            None => "",
        };
        if pin != row.rule_version.as_str() {
            return Err(RuntimeError::AdmissionDenied {
                justification_id: row.justification_id.clone(),
                reason: "rule_version",
            });
        }
        let Some(policy_pin) = self.fold.live_policy_pin_at(row.tt, row.vt) else {
            return Err(RuntimeError::AdmissionDenied {
                justification_id: row.justification_id.clone(),
                reason: "policy",
            });
        };
        let Some(bytes) = self.fold.blobs.get(&policy_pin) else {
            return Err(RuntimeError::AdmissionDenied {
                justification_id: row.justification_id.clone(),
                reason: "policy",
            });
        };
        if std::str::from_utf8(bytes).is_ok_and(|s| s.trim() == "deny") {
            return Err(RuntimeError::AdmissionDenied {
                justification_id: row.justification_id.clone(),
                reason: "policy",
            });
        }
        Ok(())
    }

    /// Partition live supports of `claim` by stored `Fact.polarity` (ING-03 / D-03).
    /// `None` polarity is in neither bucket. Caller TermIds are not the polarity source.
    pub fn conflict_report_at(
        &self,
        claim: EventId,
        tt: impl Into<TransactionTime>,
        vt: impl Into<ValidTime>,
    ) -> ConflictReport {
        let tt = tt.into().as_u64();
        let vt = vt.into().as_u64();
        let mut positive_supports = Vec::new();
        let mut negative_supports = Vec::new();
        for f in self.fold.live_supports(claim, tt, vt) {
            match f.polarity {
                Some(kutha_common::SupportPolarity::Positive) => positive_supports.push(f.seq),
                Some(kutha_common::SupportPolarity::Negative) => negative_supports.push(f.seq),
                None => {}
            }
        }
        ConflictReport {
            positive_supports,
            negative_supports,
        }
    }
    pub(crate) fn admit(&self, op: &Op, mode: ReceiptMode) -> Result<(), RuntimeError> {
        if let Op::StoreBlob { digest, bytes } = op {
            if !blob_digest_matches(digest, bytes) {
                return Err(RuntimeError::UnknownRuleVersion {
                    pin: digest.clone(),
                });
            }
            return Ok(());
        }
        if matches!(op, Op::EnrichmentProposal { .. }) {
            return crate::enrichment::validate_proposal(self.log.as_slice(), &self.fold.blobs, op)
                .map_err(|err| RuntimeError::EnrichmentRejected {
                    reason: err.to_string(),
                });
        }
        if matches!(
            op,
            Op::PackInstall { .. } | Op::PackActivate { .. } | Op::PackRetire { .. }
        ) {
            Self::check_pack_lifecycle(self.log.as_slice(), op)?;
            if let Op::PackInstall {
                manifest_digest: Some(_),
                ..
            } = op
            {
                crate::pack_manifest::validate_install(self.log.as_slice(), &self.fold.blobs, op)
                    .map_err(|err| RuntimeError::PackLifecycleRejected {
                    pack_id: match op {
                        Op::PackInstall { pack_id, .. } => pack_id.clone(),
                        _ => String::new(),
                    },
                    reason: format!("manifest: {err}"),
                })?;
            }
            return Ok(());
        }
        if mode == ReceiptMode::User {
            if let Op::Assert { relation, .. } | Op::Behavior { relation, .. } = op {
                if self.relation_name_reserved(*relation) {
                    return Err(RuntimeError::MetaOpRejected);
                }
            }
            if let Op::Correct { event_id, .. }
            | Op::CorrectInterval { event_id, .. }
            | Op::Retract { event_id } = op
            {
                if let Some(idx) = self.fold.fact_index_by_event_id(*event_id) {
                    let rel = self.fold.facts()[idx].relation;
                    if self.relation_name_reserved(rel) {
                        return Err(RuntimeError::MetaOpRejected);
                    }
                }
            }
        }
        if let Op::TtAnchor { wall } = op {
            if let Some((&max_wall, _)) = self.fold.tt_anchors.last_key_value() {
                if *wall <= max_wall {
                    return Err(RuntimeError::TtAnchorRejected {
                        wall: *wall,
                        max_wall: Some(max_wall),
                    });
                }
            }
            return Ok(());
        }
        let Some(rel) = op_relation(op) else {
            return Ok(());
        };
        let name = self.dict.lookup(rel).unwrap_or("").to_string();
        if name.starts_with("kutha:") {
            return Ok(());
        }
        let vt = match op {
            Op::Assert { valid_from, .. } | Op::Behavior { valid_from, .. } => *valid_from,
            _ => 0,
        };
        let tt = self.next_tt.saturating_sub(1);
        if !name.is_empty() && self.fold.relation_allowed_at(&name, tt, vt) {
            return Ok(());
        }
        Err(RuntimeError::UnknownRelation {
            name: if name.is_empty() {
                format!("#{rel}")
            } else {
                name
            },
        })
    }

    fn relation_name_reserved(&self, rel: TermId) -> bool {
        self.dict
            .lookup(rel)
            .is_some_and(|n| KUTHA_RESERVED_RELS.contains(&n) || n.starts_with("kutha:"))
    }

    /// Replay D021 lifecycle facts from the existing log and validate the next
    /// transition. Lifecycle state is meta-only and never enters the graph fold.
    pub(crate) fn check_pack_lifecycle(
        log: &[kutha_common::Event],
        next: &Op,
    ) -> Result<(), RuntimeError> {
        use kutha_common::Op;

        #[derive(Clone, Copy, PartialEq, Eq)]
        enum PackState {
            Installed,
            Active,
            Retired,
        }

        let (next_pack_id, next_name, next_version) = match next {
            Op::PackInstall {
                pack_id,
                name,
                version,
                ..
            } => (pack_id.as_str(), Some(name.as_str()), version.as_str()),
            Op::PackActivate { pack_id, version } | Op::PackRetire { pack_id, version } => {
                (pack_id.as_str(), None, version.as_str())
            }
            _ => return Ok(()),
        };
        if next_pack_id.trim().is_empty()
            || next_version.trim().is_empty()
            || next_name.is_some_and(|name| name.trim().is_empty())
        {
            return Err(RuntimeError::PackLifecycleRejected {
                pack_id: next_pack_id.to_string(),
                reason: "empty_identifier".to_string(),
            });
        }
        // Exact spelling is identity: a padded id must not alias a clean one.
        if next_pack_id != next_pack_id.trim()
            || next_version != next_version.trim()
            || next_name.is_some_and(|name| name != name.trim())
        {
            return Err(RuntimeError::PackLifecycleRejected {
                pack_id: next_pack_id.to_string(),
                reason: "padded_identifier".to_string(),
            });
        }

        let mut state: std::collections::BTreeMap<&str, (PackState, &str)> =
            std::collections::BTreeMap::new();
        for event in log {
            match &event.op {
                Op::PackInstall {
                    pack_id, version, ..
                } => {
                    if state
                        .insert(pack_id.as_str(), (PackState::Installed, version.as_str()))
                        .is_some()
                    {
                        return Err(RuntimeError::PackLifecycleRejected {
                            pack_id: pack_id.clone(),
                            reason: "pack_id_already_known".to_string(),
                        });
                    }
                }
                Op::PackActivate { pack_id, version } => match state.get(pack_id.as_str()) {
                    Some((PackState::Installed, installed_version))
                        if installed_version == &version =>
                    {
                        state.insert(pack_id, (PackState::Active, version));
                    }
                    _ => {
                        return Err(RuntimeError::PackLifecycleRejected {
                            pack_id: pack_id.clone(),
                            reason: "activate_requires_same_installed_version".to_string(),
                        });
                    }
                },
                Op::PackRetire { pack_id, version } => match state.get(pack_id.as_str()) {
                    Some((PackState::Installed | PackState::Active, installed_version))
                        if installed_version == &version =>
                    {
                        state.insert(pack_id, (PackState::Retired, version));
                    }
                    _ => {
                        return Err(RuntimeError::PackLifecycleRejected {
                            pack_id: pack_id.clone(),
                            reason: "retire_requires_same_installed_version".to_string(),
                        });
                    }
                },
                _ => {}
            }
        }
        match next {
            Op::PackInstall { pack_id, .. } if state.contains_key(pack_id.as_str()) => {
                Err(RuntimeError::PackLifecycleRejected {
                    pack_id: pack_id.clone(),
                    reason: "pack_id_already_known".to_string(),
                })
            }
            Op::PackActivate { pack_id, version } => match state.get(pack_id.as_str()) {
                Some((PackState::Installed, installed_version))
                    if installed_version == &version =>
                {
                    Ok(())
                }
                _ => Err(RuntimeError::PackLifecycleRejected {
                    pack_id: pack_id.clone(),
                    reason: "activate_requires_same_installed_version".to_string(),
                }),
            },
            Op::PackRetire { pack_id, version } => match state.get(pack_id.as_str()) {
                Some((PackState::Installed | PackState::Active, installed_version))
                    if installed_version == &version =>
                {
                    Ok(())
                }
                _ => Err(RuntimeError::PackLifecycleRejected {
                    pack_id: pack_id.clone(),
                    reason: "retire_requires_same_installed_version".to_string(),
                }),
            },
            _ => Ok(()),
        }
    }

    pub(crate) fn admit_claim(&self, op: &Op) -> Result<(), RuntimeError> {
        let Op::Assert {
            claim: Some(claim), ..
        } = op
        else {
            return Ok(());
        };
        self.ingest_examine.bump();
        if self.fold.claim_known(*claim) {
            return Ok(());
        }
        Err(RuntimeError::UnknownClaim { claim: *claim })
    }

    fn delivery_key_active(key: &Option<String>) -> Option<&str> {
        match key {
            Some(s) if !s.is_empty() => Some(s.as_str()),
            _ => None,
        }
    }

    fn assert_payload_matches(left: &Op, right: &Op) -> bool {
        match (left, right) {
            (
                Op::Assert {
                    subject: s1,
                    relation: r1,
                    object: o1,
                    valid_from: vf1,
                    valid_to: vt1,
                    claim: c1,
                    polarity: p1,
                    delivery_key: d1,
                },
                Op::Assert {
                    subject: s2,
                    relation: r2,
                    object: o2,
                    valid_from: vf2,
                    valid_to: vt2,
                    claim: c2,
                    polarity: p2,
                    delivery_key: d2,
                },
            ) => {
                s1 == s2
                    && r1 == r2
                    && o1 == o2
                    && vf1 == vf2
                    && vt1 == vt2
                    && c1 == c2
                    && p1 == p2
                    && d1 == d2
            }
            _ => false,
        }
    }

    /// Identical keyed Assert returns the original EventId without appending (ING-01 / D-01).
    /// Lookup is namespaced by relation so shared pin strings across
    /// `kutha:policy` / `kutha:admission` / product relations do not collide.
    pub(crate) fn delivery_key_retry(
        &self,
        op: &Op,
    ) -> Result<Option<QuantumOutcome>, RuntimeError> {
        let Op::Assert {
            relation,
            delivery_key,
            ..
        } = op
        else {
            return Ok(None);
        };
        let Some(key) = Self::delivery_key_active(delivery_key) else {
            return Ok(None);
        };
        self.ingest_examine.bump();
        let Some(existing_id) = self
            .by_delivery_key
            .get(&(*relation, key.to_string()))
            .copied()
        else {
            return Ok(None);
        };
        // Retracted keyed Assert must not short-circuit: remint a live support.
        // Correct / CorrectInterval move the delivery to rows under their own event
        // id (same claim_id, relation, and delivery_key). Liveness is scoped to
        // this relation so a sibling sharing claim+key cannot mask a retract.
        let live = self
            .fold
            .fact_index_by_event_id(existing_id)
            .map(|idx| self.fold.facts()[idx].claim_id)
            .is_some_and(|claim| self.fold.has_live_keyed_support(claim, *relation, key));
        if !live {
            return Ok(None);
        }
        let Some(e) = self.event_by_id(existing_id) else {
            return Ok(None);
        };
        if Self::assert_payload_matches(&e.op, op) {
            let receipt = QuantumReceipt::from_events(vec![e.id], &[e.digest_bytes()], false);
            return Ok(Some(QuantumOutcome {
                receipt,
                events_in_quantum: 0,
            }));
        }
        Err(RuntimeError::DeliveryKeyConflict {
            key: key.to_string(),
        })
    }
}
