//! Ingest `.kutha/events.jsonl` onto the product runtime (H2 tenant).
//! Paths from env: `KUTHA_HARNESS_LOG`, `KUTHA_TENANT_DIR`.

use kutha_runtime::{ingest_harness_jsonl, store, Runtime};
use std::path::Path;
use std::process::ExitCode;

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| default.to_string())
}

fn main() -> ExitCode {
    let harness = env_or("KUTHA_HARNESS_LOG", ".kutha/events.jsonl");
    let dir = env_or("KUTHA_TENANT_DIR", ".kutha/tenant");
    let harness_path = Path::new(&harness);
    if !harness_path.is_file() {
        eprintln!("tenant: missing harness log {harness} (evidence, not SoT)");
        return ExitCode::from(2);
    }
    let mut rt = Runtime::default();
    let report = match ingest_harness_jsonl(&mut rt, harness_path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("tenant: ingest error: {e}");
            return ExitCode::from(1);
        }
    };
    if report.status_facts == 0 {
        eprintln!("tenant: ingested=0 last_status=none as_of_match=0");
        return ExitCode::from(1);
    }
    if let Err(e) = rt.replay_check() {
        eprintln!("tenant: replay: {e}");
        return ExitCode::from(1);
    }
    let last_vf = report.last_valid_from.unwrap_or(0);
    let last_status = report.last_status.clone().unwrap_or_else(|| "none".into());
    let run = rt.intern("harness.run");
    let rel = rt.intern("runStatus");
    let obj = rt.intern(&last_status);
    let matched = rt.fold().as_of(last_vf).contains(&(run, rel, obj));
    if let Err(e) = store::persist(&rt, Path::new(&dir)) {
        eprintln!("tenant: persist {dir}: {e}");
        return ExitCode::from(1);
    }
    println!(
        "tenant: ingested={} last_status={} last_vf={} as_of_match={} dir={}  (tenant log, not architecture SoT)",
        report.status_facts, last_status, last_vf, u8::from(matched), dir
    );
    if matched {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}
