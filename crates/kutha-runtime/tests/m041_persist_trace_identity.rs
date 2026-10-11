//! M041 S01 / Phase 101 — persist+open EventId identity across layouts + shrink gate.
//! Stamps already covered by SEM-persist-roundtrip-random; this pins non-Define EventIds
//! after open (Define ops leave the log; see `expected_open_event_ids`).
//! ADR-023 Clarification Phase 101. Honeycomb stays Proposed.

mod common;

use common::sem::generator::{MixStyle, assert_seed_budget, ingest_mix, persist_seed_count, seeds};
use common::sem::oracles::persist_ids_disagree;
use common::sem::persistence::{
    PersistLayout, expected_open_event_ids, persist_dir, persist_with_layout, rm_dir, sealed_count,
    try_persist_with_layout,
};
use common::sem::shrink::{candidate_deps_hold, fail_shrunk, shrink_events_ddmin};
use kutha_common::{Event, EventId, Op};
use kutha_runtime::{FoldIndexMode, Runtime, store};

#[test]
fn candidate_deps_hold_rejects_orphan_retract() {
    let mut rt = Runtime::default();
    let s = rt.intern("m041-s");
    let r = rt.intern("m041-rel");
    let o = rt.intern("m041-o");
    let _ = rt.emit_allow("m041-rel", 2010, None).unwrap();
    let q = rt
        .emit(Op::Assert {
            subject: s,
            relation: r,
            object: o,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
    let assert_id = q.receipt.event_ids[0];
    let orphan = Event::new(
        Op::Retract {
            event_id: EventId::from_u128(0xdead_beef),
        },
        1u64,
    );
    assert!(
        !candidate_deps_hold(std::slice::from_ref(&orphan)),
        "orphan Retract must fail deps gate"
    );
    let linked = Event::new(
        Op::Retract {
            event_id: assert_id,
        },
        2u64,
    );
    let mut cand = rt.log().as_slice().to_vec();
    cand.push(linked);
    assert!(
        candidate_deps_hold(&cand),
        "Retract targeting an Assert in the candidate must pass"
    );
}

#[test]
fn shrink_events_ddmin_drops_middle_when_prefix_biased() {
    let events: Vec<Event> = (0..5)
        .map(|i| {
            Event::new(
                Op::StoreBlob {
                    digest: format!("b{i}"),
                    bytes: vec![i as u8],
                },
                i as u64,
            )
        })
        .collect();
    let first = events[0].id;
    let last = events[4].id;
    let pred = |cand: &[Event]| {
        let ids: Vec<_> = cand.iter().map(|e| e.id).collect();
        ids.contains(&first) && ids.contains(&last)
    };
    let shrunk = shrink_events_ddmin(&events, pred);
    assert!(
        shrunk.len() <= 2,
        "ddmin should strip middle noise; got n={} ops",
        shrunk.len()
    );
    let ids: Vec<_> = shrunk.iter().map(|e| e.id).collect();
    assert!(ids.contains(&first) && ids.contains(&last));
}

#[test]
fn try_persist_rollover_tiny_does_not_panic() {
    // Default Runtime may still seal ≥2 under tiny limits (dict Defines). Goal: no panic;
    // seam reject is sealed < 2, not "empty Runtime".
    let rt = Runtime::default();
    let dir = persist_dir("tiny-roll");
    let ok = try_persist_with_layout(&rt, &dir, PersistLayout::Rollover);
    let sealed = if dir.join(store::MANIFEST_REL).exists() {
        sealed_count(&dir)
    } else {
        0
    };
    assert!(
        ok == (sealed >= 2),
        "try_persist Rollover iff seam sealed>=2; ok={ok} sealed={sealed}"
    );
    rm_dir(&dir);
}

#[test]
fn persist_roundtrip_trace_identity_across_layouts() {
    assert_seed_budget();
    let layouts = [
        PersistLayout::Legacy,
        PersistLayout::Segmented,
        PersistLayout::Rollover,
        PersistLayout::Migrated,
    ];
    for (i, seed) in seeds().into_iter().take(persist_seed_count()).enumerate() {
        let mut rt = Runtime::with_index_mode(32, FoldIndexMode::Eager);
        ingest_mix(&mut rt, seed, MixStyle::BehaviorFree);
        let events = rt.log().as_slice();
        let want = expected_open_event_ids(&rt);
        let mut layout_ids: Vec<(PersistLayout, Vec<EventId>)> = Vec::new();
        for (pass, layout) in layouts.into_iter().enumerate() {
            let case = i * 10 + pass;
            let dir = persist_dir(&format!("id-{layout:?}"));
            persist_with_layout(&rt, &dir, layout);
            let opened = store::open(&dir).unwrap_or_else(|e| {
                fail_shrunk(
                    seed,
                    case,
                    events,
                    |prefix| persist_ids_disagree(&rt, prefix, layout, seed),
                    &format!("persist+open failed under {layout:?}: {e}"),
                );
            });
            let got: Vec<EventId> = opened.log().iter().map(|e| e.id).collect();
            if got != want {
                fail_shrunk(
                    seed,
                    case,
                    events,
                    |prefix| persist_ids_disagree(&rt, prefix, layout, seed),
                    &format!("{layout:?} must preserve open EventId trace (non-Define)"),
                );
            }
            layout_ids.push((layout, got));
            rm_dir(&dir);
        }
        let first = &layout_ids[0].1;
        for (layout, ids) in &layout_ids[1..] {
            assert_eq!(
                ids, first,
                "seed={seed}: {layout:?} EventId trace must match Legacy"
            );
        }
    }
}
