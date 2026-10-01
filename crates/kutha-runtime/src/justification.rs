//! Justification and Action constructors (typed records → pipeline).
use crate::pipeline::ReceiptMode;
use crate::quantum::{ActionRecord, Justification, Runtime, RuntimeError};
use kutha_common::{
    Event, EventId, Op, SupportPolarity, KUTHA_ACTION, KUTHA_ADMISSION,
};

impl Runtime {
    pub(crate) fn cite_from_log(&self, justification_id: &str) -> Option<Justification> {
        self.log.as_slice().iter().rev().find_map(|e| match &e.op {
            Op::JustificationCite {
                justification_id: jid,
                target_claim,
                source_claim_ids,
                source_event_ids,
                rule_version,
                tt,
                vt,
            } if jid == justification_id => Some(Justification {
                justification_id: jid.clone(),
                target_claim: *target_claim,
                source_claim_ids: source_claim_ids.clone(),
                source_event_ids: source_event_ids.clone(),
                rule_version: rule_version.clone(),
                tt: *tt,
                vt: *vt,
            }),
            _ => None,
        })
    }
    pub fn justification_records(&self) -> Vec<Justification> {
        self.log
            .iter()
            .filter_map(|e| match &e.op {
                Op::JustificationCite {
                    justification_id,
                    target_claim,
                    source_claim_ids,
                    source_event_ids,
                    rule_version,
                    tt,
                    vt,
                } => Some(Justification {
                    justification_id: justification_id.clone(),
                    target_claim: *target_claim,
                    source_claim_ids: source_claim_ids.clone(),
                    source_event_ids: source_event_ids.clone(),
                    rule_version: rule_version.clone(),
                    tt: *tt,
                    vt: *vt,
                }),
                _ => None,
            })
            .collect()
    }

    /// Append one justification cite, then a `kutha:admission` Assert citing the live policy pin.
    /// Does not run from `emit`. Returns the admission EventId as a string even when denied.
    pub fn record_justification(
        &mut self,
        target_claim: EventId,
        source_claim_ids: Vec<EventId>,
        source_event_ids: Vec<EventId>,
        rule_version: impl Into<String>,
        tt: u64,
        vt: u64,
    ) -> Result<String, RuntimeError> {
        let Some(policy_version) = self.fold.live_policy_pin_at(tt, vt) else {
            return Err(RuntimeError::UnknownPolicyVersion { pin: String::new() });
        };
        let rule_version = rule_version.into();
        let row = Justification {
            justification_id: String::new(),
            target_claim,
            source_claim_ids,
            source_event_ids,
            rule_version,
            tt,
            vt,
        };
        let admitted = self.check_admission_row(&row).is_ok();
        let relation = self.intern_silent(KUTHA_ADMISSION);
        let object = self.intern_silent(if admitted { "true" } else { "false" });
        let polarity = Some(if admitted {
            SupportPolarity::Positive
        } else {
            SupportPolarity::Negative
        });
        let mut event = Event::new(
            Op::Assert {
                subject: 0,
                relation,
                object,
                valid_from: vt,
                valid_to: None,
                claim: None,
                delivery_key: Some(policy_version),
                polarity,
            },
            self.next_tt(),
        );
        let jid = event.id.to_string();
        let subject = self.intern_silent(&jid);
        if let Op::Assert {
            subject: subj,
            relation: rel,
            object: obj,
            ..
        } = &mut event.op
        {
            *subj = subject;
            event.object_ids = vec![*subj, *rel, *obj];
        }
        let cite = Event::new(
            Op::JustificationCite {
                justification_id: jid.clone(),
                target_claim,
                source_claim_ids: row.source_claim_ids,
                source_event_ids: row.source_event_ids,
                rule_version: row.rule_version,
                tt,
                vt,
            },
            self.next_tt(),
        );
        self.pipeline_events(vec![event, cite], ReceiptMode::Typed)?;
        Ok(jid)
    }

    /// AS OF admission status for a justification (ADM-01). Delegates to the fold.
    pub fn admission_status_at(&self, justification_id: &str, tt: u64, vt: u64) -> Option<bool> {
        self.fold.admission_status_at(justification_id, tt, vt)
    }

    /// Append a thin Action bound to the live admission pin (ACT-01 / T-20-01 / T-20-02).
    pub fn record_action(
        &mut self,
        justification_id: &str,
        tt: u64,
        vt: u64,
    ) -> Result<EventId, RuntimeError> {
        let cite =
            self.cite_from_log(justification_id)
                .ok_or_else(|| RuntimeError::UnknownAdmission {
                    justification_id: justification_id.to_string(),
                })?;
        let Some((admitted, _policy_version)) =
            self.fold.live_admission_at(justification_id, tt, vt)
        else {
            return Err(RuntimeError::UnknownAdmission {
                justification_id: justification_id.to_string(),
            });
        };
        let relation = self.intern_silent(KUTHA_ACTION);
        let object = self.intern_silent(&cite.target_claim.to_string());
        let subject = self.intern_silent(justification_id);
        let polarity = Some(if admitted {
            SupportPolarity::Positive
        } else {
            SupportPolarity::Negative
        });
        let event = Event::new(
            Op::Assert {
                subject,
                relation,
                object,
                valid_from: vt,
                valid_to: None,
                claim: None,
                delivery_key: Some(justification_id.to_string()),
                polarity,
            },
            self.next_tt(),
        );
        let id = event.id;
        self.pipeline_events(vec![event], ReceiptMode::Typed)?;
        Ok(id)
    }

    /// AS OF Action bind for a justification (ACT-01).
    pub fn action_record_at(
        &self,
        justification_id: &str,
        tt: u64,
        vt: u64,
    ) -> Option<ActionRecord> {
        let fact = self.fold.live_action_fact(justification_id, tt, vt)?;
        let cite = self.cite_from_log(justification_id)?;
        let (admitted, policy_version) = self.fold.live_admission_at(justification_id, tt, vt)?;
        let _ = fact;
        Some(ActionRecord {
            justification_id: justification_id.to_string(),
            target_claim: cite.target_claim,
            source_claim_ids: cite.source_claim_ids,
            source_event_ids: cite.source_event_ids,
            rule_version: cite.rule_version,
            admitted,
            policy_version,
        })
    }

    /// Fold-cut policy pin liveness (ADM-02).
    pub fn policy_hash_live_at(&self, pin: &str, tt: u64, vt: u64) -> bool {
        self.fold.policy_hash_live_at(pin, tt, vt)
    }

    /// Latest ingested live policy hash at `(tt, vt)` (ADM-02 / ACT-02).
    pub fn live_policy_pin_at(&self, tt: u64, vt: u64) -> Option<String> {
        self.fold.live_policy_pin_at(tt, vt)
    }
}
