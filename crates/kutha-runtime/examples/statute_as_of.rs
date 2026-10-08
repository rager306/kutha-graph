//! D-04 GREEN (PIPE-04): statute AS OF via the public Runtime facade only.

use kutha_common::{ValidTime};

use kutha_runtime::{Runtime, RuntimeError};
use std::path::Path;

fn main() -> Result<(), RuntimeError> {
    let mut rt = Runtime::default();
    let first = rt.assert("art-12-speed", "inForceAs", "50-kmh", 2010, Some(2020))?;
    rt.assert("art-12-speed", "inForceAs", "30-kmh", 2020, None)?;

    let old = rt.as_of(ValidTime::from(2015));
    let new = rt.as_of(ValidTime::from(2021));
    assert_ne!(old, new, "AS OF 2015 must differ from AS OF 2021");
    let _live = rt.live_at(u64::MAX, 2015);

    rt.emit_policy("leased-policy", 2010, None)?;
    let claim = first.receipt.event_ids[0];
    let jid = rt.record_justification(claim, vec![claim], vec![claim], "", u64::MAX, 2015)?;
    let _action = rt.record_action(&jid, u64::MAX, 2015)?;
    rt.derive("statute-fold", "art-12-speed", 2015)?;
    rt.correct("art-12-speed", "inForceAs", "50-kmh", 2010)?;
    rt.retract("art-12-speed", 2010)?;

    let dir = Path::new("tmp/p27-ex-statute");
    rt.persist(dir)?;
    let opened = Runtime::open(dir)?;
    let _fork = opened.fork_at(0);
    opened.replay_check()?;
    Ok(())
}
