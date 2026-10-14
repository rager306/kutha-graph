//! Operator CLI for directory Op-wire then pin migrate (`store::migrate_compat`).
//!
//! Usage: `kutha-migrate <store-dir>` or `KUTHA_MIGRATE_DIR=<store-dir> kutha-migrate`.
//! Reuses the M076 sequence facade; does not invent remap steps.

use kutha_runtime::store::{self, CompatMigrateReport};
use std::env;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

fn resolve_dir(args: &[String]) -> Result<PathBuf, i32> {
    if let Some(arg) = args.get(1) {
        if arg == "-h" || arg == "--help" {
            eprintln!(
                "usage: kutha-migrate <store-dir>\n       KUTHA_MIGRATE_DIR=<store-dir> kutha-migrate"
            );
            return Err(2);
        }
        return Ok(PathBuf::from(arg));
    }
    match env::var("KUTHA_MIGRATE_DIR") {
        Ok(v) if !v.trim().is_empty() => Ok(PathBuf::from(v.trim())),
        _ => {
            eprintln!("kutha-migrate: missing store dir (arg or KUTHA_MIGRATE_DIR)");
            Err(2)
        }
    }
}

fn format_report(dir: &Path, report: &CompatMigrateReport) -> String {
    format!(
        "migrate: op_wire_remapped={} op_wire_already={} pin_wire_remapped={} pin_wire_already={} pin_cite_remapped={} pin_cite_already={} dir={}  (compat sequence, not honeycomb Acceptance)",
        report.op_wire.remapped_lines,
        u8::from(report.op_wire.already_migrated),
        report.pins.wire.remapped_lines,
        u8::from(report.pins.wire.already_migrated),
        report.pins.cite.remapped_lines,
        u8::from(report.pins.cite.already_migrated),
        dir.display()
    )
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let dir = match resolve_dir(&args) {
        Ok(p) => p,
        Err(code) => return ExitCode::from(code as u8),
    };
    if !dir.is_dir() {
        eprintln!("kutha-migrate: not a directory: {}", dir.display());
        return ExitCode::from(2);
    }
    match store::migrate_compat(&dir) {
        Ok(report) => {
            println!("{}", format_report(&dir, &report));
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("kutha-migrate: {e}");
            ExitCode::from(1)
        }
    }
}
