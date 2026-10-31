//! C5: ADR-052 enrichment proposals and ledger.

use kutha_common::Op;
use kutha_runtime::{
    EnrichmentError, PackLifecycleState, PackManifest, Runtime, RuntimeError, store,
};
use std::sync::atomic::{AtomicU64, Ordering};

mod common;

/// Repo-local `tmp/` scratch with pid + nanos + seq (`m019_dur_a` idiom), so two
/// concurrent runs of this binary never share a store directory.
fn scratch(label: &str) -> common::PersistScratch {
    static N: AtomicU64 = AtomicU64::new(0);
    let seq = N.fetch_add(1, Ordering::Relaxed);
    common::PersistScratch::new(&format!("m178-{label}-{seq}"))
}

fn build_runtime() -> Runtime {
    let mut rt = Runtime::default();
    let manifest = PackManifest::new("enrich", "Enrich Pack", "1.0.0", vec!["summarize".into()]);
    rt.emit_pack_install_with_manifest("enrich", "Enrich Pack", "1.0.0", Some(manifest))
        .expect("install");
    rt.emit_pack_activate("enrich", "1.0.0").expect("activate");
    rt
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

fn propose(rt: &mut Runtime, source: kutha_common::EventId, key: &str) -> Result<(), RuntimeError> {
    rt.emit_enrichment_proposal(
        "enrich",
        "1.0.0",
        vec![source],
        "model-x",
        "prompt-v1",
        key,
        b"summary".to_vec(),
    )
    .map(|_| ())
}

#[test]
fn proposal_is_typed_meta_and_does_not_change_facts() {
    let mut rt = build_runtime();
    let source = source(&mut rt);
    let before = rt.fold().fingerprint();
    let facts_before = rt.fold().facts().len();
    propose(&mut rt, source, "cache-1").expect("proposal");
    assert_eq!(rt.fold().fingerprint(), before);
    assert_eq!(rt.fold().facts().len(), facts_before);
    let ledger = rt.enrichment_ledger();
    assert_eq!(ledger.len(), 1);
    assert_eq!(ledger[0].pack_id, "enrich");
    assert_eq!(ledger[0].state, PackLifecycleState::Active);
    assert_eq!(ledger[0].source_event_ids, vec![source]);
}

#[test]
fn retired_or_inactive_pack_fails_closed() {
    let mut rt = build_runtime();
    let src_event = source(&mut rt);
    rt.emit_pack_retire("enrich", "1.0.0").expect("retire");
    let err = propose(&mut rt, src_event, "cache-retired").expect_err("retired");
    assert!(matches!(
        err,
        RuntimeError::EnrichmentRejected { reason } if reason.contains("retired")
    ));

    let mut installed = Runtime::default();
    installed
        .emit_pack_install("idle", "Idle Pack", "1.0.0")
        .expect("install");
    let src = source(&mut installed);
    let err = installed
        .emit_enrichment_proposal(
            "idle",
            "1.0.0",
            vec![src],
            "model-x",
            "prompt-v1",
            "cache-idle",
            b"summary".to_vec(),
        )
        .expect_err("installed is not active");
    assert!(matches!(
        err,
        RuntimeError::EnrichmentRejected { reason } if reason.contains("not active")
    ));
}

#[test]
fn unknown_source_and_duplicate_key_fail_closed() {
    let mut rt = build_runtime();
    let source = source(&mut rt);
    let missing = kutha_common::EventId::nil();
    let err = rt
        .emit_enrichment_proposal(
            "enrich",
            "1.0.0",
            vec![missing],
            "model-x",
            "prompt-v1",
            "cache-missing",
            b"summary".to_vec(),
        )
        .expect_err("missing source");
    assert!(matches!(
        err,
        RuntimeError::EnrichmentRejected { reason } if reason.contains("source event")
    ));

    propose(&mut rt, source, "cache-dup").expect("first");
    let err = propose(&mut rt, source, "cache-dup").expect_err("duplicate");
    assert!(matches!(
        err,
        RuntimeError::EnrichmentRejected { reason } if reason.contains("cache key")
    ));
    assert_eq!(rt.enrichment_ledger().len(), 1);
}

#[test]
fn wrong_version_fails_closed_without_orphan_blob() {
    let mut rt = build_runtime();
    let source = source(&mut rt);
    let before = rt.log().len();
    let err = rt
        .emit_enrichment_proposal(
            "enrich",
            "2.0.0",
            vec![source],
            "model-x",
            "prompt-v1",
            "cache-version",
            b"summary".to_vec(),
        )
        .expect_err("wrong version");
    assert!(matches!(
        err,
        RuntimeError::EnrichmentRejected { reason } if reason.contains("version")
    ));
    assert_eq!(
        rt.log().len(),
        before,
        "rejected proposal left an orphan event"
    );
}

#[test]
fn empty_pins_and_payload_fail_closed() {
    let mut rt = build_runtime();
    let source = source(&mut rt);
    let err = rt
        .emit_enrichment_proposal("enrich", "1.0.0", vec![source], "", "p", "k", b"x".to_vec())
        .expect_err("empty model");
    assert!(matches!(
        err,
        RuntimeError::EnrichmentRejected { reason } if reason.contains("model")
    ));
    let err = rt
        .emit_enrichment_proposal("enrich", "1.0.0", vec![source], "m", "p", "k", vec![])
        .expect_err("empty payload");
    assert!(matches!(
        err,
        RuntimeError::EnrichmentRejected { reason } if reason.contains("payload")
    ));
    assert!(rt.enrichment_ledger().is_empty());
}

#[test]
fn user_forged_proposal_is_rejected() {
    let mut rt = build_runtime();
    let source = source(&mut rt);
    let err = rt
        .emit(Op::EnrichmentProposal {
            pack_id: "enrich".into(),
            pack_version: "1.0.0".into(),
            source_event_ids: vec![source],
            model: "m".into(),
            prompt: "p".into(),
            cache_key: "k".into(),
            blob_digest: "00".repeat(32),
        })
        .expect_err("user forged");
    assert!(matches!(err, RuntimeError::MetaOpRejected));
}

#[test]
fn ledger_survives_persist_open() {
    let mut rt = build_runtime();
    let source = source(&mut rt);
    propose(&mut rt, source, "cache-persist").expect("proposal");
    let dir = scratch("enrichment-persist");
    rt.persist(&dir).expect("persist");
    let opened = Runtime::open(&dir).expect("open");
    let ledger = opened.enrichment_ledger();
    assert_eq!(ledger.len(), 1);
    assert_eq!(ledger, rt.enrichment_ledger());
}

#[test]
fn ledger_survives_checkpoint_sealed_prefix() {
    struct Reset;
    impl Drop for Reset {
        fn drop(&mut self) {
            store::reset_test_segment_limits();
        }
    }
    let _reset = Reset;
    store::set_test_segment_limits(Some(4096), Some(2));

    let mut rt = build_runtime();
    let source = source(&mut rt);
    propose(&mut rt, source, "cache-checkpoint").expect("proposal");
    let dir = scratch("enrichment-checkpoint");
    rt.persist(&dir).expect("persist proposal prefix");
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
    rt.persist(&dir).expect("persist sealed prefix");
    store::checkpoint_sealed_prefix(&dir).expect("checkpoint");
    let opened = Runtime::open(&dir).expect("open");
    let ledger = opened.enrichment_ledger();
    assert_eq!(ledger.len(), 1);
    assert_eq!(ledger[0].cache_key, "cache-checkpoint");
}

#[test]
fn ledger_fork_cut_preserves_prefix_proposals() {
    let mut rt = build_runtime();
    let source = source(&mut rt);
    propose(&mut rt, source, "cache-fork").expect("proposal");
    let fork = rt.fork_at(rt.log().len()).expect("fork");
    assert_eq!(fork.enrichment_ledger(), rt.enrichment_ledger());

    let cut = rt.log().len() - 1;
    let fork = rt.fork_at(cut).expect("fork before proposal");
    assert!(fork.enrichment_ledger().is_empty());
}

#[test]
fn error_display_is_specific() {
    let err = EnrichmentError::RetiredPack {
        pack_id: "p".into(),
    };
    assert!(err.to_string().contains("retired"));
}
