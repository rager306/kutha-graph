//! Replay and provenance (ADR-060). Not a MATCH compiler.
use crate::fold::{GraphFold, checkpoint_prefix_meta_op, checkpoint_snapshot_if_receipt_ok};
use crate::quantum::{Runtime, RuntimeError};
use crate::receipt::digest_to_hex;
use kutha_common::{Event, EventId, Op, TransactionTime, ValidTime, blob_digest_matches};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};

fn parse_hex32(s: &str) -> Option<[u8; 32]> {
    if s.len() != 64 {
        return None;
    }
    let mut out = [0u8; 32];
    for i in 0..32 {
        out[i] = u8::from_str_radix(&s[2 * i..2 * i + 2], 16).ok()?;
    }
    Some(out)
}

fn sha256_bytes(bytes: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(bytes);
    h.finalize().into()
}

fn checkpoint_receipt_error(claimed: &str, actual: [u8; 32]) -> RuntimeError {
    let expected = parse_hex32(claimed).unwrap_or_else(|| {
        let mut h = Sha256::new();
        h.update(b"kutha-checkpoint-receipt");
        h.update(claimed.as_bytes());
        h.finalize().into()
    });
    RuntimeError::ReplayDivergence { expected, actual }
}

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
        // Bind reserved kutha:* so the returned fold matches live product queries
        // (as_of / known_at). Fingerprint is unchanged — it walks facts directly.
        let rebuilt = GraphFold::replay_with_dict(self.log.as_slice(), &self.dict);
        let expected = self.fold.fingerprint();
        let actual = rebuilt.fingerprint();
        if expected != actual {
            return Err(RuntimeError::ReplayDivergence { expected, actual });
        }
        Self::check_checkpoint_receipts(self.log.as_slice())?;
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
        // After the Behavior walk so a missing Horn blob stays ExecutionReplayFailed.
        for e in self.log.iter() {
            if let Op::StoreBlob { digest, bytes } = &e.op {
                if !blob_digest_matches(digest, bytes) {
                    return Err(RuntimeError::UnknownRuleVersion {
                        pin: digest.clone(),
                    });
                }
            }
        }
        Ok(rebuilt)
    }

    /// Snapshot-held walls + `ingested_at` vs log `Op::TtAnchor` (D-02). Not fingerprint input.
    /// `ingested_before`: on the open path, pass `snap.next_tt` so log anchors at or after
    /// that cut are tail and allowed. `None` keeps exact equality (`replay_check`).
    pub(crate) fn check_tt_anchors(
        snapshot_held: &std::collections::BTreeMap<u64, u64>,
        events: &[kutha_common::Event],
    ) -> Result<(), RuntimeError> {
        Self::check_tt_anchors_range(snapshot_held, events, None)
    }

    pub(crate) fn check_tt_anchors_range(
        snapshot_held: &std::collections::BTreeMap<u64, u64>,
        events: &[kutha_common::Event],
        ingested_before: Option<u64>,
    ) -> Result<(), RuntimeError> {
        let mut from_log = GraphFold::tt_anchors_from_events(events);
        if let Some(cut) = ingested_before {
            from_log.retain(|_, ingested_at| *ingested_at < cut);
        }
        if snapshot_held != &from_log {
            return Err(RuntimeError::ReplayDivergence {
                expected: tt_anchors_digest(snapshot_held),
                actual: tt_anchors_digest(&from_log),
            });
        }
        Ok(())
    }

    /// Receipt fields on `Op::Checkpoint` must match the blob Snapshot (M154).
    /// A checkpoint restores a prefix: only `Define` / kept `StoreBlob` /
    /// `QuantumOutcome` / `JustificationCite` may precede it (empty fold).
    fn check_checkpoint_receipts(events: &[Event]) -> Result<(), RuntimeError> {
        let mut blobs: HashMap<&str, &[u8]> = HashMap::new();
        let mut only_prefix_meta = true;
        for e in events {
            match &e.op {
                Op::StoreBlob { digest, bytes } => {
                    blobs.insert(digest.as_str(), bytes.as_slice());
                }
                Op::Checkpoint {
                    fingerprint_hex,
                    fact_count,
                    blob_digest,
                    ..
                } => {
                    if !only_prefix_meta {
                        return Err(checkpoint_receipt_error(fingerprint_hex, [0u8; 32]));
                    }
                    let Some(bytes) = blobs.get(blob_digest.as_str()).copied() else {
                        return Err(checkpoint_receipt_error(blob_digest, [0u8; 32]));
                    };
                    match checkpoint_snapshot_if_receipt_ok(
                        bytes,
                        blob_digest,
                        fingerprint_hex,
                        *fact_count,
                    ) {
                        Ok(snap) => {
                            let actual = snap.fold.fingerprint();
                            if digest_to_hex(&actual) != *fingerprint_hex {
                                return Err(checkpoint_receipt_error(fingerprint_hex, actual));
                            }
                        }
                        Err(()) => {
                            let actual = sha256_bytes(bytes);
                            return Err(checkpoint_receipt_error(fingerprint_hex, actual));
                        }
                    }
                    // Receipt consumed the empty prefix; a second Checkpoint is invalid.
                    only_prefix_meta = false;
                }
                op if checkpoint_prefix_meta_op(op) => {}
                _ => {
                    only_prefix_meta = false;
                }
            }
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
    /// and its immediate premise claim is live. `caused_by` Correct /
    /// CorrectInterval is premise-bearing: the original `event_id` is the
    /// premise claim (Assert leaf, or walk when that original is a Behavior).
    /// When the premise is itself a Behavior, eligibility walks `caused_by`
    /// through ancestor Behaviors with a visited [`EventId`] set (already-seen
    /// id is ineligible). Correct of an ineligible derived Behavior stays
    /// ineligible (M037).
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
        let (premise, walk_original) = match &cause.op {
            Op::Assert { claim, .. } => (claim.unwrap_or(cause.id), None),
            Op::Behavior { .. } => (cause.id, Some(cause.id)),
            Op::Correct { event_id, .. } | Op::CorrectInterval { event_id, .. } => {
                let original = *event_id;
                let walk = matches!(
                    self.event_by_id(original).map(|e| &e.op),
                    Some(Op::Behavior { .. })
                );
                (original, walk.then_some(original))
            }
            Op::Retract { .. }
            | Op::Define { .. }
            | Op::QuantumOutcome { .. }
            | Op::JustificationCite { .. }
            | Op::StoreBlob { .. }
            | Op::TtAnchor { .. }
            | Op::Checkpoint { .. } => return false,
        };
        if !self.fold.rule_hash_live_at(&rule_version, tt, vt) {
            return false;
        }
        if !self.fold.claim_supported_at(premise, tt, vt) {
            return false;
        }
        if let Some(ev) = self.event_by_id(derived) {
            if matches!(ev.op, Op::Behavior { .. }) && !self.horn_behavior_reproducible(ev) {
                return false;
            }
        }
        match walk_original {
            Some(id) => self.derivation_eligible_at_walk(id, tt, vt, visited),
            None => true,
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
