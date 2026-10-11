//! M025 S04 / Phase 84 — real-text statute fixtures (test data only) + demo envelope.
//!
//! Fixtures under `fixtures/statute/` are quotes for oracles, not a legal pack
//! (`I-legal-pack-not-next`). Thin consumer path: `examples/statute_as_of`.

use kutha_common::{Op, ValidTime, WallTime};
use kutha_runtime::{NormView, Runtime};

const ART12: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/fixtures/statute/art12_speed.txt"
));
/// Russian-only payload (CE #36). English metadata stays in this module, not
/// in the fixture file (law-nexus syn-001 / C01 quote — test data only; not an
/// ADR-090 pack thaw).
const PUNKT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/fixtures/statute/punkt_15_1.txt"
));
const EDITION: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/fixtures/statute/edition_promulgation.txt"
));

/// Fixture files are non-empty real-text excerpts (m016 pattern).
#[test]
fn demo_fixtures_are_nonempty_real_text() {
    assert!(ART12.contains("50 km/h") && ART12.contains("30 km/h"));
    assert!(PUNKT.contains("15.1") && PUNKT.contains("2028"));
    assert!(EDITION.contains("2017") && EDITION.contains("2018"));
    assert!(
        !ART12.contains("ADR-090 pack Accepted"),
        "fixtures must not claim pack thaw"
    );
}

/// M056 S05 / CE #36: punkt fixture is single-language (Russian payload only).
#[test]
fn demo_punkt_fixture_is_single_language() {
    assert!(
        PUNKT.contains("пункт") && PUNKT.contains("действует"),
        "payload must stay Russian"
    );
    assert!(
        !PUNKT.contains("law-nexus")
            && !PUNKT.contains("ADR-090")
            && !PUNKT.contains("test data only"),
        "English metadata must not live in the fixture file"
    );
}

/// Art.12 speed: norm in force on 2015 as known on 2015 is 50; on 2021/2021 is 30.
#[test]
fn demo_art12_norm_view_force_cuts_differ() {
    let _quote = ART12; // pin include
    let mut rt = Runtime::default();
    let art = rt.intern("art-12-speed");
    let rel = rt.intern("inForceAs");
    let lim50 = rt.intern("50-kmh");
    let lim30 = rt.intern("30-kmh");

    rt.emit(Op::Assert {
        subject: art,
        relation: rel,
        object: lim50,
        valid_from: 2010,
        valid_to: Some(2020),
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit(Op::Assert {
        subject: art,
        relation: rel,
        object: lim30,
        valid_from: 2020,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit_tt_anchor(2015).unwrap();
    rt.emit_tt_anchor(2021).unwrap();

    match rt.norm_view(ValidTime::from(2015), WallTime::from(2015)) {
        NormView::Ok(triples) => {
            assert!(
                triples.iter().any(|t| t.object == lim50 && t.eligible),
                "2015 force∩knowledge must list 50-kmh, got {triples:?}"
            );
            assert!(
                !triples.iter().any(|t| t.object == lim30),
                "2015 must not list 30-kmh"
            );
        }
        other => panic!("expected Ok at 2015/2015, got {other:?}"),
    }

    match rt.norm_view(ValidTime::from(2021), WallTime::from(2021)) {
        NormView::Ok(triples) => {
            assert!(
                triples.iter().any(|t| t.object == lim30 && t.eligible),
                "2021 force∩knowledge must list 30-kmh, got {triples:?}"
            );
        }
        other => panic!("expected Ok at 2021/2021, got {other:?}"),
    }
}

/// C01 sunset quote: in force through 2027; half-open 2028 → Unknown on norm_view.
#[test]
fn demo_punkt_sunset_norm_view_unknown_after_valid_to() {
    let _quote = PUNKT;
    let mut rt = Runtime::default();
    let punkt = rt.intern("punkt-15-1");
    let in_force = rt.intern("inForceAs");
    let true_ = rt.intern("true");

    rt.emit(Op::Assert {
        subject: punkt,
        relation: in_force,
        object: true_,
        valid_from: 2010,
        valid_to: Some(2028),
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit_tt_anchor(2026).unwrap();

    match rt.norm_view(ValidTime::from(2027), WallTime::from(2026)) {
        NormView::Ok(triples) => {
            assert!(triples.iter().any(|t| {
                t.subject == punkt && t.relation == in_force && t.object == true_
            }));
        }
        other => panic!("expected Ok before sunset, got {other:?}"),
    }
    assert_eq!(
        rt.norm_view(ValidTime::from(2028), WallTime::from(2026)),
        NormView::Unknown,
        "half-open valid_to=2028 is not in force"
    );
}

/// Promulgation 2017 / force 2018: knowledge cut sees fact; norm_view at vt=2017 is Unknown.
#[test]
fn demo_edition_promulgation_knowledge_not_force() {
    let _quote = EDITION;
    let mut rt = Runtime::default();
    let act = rt.intern("act-edition-1");
    let in_force = rt.intern("inForceAs");
    let true_ = rt.intern("true");

    // Ingest then anchor so knowledge TT includes the fact (TT-only known-on).
    rt.emit(Op::Assert {
        subject: act,
        relation: in_force,
        object: true_,
        valid_from: 2018,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit_tt_anchor(2017).unwrap();

    let known = rt
        .as_known_on(WallTime::from(2017))
        .expect("promulgation anchor");
    assert!(
        known.contains(&(act, in_force, true_)),
        "as_known_on is TT-only: promulgation wall sees ingested fact"
    );
    assert_eq!(
        rt.norm_view(ValidTime::from(2017), WallTime::from(2017)),
        NormView::Unknown,
        "norm_view force cut must not treat promulgation year as in-force"
    );
    match rt.norm_view(ValidTime::from(2018), WallTime::from(2017)) {
        NormView::Ok(triples) => {
            assert!(triples.iter().any(|t| t.subject == act && t.eligible));
        }
        other => panic!("expected Ok at force 2018 / knowledge 2017, got {other:?}"),
    }
}
