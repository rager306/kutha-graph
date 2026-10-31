//! Droppable C1 lifecycle lease derived from admitted D021 events.
//!
//! This module does not load pack artifacts, register operators, rebuild
//! dictionaries/leases, or execute pack code. It is a deterministic read model
//! over lifecycle log events.

use kutha_common::{Event, Op};

thread_local! {
    /// Test seam: actual `replay` executions on this thread. A counter stored on
    /// the tracker cannot see work done by a per-prefix rebuild, because the
    /// rebuilt tracker starts at zero; this counter sees every call.
    static REPLAY_CALLS: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// Test seam: replay calls executed on this thread since the last reset.
#[doc(hidden)]
pub fn thread_replay_calls() -> u64 {
    REPLAY_CALLS.with(std::cell::Cell::get)
}

/// Test seam: reset this thread's replay counter.
#[doc(hidden)]
pub fn reset_thread_replay_calls() {
    REPLAY_CALLS.with(|calls| calls.set(0));
}

/// Replay-derived lifecycle state for one pack id.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackLifecycle {
    pub pack_id: String,
    pub name: String,
    pub version: String,
    pub state: PackLifecycleState,
    pub event_ids: Vec<kutha_common::EventId>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
                ..
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

/// Incremental lifecycle state machine, shared by admission and replay.
///
/// Admission builds it from the log once and applies one op; replay applies
/// every op in order. Keeping one implementation prevents the two gates from
/// drifting apart.
#[derive(Clone, Debug, Default)]
pub struct PackLifecycleTracker {
    state: std::collections::BTreeMap<String, (PackLifecycleState, String)>,
    replay_calls: u64,
}

impl PackLifecycleTracker {
    pub fn new() -> Self {
        Self::default()
    }

    /// Replay every lifecycle event in `events`, failing on an invalid one.
    pub fn from_log(events: &[Event]) -> Result<Self, crate::RuntimeError> {
        let mut tracker = Self::new();
        for event in events {
            tracker.replay(&event.op)?;
        }
        Ok(tracker)
    }

    /// Validate one op against the current state without mutating it.
    ///
    /// Admission uses this so it does not rebuild the tracker per write;
    /// [`Self::replay`] then applies the same op after the check.
    pub fn check(&self, op: &Op) -> Result<(), crate::RuntimeError> {
        match op {
            Op::PackInstall {
                pack_id,
                name,
                version,
                ..
            } => {
                if pack_id.trim().is_empty() || version.trim().is_empty() || name.trim().is_empty()
                {
                    return Err(reject(pack_id, "empty_identifier"));
                }
                if pack_id != pack_id.trim() || version != version.trim() || name != name.trim() {
                    return Err(reject(pack_id, "padded_identifier"));
                }
                if self.state.contains_key(pack_id) {
                    return Err(reject(pack_id, "pack_id_already_known"));
                }
                Ok(())
            }
            Op::PackActivate { pack_id, version } | Op::PackRetire { pack_id, version } => {
                let retire = matches!(op, Op::PackRetire { .. });
                if pack_id.trim().is_empty() || version.trim().is_empty() {
                    return Err(reject(pack_id, "empty_identifier"));
                }
                if pack_id != pack_id.trim() || version != version.trim() {
                    return Err(reject(pack_id, "padded_identifier"));
                }
                let allowed = match self.state.get(pack_id) {
                    Some((
                        PackLifecycleState::Installed | PackLifecycleState::Active,
                        installed,
                    )) if retire && installed == version => true,
                    Some((PackLifecycleState::Installed, installed)) if installed == version => {
                        true
                    }
                    _ => false,
                };
                if !allowed {
                    return Err(reject(
                        pack_id,
                        if retire {
                            "retire_requires_same_installed_version"
                        } else {
                            "activate_requires_same_installed_version"
                        },
                    ));
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// Apply one op if it is a lifecycle op; other ops are ignored.
    pub fn replay(&mut self, op: &Op) -> Result<(), crate::RuntimeError> {
        REPLAY_CALLS.with(|calls| calls.set(calls.get().saturating_add(1)));
        self.check(op)?;
        self.replay_calls += 1;
        match op {
            Op::PackInstall {
                pack_id, version, ..
            } => {
                self.state.insert(
                    pack_id.clone(),
                    (PackLifecycleState::Installed, version.clone()),
                );
            }
            Op::PackActivate { pack_id, version } | Op::PackRetire { pack_id, version } => {
                let next = if matches!(op, Op::PackRetire { .. }) {
                    PackLifecycleState::Retired
                } else {
                    PackLifecycleState::Active
                };
                self.state.insert(pack_id.clone(), (next, version.clone()));
            }
            _ => {}
        }
        Ok(())
    }

    pub fn state(&self, pack_id: &str) -> Option<PackLifecycleState> {
        self.state.get(pack_id).map(|(state, _)| *state)
    }

    pub fn version(&self, pack_id: &str) -> Option<&str> {
        self.state.get(pack_id).map(|(_, version)| version.as_str())
    }

    /// Ops applied so far. Test seam for the replay-scaling oracles: they
    /// assert one replay per op rather than measuring wall-clock.
    #[doc(hidden)]
    pub fn replay_calls(&self) -> u64 {
        self.replay_calls
    }
}

fn reject(pack_id: &str, reason: &str) -> crate::RuntimeError {
    crate::RuntimeError::PackLifecycleRejected {
        pack_id: pack_id.to_string(),
        reason: reason.to_string(),
    }
}
