use kutha_common::EventId;
use sha2::{Digest, Sha256};

/// Constant-shape evidence for one emit→idle quantum (ADR-014). Not a blockchain SoT.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuantumReceipt {
    pub event_ids: Vec<EventId>,
    pub digest: [u8; 32],
    pub aborted_on_budget: bool,
}

impl QuantumReceipt {
    pub fn from_events(ids: Vec<EventId>, event_digests: &[[u8; 32]], aborted: bool) -> Self {
        let mut h = Sha256::new();
        for d in event_digests {
            h.update(d);
        }
        h.update([u8::from(aborted)]);
        Self {
            event_ids: ids,
            digest: h.finalize().into(),
            aborted_on_budget: aborted,
        }
    }
}
