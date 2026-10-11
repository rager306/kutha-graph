//! M053 S01 — CE whole-repo (2026-10-11) regression pack.
//! Pins advisor withdrawals + closed finding classes so they cannot silent-regress.
//! Open residual classes are matrix `gap` rows owned by M054 slices (not leased here).

mod common;

use std::path::{Path, PathBuf};

use common::sem::persistence::{
    PersistLayout, persist_dir, rm_dir, sealed_count, try_persist_with_layout,
};
use kutha_common::Op;
use kutha_runtime::{Runtime, store};

fn manifest_tmp_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .canonicalize()
        .unwrap_or_else(|_| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .join("tmp")
        })
}

fn path_under(root: &Path, child: &Path) -> bool {
    let Ok(root) = root.canonicalize() else {
        return child.starts_with(root);
    };
    let Ok(child) = child.canonicalize() else {
        return child.starts_with(&root);
    };
    child.starts_with(root)
}

/// Advisor withdrawal: hydrate rebuilds outcomes from log QuantumOutcome events
/// (not a droppable sidecar-only lease).
#[test]
fn fork_at_rebuilds_outcomes_from_log_quantum_outcomes() {
    let mut rt = Runtime::default();
    let alice = rt.intern("Alice");
    let bob = rt.intern("Bob");
    let knows = rt.intern("knows");
    rt.emit(Op::Assert {
        subject: alice,
        relation: knows,
        object: bob,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .expect("knows assert");

    let qo_in_log = rt
        .log()
        .iter()
        .filter(|e| matches!(e.op, Op::QuantumOutcome { .. }))
        .count();
    assert!(
        qo_in_log >= 1,
        "user emit must persist Op::QuantumOutcome on the log"
    );
    assert_eq!(rt.outcome_records().len(), qo_in_log);

    // Wipe the in-memory lease — hydrate must rebuild from the log SoT.
    rt.attach_outcomes(Vec::new());
    assert!(rt.outcome_records().is_empty());

    let rebuilt = rt.fork_at(rt.log().len());
    assert_eq!(
        rebuilt.outcome_records().len(),
        qo_in_log,
        "fork_at/hydrate_from_log must rebuild outcomes from log QuantumOutcome rows"
    );
}

/// Advisor withdrawal: shared persist_dir is anchored under the crate tree, not process CWD.
#[test]
fn common_persist_dir_anchors_under_manifest_tmp_not_cwd() {
    let dir = persist_dir("m053-anchor");
    let tmp_root = manifest_tmp_root();
    assert!(
        path_under(&tmp_root, &dir),
        "persist_dir must live under CARGO_MANIFEST_DIR/../../tmp, got {dir:?} (tmp_root={tmp_root:?})"
    );
    let cwd = std::env::current_dir().expect("cwd");
    if cwd.canonicalize().ok().as_ref() != tmp_root.canonicalize().ok().as_ref() {
        // When cwd is not the tmp root, a CWD-only helper would not nest under tmp_root.
        assert!(
            path_under(&tmp_root, &dir),
            "must not be CWD-coupled when cwd={cwd:?}"
        );
    }
    assert!(
        dir.file_name()
            .and_then(|s| s.to_str())
            .is_some_and(|n| n.contains(&std::process::id().to_string())),
        "persist_dir label must include pid for shard uniqueness: {dir:?}"
    );
    rm_dir(&dir);
}

/// Advisor withdrawal: try_persist_with_layout keeps the requested layout contract
/// (Rollover ok iff sealed>=2; reject ≠ oracle fail).
#[test]
fn try_persist_with_layout_preserves_rollover_seam_contract() {
    let rt = Runtime::default();
    let dir = persist_dir("m053-roll-seam");
    let ok = try_persist_with_layout(&rt, &dir, PersistLayout::Rollover);
    let sealed = if dir.join(store::MANIFEST_REL).exists() {
        sealed_count(&dir)
    } else {
        0
    };
    assert_eq!(
        ok,
        sealed >= 2,
        "Rollover must accept only when seam sealed>=2; ok={ok} sealed={sealed}"
    );
    rm_dir(&dir);
}

/// Closed finding #1 class (documented break): all five removed meta-ops fail open.
#[test]
fn all_five_legacy_dictionary_meta_ops_fail_deserialize() {
    let samples = [
        r#"{"AllowRelation":{"name":"relatedTo","valid_from":0,"valid_to":null}}"#,
        r#"{"RegisterRule":{"definition":"R","valid_from":0,"valid_to":null}}"#,
        r#"{"PinPolicy":{"definition":"P","valid_from":0,"valid_to":null}}"#,
        r#"{"RecordAdmission":{"justification_id":"j","admitted":true,"policy_version":"p","valid_from":0,"valid_to":null}}"#,
        r#"{"RecordAction":{"justification_id":"j","target_claim":"00000000-0000-0000-0000-000000000000","source_claim_ids":[],"source_event_ids":[],"rule_version":"r","admitted":true,"policy_version":"p","valid_from":0,"valid_to":null}}"#,
    ];
    for raw in samples {
        assert!(
            serde_json::from_str::<kutha_common::Op>(raw).is_err(),
            "legacy meta-op must fail open (documented break): {raw}"
        );
    }
}
