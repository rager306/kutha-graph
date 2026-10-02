//! HARD-03: FF5 statute under Eager vs Lazy (cold scan, no ensure_fold_indexes before reads).

use kutha_common::{EventId, Op};
use kutha_runtime::{FoldIndexMode, Runtime};

const T_OLD: u64 = 2015;
const T_NEW: u64 = 2021;
const VF_FIRST: u64 = 2010;
const VF_SECOND: u64 = 2020;

fn ff5_statute(rt: &mut Runtime) -> EventId {
    let article = rt.intern("art-12-speed");
    let in_force = rt.intern("inForceAs");
    let limit_50 = rt.intern("50-kmh");
    let limit_30 = rt.intern("30-kmh");

    let first_event = rt
        .emit(Op::Assert {
            subject: article,
            relation: in_force,
            object: limit_50,
            valid_from: VF_FIRST,
            valid_to: Some(VF_SECOND),
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap()
        .receipt
        .event_ids[0];
    rt.emit(Op::Assert {
        subject: article,
        relation: in_force,
        object: limit_30,
        valid_from: VF_SECOND,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    first_event
}

fn assert_ff5_shape(rt: &mut Runtime) {
    let article = rt.intern("art-12-speed");
    let in_force = rt.intern("inForceAs");
    let limit_50 = rt.intern("50-kmh");
    let limit_30 = rt.intern("30-kmh");

    let old = rt.as_of(T_OLD);
    let new = rt.as_of(T_NEW);
    assert!(old.contains(&(article, in_force, limit_50)));
    assert!(!old.contains(&(article, in_force, limit_30)));
    assert!(new.contains(&(article, in_force, limit_30)));
    assert!(!new.contains(&(article, in_force, limit_50)));
}

fn sorted_as_of(rt: &Runtime, vt: u64) -> Vec<(u32, u32, u32)> {
    let mut v = rt.as_of(vt);
    v.sort_unstable();
    v
}

fn sorted_live_at(rt: &Runtime, tt: u64, vt: u64) -> Vec<(u32, u32, u32)> {
    let mut v = rt.live_at(tt, vt);
    v.sort_unstable();
    v
}

fn assert_modes_agree(
    eager: &Runtime,
    lazy: &Runtime,
    eager_mint: EventId,
    lazy_mint: EventId,
) {
    for &vt in &[T_OLD, T_NEW] {
        assert_eq!(
            sorted_as_of(eager, vt),
            sorted_as_of(lazy, vt),
            "as_of({vt}) Eager vs Lazy"
        );
        assert_eq!(
            sorted_live_at(eager, u64::MAX, vt),
            sorted_live_at(lazy, u64::MAX, vt),
            "live_at(MAX,{vt})"
        );
    }
    for &vt in &[T_OLD, T_NEW, VF_FIRST, VF_SECOND] {
        assert_eq!(
            eager.fold().claim_supported_at(eager_mint, u64::MAX, vt),
            lazy.fold().claim_supported_at(lazy_mint, u64::MAX, vt),
            "claim_supported_at mint at vt={vt}"
        );
    }
}

fn first_interval_event_id(rt: &Runtime) -> EventId {
    rt.fold()
        .facts()
        .iter()
        .find(|f| f.valid_from == VF_FIRST && f.valid_to == Some(VF_SECOND))
        .expect("2010 interval fact")
        .event_id
}

fn second_open_interval_fact(rt: &Runtime) -> EventId {
    rt.fold()
        .facts()
        .iter()
        .find(|f| {
            f.valid_from == VF_SECOND
                && f.valid_to.is_none()
                && f.invalidated_at.is_none()
        })
        .expect("2020 open interval fact")
        .event_id
}

#[test]
fn dual_mode_ff5_retract_correct_claim_agree() {
    let mut eager = Runtime::with_index_mode(32, FoldIndexMode::Eager);
    let mut lazy = Runtime::with_index_mode(32, FoldIndexMode::Lazy);

    let eager_mint = ff5_statute(&mut eager);
    let lazy_mint = ff5_statute(&mut lazy);

    assert_ff5_shape(&mut eager);
    assert_ff5_shape(&mut lazy);
    assert_modes_agree(&eager, &lazy, eager_mint, lazy_mint);

    eager
        .emit(Op::Retract {
            event_id: first_interval_event_id(&eager),
        })
        .unwrap();
    lazy.emit(Op::Retract {
        event_id: first_interval_event_id(&lazy),
    })
    .unwrap();
    assert_modes_agree(&eager, &lazy, eager_mint, lazy_mint);

    let corrected_eager = eager.intern("40-kmh");
    let corrected_lazy = lazy.intern("40-kmh");
    eager
        .emit(Op::Correct {
            event_id: second_open_interval_fact(&eager),
            object: corrected_eager,
            valid_from: VF_SECOND,
            valid_to: None,
        })
        .unwrap();
    lazy.emit(Op::Correct {
        event_id: second_open_interval_fact(&lazy),
        object: corrected_lazy,
        valid_from: VF_SECOND,
        valid_to: None,
    })
    .unwrap();

    let article = eager.intern("art-12-speed");
    let in_force = eager.intern("inForceAs");
    let limit_30 = eager.intern("30-kmh");
    for (rt, limit_40) in [(&eager, corrected_eager), (&lazy, corrected_lazy)] {
        let at_new = rt.as_of(T_NEW);
        assert!(
            at_new.contains(&(article, in_force, limit_40)),
            "Correct must surface 40-kmh at AS OF {T_NEW}"
        );
        assert!(
            !at_new.contains(&(article, in_force, limit_30)),
            "Correct must drop 30-kmh at AS OF {T_NEW}"
        );
    }
    assert_modes_agree(&eager, &lazy, eager_mint, lazy_mint);

    eager.replay_check().unwrap();
    lazy.replay_check().unwrap();
}
