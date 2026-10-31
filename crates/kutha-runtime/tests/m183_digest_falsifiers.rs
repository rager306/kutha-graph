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

    let small = runtime(build(400));
    let start = std::time::Instant::now();
    small.replay_check().expect("valid");
    let t_small = start.elapsed();

    let large = runtime(build(1600));
    let start = std::time::Instant::now();
    large.replay_check().expect("valid");
    let t_large = start.elapsed();

    // 4x the packs. Linear stays near 4x; the old O(P^2) scan was near 16x.
    assert!(
        t_large < t_small * 10 + std::time::Duration::from_millis(50),
        "4x packs took {t_large:?} vs {t_small:?}; collision scan looks super-linear"
    );
}
