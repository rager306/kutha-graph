//! Thin demo: statute AS OF + "norm in force on D, as known on W" via `norm_view`.
//! Not ADR-070 Cypher. Fixture texts live under `fixtures/statute/` (test data only).

use kutha_common::{ValidTime, WallTime};
use kutha_runtime::{NormView, Runtime, RuntimeError};

fn facade_example_store_dir() -> std::path::PathBuf {
    // Unique even when two Cargo processes run this example together.
    static N: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let seq = N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .join(format!(
            "p27-ex-statute-{}-{nanos}-{seq}",
            std::process::id()
        ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("persist dir");
    dir
}

fn fmt_norm(rt: &Runtime, view: &NormView) -> String {
    let dict = rt.dictionary();
    match view {
        NormView::Ok(triples) => {
            let parts: Vec<String> = triples
                .iter()
                .map(|t| {
                    format!(
                        "({} {} {})",
                        dict.lookup(t.subject).unwrap_or("?"),
                        dict.lookup(t.relation).unwrap_or("?"),
                        dict.lookup(t.object).unwrap_or("?")
                    )
                })
                .collect();
            format!("Ok [{}]", parts.join(", "))
        }
        NormView::Conflict { claim, report } => format!(
            "Conflict(claim={claim}, +={}, -={})",
            report.positive_supports.len(),
            report.negative_supports.len()
        ),
        NormView::Abstain => "Abstain".into(),
        NormView::Unknown => "Unknown".into(),
        NormView::MissingAnchor => "MissingAnchor".into(),
    }
}

fn main() -> Result<(), RuntimeError> {
    let mut rt = Runtime::default();
    let first = rt.assert("art-12-speed", "inForceAs", "50-kmh", 2010, Some(2020))?;
    rt.assert("art-12-speed", "inForceAs", "30-kmh", 2020, None)?;

    let old = rt.as_of(ValidTime::from(2015));
    let new = rt.as_of(ValidTime::from(2021));
    assert_ne!(old, new, "AS OF 2015 must differ from AS OF 2021");
    let _live = rt.live_at(u64::MAX, 2015);

    // Knowledge anchors for the envelope demo (force ∩ knowledge).
    rt.emit_tt_anchor(2015)?;
    rt.emit_tt_anchor(2021)?;

    let nv_2015 = rt.norm_view(ValidTime::from(2015), WallTime::from(2015));
    let nv_2021 = rt.norm_view(ValidTime::from(2021), WallTime::from(2021));
    println!(
        "norm in force on 2015, as known on 2015 → {}",
        fmt_norm(&rt, &nv_2015)
    );
    println!(
        "norm in force on 2021, as known on 2021 → {}",
        fmt_norm(&rt, &nv_2021)
    );
    match (&nv_2015, &nv_2021) {
        (NormView::Ok(a), NormView::Ok(b)) => {
            assert!(
                a.iter()
                    .any(|t| rt.dictionary().lookup(t.object) == Some("50-kmh")),
                "2015 envelope must list 50-kmh"
            );
            assert!(
                b.iter()
                    .any(|t| rt.dictionary().lookup(t.object) == Some("30-kmh")),
                "2021 envelope must list 30-kmh"
            );
            assert_ne!(
                a.iter().map(|t| t.object).collect::<Vec<_>>(),
                b.iter().map(|t| t.object).collect::<Vec<_>>(),
                "force cuts must differ"
            );
        }
        other => panic!("expected Ok/Ok from norm_view demo, got {other:?}"),
    }

    rt.emit_policy("leased-policy", 2010, None)?;
    let claim = first.receipt.event_ids[0];
    let jid = rt.record_justification(claim, vec![claim], vec![claim], "", u64::MAX, 2015)?;
    let _action = rt.record_action(&jid, u64::MAX, 2015)?;
    rt.derive("statute-fold", "art-12-speed", 2015)?;
    rt.correct("art-12-speed", "inForceAs", "50-kmh", 2010)?;
    rt.retract("art-12-speed", 2010)?;

    let dir = facade_example_store_dir();
    rt.persist(&dir)?;
    let opened = Runtime::open(&dir)?;
    let _fork = opened.fork_at(0).unwrap();
    opened.replay_check()?;
    let _ = std::fs::remove_dir_all(dir);
    Ok(())
}
