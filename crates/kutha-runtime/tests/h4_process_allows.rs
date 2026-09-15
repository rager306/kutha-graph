//! H4: process-relations membership editions as a chained tenant projection.
//! Overlay dogfood, not a legal pack. Tip YAML remains the admit lease.

use kutha_runtime::Runtime;

const CUT_A: u64 = 1000;
const CUT_B: u64 = 2000;

fn snapshot_has(rt: &mut Runtime, at: u64, object: &str) -> bool {
    let subj = rt.intern("process.relations");
    let rel = rt.intern("processAllows");
    let obj = rt.intern(object);
    rt.fold().as_of(at).contains(&(subj, rel, obj))
}

#[test]
fn h4_prior_cut_keeps_status_membership_after_later_edition_drops_it() {
    let mut rt = Runtime::default();
    kutha_runtime::ingest_harness_jsonl_str(
        &mut rt,
        concat!(
            r#"{"op":"assert","subject":"process.relations","relation":"allows","object":"allows,status","ingested_at":1000,"valid_from":1000}"#,
            "\n",
            r#"{"op":"assert","subject":"process.relations","relation":"allows","object":"allows","ingested_at":2000,"valid_from":2000}"#,
        ),
    )
    .unwrap();

    assert!(
        snapshot_has(&mut rt, CUT_A, "allows,status"),
        "AS OF prior cut must decode a set containing status"
    );
    assert!(!snapshot_has(&mut rt, CUT_A, "allows"));
    assert!(snapshot_has(&mut rt, CUT_B, "allows"));
    assert!(!snapshot_has(&mut rt, CUT_B, "allows,status"));
    rt.replay_check().unwrap();
}

#[test]
fn h4_same_second_membership_as_of_uses_emitted_cut() {
    let mut rt = Runtime::default();
    kutha_runtime::ingest_harness_jsonl_str(
        &mut rt,
        concat!(
            r#"{"op":"assert","subject":"process.relations","relation":"allows","object":"allows,status","ingested_at":1000,"valid_from":1000}"#,
            "\n",
            r#"{"op":"assert","subject":"process.relations","relation":"allows","object":"allows","ingested_at":1000,"valid_from":1000}"#,
            "\n",
            r#"{"op":"assert","subject":"harness.run","relation":"status","object":"ok","ingested_at":1000,"valid_from":1000}"#,
        ),
    )
    .unwrap();

    assert!(
        snapshot_has(&mut rt, 1000, "allows,status"),
        "first same-second edition stays at source valid_from"
    );
    assert!(
        snapshot_has(&mut rt, 1001, "allows"),
        "later same-second edition must bump the membership cut"
    );
    assert!(!snapshot_has(&mut rt, 1001, "allows,status"));
    rt.replay_check().unwrap();
}
