//! v1.75: adversarial tests for the meta surfaces (C1/C1b/C5 + replay gate).
//!
//! Each test is the falsifier for a mitigated threat in
//! `.planning/phases/269-meta-surface-threat-audit-and-adversarial-tests/269-SECURITY.md`.

use kutha_common::{Event, Op};
use kutha_runtime::{PackManifest, Runtime, RuntimeError};

fn dict() -> Vec<String> {
    vec!["knows".into(), "knownBy".into()]
}

fn runtime(events: Vec<Event>) -> Runtime {
    Runtime::from_dict_and_events(dict(), events, 32).expect("build")
}

fn install(pack_id: &str, digest: Option<String>) -> Op {
    Op::PackInstall {
        pack_id: pack_id.into(),
        name: format!("pack-{pack_id}"),
        version: "1.0.0".into(),
        manifest_digest: digest,
    }
}

fn blob(manifest: &PackManifest) -> (String, Vec<u8>) {
    (manifest.digest(), manifest.canonical_bytes())
}

/// T-269-01 (resource exhaustion): replay must stay near-linear in meta ops.
/// The pre-fix implementation was O(n^2): 2000 installs took ~2.18 s here.
#[test]
fn replay_meta_is_not_quadratic() {
    let n = 4000u64;
    let events: Vec<Event> = (0..n)
        .map(|i| {
            Event::new(
                Op::PackInstall {
                    pack_id: format!("p{i}"),
                    name: format!("pack-p{i}"),
                    version: "1.0.0".into(),
                    manifest_digest: None,
                },
                i,
            )
        })
        .collect();
    let rt = runtime(events);
    let start = std::time::Instant::now();
    rt.replay_check().expect("valid log");
    let elapsed = start.elapsed();
    assert!(
        elapsed < std::time::Duration::from_secs(3),
        "meta replay took {elapsed:?} for {n} ops; a quadratic regression is back"
    );
}

/// T-269-02 (spoofing): a manifest blob whose bytes do not match its digest
/// cannot be admitted as a manifest, even though generic StoreBlob admission
/// also accepts rule-body and policy digests.
#[test]
fn manifest_blob_digest_mismatch_fails_closed() {
    let manifest = PackManifest::new("core", "pack-core", "1.0.0", vec!["op".into()]);
    let (digest, mut bytes) = blob(&manifest);
    bytes.push(b' ');
    let events = vec![
        Event::new(
            Op::StoreBlob {
                digest: digest.clone(),
                bytes,
            },
            0,
        ),
        Event::new(install("core", Some(digest)), 1),
    ];
    let rt = runtime(events);
    // The generic StoreBlob digest check runs before meta validation, so this
    // is rejected there; either way the store cannot open.
    assert!(matches!(
        rt.replay_check(),
        Err(RuntimeError::UnknownRuleVersion { .. })
            | Err(RuntimeError::PackLifecycleRejected { .. })
    ));
}

/// T-269-03 (tampering): a manifest blob placed AFTER its install must not be
/// usable. Replay is prefix-scoped, so order cannot be laundered.
#[test]
fn manifest_blob_after_install_fails_closed() {
    let manifest = PackManifest::new("core", "pack-core", "1.0.0", vec!["op".into()]);
    let (digest, bytes) = blob(&manifest);
    let events = vec![
        Event::new(install("core", Some(digest.clone())), 0),
        Event::new(Op::StoreBlob { digest, bytes }, 1),
    ];
    let rt = runtime(events);
    assert!(matches!(
        rt.replay_check(),
        Err(RuntimeError::PackLifecycleRejected { .. })
    ));
}

/// T-269-04 (elevation): one pack cannot declare an operator already owned by
/// another installed pack.
#[test]
fn cross_pack_operator_collision_fails_on_replay() {
    let a = PackManifest::new("a", "pack-a", "1.0.0", vec!["shared".into()]);
    let b = PackManifest::new("b", "pack-b", "1.0.0", vec!["shared".into()]);
    let (da, ba) = blob(&a);
    let (db, bb) = blob(&b);
    let events = vec![
        Event::new(
            Op::StoreBlob {
                digest: da.clone(),
                bytes: ba,
            },
            0,
        ),
        Event::new(install("a", Some(da)), 1),
        Event::new(
            Op::StoreBlob {
                digest: db.clone(),
                bytes: bb,
            },
            2,
        ),
        Event::new(install("b", Some(db)), 3),
    ];
    let rt = runtime(events);
    assert!(matches!(
        rt.replay_check(),
        Err(RuntimeError::PackLifecycleRejected { reason, .. }) if reason.contains("already declared")
    ));
}

/// T-269-05 (identity spoofing): a manifest for another pack cannot be pinned.
#[test]
fn manifest_identity_spoof_fails_on_replay() {
    let other = PackManifest::new("other", "pack-core", "1.0.0", vec![]);
    let (digest, bytes) = blob(&other);
    let events = vec![
        Event::new(
            Op::StoreBlob {
                digest: digest.clone(),
                bytes,
            },
            0,
        ),
        Event::new(install("core", Some(digest)), 1),
    ];
    let rt = runtime(events);
    assert!(matches!(
        rt.replay_check(),
        Err(RuntimeError::PackLifecycleRejected { .. })
    ));
}

