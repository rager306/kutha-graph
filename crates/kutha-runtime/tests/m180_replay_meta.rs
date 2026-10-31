//! v1.73: replay revalidates C1/C1b/C5 meta instead of deriving a wrong lease.

use kutha_common::{Event, Op};
use kutha_runtime::{PackManifest, Runtime, RuntimeError};

fn dict() -> Vec<String> {
    vec!["knows".into(), "knownBy".into()]
}

fn runtime(events: Vec<Event>) -> Result<Runtime, RuntimeError> {
    Runtime::from_dict_and_events(dict(), events, 32)
}

fn install(pack_id: &str, version: &str, digest: Option<String>) -> Op {
    Op::PackInstall {
        pack_id: pack_id.into(),
        name: format!("pack-{pack_id}"),
        version: version.into(),
        manifest_digest: digest,
    }
}

fn at(op: Op, tt: u64) -> Event {
    Event::new(op, tt)
}

/// A real product source event, so source-existence checks are actually reached.
fn source_event(tt: u64) -> Event {
    Event::new(
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
        tt,
    )
}

#[test]
fn valid_pack_with_manifest_replays() {
    let manifest = PackManifest::new("core", "pack-core", "1.0.0", vec!["normalize".into()]);
    let bytes = manifest.canonical_bytes();
    let digest = manifest.digest();
    let events = vec![
        Event::new(
            Op::StoreBlob {
                digest: digest.clone(),
                bytes,
            },
            0,
        ),
        at(install("core", "1.0.0", Some(digest)), 1),
        at(
            Op::PackActivate {
                pack_id: "core".into(),
                version: "1.0.0".into(),
            },
            2,
        ),
    ];
    let rt = runtime(events).expect("valid log");
    rt.replay_check().expect("valid replay");
    assert_eq!(rt.pack_lifecycle().len(), 1);
}

#[test]
fn activate_before_install_fails_replay() {
    let events = vec![at(
        Op::PackActivate {
            pack_id: "core".into(),
            version: "1.0.0".into(),
        },
        0,
    )];
    let rt = runtime(events).expect("build");
    assert!(matches!(
        rt.replay_check(),
        Err(RuntimeError::PackLifecycleRejected { .. })
    ));
}

#[test]
fn duplicate_install_fails_replay() {
    let events = vec![
        at(install("core", "1.0.0", None), 0),
        at(install("core", "1.0.0", None), 1),
    ];
    let rt = runtime(events).expect("build");
    assert!(matches!(
        rt.replay_check(),
        Err(RuntimeError::PackLifecycleRejected { .. })
    ));
}

#[test]
fn retire_unknown_pack_fails_replay() {
    let events = vec![at(
        Op::PackRetire {
            pack_id: "ghost".into(),
            version: "1.0.0".into(),
        },
        0,
    )];
    let rt = runtime(events).expect("build");
    assert!(matches!(
        rt.replay_check(),
        Err(RuntimeError::PackLifecycleRejected { .. })
    ));
}

#[test]
fn activate_with_wrong_version_fails_replay() {
    // pack_lifecycle.rs:144-163: an activate is allowed only from Installed
    // at the version the activation names.
    let events = vec![
        at(install("core", "1.0.0", None), 0),
        at(
            Op::PackActivate {
                pack_id: "core".into(),
                version: "2.0.0".into(),
            },
            1,
        ),
    ];
    let rt = runtime(events).expect("build");
    assert!(matches!(
        rt.replay_check(),
        Err(RuntimeError::PackLifecycleRejected { pack_id, reason })
            if pack_id == "core" && reason == "activate_requires_same_installed_version"
    ));
}

#[test]
fn second_activate_fails_replay() {
    // pack_lifecycle.rs:144-163: only Installed transitions to Active, so an
    // already active pack rejects a further activate.
    let events = vec![
        at(install("core", "1.0.0", None), 0),
        at(
            Op::PackActivate {
                pack_id: "core".into(),
                version: "1.0.0".into(),
            },
            1,
        ),
        at(
            Op::PackActivate {
                pack_id: "core".into(),
                version: "1.0.0".into(),
            },
            2,
        ),
    ];
    let rt = runtime(events).expect("build");
    assert!(matches!(
        rt.replay_check(),
        Err(RuntimeError::PackLifecycleRejected { pack_id, reason })
            if pack_id == "core" && reason == "activate_requires_same_installed_version"
    ));
}

