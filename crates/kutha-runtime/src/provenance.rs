//! Replay and provenance (ADR-060). Not a MATCH compiler.
use crate::fold::GraphFold;
use crate::quantum::{Runtime, RuntimeError};
use kutha_common::{EventId, Op, TransactionTime, ValidTime};
use sha2::{Digest, Sha256};
use std::collections::HashSet;

fn tt_anchors_digest(anchors: &std::collections::BTreeMap<u64, u64>) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(b"kutha-tt-anchors");
    for (wall, ingested_at) in anchors {
        h.update(wall.to_le_bytes());
        h.update(ingested_at.to_le_bytes());
    }
    h.finalize().into()
}

impl Runtime {
    pub fn replay_check(&self) -> Result<GraphFold, RuntimeError> {
        let rebuilt = GraphFold::replay(self.log.as_slice());
        let expected = self.fold.fingerprint();
        let actual = rebuilt.fingerprint();
        if expected != actual {
            return Err(RuntimeError::ReplayDivergence { expected, actual });
        }
        Self::check_tt_anchors(&self.fold.tt_anchors, self.log.as_slice())?;
        let mut seen = HashSet::new();
        for e in self.log.iter() {
            if let Op::Behavior { caused_by, .. } = &e.op {
                if !seen.contains(caused_by) {
                    return Err(RuntimeError::BrokenLineage {
                        caused_by: *caused_by,
                    });
                }
                if !self.horn_behavior_reproducible(e) {
                    return Err(RuntimeError::ExecutionReplayFailed { event_id: e.id });
                }
            }
            seen.insert(e.id);
        }
        Ok(rebuilt)
    }

    /// Snapshot-held walls + `ingested_at` vs log `Op::TtAnchor` (D-02). Not fingerprint input.
    pub(crate) fn check_tt_anchors(
        snapshot_held: &std::collections::BTreeMap<u64, u64>,
        events: &[kutha_common::Event],
    ) -> Result<(), RuntimeError> {
        let from_log = GraphFold::tt_anchors_from_events(events);
        if snapshot_held != &from_log {
            return Err(RuntimeError::ReplayDivergence {
                expected: tt_anchors_digest(snapshot_held),
                actual: tt_anchors_digest(&from_log),
            });
        }
        Ok(())
    }

    /// Lineage digest over Behavior plus log-native outcome/cite Events (ADR-060 obligation 2, LOG-03).
    /// Behavior mix is `(event.id, caused_by, name, rule_version)` with length-prefixed strings.
    /// QuantumOutcome and JustificationCite mix `Event::digest_bytes` in the same log walk.
    /// Does not include Fact triples and is not called from [`Self::replay_check`].
    pub fn provenance_fingerprint(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update(b"kutha-prov-log-native");
        for e in self.log.iter() {
            match &e.op {
                Op::Behavior {
                    name,
                    caused_by,
                    rule_version,
                    ..
                } => {
                    h.update(e.id.as_bytes());
                    h.update(caused_by.as_bytes());
                    h.update((name.len() as u64).to_le_bytes());
                    h.update(name.as_bytes());
                    h.update((rule_version.len() as u64).to_le_bytes());
                    h.update(rule_version.as_bytes());
                }
                Op::QuantumOutcome { .. } | Op::JustificationCite { .. } | Op::StoreBlob { .. } => {
                    h.update(e.digest_bytes());
                }
                Op::Assert { relation, .. } if self.fold.is_dictionary_rel(*relation) => {
                    h.update(e.digest_bytes());
                }
                _ => {}
            }
        }
        h.finalize().into()
    }

    /// Compare a previously captured lineage digest. State replay stays [`Self::replay_check`].
    pub fn provenance_check(&self, expected: [u8; 32]) -> Result<(), RuntimeError> {
        let actual = self.provenance_fingerprint();
        if expected != actual {
            return Err(RuntimeError::ProvenanceMismatch { expected, actual });
        }
        Ok(())
    }

    /// Derivation eligibility at a named cut. A Behavior-derived claim is
    /// eligible iff the derived fact is live, its pin is a live registry hash,
    /// and its immediate premise claim is live. When that premise is itself a
    /// Behavior, eligibility also walks `caused_by` through ancestor Behaviors
    /// with a visited [`EventId`] set (already-seen id is ineligible).
    ///
    /// This is not a provenance-polynomial evaluator and is not a MATCH compiler.
    pub fn derivation_eligible_at(
        &self,
        derived: EventId,
        tt: impl Into<TransactionTime>,
        vt: impl Into<ValidTime>,
    ) -> bool {
        let tt = tt.into().as_u64();
        let vt = vt.into().as_u64();
        let mut visited = HashSet::new();
        self.derivation_eligible_at_walk(derived, tt, vt, &mut visited)
    }

    fn derivation_eligible_at_walk(
        &self,
        derived: EventId,
        tt: u64,
        vt: u64,
        visited: &mut HashSet<EventId>,
    ) -> bool {
        if !visited.insert(derived) {
            return false;
        }
        if !self.fold.claim_supported_at(derived, tt, vt) {
            return false;
        }
        let (caused_by, rule_version) = match self.event_by_id(derived).map(|e| &e.op) {
            Some(Op::Behavior {
                caused_by,
                rule_version,
                ..
            }) => (*caused_by, rule_version.clone()),
            _ => return false,
        };
        let Some(cause) = self.event_by_id(caused_by) else {
            return false;
        };
        let premise = match &cause.op {
            Op::Assert { claim, .. } => claim.unwrap_or(cause.id),
            Op::Behavior { .. } => cause.id,
            Op::Retract { .. }
            | Op::Correct { .. }
            | Op::CorrectInterval { .. }
            | Op::Define { .. }
            | Op::QuantumOutcome { .. }
            | Op::JustificationCite { .. }
            | Op::StoreBlob { .. }
            | Op::TtAnchor { .. } => return false,
        };
        if !self.fold.rule_hash_live_at(&rule_version, tt, vt) {
            return false;
        }
        if !self.fold.claim_supported_at(premise, tt, vt) {
            return false;
        }
        match &cause.op {
            Op::Assert { .. } => true,
            Op::Behavior { .. } => self.derivation_eligible_at_walk(cause.id, tt, vt, visited),
            _ => false,
        }
    }

    #[cfg(test)]
    pub(crate) fn tamper_fold(&mut self) {
        self.fold.tamper_invalidate_first();
    }

    #[cfg(test)]
    pub(crate) fn tamper_inverse_pin(&mut self, pin: &str) {
        self.log.tamper_inverse_pin(pin.to_string());
    }
}