/// T-269-06 (forgery): a user cannot emit a meta op directly.
#[test]
fn user_forged_meta_ops_are_rejected() {
    let mut rt = Runtime::default();
    for op in [
        Op::PackInstall {
            pack_id: "core".into(),
            name: "pack-core".into(),
            version: "1.0.0".into(),
            manifest_digest: None,
        },
        Op::PackActivate {
            pack_id: "core".into(),
            version: "1.0.0".into(),
        },
        Op::PackRetire {
            pack_id: "core".into(),
            version: "1.0.0".into(),
        },
        Op::EnrichmentProposal {
            pack_id: "core".into(),
            pack_version: "1.0.0".into(),
            source_event_ids: vec![],
            model: "m".into(),
            prompt: "p".into(),
            cache_key: "k".into(),
            blob_digest: "00".repeat(32),
        },
    ] {
        assert!(matches!(rt.emit(op), Err(RuntimeError::MetaOpRejected)));
    }
}

/// T-269-07 (integrity): enrichment payload must match its digest on replay.
#[test]
fn enrichment_payload_tamper_fails_on_replay() {
    let manifest = PackManifest::new("enrich", "pack-enrich", "1.0.0", vec![]);
    let (md, mb) = blob(&manifest);
    let payload = b"summary".to_vec();
    let digest = kutha_common::sha256_hex(&payload);
    let mut tampered = payload.clone();
    tampered.push(b'!');
    let source = Event::new(
        Op::Assert {
            subject: 0,
            relation: 0,
            object: 1,
            valid_from: 0,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        },
        0,
    );
    let source_id = source.id;
    let events = vec![
        source,
        Event::new(
            Op::StoreBlob {
                digest: md.clone(),
                bytes: mb,
            },
            1,
        ),
        Event::new(install("enrich", Some(md)), 2),
        Event::new(
            Op::PackActivate {
                pack_id: "enrich".into(),
                version: "1.0.0".into(),
            },
            3,
        ),
        Event::new(
            Op::StoreBlob {
                digest: digest.clone(),
                bytes: tampered,
            },
            4,
        ),
        Event::new(
            Op::EnrichmentProposal {
                pack_id: "enrich".into(),
                pack_version: "1.0.0".into(),
                source_event_ids: vec![source_id],
                model: "m".into(),
                prompt: "p".into(),
                cache_key: "k".into(),
                blob_digest: digest,
            },
            5,
        ),
    ];
    let rt = runtime(events);
    // StoreBlob digest verification precedes meta validation; the tamper is
    // still fail-closed, which is the property under test.
    assert!(matches!(
        rt.replay_check(),
        Err(RuntimeError::UnknownRuleVersion { .. }) | Err(RuntimeError::EnrichmentRejected { .. })
    ));
}

/// T-269-08 (replay/emit drift): a log that admission accepted must reopen.
/// Regression guard for the retired-pack operator release.
#[test]
fn admission_accepted_log_reopens_after_retire() {
    let mut rt = Runtime::default();
    let first = PackManifest::new("core", "pack-core", "1.0.0", vec!["normalize".into()]);
    rt.emit_pack_install_with_manifest("core", "pack-core", "1.0.0", Some(first))
        .expect("install");
    rt.emit_pack_activate("core", "1.0.0").expect("activate");
    rt.emit_pack_retire("core", "1.0.0").expect("retire");
    let second = PackManifest::new("extra", "pack-extra", "1.0.0", vec!["normalize".into()]);
    rt.emit_pack_install_with_manifest("extra", "pack-extra", "1.0.0", Some(second))
        .expect("retired pack releases the name");
    rt.replay_check()
        .expect("replay must accept a log admission accepted");
}

/// T-269-08 (replay/emit drift): replay must also reject what admission
/// rejects, not merely accept what admission accepts.
#[test]
fn replay_rejects_what_admission_rejects() {
    let events = vec![Event::new(
        Op::PackActivate {
            pack_id: "ghost".into(),
            version: "1.0.0".into(),
        },
        0,
    )];
    let rt = runtime(events);
    assert!(matches!(
        rt.replay_check(),
        Err(RuntimeError::PackLifecycleRejected { .. })
    ));
}

/// T-269-08 (replay/emit drift): admission and replay share one lifecycle
/// state machine, so a transition accepted at write time is accepted on open.
#[test]
fn admission_and_replay_agree_on_lifecycle() {
    let mut rt = Runtime::default();
    rt.emit_pack_install("core", "pack-core", "1.0.0")
        .expect("install");
    rt.emit_pack_activate("core", "1.0.0").expect("activate");
    rt.emit_pack_retire("core", "1.0.0").expect("retire");
    rt.replay_check().expect("replay agrees with admission");
}
