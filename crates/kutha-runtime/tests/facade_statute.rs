//! D-04 GREEN (PIPE-03/04): facade-only statute consumer.
//! Import only the public crate surface. Never edit `.kutha/STATE.md`.

use kutha_common::ValidTime;

use kutha_runtime::{Runtime, RuntimeError};

fn facade_statute_store_dir() -> std::path::PathBuf {
    // Unique even when two Cargo processes run this binary together.
    // A fixed `tmp/p27-facade-statute` is removed by the other process mid-persist.
    static N: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let seq = N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .join(format!(
            "p27-facade-statute-{}-{nanos}-{seq}",
            std::process::id()
        ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("persist dir");
    dir
}

/// YearCe valid-time instants (`VALID_TIME_SCALE` = `TimeScale::YearCe`); not Unix epoch.
const T_OLD: u64 = 2015;
const T_NEW: u64 = 2021;
const VF_FIRST: u64 = 2010;
const VF_SECOND: u64 = 2020;

fn sorted_as_of(rt: &Runtime, vt: u64) -> Vec<(u32, u32, u32)> {
    let mut v = rt.as_of(ValidTime::from(vt));
    v.sort_unstable();
    v
}

fn sorted_live_at(rt: &Runtime, tt: u64, vt: u64) -> Vec<(u32, u32, u32)> {
    let mut v = rt.live_at(tt, vt);
    v.sort_unstable();
    v
}

fn triple(rt: &mut Runtime, s: &str, r: &str, o: &str) -> (u32, u32, u32) {
    (rt.intern(s), rt.intern(r), rt.intern(o))
}

#[test]
fn statute_as_of_via_facade() -> Result<(), RuntimeError> {
    let mut rt = Runtime::default();
    let t50 = triple(&mut rt, "art-12-speed", "inForceAs", "50-kmh");
    let t40 = triple(&mut rt, "art-12-speed", "inForceAs", "40-kmh");
    let t30 = triple(&mut rt, "art-12-speed", "inForceAs", "30-kmh");

    let first = rt.assert(
        "art-12-speed",
        "inForceAs",
        "50-kmh",
        VF_FIRST,
        Some(VF_SECOND),
    )?;
    rt.assert("art-12-speed", "inForceAs", "30-kmh", VF_SECOND, None)?;

    let old = rt.as_of(ValidTime::from(T_OLD));
    let new = rt.as_of(ValidTime::from(T_NEW));
    assert_ne!(old, new, "AS OF 2015 must differ from AS OF 2021");
    assert_eq!(sorted_as_of(&rt, T_OLD), vec![t50]);
    assert_eq!(sorted_as_of(&rt, T_NEW), vec![t30]);

    rt.emit_policy("leased-policy", VF_FIRST, None)?;
    let claim = first.receipt.event_ids[0];
    let jid = rt.record_justification(claim, vec![claim], vec![claim], "", u64::MAX, T_OLD)?;
    let _action = rt.record_action(&jid, u64::MAX, T_OLD)?;
    rt.derive("statute-fold", "art-12-speed", T_OLD)?;

    rt.correct("art-12-speed", "inForceAs", "40-kmh", VF_FIRST)?;
    let after_correct_2015 = vec![t40, t30];
    let after_correct_2021 = vec![t30, t30];
    assert_eq!(sorted_as_of(&rt, T_OLD), after_correct_2015);
    assert_eq!(sorted_as_of(&rt, T_NEW), after_correct_2021);
    assert_eq!(sorted_live_at(&rt, u64::MAX, T_OLD), after_correct_2015);
    assert_eq!(sorted_live_at(&rt, u64::MAX, T_NEW), after_correct_2021);
    assert!(
        !sorted_as_of(&rt, T_OLD).contains(&t50),
        "Correct must replace 50-kmh at AS OF 2015"
    );
    assert!(
        sorted_as_of(&rt, T_OLD).contains(&t40),
        "Correct must surface 40-kmh at AS OF 2015"
    );

    rt.retract("art-12-speed", VF_FIRST)?;
    let after_retract_2015 = vec![t30];
    let after_retract_2021 = vec![t30, t30];
    assert_eq!(sorted_as_of(&rt, T_OLD), after_retract_2015);
    assert_eq!(sorted_as_of(&rt, T_NEW), after_retract_2021);
    assert_eq!(sorted_live_at(&rt, u64::MAX, T_OLD), after_retract_2015);
    assert_eq!(sorted_live_at(&rt, u64::MAX, T_NEW), after_retract_2021);

    let pre_persist_2015 = sorted_as_of(&rt, T_OLD);
    let pre_persist_2021 = sorted_as_of(&rt, T_NEW);

    let dir = facade_statute_store_dir();
    rt.persist(&dir)?;
    let opened = Runtime::open(&dir)?;
    assert_eq!(sorted_as_of(&opened, T_OLD), pre_persist_2015);
    assert_eq!(sorted_as_of(&opened, T_NEW), pre_persist_2021);

    let fork = opened.fork_at(0).unwrap();
    assert_eq!(sorted_as_of(&fork, T_OLD), vec![] as Vec<(u32, u32, u32)>);
    assert_eq!(sorted_as_of(&fork, T_NEW), vec![]);

    opened.replay_check()?;
    let _ = std::fs::remove_dir_all(dir);
    Ok(())
}

#[test]
fn facade_statute_store_dirs_differ_across_calls() {
    let a = facade_statute_store_dir();
    let b = facade_statute_store_dir();
    assert_ne!(a, b, "parallel cargo must not share one persist directory");
    assert!(
        !a.ends_with("p27-facade-statute"),
        "fixed tmp/p27-facade-statute races across processes"
    );
    let _ = std::fs::remove_dir_all(a);
    let _ = std::fs::remove_dir_all(b);
}
