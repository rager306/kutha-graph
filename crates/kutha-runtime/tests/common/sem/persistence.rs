//! Persist layouts, import/export, and segment-tamper fixtures (CE #12).
#![allow(dead_code)]

use super::generator::{MixStyle, persist_style};
use super::shrink::fail_seed;
use kutha_common::{Event, EventId, Op};
use kutha_runtime::{Runtime, store};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

static DIR_SEQ: AtomicU64 = AtomicU64::new(1);
pub struct LimitGuard;

impl Drop for LimitGuard {
    fn drop(&mut self) {
        store::reset_test_segment_limits();
    }
}

pub fn tiny_segment_limits() -> LimitGuard {
    store::set_test_segment_limits(Some(4096), Some(3));
    LimitGuard
}

pub fn persist_dir(label: &str) -> PathBuf {
    let n = DIR_SEQ.fetch_add(1, Ordering::Relaxed);
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .join("p62-sem")
        .join(format!("{label}-{n}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

pub fn rm_dir(dir: &Path) {
    let _ = fs::remove_dir_all(dir);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PersistLayout {
    Legacy,
    Segmented,
    Rollover,
    Migrated,
}

pub fn persist_layout(i: usize) -> PersistLayout {
    match i % 4 {
        0 => PersistLayout::Legacy,
        1 => PersistLayout::Segmented,
        2 => PersistLayout::Rollover,
        _ => PersistLayout::Migrated,
    }
}

/// One persist+open (and optional export_jsonl) case.
#[derive(Clone, Copy, Debug)]
pub struct PersistCase {
    pub style: MixStyle,
    pub layout: PersistLayout,
    /// Run export_jsonl + import + log-order pin (kills middle-event swap).
    pub export: bool,
}

fn layout_exports(layout: PersistLayout) -> bool {
    matches!(layout, PersistLayout::Segmented | PersistLayout::Rollover)
}

/// Two deterministic passes per persist index.
/// Pass 0: `persist_layout(i)` + `persist_style(i)`.
/// Pass 1: `persist_layout(i+2)` and WithHorn on segmented/migrated/rollover.
/// Together, any 16 consecutive indices cover all four layouts and export-imported.
pub fn persist_schedule(i: usize) -> [PersistCase; 2] {
    let lay0 = persist_layout(i);
    let lay1 = persist_layout(i + 2);
    let style1 = match lay1 {
        PersistLayout::Segmented | PersistLayout::Migrated | PersistLayout::Rollover => {
            MixStyle::WithHorn
        }
        PersistLayout::Legacy => persist_style(i),
    };
    [
        PersistCase {
            style: persist_style(i),
            layout: lay0,
            export: layout_exports(lay0),
        },
        PersistCase {
            style: style1,
            layout: lay1,
            export: layout_exports(lay1),
        },
    ]
}
pub fn persist_legacy(rt: &Runtime, dir: &Path) {
    store::ensure_legacy_dir(dir).unwrap();
    store::persist(rt, dir).unwrap();
}

pub fn persist_segmented(rt: &Runtime, dir: &Path) {
    store::persist(rt, dir).unwrap();
    assert!(
        dir.join(store::MANIFEST_REL).exists(),
        "empty dir must persist as segments"
    );
}

pub fn sealed_count(dir: &Path) -> usize {
    let v: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(dir.join(store::MANIFEST_REL)).unwrap()).unwrap();
    v["sealed"].as_array().map(|a| a.len()).unwrap_or(0)
}

pub fn persist_segmented_rollover(rt: &Runtime, dir: &Path) -> usize {
    let _g = tiny_segment_limits();
    persist_segmented(rt, dir);
    let n = sealed_count(dir);
    assert!(
        n >= 2,
        "rollover seam must force several sealed segments; sealed={n}"
    );
    n
}

pub fn persist_migrated(rt: &Runtime, dir: &Path) {
    persist_legacy(rt, dir);
    store::migrate(dir).unwrap();
    assert!(dir.join(store::MANIFEST_REL).exists());
}

pub fn persist_with_layout(rt: &Runtime, dir: &Path, layout: PersistLayout) -> usize {
    match layout {
        PersistLayout::Legacy => {
            persist_legacy(rt, dir);
            0
        }
        PersistLayout::Segmented => {
            persist_segmented(rt, dir);
            sealed_count(dir)
        }
        PersistLayout::Rollover => persist_segmented_rollover(rt, dir),
        PersistLayout::Migrated => {
            persist_migrated(rt, dir);
            sealed_count(dir)
        }
    }
}

/// Persist for shrink predicates: keep the original layout; reject candidates that
/// cannot reach layout preconditions (Rollover needs ≥2 sealed). Reject ≠ oracle fail.
pub fn try_persist_with_layout(rt: &Runtime, dir: &Path, layout: PersistLayout) -> bool {
    match layout {
        PersistLayout::Rollover => {
            let _g = tiny_segment_limits();
            if store::persist(rt, dir).is_err() {
                return false;
            }
            if !dir.join(store::MANIFEST_REL).exists() {
                return false;
            }
            sealed_count(dir) >= 2
        }
        other => {
            persist_with_layout(rt, dir, other);
            true
        }
    }
}

pub fn jsonl_event_ids(path: &Path) -> Vec<EventId> {
    let text = fs::read_to_string(path).unwrap();
    text.lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let e: Event = serde_json::from_str(l).expect("events.jsonl line");
            e.id
        })
        .collect()
}

/// export_jsonl into a fresh jsonl-only dir (no manifest / WAL). Open uses jsonl as SoT.
/// Pins log event-id order so a middle-event swap in `export_jsonl` dies even when
/// the swapped prefix commutes under `runtime_stamps`.
pub fn open_export_imported(src: &Path, dest: &Path) -> Runtime {
    store::export_jsonl(src).unwrap();
    fs::create_dir_all(dest).unwrap();
    fs::copy(src.join("events.jsonl"), dest.join("events.jsonl")).unwrap();
    store::open(dest).unwrap()
}

pub fn expected_persist_event_ids(rt: &Runtime) -> Vec<EventId> {
    let mut out: Vec<EventId> = rt
        .dictionary()
        .strings()
        .iter()
        .map(|name| Event::stable_define(name.clone(), 0).id)
        .collect();
    for e in rt.log().iter() {
        if !matches!(e.op, Op::Define { .. }) {
            out.push(e.id);
        }
    }
    out
}

/// EventIds after `persist`+`open`: Define ops are not retained on the opened log
/// (meanings recover via dict / stable-Define encoding). Non-Define live ids only.
/// Distinct from `expected_persist_event_ids` (export_jsonl Define prefix).
pub fn expected_open_event_ids(rt: &Runtime) -> Vec<EventId> {
    rt.log()
        .iter()
        .filter(|e| !matches!(e.op, Op::Define { .. }))
        .map(|e| e.id)
        .collect()
}

pub fn assert_export_preserves_event_order(src: &Path, rt: &Runtime, seed: u32, case: usize) {
    let ids = jsonl_event_ids(&src.join("events.jsonl"));
    let live = expected_persist_event_ids(rt);
    if ids.len() < 3 {
        fail_seed(
            seed,
            case,
            rt.log().as_slice(),
            "export_jsonl order pin needs ≥3 events so a middle swap is observable",
        );
    }
    if ids != live {
        fail_seed(
            seed,
            case,
            rt.log().as_slice(),
            "export_jsonl must preserve persist-encoded event-id order (middle-event swap dies)",
        );
    }
}
fn is_seal_payload(payload: &[u8]) -> bool {
    serde_json::from_slice::<serde_json::Value>(payload)
        .ok()
        .and_then(|v| v.get("k").and_then(|k| k.as_str()).map(|s| s == "seal"))
        .unwrap_or(false)
}

pub fn event_frame_ranges(bytes: &[u8]) -> Vec<(usize, usize)> {
    let hdr = store::SEGMENT_HEADER_LEN as usize;
    assert!(bytes.len() >= hdr, "segment header");
    assert_eq!(&bytes[0..8], b"KUTHASEG");
    let mut off = hdr;
    let mut out = Vec::new();
    while off + 8 <= bytes.len() {
        let mut len_buf = [0u8; 4];
        len_buf.copy_from_slice(&bytes[off..off + 4]);
        let len = u32::from_le_bytes(len_buf) as usize;
        let end = off + 8 + len;
        if end > bytes.len() {
            break;
        }
        if is_seal_payload(&bytes[off + 8..end]) {
            break;
        }
        out.push((off, end));
        off = end;
    }
    out
}

pub fn sealed_path(dir: &Path, i: usize) -> PathBuf {
    let v: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(dir.join(store::MANIFEST_REL)).unwrap()).unwrap();
    dir.join(
        v["sealed"][i]["name"]
            .as_str()
            .unwrap_or_else(|| panic!("sealed[{i}].name")),
    )
}

