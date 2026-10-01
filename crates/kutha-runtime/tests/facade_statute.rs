//! D-08 RED (PIPE-03/04): facade-only statute consumer.
//! Import only the public crate surface. Do not call emit, intern, fold, log, or store.
//! Never edit `.kutha/STATE.md`.

use kutha_runtime::{Runtime, RuntimeError};
use std::path::Path;

/// YearCe valid-time instants (`VALID_TIME_SCALE` = `TimeScale::YearCe`); not Unix epoch.
const T_OLD: u64 = 2015;
const T_NEW: u64 = 2021;
const VF_FIRST: u64 = 2010;
const VF_SECOND: u64 = 2020;

#[test]
fn statute_as_of_via_facade() -> Result<(), RuntimeError> {
    let mut rt = Runtime::default();
    rt.assert(
        "art-12-speed",
        "inForceAs",
        "50-kmh",
        VF_FIRST,
        Some(VF_SECOND),
    )?;
    rt.assert("art-12-speed", "inForceAs", "30-kmh", VF_SECOND, None)?;

    let old = rt.as_of(T_OLD);
    let new = rt.as_of(T_NEW);
    assert_ne!(old, new, "AS OF 2015 must differ from AS OF 2021");
    let _live = rt.live_at(u64::MAX, T_OLD);

    let jid = rt.record_justification("admit", T_OLD)?;
    let _action = rt.record_action(&jid, T_OLD)?;
    rt.derive("statute-fold", "art-12-speed", T_OLD)?;
    rt.correct("art-12-speed", "inForceAs", "50-kmh", VF_FIRST)?;
    rt.retract("art-12-speed", VF_FIRST)?;

    let dir = Path::new("tmp/p27-facade-statute");
    rt.persist(dir)?;
    let opened = Runtime::open(dir)?;
    let _fork = opened.fork_at(0);
    opened.replay_check()?;
    Ok(())
}
