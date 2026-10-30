//! Droppable C1b operator registry derived from admitted pack manifests.
//!
//! Read model only: no operator is executed, loaded, or resolved to code.
//! Retired packs keep their rows for audit but are reported as retired.

use crate::pack_lifecycle::{PackLifecycleState, derive_pack_lifecycle};
use crate::pack_manifest::{PackManifest, load_manifest};
use kutha_common::Event;
use std::collections::{BTreeMap, HashMap};

/// One declared operator with its owning pack lifecycle state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackOperator {
    pub pack_id: String,
    pub version: String,
    pub operator: String,
    pub state: PackLifecycleState,
}

/// Derive registry rows in pack-install order, then manifest operator order.
pub fn derive_pack_operators(
    log: &[Event],
    blobs: &HashMap<String, Vec<u8>>,
) -> Result<Vec<PackOperator>, crate::pack_manifest::PackManifestError> {
    let states: BTreeMap<String, PackLifecycleState> = derive_pack_lifecycle(log)
        .into_iter()
        .map(|record| (record.pack_id, record.state))
        .collect();
    let mut out = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for event in log {
        let kutha_common::Op::PackInstall {
            pack_id,
            name,
            version,
            manifest_digest: Some(digest),
        } = &event.op
        else {
            continue;
        };
        let Some(state) = states.get(pack_id).copied() else {
            continue;
        };
        let manifest: PackManifest = load_manifest(blobs, digest)?;
        manifest.validate_identity(pack_id, name, version)?;
        for operator in manifest.operators {
            if seen.insert((pack_id.clone(), operator.clone())) {
                out.push(PackOperator {
                    pack_id: pack_id.clone(),
                    version: version.clone(),
                    operator,
                    state,
                });
            }
        }
    }
    Ok(out)
}
