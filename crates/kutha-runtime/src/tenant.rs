//! H2 tenant: map process JSONL onto `Op` and admit through the product runtime.
//! Not architecture SoT. Not ADR-050's six kinds — two rows on the FF6 list.

use crate::quantum::{Runtime, RuntimeError};
use kutha_common::Op;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug)]
pub enum TenantError {
    Io(std::io::Error),
    Json(serde_json::Error),
    Runtime(RuntimeError),
}

impl std::fmt::Display for TenantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TenantError::Io(e) => write!(f, "{e}"),
            TenantError::Json(e) => write!(f, "{e}"),
            TenantError::Runtime(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for TenantError {}

impl From<std::io::Error> for TenantError {
    fn from(e: std::io::Error) -> Self {
        TenantError::Io(e)
    }
}

impl From<serde_json::Error> for TenantError {
    fn from(e: serde_json::Error) -> Self {
        TenantError::Json(e)
    }
}

impl From<RuntimeError> for TenantError {
    fn from(e: RuntimeError) -> Self {
        TenantError::Runtime(e)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IngestReport {
    pub status_facts: usize,
    pub observed_facts: usize,
    pub last_status: Option<String>,
    pub last_valid_from: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct HarnessRow {
    op: String,
    subject: String,
    relation: String,
    object: String,
    valid_from: u64,
}

pub fn ingest_harness_jsonl(
    rt: &mut Runtime,
    path: impl AsRef<Path>,
) -> Result<IngestReport, TenantError> {
    let text = std::fs::read_to_string(path.as_ref())?;
    ingest_harness_jsonl_str(rt, &text)
}

pub fn ingest_harness_jsonl_str(rt: &mut Runtime, text: &str) -> Result<IngestReport, TenantError> {
    let mut statuses: Vec<(u64, String)> = Vec::new();
    let mut cargo: Vec<(u64, String)> = Vec::new();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        let row: HarnessRow = serde_json::from_str(line)?;
        if row.op != "assert" {
            continue;
        }
        if row.subject == "harness.run" && row.relation == "status" {
            statuses.push((row.valid_from, row.object));
        } else if row.subject == "harness.observe" && row.relation == "cargo" {
            cargo.push((row.valid_from, row.object));
        }
    }

    let mut report = IngestReport::default();
    emit_chained(rt, "harness.run", "runStatus", &statuses)?;
    report.status_facts = statuses.len();
    if let Some((vf, obj)) = statuses.last() {
        report.last_valid_from = Some(*vf);
        report.last_status = Some(obj.clone());
    }
    emit_chained(rt, "harness.observe", "observed", &cargo)?;
    report.observed_facts = cargo.len();
    Ok(report)
}

fn emit_chained(
    rt: &mut Runtime,
    subject: &str,
    relation: &str,
    rows: &[(u64, String)],
) -> Result<(), TenantError> {
    if rows.is_empty() {
        return Ok(());
    }
    let mut cursor = 0u64;
    let mut timed: Vec<(u64, &str)> = Vec::with_capacity(rows.len());
    for (vf, obj) in rows {
        let start = (*vf).max(cursor);
        timed.push((start, obj.as_str()));
        cursor = start.saturating_add(1);
    }
    let subj = rt.intern(subject);
    let rel = rt.intern(relation);
    for i in 0..timed.len() {
        let object = rt.intern(timed[i].1);
        let valid_to = timed.get(i + 1).map(|row| row.0);
        rt.emit(Op::Assert {
            subject: subj,
            relation: rel,
            object,
            valid_from: timed[i].0,
            valid_to,
        })?;
    }
    Ok(())
}
