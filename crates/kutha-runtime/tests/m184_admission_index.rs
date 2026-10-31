//! v1.77: admission uses the runtime operator index, in parity with replay.

use kutha_common::Op;
use kutha_runtime::{PackManifest, Runtime, RuntimeError, store};

fn install(rt: &mut Runtime, pack_id: &str, ops: &[&str]) -> Result<(), RuntimeError> {
    let manifest = PackManifest::new(
        pack_id,
        format!("pack-{pack_id}"),
        "1.0.0",
        ops.iter().map(|s| s.to_string()).collect(),
    );
    rt.emit_pack_install_with_manifest(pack_id, format!("pack-{pack_id}"), "1.0.0", Some(manifest))
        .map(|_| ())
}

#[test]
fn admission_rejects_collision_without_rescanning() {
    let mut rt = Runtime::default();
    install(&mut rt, "a", &["shared"]).expect("first");
    let err = install(&mut rt, "b", &["shared"]).expect_err("collision");
    assert!(matches!(
        err,
        RuntimeError::PackLifecycleRejected { reason, .. } if reason.contains("already declared")
    ));
    assert_eq!(rt.pack_lifecycle().len(), 1);
}

#[test]
fn admission_and_replay_reject_the_same_collision() {
    use kutha_common::Event;

    // Build the colliding log by hand: admission refuses to append it, so the
    // only way to reach replay is a foreign-written segment.
    let a = PackManifest::new("a", "pack-a", "1.0.0", vec!["shared".into()]);
    let b = PackManifest::new("b", "pack-b", "1.0.0", vec!["shared".into()]);
    let events = vec![
        Event::new(
            Op::StoreBlob {
                digest: a.digest(),
                bytes: a.canonical_bytes(),
            },
            0,
        ),
        Event::new(
            Op::PackInstall {
                pack_id: "a".into(),
                name: "pack-a".into(),
                version: "1.0.0".into(),
                manifest_digest: Some(a.digest()),
            },
            1,
        ),
        Event::new(
            Op::StoreBlob {
                digest: b.digest(),
                bytes: b.canonical_bytes(),
            },
            2,
        ),
        Event::new(
            Op::PackInstall {
                pack_id: "b".into(),
                name: "pack-b".into(),
                version: "1.0.0".into(),
                manifest_digest: Some(b.digest()),
            },
            3,
        ),
    ];
    let foreign = Runtime::from_dict_and_events(vec!["knows".into(), "knownBy".into()], events, 32)
        .expect("build");
    assert!(
        matches!(
            foreign.replay_check(),
            Err(RuntimeError::PackLifecycleRejected { reason, .. })
                if reason.contains("already declared")
        ),
        "replay must reject the same collision admission rejects"
    );
}

#[test]
fn retire_releases_names_for_admission() {
    let mut rt = Runtime::default();
    install(&mut rt, "a", &["normalize"]).expect("install");
    rt.emit_pack_activate("a", "1.0.0").expect("activate");
    rt.emit_pack_retire("a", "1.0.0").expect("retire");
    install(&mut rt, "b", &["normalize"]).expect("released");
    rt.replay_check().expect("replay agrees");
}

#[test]
fn index_is_rebuilt_on_open_and_fork() {
    let mut rt = Runtime::default();
    install(&mut rt, "a", &["shared"]).expect("install");
    let dir = std::path::PathBuf::from("tmp/m184-admission-index");
    std::fs::remove_dir_all(&dir).ok();
    rt.persist(&dir).expect("persist");
    let mut opened = Runtime::open(&dir).expect("open");
    let err = install(&mut opened, "b", &["shared"]).expect_err("collision after open");
    assert!(matches!(
        err,
        RuntimeError::PackLifecycleRejected { reason, .. } if reason.contains("already declared")
    ));

    let mut fork = rt.fork_at(rt.log().len()).expect("fork");
    let err = install(&mut fork, "b", &["shared"]).expect_err("collision after fork");
    assert!(matches!(
        err,
        RuntimeError::PackLifecycleRejected { reason, .. } if reason.contains("already declared")
    ));
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn index_survives_checkpoint_sealed_prefix() {
    struct Reset;
    impl Drop for Reset {
        fn drop(&mut self) {
            store::reset_test_segment_limits();
        }
    }
    let _reset = Reset;
    store::set_test_segment_limits(Some(4096), Some(2));

    let mut rt = Runtime::default();
    install(&mut rt, "a", &["shared"]).expect("install");
    let dir = std::path::PathBuf::from("tmp/m184-admission-index-cp");
    std::fs::remove_dir_all(&dir).ok();
    rt.persist(&dir).expect("persist");
    let article = rt.intern("article");
    let relation = rt.intern("relatedTo");
    let object = rt.intern("value");
    for _ in 0..6 {
        rt.emit(Op::Assert {
            subject: article,
            relation,
            object,
            valid_from: 2011,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .expect("assert");
    }
    rt.persist(&dir).expect("persist sealed");
    store::checkpoint_sealed_prefix(&dir).expect("checkpoint");
    let mut opened = Runtime::open(&dir).expect("open");
    let err = install(&mut opened, "b", &["shared"]).expect_err("collision after checkpoint");
    assert!(matches!(
        err,
        RuntimeError::PackLifecycleRejected { reason, .. } if reason.contains("already declared")
    ));
    std::fs::remove_dir_all(&dir).ok();
}

/// OPS-01: admission is indexed, so many installs are not quadratic.
#[test]
fn admission_scales_with_many_installs() {
    fn run(n: usize) -> std::time::Duration {
        let mut rt = Runtime::default();
        let start = std::time::Instant::now();
        for i in 0..n {
            let manifest = PackManifest::new(
                format!("p{i}"),
                format!("pack-p{i}"),
                "1.0.0",
                vec![format!("op{i}")],
            );
            rt.emit_pack_install_with_manifest(
                format!("p{i}"),
                format!("pack-p{i}"),
                "1.0.0",
                Some(manifest),
            )
            .expect("install");
        }
        start.elapsed()
    }

    let small = run(150);
    let large = run(600);
    assert!(
        large < small * 12 + std::time::Duration::from_millis(100),
        "4x installs took {large:?} vs {small:?}; admission looks super-linear"
    );
}