/// XOR one payload byte in a middle sealed segment. Open must fail closed: CRC mismatch.
pub fn flip_middle_segment_payload(dir: &Path) -> usize {
    let n = sealed_count(dir);
    assert!(
        n >= 3,
        "need ≥3 sealed segments to flip a middle file; n={n}"
    );
    let mid = n / 2;
    let path = sealed_path(dir, mid);
    let mut bytes = fs::read(&path).unwrap();
    let ranges = event_frame_ranges(&bytes);
    assert!(
        ranges.len() >= 2,
        "middle sealed {} must have a payload frame to flip",
        path.display()
    );
    let payload = ranges[1].0 + 8;
    bytes[payload] ^= 0x01;
    fs::write(&path, &bytes).unwrap();
    mid
}

/// XOR one nibble of the middle sealed entry's manifest `chain` field.
pub fn flip_manifest_chain(dir: &Path) -> usize {
    let path = dir.join(store::MANIFEST_REL);
    let mut v: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
    let n = v["sealed"].as_array().map(|a| a.len()).unwrap_or(0);
    assert!(n >= 3, "need ≥3 sealed entries to flip middle chain; n={n}");
    let mid = n / 2;
    let chain = v["sealed"][mid]["chain"]
        .as_str()
        .expect("sealed.chain")
        .to_string();
    let mut chars: Vec<char> = chain.chars().collect();
    if let Some(last) = chars.last_mut() {
        *last = if *last == '0' { '1' } else { '0' };
    }
    v["sealed"][mid]["chain"] = serde_json::Value::String(chars.into_iter().collect());
    fs::write(&path, serde_json::to_vec(&v).unwrap()).unwrap();
    mid
}

/// Fail-closed property: `open` must return `InvalidData` (message unrestricted).
/// Skip-CRC still fail-closes here (`sealed frame is not an Event`).
pub fn open_must_fail_invalid_data(dir: &Path, seed: u32, case: usize) -> std::io::Error {
    match store::open(dir) {
        Ok(_) => fail_seed(seed, case, &[], "open must fail closed (InvalidData)"),
        Err(e) => {
            assert_eq!(
                e.kind(),
                std::io::ErrorKind::InvalidData,
                "seed={seed} case={case}: fail closed InvalidData, got {e}"
            );
            e
        }
    }
}

/// Named-detector pin (not generic fail-closed). CRC-disabled dies on this needle;
/// the JSON-parse fallback is a different message and fails this assert.
pub fn pin_error_detector(err: &std::io::Error, needle: &str, seed: u32, case: usize) {
    let msg = err.to_string();
    assert!(
        msg.contains(needle),
        "seed={seed} case={case}: detector pin {needle:?}; got {msg}"
    );
}

pub fn open_fails_closed(dir: &Path, needle: &str, seed: u32, case: usize) {
    let err = open_must_fail_invalid_data(dir, seed, case);
    pin_error_detector(&err, needle, seed, case);
}
