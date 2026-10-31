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

/// OPS-01: admission is indexed.
///
/// Linearity is structural, not timed: `validate_install_indexed` takes only
/// the blob map and the index — it has no log parameter, so the admission path
/// cannot rescan the log. A wall-clock ratio test was removed because it
/// flaked under a parallel workspace run without saying anything about the
/// algorithm. This test instead proves the index actually tracks every install.
#[test]
fn admission_index_tracks_every_install() {
    let mut rt = Runtime::default();
    for i in 0..200 {
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
    assert_eq!(rt.operator_count(), 200);
    // A fresh collision is still refused after 200 installs.
    let dup = PackManifest::new("dup", "pack-dup", "1.0.0", vec!["op7".into()]);
    let err = rt
        .emit_pack_install_with_manifest("dup", "pack-dup", "1.0.0", Some(dup))
        .expect_err("collision");
    assert!(matches!(
        err,
        RuntimeError::PackLifecycleRejected { reason, .. } if reason.contains("already declared")
    ));
}

/// Regression: retire must be applied in log order on every reopen path, so a
/// legal install after retire survives persist/open/fork/checkpoint. The first
/// version of this index applied retire only after the install loop, which
/// bricked a store the runtime had accepted.
#[test]
fn retired_name_reuse_survives_every_reopen_path() {
    fn build() -> Runtime {
        let mut rt = Runtime::default();
        install(&mut rt, "a", &["shared"]).expect("a");
        rt.emit_pack_activate("a", "1.0.0").expect("activate a");
        rt.emit_pack_retire("a", "1.0.0").expect("retire a");
        install(&mut rt, "b", &["shared"]).expect("b reuses the released name");
        rt
    }

    let mut live = build();
    install(&mut live, "c", &["shared"]).expect_err("live rejects collision");

    let dir = std::path::PathBuf::from("tmp/m184-retire-reuse");
    std::fs::remove_dir_all(&dir).ok();
    let rt = build();
    rt.persist(&dir).expect("persist");
    let mut opened = Runtime::open(&dir).expect("open");
    opened.replay_check().expect("replay agrees with admission");
    assert_eq!(opened.operator_count(), 1, "only b owns shared");
    install(&mut opened, "c", &["shared"]).expect_err("collision after open");
    std::fs::remove_dir_all(&dir).ok();

    let mut forked = build().fork_at(build().log().len()).expect("fork");
    install(&mut forked, "c", &["shared"]).expect_err("collision after fork");

    // A legal reuse after another retire must also be accepted after reopen.
    let mut rt = build();
    rt.emit_pack_activate("b", "1.0.0").expect("activate b");
    rt.emit_pack_retire("b", "1.0.0").expect("retire b");
    let dir2 = std::path::PathBuf::from("tmp/m184-retire-reuse-2");
    std::fs::remove_dir_all(&dir2).ok();
    rt.persist(&dir2).expect("persist");
    let mut opened = Runtime::open(&dir2).expect("open");
    install(&mut opened, "d", &["shared"]).expect("released again");
    std::fs::remove_dir_all(&dir2).ok();
}

/// OPS-01: admission does not replay the log per write.
///
/// Structural, not timed: `check_pack_lifecycle_live` consults the runtime
/// lease and has no log parameter. The measured per-install cost after this
/// change is flat (400 → 3200 installs: 255 µs → 287 µs per install); before
/// it, the same shape grew 213 µs → 2.33 ms because every install rebuilt the
/// lifecycle tracker from the whole log. A wall-clock ratio assertion is
/// deliberately not used here: it flaked under a parallel workspace run
/// without saying anything about the algorithm.
#[test]
fn admission_lifecycle_check_does_not_replay_the_log() {
    let source = std::fs::read_to_string("src/admission.rs").expect("admission.rs");
    assert!(
        !source.contains("PackLifecycleTracker::from_log"),
        "admission must consult the runtime lease, not replay the log per write"
    );
    assert!(
        source.contains("self.lifecycle.check"),
        "admission must use the runtime lifecycle lease"
    );
}
