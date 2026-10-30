//! C1b: pack manifest declaration and fail-closed admission.

use kutha_common::Op;
use kutha_runtime::{
    PACK_MANIFEST_SCHEMA, PackManifest, PackManifestError, Runtime, RuntimeError,
    valid_operator_name,
};

fn runtime() -> Runtime {
    Runtime::default()
}

fn install(rt: &mut Runtime, id: &str, ops: &[&str]) -> Result<(), RuntimeError> {
    let manifest = PackManifest::new(
        id,
        format!("pack-{id}"),
        "1.0.0",
        ops.iter().map(|s| s.to_string()).collect(),
    );
    rt.emit_pack_install_with_manifest(id, format!("pack-{id}"), "1.0.0", Some(manifest))
        .map(|_| ())
}

#[test]
fn manifest_is_content_addressed_and_roundtrips() {
    let manifest = PackManifest::new("core", "pack-core", "1.0.0", vec!["normalize".into()]);
    let bytes = manifest.canonical_bytes();
    assert_eq!(manifest.digest(), kutha_common::sha256_hex(&bytes));
    let parsed = PackManifest::parse(&bytes).expect("valid manifest");
    assert_eq!(parsed, manifest);
    assert_eq!(parsed.schema, PACK_MANIFEST_SCHEMA);
    assert!(valid_operator_name("normalize"));
    assert!(!valid_operator_name("kutha:allows"));
    assert!(!valid_operator_name("bad name"));
}

#[test]
fn install_with_manifest_declares_operators() {
    let mut rt = runtime();
    install(&mut rt, "core", &["normalize", "cite"]).expect("install");
    let registry = rt.pack_operators().expect("registry");
    assert_eq!(registry.len(), 2);
    assert_eq!(registry[0].pack_id, "core");
    assert_eq!(registry[0].operator, "normalize");
    assert_eq!(registry[1].operator, "cite");
    assert_eq!(registry[0].state.as_str(), "installed");
}

#[test]
fn install_without_manifest_stays_legal_and_has_no_operators() {
    let mut rt = runtime();
    rt.emit_pack_install("legacy", "pack-legacy", "1.0.0")
        .expect("legacy install");
    assert!(rt.pack_operators().expect("registry").is_empty());
    assert_eq!(rt.pack_lifecycle().len(), 1);
}

#[test]
fn padded_pack_identifiers_fail_closed() {
    let mut rt = runtime();
    let before = rt.log().len();
    let err = rt
        .emit_pack_install(" core", "pack-core", "1.0.0")
        .expect_err("padded pack id");
    assert!(matches!(
        err,
        RuntimeError::PackLifecycleRejected { reason, .. } if reason == "padded_identifier"
    ));
    assert_eq!(rt.log().len(), before);
}

#[test]
fn manifest_identity_mismatch_fails_closed() {
    let mut rt = runtime();
    let before = rt.log().len();
    let manifest = PackManifest::new("other", "pack-core", "1.0.0", vec![]);
    let err = rt
        .emit_pack_install_with_manifest("core", "pack-core", "1.0.0", Some(manifest))
        .expect_err("identity mismatch");
    assert!(matches!(
        err,
        RuntimeError::PackLifecycleRejected { reason, .. } if reason.contains("pack_id")
    ));
    assert_eq!(rt.log().len(), before);
}

#[test]
fn malformed_and_reserved_manifests_fail_closed() {
    let mut bad = PackManifest::new("core", "pack-core", "1.0.0", vec!["kutha:allows".into()]);
    assert!(matches!(
        bad.validate_shape(),
        Err(PackManifestError::InvalidOperator { .. })
    ));
    bad.operators = vec!["dup".into(), "dup".into()];
    assert!(matches!(
        bad.validate_shape(),
        Err(PackManifestError::DuplicateOperator { .. })
    ));
    bad.operators = vec!["  ".into()];
    assert!(matches!(
        bad.validate_shape(),
        Err(PackManifestError::EmptyOperator)
    ));
    bad.operators = vec![" normalize ".into()];
    assert!(matches!(
        bad.validate_shape(),
        Err(PackManifestError::InvalidOperator { .. })
    ));
    assert!(matches!(
        PackManifest::parse(b"not json"),
        Err(PackManifestError::Malformed)
    ));
}

#[test]
fn cross_pack_operator_collision_fails_closed() {
    let mut rt = runtime();
    install(&mut rt, "core", &["normalize"]).expect("first install");
    let err = install(&mut rt, "extra", &["normalize"]).expect_err("collision");
    assert!(matches!(
        err,
        RuntimeError::PackLifecycleRejected { reason, .. } if reason.contains("already declared")
    ));
    assert_eq!(rt.pack_lifecycle().len(), 1);
}

#[test]
fn retired_pack_releases_operator_names() {
    let mut rt = runtime();
    install(&mut rt, "core", &["normalize"]).expect("install");
    rt.emit_pack_activate("core", "1.0.0").expect("activate");
    rt.emit_pack_retire("core", "1.0.0").expect("retire");
    install(&mut rt, "extra", &["normalize"]).expect("name released");
    assert_eq!(rt.pack_lifecycle().len(), 2);
    assert_eq!(rt.pack_operators().expect("registry").len(), 2);
}

#[test]
fn manifest_ops_do_not_change_graph_facts() {
    let mut rt = runtime();
    let before = rt.fold().fingerprint();
    let facts_before = rt.fold().facts().len();
    install(&mut rt, "core", &["normalize"]).expect("install");
    assert_eq!(rt.fold().fingerprint(), before);
    assert_eq!(rt.fold().facts().len(), facts_before);
}

#[test]
fn user_forged_install_with_manifest_digest_is_rejected() {
    let mut rt = runtime();
    let err = rt
        .emit(Op::PackInstall {
            pack_id: "core".into(),
            name: "pack-core".into(),
            version: "1.0.0".into(),
            manifest_digest: Some("00".repeat(32)),
        })
        .expect_err("user forged");
    assert!(matches!(err, RuntimeError::MetaOpRejected));
}
