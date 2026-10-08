//! M024 S02 / Phase 80 — write-path atomicity characterization (G-A09 / G-A10).
//! Decisions: Define stays SelfCommit with a named crash window; D014-1 constant
//! commitment is receipt digest (32 B); event_ids / events_in_quantum remain
//! resume leases that may expose quantum width (residual until a format bump).

use kutha_common::{Event, Op};
use kutha_runtime::{Runtime, store};

fn uuid_like() -> String {
    format!(
        "{:x}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    )
}

/// G-A09: intern Define is SelfCommit — kept by committed_prefix without a QO.
#[test]
fn intern_define_is_self_commit_without_quantum_outcome() {
    let defines = vec![
        Event::new(Op::Define { name: "wp-alice".into() }, 0),
        Event::new(Op::Define { name: "wp-bob".into() }, 1),
    ];
    let kept = store::committed_prefix(defines.clone());
    assert_eq!(
        kept.len(),
        defines.len(),
        "Define prefix must survive with no QuantumOutcome"
    );
    assert!(kept.iter().all(|e| matches!(e.op, Op::Define { .. })));
}

/// G-A09: uncommitted Assert (NeedsOutcome, no trailing QO) is dropped; prior Defines stay.
#[test]
fn crash_after_assert_keeps_defines_drops_uncommitted_assert() {
    let events = vec![
        Event::new(Op::Define { name: "wp-s".into() }, 0),
        Event::new(Op::Define { name: "wp-r".into() }, 1),
        Event::new(Op::Define { name: "wp-o".into() }, 2),
        Event::new(
            Op::Assert {
                subject: 1,
                relation: 2,
                object: 3,
                valid_from: 2010,
                valid_to: None,
                claim: None,
                delivery_key: None,
                polarity: None,
            },
            3,
        ),
    ];
    let kept = store::committed_prefix(events);
    assert_eq!(kept.len(), 3, "only SelfCommit Defines remain");
    assert!(kept.iter().all(|e| matches!(e.op, Op::Define { .. })));
    assert!(
        !kept.iter().any(|e| matches!(e.op, Op::Assert { .. })),
        "NeedsOutcome Assert without QO must not be visible"
    );
}

/// G-A09: persisted intern Defines reopen with meanings intact.
#[test]
fn open_after_intern_only_recovers_define_meanings() {
    let mut rt = Runtime::default();
    let a = rt.intern("wp-open-a");
    let b = rt.intern("wp-open-b");
    let dir = std::env::temp_dir().join(format!("kutha-m024-wp-{}", uuid_like()));
    store::persist(&rt, &dir).unwrap();
    let opened = store::open(&dir).unwrap();
    assert_eq!(opened.dictionary().id("wp-open-a"), Some(a));
    assert_eq!(opened.dictionary().id("wp-open-b"), Some(b));
    // Crash window for Assert+no-QO is pinned by crash_after_assert_* (committed_prefix).
    let _ = std::fs::remove_dir_all(&dir);
}

/// G-A10: digest commitment is constant 32 bytes across cascade widths.
/// events_in_quantum may differ (width leak residual — resume lease, not D014-1 SoT).
#[test]
fn receipt_digest_len_constant_across_cascade_widths() {
    let mut narrow = Runtime::default();
    let art = narrow.intern("art-wp");
    let rel = narrow.intern("relatedTo");
    let obj = narrow.intern("obj-wp");
    let n_out = narrow
        .emit(Op::Assert {
            subject: art,
            relation: rel,
            object: obj,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();

    let mut wide = Runtime::default();
    let alice = wide.intern("Alice-wp");
    let bob = wide.intern("Bob-wp");
    let knows = wide.intern("knows");
    let w_out = wide
        .emit(Op::Assert {
            subject: alice,
            relation: knows,
            object: bob,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();

    assert_eq!(n_out.receipt.digest.len(), 32);
    assert_eq!(w_out.receipt.digest.len(), 32);
    assert_eq!(
        n_out.receipt.digest.len(),
        w_out.receipt.digest.len(),
        "D014-1: digest commitment size must not depend on cascade width"
    );
    assert!(
        w_out.events_in_quantum > n_out.events_in_quantum,
        "characterization: cascade width differs (knows+inverse vs relatedTo)"
    );
    assert_eq!(
        n_out.events_in_quantum,
        n_out.receipt.event_ids.len(),
        "event_ids lease length tracks width"
    );

    let n_hex = &narrow.outcome_records().last().unwrap().receipt_digest_hex;
    let w_hex = &wide.outcome_records().last().unwrap().receipt_digest_hex;
    assert_eq!(n_hex.len(), 64);
    assert_eq!(w_hex.len(), 64);
}
