//! Droppable C1 lifecycle lease derived from admitted D021 events.
//!
//! This module does not load pack artifacts, register operators, rebuild
//! dictionaries/leases, or execute pack code. It is a deterministic read model
//! over lifecycle log events.

use kutha_common::{Event, Op};

/// Replay-derived lifecycle state for one pack id.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackLifecycle {
    pub pack_id: String,
    pub name: String,
    pub version: String,
    pub state: PackLifecycleState,
    pub event_ids: Vec<kutha_common::EventId>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PackLifecycleState {
    Installed,
    Active,
    Retired,
}

impl PackLifecycleState {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Installed => "installed",
            Self::Active => "active",
            Self::Retired => "retired",
        }
    }
}

/// Derive lifecycle records in first-install order.
pub fn derive_pack_lifecycle(events: &[Event]) -> Vec<PackLifecycle> {
    let mut order: Vec<String> = Vec::new();
    let mut records: std::collections::BTreeMap<String, PackLifecycle> =
        std::collections::BTreeMap::new();
    for event in events {
        match &event.op {
            Op::PackInstall {
                pack_id,
                name,
                version,
            } => {
                if records
                    .insert(
                        pack_id.clone(),
                        PackLifecycle {
                            pack_id: pack_id.clone(),
                            name: name.clone(),
                            version: version.clone(),
                            state: PackLifecycleState::Installed,
                            event_ids: vec![event.id],
                        },
                    )
                    .is_none()
                {
                    order.push(pack_id.clone());
                }
            }
            Op::PackActivate { pack_id, version } => {
                if let Some(record) = records.get_mut(pack_id) {
                    record.version = version.clone();
                    record.state = PackLifecycleState::Active;
                    record.event_ids.push(event.id);
                }
            }
            Op::PackRetire { pack_id, version } => {
                if let Some(record) = records.get_mut(pack_id) {
                    record.version = version.clone();
                    record.state = PackLifecycleState::Retired;
                    record.event_ids.push(event.id);
                }
            }
            _ => {}
        }
    }
    order
        .into_iter()
        .filter_map(|pack_id| records.remove(&pack_id))
        .collect()
}
