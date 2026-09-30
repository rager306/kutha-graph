//! M012a S05 / TIME-01..02: fixture years are YearCe; transaction time is log sequence.

use kutha_common::{Op, TimeScale, ValidTime, TRANSACTION_TIME_SCALE, VALID_TIME_SCALE};
use kutha_runtime::Runtime;

/// FF5 `T_OLD` 2015 and `T_NEW` 2021 are the same YearCe mapping as these fixtures.
#[test]
fn fixture_years_map_to_year_ce_scale() {
    assert_eq!(VALID_TIME_SCALE, TimeScale::YearCe);
    let y2015: ValidTime = 2015;
    let y2017: ValidTime = 2017;
    let y2021: ValidTime = 2021;
    assert_eq!(y2015, 2015);
    assert_eq!(y2017, 2017);
    assert_eq!(y2021, 2021);

    let mut rt = Runtime::default();
    let a = rt.intern("Alice");
    let b = rt.intern("Bob");
    let rel = rt.intern("relatedTo");
    rt.emit(Op::Assert {
        subject: a,
        relation: rel,
        object: b,
        valid_from: 2017,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    assert!(
        rt.fold().as_of(2017).contains(&(a, rel, b)),
        "as_of(2017) must include the YearCe 2017 triple"
    );
    assert!(
        !rt.fold().as_of(2016).contains(&(a, rel, b)),
        "as_of(2016) must not include a fact valid_from 2017"
    );
}

/// Transaction-time is log sequence (`TRANSACTION_TIME_SCALE`), not wall-clock.
/// TT-to-wall mapping is out of M012a S05.
#[test]
fn transaction_time_is_log_sequence() {
    assert_eq!(TRANSACTION_TIME_SCALE, TimeScale::LogSequence);

    let mut rt = Runtime::default();
    let a = rt.intern("Alice");
    let b = rt.intern("Bob");
    let c = rt.intern("Carol");
    let rel = rt.intern("relatedTo");
    rt.emit(Op::Assert {
        subject: a,
        relation: rel,
        object: b,
        valid_from: 2017,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit(Op::Assert {
        subject: a,
        relation: rel,
        object: c,
        valid_from: 2017,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();

    let asserts: Vec<_> = rt
        .log()
        .iter()
        .filter(|e| matches!(e.op, Op::Assert { .. }))
        .collect();
    assert_eq!(asserts.len(), 2);
    assert!(
        asserts[1].ingested_at > asserts[0].ingested_at,
        "second Assert must have a later log-sequence ingested_at"
    );
    assert!(
        asserts[0].ingested_at < 10_000 && asserts[1].ingested_at < 10_000,
        "ingested_at is log sequence, not Unix epoch milliseconds"
    );
}