#[test]
fn second_retire_fails_replay() {
    // pack_lifecycle.rs:144-163: only Installed and Active may retire, so a
    // second retire of the same pack is rejected.
    let events = vec![
        at(install("core", "1.0.0", None), 0),
        at(
            Op::PackRetire {
                pack_id: "core".into(),
                version: "1.0.0".into(),
            },
            1,
        ),
        at(
            Op::PackRetire {
                pack_id: "core".into(),
                version: "1.0.0".into(),
            },
            2,
        ),
    ];
    let rt = runtime(events).expect("build");
    assert!(matches!(
        rt.replay_check(),
        Err(RuntimeError::PackLifecycleRejected { pack_id, reason })
            if pack_id == "core" && reason == "retire_requires_same_installed_version"
    ));
}

#[test]
fn install_with_missing_manifest_blob_fails_replay() {
    let events = vec![at(install("core", "1.0.0", Some("ab".repeat(32))), 0)];
    let rt = runtime(events).expect("build");
    assert!(matches!(
        rt.replay_check(),
        Err(RuntimeError::PackLifecycleRejected { reason, .. }) if reason.contains("replay manifest")
    ));
}

#[test]
fn install_with_wrong_manifest_identity_fails_replay() {
    let manifest = PackManifest::new("other", "pack-core", "1.0.0", vec![]);
    let bytes = manifest.canonical_bytes();
    let digest = manifest.digest();
    let events = vec![
        Event::new(
            Op::StoreBlob {
                digest: digest.clone(),
                bytes,
            },
            0,
        ),
        at(install("core", "1.0.0", Some(digest)), 1),
    ];
    let rt = runtime(events).expect("build");
    assert!(matches!(
        rt.replay_check(),
        Err(RuntimeError::PackLifecycleRejected { .. })
    ));
}

#[test]
fn enrichment_for_unknown_pack_fails_replay() {
    let payload = b"summary".to_vec();
    let digest = kutha_common::sha256_hex(&payload);
    let source = source_event(0);
    let source_id = source.id;
    let events = vec![
        source,
        Event::new(
            Op::StoreBlob {
                digest: digest.clone(),
                bytes: payload,
            },
            1,
        ),
        Event::new(
            Op::EnrichmentProposal {
                pack_id: "ghost".into(),
                pack_version: "1.0.0".into(),
                source_event_ids: vec![source_id],
                model: "m".into(),
                prompt: "p".into(),
                cache_key: "k".into(),
                blob_digest: digest,
            },
            2,
        ),
    ];
    let rt = runtime(events).expect("build");
    assert!(matches!(
        rt.replay_check(),
        Err(RuntimeError::EnrichmentRejected { reason })
            if reason.contains("not installed")
    ));
}

#[test]
fn duplicate_enrichment_cache_key_fails_replay() {
    let manifest = PackManifest::new("enrich", "pack-enrich", "1.0.0", vec![]);
    let manifest_bytes = manifest.canonical_bytes();
    let manifest_digest = manifest.digest();
    let payload = b"summary".to_vec();
    let digest = kutha_common::sha256_hex(&payload);
    let source = source_event(0);
    let source_id = source.id;
    let proposal = || Op::EnrichmentProposal {
        pack_id: "enrich".into(),
        pack_version: "1.0.0".into(),
        source_event_ids: vec![source_id],
        model: "m".into(),
        prompt: "p".into(),
        cache_key: "same".into(),
        blob_digest: digest.clone(),
    };
    let events = vec![
        source,
        Event::new(
            Op::StoreBlob {
                digest: manifest_digest.clone(),
                bytes: manifest_bytes,
            },
            1,
        ),
        at(install("enrich", "1.0.0", Some(manifest_digest)), 2),
        at(
            Op::PackActivate {
                pack_id: "enrich".into(),
                version: "1.0.0".into(),
            },
            3,
        ),
        Event::new(
            Op::StoreBlob {
                digest: digest.clone(),
                bytes: payload,
            },
            4,
        ),
        Event::new(proposal(), 5),
        Event::new(proposal(), 6),
    ];
    let rt = runtime(events).expect("build");
    assert!(matches!(
        rt.replay_check(),
        Err(RuntimeError::EnrichmentRejected { reason }) if reason.contains("cache key")
    ));
}

