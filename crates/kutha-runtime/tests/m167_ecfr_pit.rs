//! M167 S01 — eCFR PIT probe. Real public text, three editions per section.
//! This is test data, not a legal pack. Day-scale VT (`YYYYMMDD`) is probe-only.

use kutha_common::{Op, TermId, ValidTime, WallTime};
use kutha_runtime::{store, NormView, Runtime};

mod common;

const SECTIONS: [(&str, &str, &str, &str); 3] = [
    (
        "304.1",
        include_str!("fixtures/ecfr/304.1_2015-12-18.txt"),
        include_str!("fixtures/ecfr/304.1_2017-01-23.txt"),
        include_str!("fixtures/ecfr/304.1_2017-03-14.txt"),
    ),
    (
        "304.2",
        include_str!("fixtures/ecfr/304.2_2015-12-18.txt"),
        include_str!("fixtures/ecfr/304.2_2017-01-23.txt"),
        include_str!("fixtures/ecfr/304.2_2017-03-14.txt"),
    ),
    (
        "304.10",
        include_str!("fixtures/ecfr/304.10_2015-12-18.txt"),
        include_str!("fixtures/ecfr/304.10_2017-01-23.txt"),
        include_str!("fixtures/ecfr/304.10_2017-03-14.txt"),
    ),
];

const DATES: [u64; 3] = [20151218, 20170123, 20170314];
/// Day-scale probe cuts. `None` means before the first edition.
const AS_OF: [(u64, Option<usize>); 7] = [
    (20151217, None),
    (20151218, Some(0)),
    (20170122, Some(0)),
    (20170123, Some(1)),
    (20170313, Some(1)),
    (20170314, Some(2)),
    (20180101, Some(2)),
];

fn edition_text(section: usize, edition: usize) -> &'static str {
    match edition {
        0 => SECTIONS[section].1,
        1 => SECTIONS[section].2,
        _ => SECTIONS[section].3,
    }
}

fn object_text(rt: &Runtime, triples: &[(TermId, TermId, TermId)], subject: TermId) -> Option<String> {
    triples
        .iter()
        .find(|(s, _, _)| *s == subject)
        .and_then(|(_, _, o)| rt.dictionary().lookup(*o).map(str::to_string))
}

fn object_texts(rt: &Runtime, triples: &[(TermId, TermId, TermId)], subject: TermId) -> Vec<String> {
    triples
        .iter()
        .filter(|(s, _, _)| *s == subject)
        .filter_map(|(_, _, o)| rt.dictionary().lookup(*o).map(str::to_string))
        .collect()
}

fn norm_text(view: &NormView, subject: TermId, rt: &Runtime) -> Option<String> {
    match view {
        NormView::Ok(rows) => rows
            .iter()
            .find(|row| row.subject == subject)
            .and_then(|row| rt.dictionary().lookup(row.object).map(str::to_string)),
        _ => None,
    }
}

struct LimitGuard;
impl Drop for LimitGuard {
    fn drop(&mut self) {
        store::reset_test_segment_limits();
    }
}

#[test]
fn ecfr_pit_probe_reads_three_editions_from_real_text() {
    let _limits = LimitGuard;
    store::set_test_segment_limits(Some(4096), Some(2));

    let mut rt = Runtime::default();
    let relation = rt.intern("inForceAs");
    let mut subjects = Vec::with_capacity(SECTIONS.len());
    for (section, _, _, _) in SECTIONS.iter() {
        subjects.push(rt.intern(&format!("1 CFR {section}")));
    }

    for (edition, date) in DATES.iter().enumerate() {
        for (section, subject) in subjects.iter().enumerate() {
            let object = rt.intern(edition_text(section, edition));
            rt.emit(Op::Assert {
                subject: *subject,
                relation,
                object,
                valid_from: *date,
                valid_to: if edition + 1 < DATES.len() {
                    Some(DATES[edition + 1])
                } else {
                    None
                },
                claim: None,
                delivery_key: None,
                polarity: None,
            })
            .unwrap();
        }
        rt.emit_tt_anchor(*date).unwrap();
    }

    // as_of: force-only cuts, including the day before/at each boundary.
    // The first two editions are textually equal for all three sections; the
    // third differs, so 2017-03-13 vs 2017-03-14 is the discriminating pair.
    for (section, subject) in subjects.iter().enumerate() {
        for (cut, expected) in AS_OF.iter() {
            let triples = rt.as_of(ValidTime::from(*cut));
            match expected {
                None => assert!(object_text(&rt, &triples, *subject).is_none()),
                Some(edition) => assert_eq!(
                    object_text(&rt, &triples, *subject),
                    Some(edition_text(section, *edition).to_string()),
                    "cut {cut} section {section}"
                ),
            }
        }
        assert_eq!(edition_text(section, 0), edition_text(section, 1));
        assert_ne!(edition_text(section, 1), edition_text(section, 2));
    }

    // as_known_on is TT-only: at a later wall it lists every edition known by
    // then, including superseded VT rows. It must not show a future edition.
    for (section, subject) in subjects.iter().enumerate() {
        for (edition, wall) in DATES.iter().enumerate() {
            let known = rt.as_known_on(WallTime::from(*wall)).expect("anchor");
            let texts = object_texts(&rt, &known, *subject);
            assert!(texts.contains(&edition_text(section, 0).to_string()));
            if edition >= 1 {
                assert!(texts.contains(&edition_text(section, 1).to_string()));
            }
            if edition < 2 {
                assert!(!texts.contains(&edition_text(section, 2).to_string()));
            } else {
                assert!(texts.contains(&edition_text(section, 2).to_string()));
            }
        }
    }

    // norm_view: force ∩ knowledge. A late force cut with an early wall must not
    // expose a later edition.
    for (section, subject) in subjects.iter().enumerate() {
        let early = rt.norm_view(ValidTime::from(20151218), WallTime::from(DATES[0]));
        assert_eq!(
            norm_text(&early, *subject, &rt),
            Some(edition_text(section, 0).to_string())
        );
        let late_force_early_wall = rt.norm_view(ValidTime::from(20170314), WallTime::from(DATES[0]));
        assert_ne!(
            norm_text(&late_force_early_wall, *subject, &rt),
            Some(edition_text(section, 2).to_string()),
            "knowledge cut must hide the 2017-03 edition"
        );
        let late = rt.norm_view(ValidTime::from(20170314), WallTime::from(DATES[2]));
        assert_eq!(
            norm_text(&late, *subject, &rt),
            Some(edition_text(section, 2).to_string())
        );
    }

    rt.replay_check().unwrap();

    // Persist, checkpoint the sealed prefix, reopen; replay/answer parity.
    let dir = common::PersistScratch::new("m167-ecfr-pit");
    rt.persist(&dir).unwrap();
    store::checkpoint_sealed_prefix(&dir).unwrap();
    let opened = Runtime::open(&dir).unwrap();
    opened.replay_check().unwrap();
    for (section, subject) in subjects.iter().enumerate() {
        for (cut, expected) in AS_OF.iter() {
            let triples = opened.as_of(ValidTime::from(*cut));
            match expected {
                None => assert!(object_text(&opened, &triples, *subject).is_none()),
                Some(edition) => assert_eq!(
                    object_text(&opened, &triples, *subject),
                    Some(edition_text(section, *edition).to_string())
                ),
            }
        }
    }
}
