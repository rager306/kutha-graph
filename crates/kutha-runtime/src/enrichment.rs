//! Bounded C5 enrichment proposals (ADR-052).
//!
//! An enrichment proposal is a typed log record: a content-addressed payload
//! plus the source revisions and model/prompt/cache identity needed for
//! replay. It is **derived** data. It never creates, supersedes, or invalidates
//! a kernel fact, and the runtime never calls a model or builds a derived
//! lease. `enrichment_ledger` is a read model over admitted proposals.

use crate::pack_lifecycle::{PackLifecycleState, derive_pack_lifecycle};
use kutha_common::{Event, EventId, Op, sha256_hex};
use std::collections::{BTreeMap, HashMap};

/// One admitted enrichment proposal with its pack lifecycle state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnrichmentRecord {
    pub pack_id: String,
    pub pack_version: String,
    pub source_event_ids: Vec<EventId>,
    pub model: String,
    pub prompt: String,
    pub cache_key: String,
    pub blob_digest: String,
    pub state: PackLifecycleState,
}

/// Enrichment rejection. Every variant is fail-closed before an append.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EnrichmentError {
    UnknownPack { pack_id: String },
    RetiredPack { pack_id: String },
    PackNotActive { pack_id: String },
    VersionMismatch { pack_id: String },
    EmptyField { field: &'static str },
    MissingSource { event_id: EventId },
    MissingBlob { digest: String },
    DigestMismatch { digest: String },
    DuplicateProposal { cache_key: String },
}

impl std::fmt::Display for EnrichmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownPack { pack_id } => {
                write!(f, "enrichment pack {pack_id} is not installed")
            }
            Self::RetiredPack { pack_id } => write!(f, "enrichment pack {pack_id} is retired"),
            Self::PackNotActive { pack_id } => {
                write!(f, "enrichment pack {pack_id} is installed but not active")
            }
            Self::VersionMismatch { pack_id } => {
                write!(f, "enrichment pack {pack_id} version is not active")
            }
            Self::EmptyField { field } => write!(f, "enrichment {field} is empty"),
            Self::MissingSource { event_id } => {
                write!(f, "enrichment source event {event_id} is not in the log")
            }
            Self::MissingBlob { digest } => {
                write!(f, "enrichment payload blob {digest} is missing")
            }
            Self::DigestMismatch { digest } => {
                write!(f, "enrichment payload bytes do not match digest {digest}")
            }
            Self::DuplicateProposal { cache_key } => {
                write!(f, "enrichment cache key {cache_key} was already proposed")
            }
        }
    }
}

impl std::error::Error for EnrichmentError {}

/// Validate one proposal against the current log and blob set.
pub fn validate_proposal(
    log: &[Event],
    blobs: &HashMap<String, Vec<u8>>,
    proposal: &Op,
) -> Result<(), EnrichmentError> {
    validate_proposal_with_payload(log, blobs, proposal, None)
}

/// Validate a proposal whose payload has not been appended yet.
///
/// `pending` is the `(digest, bytes)` the caller is about to store. It is used
/// only when the digest is absent from `blobs`, so a rejected proposal does not
/// leave an orphan `StoreBlob` on the log.
pub fn validate_proposal_with_payload(
    log: &[Event],
    blobs: &HashMap<String, Vec<u8>>,
    proposal: &Op,
    pending: Option<(&str, &[u8])>,
) -> Result<(), EnrichmentError> {
    validate_proposal_inner(log, blobs, proposal, pending, true)
}

/// Validate with an explicit source-existence mode.
pub(crate) fn validate_proposal_inner(
    log: &[Event],
    blobs: &HashMap<String, Vec<u8>>,
    proposal: &Op,
    pending: Option<(&str, &[u8])>,
    require_sources: bool,
) -> Result<(), EnrichmentError> {
    let Op::EnrichmentProposal {
        pack_id,
        pack_version,
        source_event_ids,
        model,
        prompt,
        cache_key,
        blob_digest,
    } = proposal
    else {
        return Ok(());
    };
    for (field, value) in [
        ("pack_id", pack_id.as_str()),
        ("pack_version", pack_version.as_str()),
        ("model", model.as_str()),
        ("prompt", prompt.as_str()),
        ("cache_key", cache_key.as_str()),
        ("blob_digest", blob_digest.as_str()),
    ] {
        if value.trim().is_empty() || value != value.trim() {
            return Err(EnrichmentError::EmptyField { field });
        }
    }
    if source_event_ids.is_empty() {
        return Err(EnrichmentError::EmptyField {
            field: "source_event_ids",
        });
    }
    let lifecycle = derive_pack_lifecycle(log);
    let Some(record) = lifecycle.iter().find(|record| &record.pack_id == pack_id) else {
        return Err(EnrichmentError::UnknownPack {
            pack_id: pack_id.clone(),
        });
    };
    match record.state {
        PackLifecycleState::Active => {}
        PackLifecycleState::Retired => {
            return Err(EnrichmentError::RetiredPack {
                pack_id: pack_id.clone(),
            });
        }
        PackLifecycleState::Installed => {
            return Err(EnrichmentError::PackNotActive {
                pack_id: pack_id.clone(),
            });
        }
    }
    if &record.version != pack_version {
        return Err(EnrichmentError::VersionMismatch {
            pack_id: pack_id.clone(),
        });
    }
    if require_sources {
        let known: std::collections::HashSet<EventId> = log.iter().map(|event| event.id).collect();
        for event_id in source_event_ids {
            if !known.contains(event_id) {
                return Err(EnrichmentError::MissingSource {
                    event_id: *event_id,
                });
            }
        }
    }
    let pending_bytes = pending
        .filter(|(digest, _)| digest == blob_digest)
        .map(|(_, bytes)| bytes);
    let bytes = match (blobs.get(blob_digest), pending_bytes) {
        (Some(bytes), _) => bytes,
        (None, Some(bytes)) => bytes,
        (None, None) => {
            return Err(EnrichmentError::MissingBlob {
                digest: blob_digest.clone(),
            });
        }
    };
    if sha256_hex(bytes) != *blob_digest {
        return Err(EnrichmentError::DigestMismatch {
            digest: blob_digest.clone(),
        });
    }
    if log.iter().any(|event| {
        matches!(
            &event.op,
            Op::EnrichmentProposal { cache_key: existing, .. } if existing == cache_key
        )
    }) {
        return Err(EnrichmentError::DuplicateProposal {
            cache_key: cache_key.clone(),
        });
    }
    Ok(())
}

/// Derive the enrichment ledger in log order.
pub fn derive_enrichment_ledger(log: &[Event]) -> Vec<EnrichmentRecord> {
    let states: BTreeMap<String, PackLifecycleState> = derive_pack_lifecycle(log)
        .into_iter()
        .map(|record| (record.pack_id, record.state))
        .collect();
    let mut out = Vec::new();
    for event in log {
        let Op::EnrichmentProposal {
            pack_id,
            pack_version,
            source_event_ids,
            model,
            prompt,
            cache_key,
            blob_digest,
        } = &event.op
        else {
            continue;
        };
        let Some(state) = states.get(pack_id).copied() else {
            continue;
        };
        out.push(EnrichmentRecord {
            pack_id: pack_id.clone(),
            pack_version: pack_version.clone(),
            source_event_ids: source_event_ids.clone(),
            model: model.clone(),
            prompt: prompt.clone(),
            cache_key: cache_key.clone(),
            blob_digest: blob_digest.clone(),
            state,
        });
    }
    out
}
