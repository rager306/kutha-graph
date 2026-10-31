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

fn at(mut op: Op, tt: u64) -> Event {
    let mut event = Event::new(op.clone(), tt);
    // Stable ids keep the fixtures readable; Event::new already minted one.
    op = event.op.clone();
    event.op = op;
    event
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
    let events = vec![
        Event::new(
            Op::StoreBlob {
                digest: digest.clone(),
                bytes: payload,
            },
            0,
        ),
        Event::new(
            Op::EnrichmentProposal {
                pack_id: "ghost".into(),
                pack_version: "1.0.0".into(),
                source_event_ids: vec![],
                model: "m".into(),
                prompt: "p".into(),
                cache_key: "k".into(),
                blob_digest: digest,
            },
            1,
        ),
    ];
    let rt = runtime(events).expect("build");
    assert!(matches!(
        rt.replay_check(),
        Err(RuntimeError::EnrichmentRejected { .. })
    ));
}

#[test]
fn duplicate_enrichment_cache_key_fails_replay() {
    let manifest = PackManifest::new("enrich", "pack-enrich", "1.0.0", vec![]);
    let manifest_bytes = manifest.canonical_bytes();
    let manifest_digest = manifest.digest();
    let payload = b"summary".to_vec();
    let digest = kutha_common::sha256_hex(&payload);
    let proposal = || Op::EnrichmentProposal {
        pack_id: "enrich".into(),
        pack_version: "1.0.0".into(),
        source_event_ids: vec![],
        model: "m".into(),
        prompt: "p".into(),
        cache_key: "same".into(),
        blob_digest: digest.clone(),
    };
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
        Event::new(proposal(), 4),
        Event::new(proposal(), 5),
    ];
    let rt = runtime(events).expect("build");
    assert!(matches!(
        rt.replay_check(),
        Err(RuntimeError::EnrichmentRejected { .. })
    ));
}
