//! M042 S01 / Phase 102 — trailing dict Retract across PersistLayout + shrink corpus.
//! Extends SEM-m028-dict-retract (single default persist) to all layouts.
//! ADR-023 Clarification Phase 102. Honeycomb stays Proposed.

mod common;

use common::sem::persistence::{
    PersistLayout, persist_dir, persist_with_layout, rm_dir, tiny_segment_limits,
};
use common::sem::shrink::{load_shrunk, save_shrunk_to};
use kutha_common::{Event, Op};
use kutha_runtime::{Runtime, RuntimeError, store};

const LAYOUTS: [PersistLayout; 4] = [
    PersistLayout::Legacy,
    PersistLayout::Segmented,
    PersistLayout::Rollover,
    PersistLayout::Migrated,
];

/// Pad so Rollover under tiny limits can seal ≥2 segments before the trailing Retract.
fn pad_for_rollover_seam(rt: &mut Runtime) {
    let pad = "m042-pad-rel";
    let _ = rt.emit_allow(pad, 2010, None).unwrap();
    let rel = rt.intern(pad);
    for i in 0..8u32 {
        let s = rt.intern(&format!("m042-pad-s-{i}"));
        let o = rt.intern(&format!("m042-pad-o-{i}"));
        rt.emit(Op::Assert {
            subject: s,
            relation: rel,
            object: o,
            valid_from: 2010 + u64::from(i),
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
    }
}

fn assert_relation_stays_revoked(
    opened: &mut Runtime,
    name: &str,
    allow_id: kutha_common::EventId,
) {
    assert!(
        opened
            .log()
            .iter()
            .any(|e| matches!(e.op, Op::Retract { event_id } if event_id == allow_id)),
        "open must keep trailing emit_retract_dict for {name}"
    );
    let rel = opened.intern(name);
    let s = opened.intern(&format!("{name}-s"));
    let o = opened.intern(&format!("{name}-o"));
    let err = opened
        .emit(Op::Assert {
            subject: s,
            relation: rel,
            object: o,
            valid_from: 2017,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap_err();
    match err {
        RuntimeError::UnknownRelation { name: got } => {
            assert_eq!(got, name, "revoked relation name after open");
        }
        other => panic!("expected UnknownRelation for {name}, got {other:?}"),
    }
}

fn retract_case_with_user_assert(name: &str) -> (Runtime, kutha_common::EventId) {
    let mut rt = Runtime::default();
    let allow = rt.emit_allow(name, 2017, None).unwrap();
    let allow_id = allow.receipt.event_ids[0];
    let leased = rt.intern(name);
    let subject = rt.intern("m042-s");
    let object = rt.intern("m042-o");
    rt.emit(Op::Assert {
        subject,
        relation: leased,
        object,
        valid_from: 2017,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    pad_for_rollover_seam(&mut rt);
    rt.emit_retract_dict(allow_id).unwrap();
    assert!(
        matches!(
            rt.log().as_slice().last().map(|e| &e.op),
            Some(Op::Retract { .. })
        ),
        "retract must be the trailing log op"
    );
    (rt, allow_id)
}

fn retract_case_solo(name: &str) -> (Runtime, kutha_common::EventId) {
    let mut rt = Runtime::default();
    let allow = rt.emit_allow(name, 2017, None).unwrap();
    let allow_id = allow.receipt.event_ids[0];
    pad_for_rollover_seam(&mut rt);
    rt.emit_retract_dict(allow_id).unwrap();
    (rt, allow_id)
}

fn assert_persist_open_revoked(
    rt: &Runtime,
    allow_id: kutha_common::EventId,
    name: &str,
    layout: PersistLayout,
) {
    let dir = persist_dir(&format!("m042-{name}-{layout:?}"));
    if matches!(layout, PersistLayout::Rollover) {
        let _g = tiny_segment_limits();
        persist_with_layout(rt, &dir, layout);
    } else {
        persist_with_layout(rt, &dir, layout);
    }
    let mut opened = store::open(&dir).unwrap();
    assert_relation_stays_revoked(&mut opened, name, allow_id);
    rm_dir(&dir);
}

#[test]
fn trailing_dict_retract_survives_all_persist_layouts() {
    for layout in LAYOUTS {
        let name = format!("m042-open-{layout:?}");
        let (rt, allow_id) = retract_case_with_user_assert(&name);
        assert_persist_open_revoked(&rt, allow_id, &name, layout);
    }
}

#[test]
fn trailing_dict_retract_solo_survives_all_persist_layouts() {
    for layout in LAYOUTS {
        let name = format!("m042-solo-{layout:?}");
        let (rt, allow_id) = retract_case_solo(&name);
        assert_persist_open_revoked(&rt, allow_id, &name, layout);
    }
}

#[test]
fn shrink_corpus_roundtrip_load_matches_saved() {
    let events: Vec<Event> = (0..3)
        .map(|i| {
            Event::new(
                Op::StoreBlob {
                    digest: format!("corpus-{i}"),
                    bytes: vec![i as u8; 4],
                },
                i as u64,
            )
        })
        .collect();
    let root = persist_dir("shrink-corpus");
    let path = save_shrunk_to(&root, 42, &events);
    assert!(
        path.exists(),
        "save_shrunk_to must create {}",
        path.display()
    );
    let loaded = load_shrunk(&path).expect("load_shrunk");
    assert_eq!(loaded, events, "corpus roundtrip must preserve Event bytes");
    // Content-addressed: same payload → same path
    let path2 = save_shrunk_to(&root, 42, &events);
    assert_eq!(path, path2, "content-addressed name must be stable");
    rm_dir(&root);
}
