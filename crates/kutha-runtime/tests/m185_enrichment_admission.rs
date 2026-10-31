//! v1.78: enrichment proposal admission is O(1) and identical to replay.

use kutha_common::{Event, Op};
use kutha_runtime::{PackManifest, Runtime, RuntimeError};

fn install_and_activate(rt: &mut Runtime) {
    let manifest = PackManifest::new("enrich", "pack-enrich", "1.0.0", vec![]);
    rt.emit_pack_install_with_manifest("enrich", "pack-enrich", "1.0.0", Some(manifest))
        .expect("install");
    rt.emit_pack_activate("enrich", "1.0.0").expect("activate");
}

fn source(rt: &mut Runtime) -> kutha_common::EventId {
    let subject = rt.intern("article");
    let relation = rt.intern("relatedTo");
    let object = rt.intern("value");
    rt.emit(Op::Assert {
        subject,
        relation,
        object,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .expect("assert")
    .receipt
    .event_ids[0]
}

#[test]
fn admission_rejects_duplicate_cache_key_without_log_scan() {
    let mut rt = Runtime::default();
    install_and_activate(&mut rt);
    let src = source(&mut rt);
    rt.emit_enrichment_proposal(
        "enrich",
        "1.0.0",
        vec![src],
        "m",
        "p",
        "k1",
        b"payload".to_vec(),
    )
    .expect("first");
    let err = rt
        .emit_enrichment_proposal(
            "enrich",
            "1.0.0",
            vec![src],
            "m",
            "p",
            "k1",
            b"payload2".to_vec(),
        )
        .expect_err("duplicate cache key");
    assert!(matches!(err, RuntimeError::EnrichmentRejected { .. }));
    assert_eq!(rt.enrichment_ledger().len(), 1);
}

#[test]
fn admission_and_replay_agree_on_proposals() {
    let mut rt = Runtime::default();
    install_and_activate(&mut rt);
    let src = source(&mut rt);
    rt.emit_enrichment_proposal(
        "enrich",
        "1.0.0",
        vec![src],
        "m",
        "p",
        "k",
        b"payload".to_vec(),
    )
    .expect("proposal");
    rt.replay_check().expect("replay agrees");

    // The same proposal on a foreign log is rejected by replay.
    let manifest = PackManifest::new("enrich", "pack-enrich", "1.0.0", vec![]);
    let events = vec![
        Event::new(
            Op::StoreBlob {
                digest: manifest.digest(),
                bytes: manifest.canonical_bytes(),
            },
            0,
        ),
        Event::new(
            Op::PackInstall {
                pack_id: "enrich".into(),
                name: "pack-enrich".into(),
                version: "1.0.0".into(),
                manifest_digest: Some(manifest.digest()),
            },
            1,
        ),
        Event::new(
            Op::PackActivate {
                pack_id: "enrich".into(),
                version: "1.0.0".into(),
            },
            2,
        ),
        Event::new(
            Op::EnrichmentProposal {
                pack_id: "enrich".into(),
                pack_version: "1.0.0".into(),
                source_event_ids: vec![kutha_common::EventId::nil()],
                model: "m".into(),
                prompt: "p".into(),
                cache_key: "k".into(),
                blob_digest: kutha_common::sha256_hex(b"payload"),
            },
            3,
        ),
    ];
    let foreign = Runtime::from_dict_and_events(vec!["knows".into(), "knownBy".into()], events, 32)
        .expect("build");
    assert!(matches!(
        foreign.replay_check(),
        Err(RuntimeError::EnrichmentRejected { .. })
    ));
}

#[test]
fn admission_does_not_scan_the_log_for_proposals() {
    let source = std::fs::read_to_string("src/admission.rs").expect("admission.rs");
    assert!(
        !source.contains("derive_pack_lifecycle"),
        "proposal admission must not rebuild the lifecycle from the log"
    );
    assert!(
        !source.contains("validate_proposal(self.log.as_slice()"),
        "proposal admission must not scan the log"
    );
    assert!(
        source.contains("self.enrichment_cache_keys.clone()"),
        "proposal admission must use the runtime cache-key lease"
    );
}

#[test]
fn many_proposals_keep_cache_key_lease_in_step() {
    let mut rt = Runtime::default();
    install_and_activate(&mut rt);
    let src = source(&mut rt);
    for i in 0..200 {
        rt.emit_enrichment_proposal(
            "enrich",
            "1.0.0",
            vec![src],
            "m",
            "p",
            format!("key{i}"),
            format!("payload{i}").into_bytes(),
        )
        .expect("proposal");
    }
    assert_eq!(rt.enrichment_ledger().len(), 200);
    rt.replay_check()
        .expect("replay agrees after 200 proposals");
    // The 201st key that repeats is still refused.
    let err = rt
        .emit_enrichment_proposal(
            "enrich",
            "1.0.0",
            vec![src],
            "m",
            "p",
            "key7",
            b"other".to_vec(),
        )
        .expect_err("duplicate");
    assert!(matches!(err, RuntimeError::EnrichmentRejected { .. }));
}