#[test]
fn enrichment_missing_source_fails_replay() {
    let manifest = PackManifest::new("enrich", "pack-enrich", "1.0.0", vec![]);
    let manifest_bytes = manifest.canonical_bytes();
    let manifest_digest = manifest.digest();
    let payload = b"summary".to_vec();
    let digest = kutha_common::sha256_hex(&payload);
    let events = vec![
        Event::new(
            Op::StoreBlob {
                digest: manifest_digest.clone(),
                bytes: manifest_bytes,
            },
            0,
        ),
        at(install("enrich", "1.0.0", Some(manifest_digest)), 1),
        at(
            Op::PackActivate {
                pack_id: "enrich".into(),
                version: "1.0.0".into(),
            },
            2,
        ),
        Event::new(
            Op::StoreBlob {
                digest: digest.clone(),
                bytes: payload,
            },
            3,
        ),
        Event::new(
            Op::EnrichmentProposal {
                pack_id: "enrich".into(),
                pack_version: "1.0.0".into(),
                source_event_ids: vec![kutha_common::EventId::nil()],
                model: "m".into(),
                prompt: "p".into(),
                cache_key: "k".into(),
                blob_digest: digest,
            },
            4,
        ),
    ];
    let rt = runtime(events).expect("build");
    assert!(matches!(
        rt.replay_check(),
        Err(RuntimeError::EnrichmentRejected { reason }) if reason.contains("source event")
    ));
}

#[test]
fn enrichment_wrong_version_and_retired_pack_fail_replay() {
    let manifest = PackManifest::new("enrich", "pack-enrich", "1.0.0", vec![]);
    let manifest_bytes = manifest.canonical_bytes();
    let manifest_digest = manifest.digest();
    let payload = b"summary".to_vec();
    let digest = kutha_common::sha256_hex(&payload);
    let source = source_event(0);
    let source_id = source.id;
    let base = vec![
        source,
        Event::new(
            Op::StoreBlob {
                digest: manifest_digest.clone(),
                bytes: manifest_bytes,
            },
            1,
        ),
        at(install("enrich", "1.0.0", Some(manifest_digest)), 2),
        at(
            Op::PackActivate {
                pack_id: "enrich".into(),
                version: "1.0.0".into(),
            },
            3,
        ),
        Event::new(
            Op::StoreBlob {
                digest: digest.clone(),
                bytes: payload.clone(),
            },
            4,
        ),
    ];
    let proposal = |version: &str, key: &str, tt: u64| {
        Event::new(
            Op::EnrichmentProposal {
                pack_id: "enrich".into(),
                pack_version: version.into(),
                source_event_ids: vec![source_id],
                model: "m".into(),
                prompt: "p".into(),
                cache_key: key.into(),
                blob_digest: digest.clone(),
            },
            tt,
        )
    };

    let mut wrong_version = base.clone();
    wrong_version.push(proposal("2.0.0", "k1", 5));
    let rt = runtime(wrong_version).expect("build");
    assert!(matches!(
        rt.replay_check(),
        Err(RuntimeError::EnrichmentRejected { .. })
    ));

    let mut retired = base;
    retired.push(at(
        Op::PackRetire {
            pack_id: "enrich".into(),
            version: "1.0.0".into(),
        },
        5,
    ));
    retired.push(proposal("1.0.0", "k2", 6));
    let rt = runtime(retired).expect("build");
    assert!(matches!(
        rt.replay_check(),
        Err(RuntimeError::EnrichmentRejected { reason }) if reason.contains("retired")
    ));
}
