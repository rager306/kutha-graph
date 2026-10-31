//! v1.76 phase 270: direct falsifiers for the raw-SHA-256 meta digest checks.
//!
//! The generic `StoreBlob` gate accepts three address schemes: raw SHA-256, a
//! canonical rule-body hash, and a policy hash over UTF-8. A manifest or an
//! enrichment payload must be addressed by raw SHA-256 only. These tests build
//! a blob that passes the generic gate under a policy hash and show that the
//! meta check still rejects it. Remove the raw check and the test fails.

use kutha_common::{Event, Op};
use kutha_runtime::{PackManifest, Runtime, RuntimeError};

fn dict() -> Vec<String> {
    vec!["knows".into(), "knownBy".into()]
}

fn runtime(events: Vec<Event>) -> Runtime {
    Runtime::from_dict_and_events(dict(), events, 32).expect("build")
}

fn install(pack_id: &str, digest: String) -> Op {
    Op::PackInstall {
        pack_id: pack_id.into(),
        name: format!("pack-{pack_id}"),
        version: "1.0.0".into(),
        manifest_digest: Some(digest),
    }
}

#[test]
fn policy_hash_addressed_manifest_is_rejected() {
    let manifest = PackManifest::new("core", "pack-core", "1.0.0", vec!["normalize".into()]);
    let bytes = manifest.canonical_bytes();
    let text = std::str::from_utf8(&bytes).expect("manifest JSON is UTF-8");
    let policy_digest = kutha_common::policy_version_hash(text);
    // Premise: the generic StoreBlob gate accepts this address.
    assert!(
        kutha_common::blob_digest_matches(&policy_digest, &bytes),
        "policy hash must pass the generic StoreBlob gate"
    );
    // Premise: it is not the raw content address.
    assert_ne!(policy_digest, manifest.digest());

    let events = vec![
        Event::new(
            Op::StoreBlob {
                digest: policy_digest.clone(),
                bytes,
            },
            0,
        ),
        Event::new(install("core", policy_digest), 1),
    ];
    let rt = runtime(events);
    let err = rt
        .replay_check()
        .expect_err("manifest must be raw-addressed");
    assert!(
        matches!(
            &err,
            RuntimeError::PackLifecycleRejected { reason, .. } if reason.contains("replay manifest")
        ),
        "unexpected error: {err:?}"
    );
}

#[test]
fn policy_hash_addressed_enrichment_payload_is_rejected() {
    let manifest = PackManifest::new("enrich", "pack-enrich", "1.0.0", vec![]);
    let manifest_digest = manifest.digest();
    let payload = b"summary text".to_vec();
    let text = std::str::from_utf8(&payload).expect("UTF-8 payload");
    let policy_digest = kutha_common::policy_version_hash(text);
    assert!(kutha_common::blob_digest_matches(&policy_digest, &payload));
    assert_ne!(policy_digest, kutha_common::sha256_hex(&payload));

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
                digest: manifest_digest.clone(),
                bytes: manifest.canonical_bytes(),
            },
            1,
        ),
        Event::new(install("enrich", manifest_digest), 2),
        Event::new(
            Op::PackActivate {
                pack_id: "enrich".into(),
                version: "1.0.0".into(),
            },
            3,
        ),
        Event::new(
            Op::StoreBlob {
                digest: policy_digest.clone(),
                bytes: payload,
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
                blob_digest: policy_digest,
            },
            5,
        ),
    ];
    let rt = runtime(events);
    let err = rt
        .replay_check()
        .expect_err("payload must be raw-addressed");
    assert!(
        matches!(&err, RuntimeError::EnrichmentRejected { reason } if (reason.contains("do not match") || reason.contains("does not match"))),
        "unexpected error: {err:?}"
    );
}

#[test]
fn raw_sha256_addressing_still_passes_both_checks() {
    let manifest = PackManifest::new("core", "pack-core", "1.0.0", vec!["normalize".into()]);
    let digest = manifest.digest();
    let events = vec![
        Event::new(
            Op::StoreBlob {
                digest: digest.clone(),
                bytes: manifest.canonical_bytes(),
            },
            0,
        ),
        Event::new(install("core", digest), 1),
    ];
    let rt = runtime(events);
    rt.replay_check().expect("raw-addressed manifest is valid");
    assert_eq!(rt.pack_operators().expect("registry").len(), 1);
}

/// DIG-02: collision detection is indexed, so replay cost grows linearly with
/// pack count rather than quadratically. The pre-index scan compared every new
/// operator against every previously declared operator set.
///
/// The oracle is the work the product gate performs, not wall-clock: every meta
/// op in the log is replayed once and every manifest is declared once, so 4x
/// packs is 4x replays and 4x declarations.
#[test]
fn manifest_replay_scales_linearly_with_packs() {
    fn build(n: usize) -> Vec<Event> {
        let mut events = Vec::new();
        for i in 0..n {
            let manifest = PackManifest::new(
                format!("p{i}"),
                format!("pack-p{i}"),
                "1.0.0",
                vec![format!("op{i}")],
            );
            let digest = manifest.digest();
            events.push(Event::new(
                Op::StoreBlob {
                    digest: digest.clone(),
                    bytes: manifest.canonical_bytes(),
                },
                (2 * i) as u64,
            ));
            events.push(Event::new(
                Op::PackInstall {
                    pack_id: format!("p{i}"),
                    name: format!("pack-p{i}"),
                    version: "1.0.0".into(),
                    manifest_digest: Some(digest),
                },
                (2 * i + 1) as u64,
            ));
        }
        events
    }

    let small_events = build(400);
    let large_events = build(1600);

    let small = runtime(small_events);
    let large = runtime(large_events);
    let (s_replays, s_decl) = small.meta_replay_probe().expect("valid");
    let (l_replays, l_decl) = large.meta_replay_probe().expect("valid");
    assert_eq!(s_decl, 400, "one declaration per manifest install");
    assert_eq!(l_decl, 1600);
    // 4x the packs. Linear stays at 4x; the old O(P^2) scan was near 16x.
    assert_eq!(
        l_decl,
        4 * s_decl,
        "4x packs must be 4x declarations, not 16x"
    );
    assert_eq!(s_replays, 400, "one lifecycle replay per install");
    assert_eq!(l_replays, 1600);
    small.replay_check().expect("valid");
}

/// OperatorIndex unit behaviour: collision, re-declare, retire release.
#[test]
fn operator_index_handles_redeclare_and_retire() {
    use kutha_runtime::{OperatorIndex, PackManifestError};

    let mut index = OperatorIndex::new();
    index
        .declare("a", vec!["one".into(), "two".into()])
        .expect("first");
    assert_eq!(index.owner_of("one"), Some("a"));

    // Another pack cannot take an owned name.
    assert!(matches!(
        index.declare("b", vec!["one".into()]),
        Err(PackManifestError::OperatorCollision { .. })
    ));
    assert_eq!(index.owner_of("one"), Some("a"));

    // Re-declaring a smaller set drops the names no longer claimed.
    index.declare("a", vec!["one".into()]).expect("redeclare");
    assert_eq!(index.owner_of("two"), None);
    index.declare("b", vec!["two".into()]).expect("released");

    // Retire releases everything the pack owned.
    index.retire("a");
    assert_eq!(index.owner_of("one"), None);
    index
        .declare("c", vec!["one".into()])
        .expect("released by retire");
}
