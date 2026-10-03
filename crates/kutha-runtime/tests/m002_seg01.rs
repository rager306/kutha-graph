//! M002 S06 / SEG-01 oracles (Phase 54). 54-01: format/open. 54-02: persist +
//! Persister. 54-03: tamper (real writer), migrate, export_jsonl.

use kutha_common::{Event, EventId, Op};
use kutha_runtime::{store, Runtime};
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

const T_OLD: u64 = 2015;
const T_NEW: u64 = 2021;
const VF_FIRST: u64 = 2010;
const VF_SECOND: u64 = 2020;

struct LimitGuard;

impl Drop for LimitGuard {
    fn drop(&mut self) {
        store::reset_test_segment_limits();
    }
}

fn tiny_limits() -> LimitGuard {
    store::set_test_segment_limits(Some(4096), Some(3));
    LimitGuard
}

fn persist_dir(label: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .join(format!("p54-store-{label}-{nanos}"));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn seed_runtime() -> Runtime {
    let mut rt = Runtime::default();
    let article = rt.intern("art-12-speed");
    let in_force = rt.intern("inForceAs");
    let limit_50 = rt.intern("50-kmh");
    rt.emit(Op::Assert {
        subject: article,
        relation: in_force,
        object: limit_50,
        valid_from: VF_FIRST,
        valid_to: Some(VF_SECOND),
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit_tt_anchor(T_OLD).unwrap();
    for i in 0..8 {
        let _ = rt.intern(&format!("p54-pad-{i}"));
    }
    rt
}

fn log_ids(rt: &Runtime) -> Vec<EventId> {
    rt.log()
        .iter()
        .filter(|e| !matches!(e.op, Op::Define { .. }))
        .map(|e| e.id)
        .collect()
}

fn read_jsonl_events(path: &Path) -> Vec<Event> {
    let text = fs::read_to_string(path).unwrap();
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str::<Event>(l).expect("jsonl Event"))
        .collect()
}

fn persist_encoded(rt: &Runtime, dir: &Path) -> Vec<Event> {
    store::persist(rt, dir).unwrap();
    store::encoded_events(rt)
}

fn layout(n: usize) -> (Vec<usize>, usize) {
    assert!(
        n >= 10,
        "encoded log too small for sealed+active split: {n}"
    );
    (vec![4, 4], n - 8)
}

fn write_segments(dir: &Path, events: &[Event]) {
    let (sealed, active) = layout(events.len());
    store::write_test_segmented(dir, events, &sealed, active).unwrap();
}

fn drop_legacy_log(dir: &Path) {
    let _ = fs::remove_file(dir.join("events.jsonl"));
    let _ = fs::remove_file(dir.join("events.wal"));
}

fn smash_legacy_log(dir: &Path) {
    fs::write(dir.join("events.jsonl"), "{\"not\":\"the-segment-log\"}\n").unwrap();
    let _ = fs::remove_file(dir.join("events.wal"));
}

fn manifest_path(dir: &Path) -> PathBuf {
    dir.join(store::MANIFEST_REL)
}

fn first_sealed_path(dir: &Path) -> PathBuf {
    sealed_path(dir, 0)
}

fn sealed_path(dir: &Path, i: usize) -> PathBuf {
    let v: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(manifest_path(dir)).unwrap()).unwrap();
    dir.join(
        v["sealed"][i]["name"]
            .as_str()
            .unwrap_or_else(|| panic!("sealed[{i}].name")),
    )
}

fn sealed_len(dir: &Path) -> usize {
    let v: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(manifest_path(dir)).unwrap()).unwrap();
    v["sealed"].as_array().expect("sealed array").len()
}

fn trailer_json(bytes: &[u8]) -> serde_json::Value {
    let ranges = event_frame_ranges(bytes);
    let start = ranges.last().expect("event frames").1;
    let tlen = u32::from_le_bytes(bytes[start..start + 4].try_into().unwrap()) as usize;
    serde_json::from_slice(&bytes[start + 8..start + 8 + tlen]).expect("seal trailer JSON")
}

fn retarget_manifest_sealed_from_file(dir: &Path, i: usize, path: &Path) {
    let bytes = fs::read(path).unwrap();
    let t = trailer_json(&bytes);
    let mut man: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(manifest_path(dir)).unwrap()).unwrap();
    man["sealed"][i]["first"] = t["first"].clone();
    man["sealed"][i]["last"] = t["last"].clone();
    man["sealed"][i]["count"] = t["n"].clone();
    man["sealed"][i]["bytes"] = serde_json::json!(bytes.len() as u64);
    man["sealed"][i]["chain"] = t["chain"].clone();
    fs::write(manifest_path(dir), serde_json::to_vec(&man).unwrap()).unwrap();
}

fn active_path(dir: &Path) -> PathBuf {
    let v: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(manifest_path(dir)).unwrap()).unwrap();
    dir.join(v["active"].as_str().expect("active"))
}

fn is_seal_payload(payload: &[u8]) -> bool {
    serde_json::from_slice::<serde_json::Value>(payload)
        .ok()
        .and_then(|v| v.get("k").and_then(|k| k.as_str()).map(|s| s == "seal"))
        .unwrap_or(false)
}

fn event_frame_ranges(bytes: &[u8]) -> Vec<(usize, usize)> {
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

fn assert_invalid_data(dir: &Path, needle: &str) {
    match store::open(dir) {
        Ok(_) => panic!("expected InvalidData containing {needle:?}"),
        Err(e) => {
            assert_eq!(
                e.kind(),
                ErrorKind::InvalidData,
                "expected InvalidData, got {e:?}"
            );
            let msg = e.to_string();
            assert!(
                msg.contains(needle),
                "error must mention {needle:?}; got {msg}"
            );
        }
    }
}

fn expect_round_trip(dir: &Path, rt: &Runtime) {
    let opened = store::open(dir).expect("segmented dir must open");
    assert_eq!(log_ids(&opened), log_ids(rt));
    assert_eq!(opened.fold().fingerprint(), rt.fold().fingerprint());
    opened.replay_check().unwrap();
}

fn overlay_segmented(rt: &Runtime, dir: &Path) {
    let events = persist_encoded(rt, dir);
    write_segments(dir, &events);
    drop_legacy_log(dir);
}

fn assert_no_legacy_log(dir: &Path) {
    assert!(
        !dir.join("events.jsonl").exists(),
        "segmented writer must not create events.jsonl"
    );
    assert!(
        !dir.join("events.wal").exists(),
        "segmented writer must not create events.wal"
    );
}

fn persist_expect_segmented(rt: &Runtime, dir: &Path) {
    store::persist(rt, dir).unwrap();
    assert!(
        manifest_path(dir).exists(),
        "empty dir (no manifest, no events.jsonl, no events.wal) must persist as segments"
    );
    assert_no_legacy_log(dir);
}

fn persist_legacy(rt: &Runtime, dir: &Path) {
    store::ensure_legacy_dir(dir).unwrap();
    store::persist(rt, dir).unwrap();
}

fn snap_log_offset(dir: &Path) -> usize {
    let snap: kutha_runtime::Snapshot =
        serde_json::from_reader(fs::File::open(dir.join("snapshot.json")).unwrap()).unwrap();
    snap.log_offset
}

fn emit_one_more(rt: &mut Runtime, wall: u64) {
    rt.emit_tt_anchor(wall).unwrap();
}

fn emit_gap_assert(rt: &mut Runtime, subject: &str, object: &str) {
    let s = rt.intern(subject);
    let o = rt.intern(object);
    let r = rt.intern("inForceAs");
    rt.emit(Op::Assert {
        subject: s,
        relation: r,
        object: o,
        valid_from: VF_FIRST,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
}

fn emit_product_assert(rt: &mut Runtime, subject: &str, object: &str) {
    emit_gap_assert(rt, subject, object);
}

fn assert_dict_term_parity(src: &Runtime, opened: &Runtime) {
    assert_eq!(
        opened.dictionary().strings(),
        src.dictionary().strings(),
        "open must restore interned strings in intern order"
    );
    for s in src.dictionary().strings() {
        assert_eq!(
            opened.dictionary().id(s),
            src.dictionary().id(s),
            "TermId for {s} must match the source runtime"
        );
    }
}

fn crash_omit_last(p: &mut store::Persister, rt: &Runtime) {
    let n = store::pending_append_len(p, rt).unwrap();
    assert!(
        n >= 2,
        "pending batch must include payload and QuantumOutcome; n={n}"
    );
    store::append_with_event_failpoint(p, rt, n - 1)
        .expect_err("event failpoint must inject after N-1 events");
}

fn last_event_frame_range(bytes: &[u8]) -> (usize, usize) {
    let ranges = event_frame_ranges(bytes);
    *ranges.last().expect("active must have an event frame")
}

fn replace_last_event_frame(bytes: &[u8], event: &Event) -> Vec<u8> {
    let (start, end) = last_event_frame_range(bytes);
    let frame = store::event_frame_bytes(event).unwrap();
    assert_eq!(
        frame.len(),
        end - start,
        "replacement frame must keep last-record length"
    );
    let mut out = bytes.to_vec();
    out[start..end].copy_from_slice(&frame);
    out
}

fn last_event_from_segment(bytes: &[u8]) -> Event {
    let (start, end) = last_event_frame_range(bytes);
    serde_json::from_slice(&bytes[start + 8..end]).expect("last Event")
}

fn assert_guard_writes_nothing(p: &mut store::Persister, rt: &Runtime, path: &Path, before: &[u8]) {
    match p.append(rt) {
        Ok(_) => panic!("tail-guard mismatch must return InvalidData"),
        Err(e) => assert_eq!(e.kind(), ErrorKind::InvalidData),
    }
    assert_eq!(
        fs::read(path).unwrap(),
        before,
        "tail-guard mismatch must write nothing to the active segment"
    );
    assert_no_legacy_log(path.parent().expect("active parent"));
}

#[test]
fn round_trip_tiny_segmented_dir() {
    let rt = seed_runtime();
    let dir = persist_dir("round");
    let events = persist_encoded(&rt, &dir);
    write_segments(&dir, &events);
    drop_legacy_log(&dir);

    expect_round_trip(&dir, &rt);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn missing_sealed_segment_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("miss-sealed");
    let events = persist_encoded(&rt, &dir);
    write_segments(&dir, &events);
    let path = first_sealed_path(&dir);
    fs::remove_file(&path).unwrap();

    assert_invalid_data(&dir, "missing sealed segment");

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn missing_active_segment_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("miss-active");
    let events = persist_encoded(&rt, &dir);
    write_segments(&dir, &events);
    fs::remove_file(active_path(&dir)).unwrap();

    assert_invalid_data(&dir, "missing active segment");

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn sealed_without_trailer_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("no-trailer");
    let events = persist_encoded(&rt, &dir);
    write_segments(&dir, &events);
    let path = first_sealed_path(&dir);
    let bytes = fs::read(&path).unwrap();
    let ranges = event_frame_ranges(&bytes);
    assert!(
        !ranges.is_empty(),
        "sealed file must have event frames before the trailer"
    );
    fs::write(&path, &bytes[..ranges[ranges.len() - 1].1]).unwrap();

    assert_invalid_data(&dir, "truncated sealed");

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn truncated_sealed_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("trunc-sealed");
    let events = persist_encoded(&rt, &dir);
    write_segments(&dir, &events);
    let path = first_sealed_path(&dir);
    let mut bytes = fs::read(&path).unwrap();
    assert!(bytes.len() > 20, "sealed file must be larger than a header");
    bytes.truncate(bytes.len().saturating_sub(12));
    fs::write(&path, &bytes).unwrap();

    assert_invalid_data(&dir, "truncated sealed");

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn flipped_byte_in_sealed_frame_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("flip");
    let events = persist_encoded(&rt, &dir);
    write_segments(&dir, &events);
    let path = first_sealed_path(&dir);
    let mut bytes = fs::read(&path).unwrap();
    let ranges = event_frame_ranges(&bytes);
    assert!(ranges.len() >= 2, "need a payload byte to flip");
    let payload = ranges[1].0 + 8;
    bytes[payload] ^= 0x01;
    fs::write(&path, &bytes).unwrap();

    // CRC is checked before the hash chain (scan_segment documented order).
    assert_invalid_data(&dir, "CRC mismatch");

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn swapped_adjacent_sealed_frames_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("swap");
    let events = persist_encoded(&rt, &dir);
    write_segments(&dir, &events);
    let path = first_sealed_path(&dir);
    let bytes = fs::read(&path).unwrap();
    let ranges = event_frame_ranges(&bytes);
    assert!(
        ranges.len() >= 4,
        "swap two middle frames so first/last still match"
    );
    let mut rebuilt = bytes[..store::SEGMENT_HEADER_LEN as usize].to_vec();
    let mut order: Vec<usize> = (0..ranges.len()).collect();
    order.swap(1, 2);
    for i in order {
        rebuilt.extend_from_slice(&bytes[ranges[i].0..ranges[i].1]);
    }
    rebuilt.extend_from_slice(&bytes[ranges[ranges.len() - 1].1..]);
    fs::write(&path, &rebuilt).unwrap();

    assert_invalid_data(&dir, "chain");

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn reordered_or_deleted_sealed_frames_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("reorder-delete");
    let events = persist_encoded(&rt, &dir);
    write_segments(&dir, &events);

    let path = first_sealed_path(&dir);
    let mut bytes = fs::read(&path).unwrap();
    let ranges = event_frame_ranges(&bytes);
    assert!(ranges.len() >= 3);
    let drop_at = ranges[1];
    bytes.drain(drop_at.0..drop_at.1);
    fs::write(&path, &bytes).unwrap();
    // Count is checked before bytes / hash chain (verify_sealed documented order).
    assert_invalid_data(&dir, "count mismatch");

    let events = persist_encoded(&rt, &dir);
    write_segments(&dir, &events);
    let mut man: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(manifest_path(&dir)).unwrap()).unwrap();
    let a = man["sealed"][0].clone();
    let b = man["sealed"][1].clone();
    man["sealed"][0] = b;
    man["sealed"][1] = a;
    fs::write(manifest_path(&dir), serde_json::to_vec(&man).unwrap()).unwrap();
    // Contiguity is checked before chain-link (load_events documented order).
    assert_invalid_data(&dir, "not contiguous");

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn manifest_file_disagreement_is_invalid_data() {
    let fields: [&str; 5] = ["first", "last", "count", "bytes", "chain"];
    for field in fields {
        let rt = seed_runtime();
        let dir = persist_dir(&format!("disagree-{field}"));
        let events = persist_encoded(&rt, &dir);
        write_segments(&dir, &events);
        let mut man: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(manifest_path(&dir)).unwrap()).unwrap();
        match field {
            "first" | "last" => {
                man["sealed"][0][field] = serde_json::json!("00000000-0000-0000-0000-000000000000");
            }
            "count" => man["sealed"][0][field] = serde_json::json!(1),
            "bytes" => man["sealed"][0][field] = serde_json::json!(12),
            "chain" => {
                man["sealed"][0][field] = serde_json::json!(
                    "0000000000000000000000000000000000000000000000000000000000000000"
                );
            }
            _ => unreachable!(),
        }
        fs::write(manifest_path(&dir), serde_json::to_vec(&man).unwrap()).unwrap();
        assert_invalid_data(&dir, "manifest disagrees");
        let _ = fs::remove_dir_all(&dir);
    }
}

#[test]
fn torn_active_tail_dropped() {
    let rt = seed_runtime();
    let dir = persist_dir("torn-active");
    let events = persist_encoded(&rt, &dir);
    write_segments(&dir, &events);
    drop_legacy_log(&dir);
    let path = active_path(&dir);
    let mut bytes = fs::read(&path).unwrap();
    let ranges = event_frame_ranges(&bytes);
    assert!(
        !ranges.is_empty(),
        "active must have at least one event frame so a torn tail is extra"
    );
    bytes.extend_from_slice(&[8, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3]);
    fs::write(&path, &bytes).unwrap();

    expect_round_trip(&dir, &rt);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn seal_trailer_on_named_active_ends_stream() {
    let rt = seed_runtime();
    let dir = persist_dir("seal-active");
    let events = persist_encoded(&rt, &dir);
    write_segments(&dir, &events);
    drop_legacy_log(&dir);
    store::rewrite_segment_with_seal_trailer(&active_path(&dir)).unwrap();

    expect_round_trip(&dir, &rt);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn unnamed_segment_leftover_ignored() {
    let rt = seed_runtime();
    let dir = persist_dir("leftover");
    let events = persist_encoded(&rt, &dir);
    write_segments(&dir, &events);
    drop_legacy_log(&dir);
    fs::write(dir.join("seg-99999999.log"), b"not a segment leftover").unwrap();

    expect_round_trip(&dir, &rt);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn manifest_wins_over_disagreeing_jsonl() {
    let rt = seed_runtime();
    let dir = persist_dir("manifest-wins");
    let events = persist_encoded(&rt, &dir);
    write_segments(&dir, &events);
    smash_legacy_log(&dir);

    expect_round_trip(&dir, &rt);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn legacy_dir_still_opens_and_appends_as_before() {
    let mut rt = seed_runtime();
    let dir = persist_dir("legacy");
    store::ensure_legacy_dir(&dir).unwrap();
    store::persist(&rt, &dir).unwrap();
    assert!(
        !manifest_path(&dir).exists(),
        "legacy persist must not write a manifest"
    );
    let opened = store::open(&dir).expect("legacy dir still opens");
    assert_eq!(log_ids(&opened), log_ids(&rt));
    opened.replay_check().unwrap();

    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    rt.emit_tt_anchor(T_OLD + 1).unwrap();
    p.append(&rt).unwrap();
    let again = store::open(&dir).unwrap();
    assert_eq!(log_ids(&again), log_ids(&rt));

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn persist_empty_dir_writes_segments_not_jsonl() {
    let rt = seed_runtime();
    let dir = persist_dir("empty-seg");
    persist_expect_segmented(&rt, &dir);
    expect_round_trip(&dir, &rt);
    assert!(
        store::SEGMENT_MAX_BYTES == 64 * 1024 * 1024 && store::SEGMENT_MAX_EVENTS == 65536,
        "rollover pub consts"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn persist_legacy_dir_keeps_jsonl_and_wal() {
    let rt = seed_runtime();
    let dir = persist_dir("keep-legacy");
    store::ensure_legacy_dir(&dir).unwrap();
    store::persist(&rt, &dir).unwrap();
    assert!(
        !manifest_path(&dir).exists(),
        "legacy dir must not grow a manifest"
    );
    assert!(dir.join("events.jsonl").exists());
    assert!(dir.join("events.wal").exists());
    let from_jsonl = read_jsonl_events(&dir.join("events.jsonl"));
    assert_eq!(from_jsonl.len(), store::encoded_events(&rt).len());
    let opened = store::open(&dir).unwrap();
    assert_eq!(log_ids(&opened), log_ids(&rt));

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn persist_segmented_dir_does_not_create_jsonl() {
    let rt = seed_runtime();
    let dir = persist_dir("rewr-seg");
    overlay_segmented(&rt, &dir);
    store::persist(&rt, &dir).unwrap();
    assert!(manifest_path(&dir).exists());
    assert_no_legacy_log(&dir);
    expect_round_trip(&dir, &rt);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn prefix_sealed_bytes_unchanged_after_append() {
    let mut rt = seed_runtime();
    let dir = persist_dir("prefix-append");
    overlay_segmented(&rt, &dir);
    let sealed_path = first_sealed_path(&dir);
    let sealed_before = fs::read(&sealed_path).unwrap();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_one_more(&mut rt, T_NEW);
    p.append(&rt).unwrap();
    assert_eq!(
        fs::read(&sealed_path).unwrap(),
        sealed_before,
        "append must not rewrite sealed bytes"
    );
    assert_no_legacy_log(&dir);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn prefix_sealed_bytes_unchanged_after_rollover() {
    let _limits = tiny_limits();
    let mut rt = seed_runtime();
    let dir = persist_dir("prefix-roll");
    overlay_segmented(&rt, &dir);
    let sealed_path = first_sealed_path(&dir);
    let sealed_before = fs::read(&sealed_path).unwrap();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_one_more(&mut rt, T_NEW);
    p.append(&rt).unwrap();
    assert_eq!(
        fs::read(&sealed_path).unwrap(),
        sealed_before,
        "rollover must not rewrite earlier sealed bytes"
    );
    expect_round_trip(&dir, &rt);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn one_log_sync_per_quantum_on_segmented() {
    let mut rt = seed_runtime();
    let dir = persist_dir("one-sync");
    overlay_segmented(&rt, &dir);
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_one_more(&mut rt, T_NEW);
    let before = store::persist_log_sync_count();
    p.append(&rt).unwrap();
    let delta = store::persist_log_sync_count().saturating_sub(before);
    assert_eq!(
        delta, 1,
        "non-rollover segmented append must sync_data exactly once; count={delta}"
    );
    assert_no_legacy_log(&dir);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn append_creates_no_jsonl() {
    let mut rt = seed_runtime();
    let dir = persist_dir("no-jsonl");
    overlay_segmented(&rt, &dir);
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_one_more(&mut rt, T_NEW);
    p.append(&rt).unwrap();
    assert_no_legacy_log(&dir);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn append_nothing_new_does_no_io_on_segmented() {
    let rt = seed_runtime();
    let dir = persist_dir("noop-seg");
    overlay_segmented(&rt, &dir);
    let active = active_path(&dir);
    let before = fs::read(&active).unwrap();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    let sync_before = store::persist_log_sync_count();
    p.append(&rt).unwrap();
    assert_eq!(
        store::persist_log_sync_count().saturating_sub(sync_before),
        0
    );
    assert_eq!(fs::read(&active).unwrap(), before);
    assert_no_legacy_log(&dir);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn tail_guard_length_mismatch_writes_nothing() {
    let mut rt = seed_runtime();
    let dir = persist_dir("guard-len");
    overlay_segmented(&rt, &dir);
    let active = active_path(&dir);
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    let mut bytes = fs::read(&active).unwrap();
    bytes.extend_from_slice(&[0xFF, 0x00, 0xAA, 0xBB]);
    fs::write(&active, &bytes).unwrap();
    emit_one_more(&mut rt, T_NEW);
    assert_guard_writes_nothing(&mut p, &rt, &active, &bytes);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn tail_guard_last_id_mismatch_writes_nothing() {
    let mut rt = seed_runtime();
    let dir = persist_dir("guard-id");
    overlay_segmented(&rt, &dir);
    let active = active_path(&dir);
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    let bytes = fs::read(&active).unwrap();
    let mut last = last_event_from_segment(&bytes);
    last.id = EventId::from_u128(last.id.as_u128().wrapping_add(1));
    let tampered = replace_last_event_frame(&bytes, &last);
    fs::write(&active, &tampered).unwrap();
    emit_one_more(&mut rt, T_NEW);
    assert_guard_writes_nothing(&mut p, &rt, &active, &tampered);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn tail_guard_same_length_different_last_record_writes_nothing() {
    let mut rt = seed_runtime();
    let dir = persist_dir("guard-swap");
    overlay_segmented(&rt, &dir);
    let active = active_path(&dir);
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    let bytes = fs::read(&active).unwrap();
    let ranges = event_frame_ranges(&bytes);
    assert!(ranges.len() >= 2, "need a prior frame to swap");
    let last_len = ranges[ranges.len() - 1].1 - ranges[ranges.len() - 1].0;
    let prior = ranges
        .iter()
        .rev()
        .skip(1)
        .find(|(s, e)| e - s == last_len)
        .copied();
    let tampered = if let Some((s, e)) = prior {
        let mut out = bytes.clone();
        let last = last_event_frame_range(&bytes);
        let frame = bytes[s..e].to_vec();
        out[last.0..last.1].copy_from_slice(&frame);
        out
    } else {
        let mut last = last_event_from_segment(&bytes);
        last.id = EventId::from_u128(last.id.as_u128().wrapping_add(2));
        replace_last_event_frame(&bytes, &last)
    };
    assert_eq!(tampered.len(), bytes.len());
    fs::write(&active, &tampered).unwrap();
    emit_one_more(&mut rt, T_NEW);
    assert_guard_writes_nothing(&mut p, &rt, &active, &tampered);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn rollover_crash_matrix_reopens_to_prefix() {
    let _limits = tiny_limits();
    let steps = [
        store::AppendStep::RolloverTrailer,
        store::AppendStep::RolloverNewHeader,
        store::AppendStep::RolloverManifest,
    ];
    for step in steps {
        let mut rt = seed_runtime();
        let dir = persist_dir(&format!("roll-crash-{step:?}"));
        overlay_segmented(&rt, &dir);
        let committed = log_ids(&rt);
        let active_before = active_path(&dir)
            .file_name()
            .expect("active")
            .to_string_lossy()
            .into_owned();
        let mut p = store::Persister::attach(&rt, &dir).unwrap();
        emit_one_more(&mut rt, T_NEW);
        store::append_with_failpoint(&mut p, &rt, step)
            .expect_err("rollover failpoint must inject");
        match store::open(&dir) {
            Ok(opened) => {
                assert_eq!(
                    log_ids(&opened),
                    committed,
                    "crash at {step:?} must reopen to the committed prefix"
                );
                opened.replay_check().unwrap();
            }
            Err(e) => assert_eq!(
                e.kind(),
                ErrorKind::InvalidData,
                "crash at {step:?} may fail closed; {e}"
            ),
        }
        if step == store::AppendStep::RolloverNewHeader {
            let man: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(manifest_path(&dir)).unwrap()).unwrap();
            assert_eq!(
                man["active"].as_str(),
                Some(active_before.as_str()),
                "manifest must still name the old active after RolloverNewHeader"
            );
        }
        let _ = fs::remove_dir_all(&dir);
    }
}

#[test]
fn rollover_manifest_flip_fsyncs_dir() {
    let _limits = tiny_limits();
    let mut rt = seed_runtime();
    let dir = persist_dir("roll-fsync");
    overlay_segmented(&rt, &dir);
    let old_active = active_path(&dir);
    let old_name = old_active
        .file_name()
        .expect("active name")
        .to_string_lossy()
        .into_owned();
    let old_len = fs::metadata(&old_active).unwrap().len();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_one_more(&mut rt, T_NEW);
    let before = store::persist_dir_fsync_count();
    p.append(&rt).unwrap();
    let delta = store::persist_dir_fsync_count().saturating_sub(before);
    assert!(delta >= 1, "manifest flip must dir-fsync; count={delta}");
    let newly = dir.join(&old_name);
    let sealed = fs::read(&newly).unwrap();
    assert!(
        sealed.len() as u64 > old_len,
        "rollover must append a seal trailer without rewriting the prefix"
    );
    let ranges = event_frame_ranges(&sealed);
    let trailer_off = ranges
        .last()
        .map(|r| r.1)
        .unwrap_or(store::SEGMENT_HEADER_LEN as usize);
    assert!(
        trailer_off + 8 < sealed.len() && is_seal_payload(&sealed[trailer_off + 8..]),
        "rollover must write a seal trailer on the former active"
    );
    assert_ne!(
        active_path(&dir)
            .file_name()
            .map(|n| n.to_string_lossy().into_owned()),
        Some(old_name),
        "manifest active must advance"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn checkpoint_dir_fsync_count_exact_on_segmented() {
    let mut rt = seed_runtime();
    let dir = persist_dir("ckpt-fsync");
    overlay_segmented(&rt, &dir);
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_one_more(&mut rt, T_NEW);
    p.append(&rt).unwrap();
    let before = store::persist_dir_fsync_count();
    p.checkpoint(&rt).unwrap();
    let delta = store::persist_dir_fsync_count().saturating_sub(before);
    assert_eq!(
        delta, 5,
        "checkpoint dir fsync: snapshot+terms+outcomes+justifications+final; count={delta}"
    );
    assert_no_legacy_log(&dir);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn open_for_append_heals_and_appends_on_segmented() {
    let mut rt = seed_runtime();
    let dir = persist_dir("heal-seg");
    overlay_segmented(&rt, &dir);
    let committed_fp = rt.fold().fingerprint();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_product_assert(&mut rt, "p54-heal-article", "p54-heal-80");
    crash_omit_last(&mut p, &rt);

    let (mut healed, mut p) = store::open_for_append(&dir)
        .expect("open_for_append must persist the committed prefix then attach");
    assert_eq!(healed.fold().fingerprint(), committed_fp);
    emit_product_assert(&mut healed, "p54-heal2-article", "p54-heal2-90");
    p.append(&healed).unwrap();
    let again = store::open(&dir).unwrap();
    assert_eq!(again.fold().fingerprint(), healed.fold().fingerprint());
    again.replay_check().unwrap();
    assert_no_legacy_log(&dir);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn stale_leases_between_checkpoints_on_segmented() {
    let mut rt = seed_runtime();
    let dir = persist_dir("stale-seg");
    overlay_segmented(&rt, &dir);
    let offset_a = snap_log_offset(&dir);
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_one_more(&mut rt, T_NEW);
    p.append(&rt).unwrap();
    assert_eq!(
        snap_log_offset(&dir),
        offset_a,
        "small append must not rewrite snapshot.json"
    );
    let opened = store::open(&dir).expect("stale snapshot + log ahead");
    assert_eq!(log_ids(&opened), log_ids(&rt));
    opened.replay_check().unwrap();

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn interned_terms_between_appends_survive_on_segmented() {
    let mut rt = seed_runtime();
    let dir = persist_dir("intern-seg");
    overlay_segmented(&rt, &dir);
    let offset_a = snap_log_offset(&dir);
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_one_more(&mut rt, T_NEW);
    p.append(&rt).unwrap();
    emit_gap_assert(&mut rt, "p54-gap-article", "p54-gap-80");
    p.append(&rt).unwrap();
    assert_eq!(
        snap_log_offset(&dir),
        offset_a,
        "GAP A: intern-append must not checkpoint"
    );
    let opened = store::open(&dir).expect("open after intern-append without checkpoint");
    assert_dict_term_parity(&rt, &opened);
    assert_eq!(opened.fold().fingerprint(), rt.fold().fingerprint());
    assert_eq!(opened.as_of(VF_FIRST), rt.as_of(VF_FIRST));
    opened.replay_check().unwrap();

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn second_and_third_appends_keep_earlier_bytes_as_prefix() {
    let mut rt = seed_runtime();
    let dir = persist_dir("prefix-3");
    overlay_segmented(&rt, &dir);
    let sealed0 = fs::read(first_sealed_path(&dir)).unwrap();
    let active = active_path(&dir);
    let a0 = fs::read(&active).unwrap();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_one_more(&mut rt, T_NEW);
    p.append(&rt).unwrap();
    let a1 = fs::read(&active).unwrap();
    assert!(a1.starts_with(&a0) && a1.len() > a0.len());
    emit_one_more(&mut rt, T_NEW + 1);
    p.append(&rt).unwrap();
    let a2 = fs::read(&active).unwrap();
    assert!(a2.starts_with(&a1) && a2.len() > a1.len());
    p.append(&rt).unwrap();
    assert_eq!(fs::read(&active).unwrap(), a2);
    emit_one_more(&mut rt, T_NEW + 5);
    p.append(&rt).unwrap();
    let a3 = fs::read(&active).unwrap();
    assert!(a3.starts_with(&a2) && a3.len() > a2.len());
    assert_eq!(fs::read(first_sealed_path(&dir)).unwrap(), sealed0);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn segmented_replay_parity_with_legacy_full_persist() {
    let rt = seed_runtime();
    let dir_seg = persist_dir("parity-seg");
    persist_expect_segmented(&rt, &dir_seg);
    let opened_s = store::open(&dir_seg).unwrap();

    let dir_leg = persist_dir("parity-leg");
    store::ensure_legacy_dir(&dir_leg).unwrap();
    store::persist(&rt, &dir_leg).unwrap();
    let opened_l = store::open(&dir_leg).unwrap();

    assert_eq!(log_ids(&opened_s), log_ids(&opened_l));
    assert_dict_term_parity(&opened_l, &opened_s);
    assert_eq!(opened_s.fold().fingerprint(), opened_l.fold().fingerprint());
    assert_eq!(opened_s.as_of(VF_FIRST), opened_l.as_of(VF_FIRST));
    opened_s.replay_check().unwrap();
    opened_l.replay_check().unwrap();

    let _ = fs::remove_dir_all(&dir_seg);
    let _ = fs::remove_dir_all(&dir_leg);
}

fn sealed_writer_limits() -> LimitGuard {
    store::set_test_segment_limits(Some(10_000_000), Some(8));
    LimitGuard
}

fn persist_with_sealed(rt: &Runtime, dir: &Path) {
    let _g = sealed_writer_limits();
    persist_expect_segmented(rt, dir);
    let n = event_frame_ranges(&fs::read(first_sealed_path(dir)).unwrap()).len();
    assert!(
        n >= 4,
        "writer must seal ≥4 frames for swap/delete oracles; got {n}"
    );
}

fn copy_store(src: &Path, dst: &Path) {
    let _ = fs::remove_dir_all(dst);
    fs::create_dir_all(dst).unwrap();
    for ent in fs::read_dir(src).unwrap() {
        let ent = ent.unwrap();
        fs::copy(ent.path(), dst.join(ent.file_name())).unwrap();
    }
}

fn xor_at(path: &Path, off: usize) {
    let mut bytes = fs::read(path).unwrap();
    bytes[off] ^= 0x01;
    fs::write(path, &bytes).unwrap();
}

fn crc32_ieee(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &b in data {
        crc ^= u32::from(b);
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ 0xEDB8_8320
            } else {
                crc >> 1
            };
        }
    }
    !crc
}

fn rewrite_trailer_bytes_field(path: &Path) {
    let mut bytes = fs::read(path).unwrap();
    let ranges = event_frame_ranges(&bytes);
    let trailer_start = ranges[ranges.len() - 1].1;
    let tlen =
        u32::from_le_bytes(bytes[trailer_start..trailer_start + 4].try_into().unwrap()) as usize;
    let payload_off = trailer_start + 8;
    let payload = bytes[payload_off..payload_off + tlen].to_vec();
    let s = std::str::from_utf8(&payload).unwrap();
    let key = "\"bytes\":";
    let i = s.find(key).expect("trailer JSON has bytes") + key.len();
    let mut j = i;
    while j < s.len() && s.as_bytes()[j].is_ascii_digit() {
        j += 1;
    }
    assert!(j > i, "bytes field must be a JSON number");
    let mut new_payload = payload;
    let last = j - 1;
    new_payload[last] = if new_payload[last] == b'0' {
        b'1'
    } else {
        b'0'
    };
    let crc = crc32_ieee(&new_payload);
    bytes[trailer_start + 4..trailer_start + 8].copy_from_slice(&crc.to_le_bytes());
    bytes[payload_off..payload_off + tlen].copy_from_slice(&new_payload);
    fs::write(path, &bytes).unwrap();
}

#[test]
fn writer_flipped_sealed_frame_payload_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("w-flip-pay");
    persist_with_sealed(&rt, &dir);
    let path = first_sealed_path(&dir);
    let ranges = event_frame_ranges(&fs::read(&path).unwrap());
    xor_at(&path, ranges[1].0 + 8);
    assert_invalid_data(&dir, "CRC mismatch");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn writer_flipped_sealed_frame_length_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("w-flip-len");
    persist_with_sealed(&rt, &dir);
    let path = first_sealed_path(&dir);
    let ranges = event_frame_ranges(&fs::read(&path).unwrap());
    xor_at(&path, ranges[1].0);
    // A one-bit length flip still yields a complete frame; CRC is first (before chain).
    assert_invalid_data(&dir, "CRC mismatch");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn writer_flipped_sealed_trailer_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("w-flip-trl");
    persist_with_sealed(&rt, &dir);
    let path = first_sealed_path(&dir);
    let bytes = fs::read(&path).unwrap();
    let ranges = event_frame_ranges(&bytes);
    xor_at(&path, ranges[ranges.len() - 1].1 + 8);
    // Trailer is a CRC'd frame; CRC is first (before trailer JSON field checks).
    assert_invalid_data(&dir, "CRC mismatch");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn writer_flipped_manifest_chain_hex_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("w-flip-hex");
    persist_with_sealed(&rt, &dir);
    let man_path = dir.join(store::MANIFEST_REL);
    let text = fs::read_to_string(&man_path).unwrap();
    let mut v: serde_json::Value = serde_json::from_str(&text).unwrap();
    let chain = v["sealed"][0]["chain"].as_str().unwrap().to_string();
    let mut chars: Vec<char> = chain.chars().collect();
    chars[0] = if chars[0] == '0' { '1' } else { '0' };
    v["sealed"][0]["chain"] = serde_json::Value::String(chars.into_iter().collect());
    fs::write(&man_path, serde_json::to_vec(&v).unwrap()).unwrap();
    assert_invalid_data(&dir, "manifest disagrees");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn writer_swapped_adjacent_sealed_frames_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("w-swap");
    persist_with_sealed(&rt, &dir);
    let path = first_sealed_path(&dir);
    let bytes = fs::read(&path).unwrap();
    let ranges = event_frame_ranges(&bytes);
    let mut rebuilt = bytes[..store::SEGMENT_HEADER_LEN as usize].to_vec();
    let mut order: Vec<usize> = (0..ranges.len()).collect();
    order.swap(1, 2);
    for i in order {
        rebuilt.extend_from_slice(&bytes[ranges[i].0..ranges[i].1]);
    }
    rebuilt.extend_from_slice(&bytes[ranges[ranges.len() - 1].1..]);
    fs::write(&path, &rebuilt).unwrap();
    assert_invalid_data(&dir, "chain");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn writer_deleted_sealed_frame_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("w-del-frm");
    persist_with_sealed(&rt, &dir);
    let path = first_sealed_path(&dir);
    let bytes = fs::read(&path).unwrap();
    let ranges = event_frame_ranges(&bytes);
    let mut rebuilt = bytes[..store::SEGMENT_HEADER_LEN as usize].to_vec();
    for (i, r) in ranges.iter().enumerate() {
        if i == 1 {
            continue;
        }
        rebuilt.extend_from_slice(&bytes[r.0..r.1]);
    }
    rebuilt.extend_from_slice(&bytes[ranges[ranges.len() - 1].1..]);
    fs::write(&path, &rebuilt).unwrap();
    assert_invalid_data(&dir, "count mismatch");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn writer_deleted_sealed_file_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("w-del-file");
    persist_with_sealed(&rt, &dir);
    fs::remove_file(first_sealed_path(&dir)).unwrap();
    assert_invalid_data(&dir, "missing sealed");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn writer_replaced_sealed_by_older_copy_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("w-older");
    {
        store::set_test_segment_limits(Some(10_000_000), Some(3));
        let _g = LimitGuard;
        persist_expect_segmented(&rt, &dir);
    }
    let old = fs::read(first_sealed_path(&dir)).unwrap();
    {
        let _g = sealed_writer_limits();
        persist_expect_segmented(&rt, &dir);
    }
    let newer = fs::read(first_sealed_path(&dir)).unwrap();
    assert_ne!(
        old, newer,
        "second persist must rewrite the first sealed file"
    );
    fs::write(first_sealed_path(&dir), &old).unwrap();
    assert_invalid_data(&dir, "manifest disagrees");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn writer_truncated_sealed_at_frame_boundary_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("w-trunc-bd");
    persist_with_sealed(&rt, &dir);
    let path = first_sealed_path(&dir);
    let bytes = fs::read(&path).unwrap();
    let ranges = event_frame_ranges(&bytes);
    fs::write(&path, &bytes[..ranges[1].1]).unwrap();
    assert_invalid_data(&dir, "truncated sealed");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn writer_appended_extra_frame_on_sealed_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("w-extra");
    persist_with_sealed(&rt, &dir);
    let path = first_sealed_path(&dir);
    let bytes = fs::read(&path).unwrap();
    let ranges = event_frame_ranges(&bytes);
    let extra = bytes[ranges[ranges.len() - 1].0..ranges[ranges.len() - 1].1].to_vec();
    let mut out = bytes;
    out.extend_from_slice(&extra);
    fs::write(&path, &out).unwrap();
    assert_invalid_data(&dir, "extra bytes after sealed trailer");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn writer_sealed_trailer_bytes_field_mismatch_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("w-trl-bytes");
    persist_with_sealed(&rt, &dir);
    rewrite_trailer_bytes_field(&first_sealed_path(&dir));
    assert_invalid_data(&dir, "trailer bytes");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn writer_torn_active_tail_is_dropped() {
    let rt = seed_runtime();
    let dir = persist_dir("w-torn");
    persist_with_sealed(&rt, &dir);
    let active = active_path(&dir);
    let mut bytes = fs::read(&active).unwrap();
    bytes.extend_from_slice(&[0xAA, 0xBB, 0xCC]);
    fs::write(&active, &bytes).unwrap();
    let opened = store::open(&dir).unwrap();
    assert_eq!(log_ids(&opened), log_ids(&rt));
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn writer_unnamed_leftover_is_ignored() {
    let rt = seed_runtime();
    let dir = persist_dir("w-left");
    persist_with_sealed(&rt, &dir);
    fs::write(dir.join("orphan.bin"), b"not-a-segment").unwrap();
    expect_round_trip(&dir, &rt);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn migrate_preserves_ids_and_fold_fingerprint() {
    let rt = seed_runtime();
    let dir = persist_dir("mig-ids");
    persist_legacy(&rt, &dir);
    let want = log_ids(&rt);
    let want_fp = rt.fold().fingerprint();
    let report = store::migrate(&dir).unwrap();
    assert_eq!(report.dropped_uncommitted, 0);
    assert!(dir.join(store::MANIFEST_REL).exists());
    assert!(dir.join("events.jsonl").exists(), "jsonl remains as export");
    assert!(!dir.join("events.wal").exists(), "wal removed after verify");
    let opened = store::open(&dir).unwrap();
    assert_eq!(log_ids(&opened), want);
    assert_eq!(opened.fold().fingerprint(), want_fp);
    opened.replay_check().unwrap();
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn migrate_drops_uncommitted_suffix_and_reports() {
    let mut rt = seed_runtime();
    let dir = persist_dir("mig-drop");
    persist_legacy(&rt, &dir);
    let committed = log_ids(&rt);
    let want_fp = rt.fold().fingerprint();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_product_assert(&mut rt, "art-mig-drop", "99-kmh");
    crash_omit_last(&mut p, &rt);
    let report = store::migrate(&dir).unwrap();
    assert!(
        report.dropped_uncommitted > 0,
        "uncommitted suffix must be reported"
    );
    let opened = store::open(&dir).unwrap();
    assert_eq!(log_ids(&opened), committed);
    assert_eq!(opened.fold().fingerprint(), want_fp);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn migrate_is_idempotent() {
    let rt = seed_runtime();
    let dir = persist_dir("mig-idem");
    persist_legacy(&rt, &dir);
    store::migrate(&dir).unwrap();
    let man = fs::read(dir.join(store::MANIFEST_REL)).unwrap();
    let report = store::migrate(&dir).unwrap();
    assert_eq!(report.dropped_uncommitted, 0);
    assert_eq!(fs::read(dir.join(store::MANIFEST_REL)).unwrap(), man);
    expect_round_trip(&dir, &rt);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn migrate_crash_matrix_reopens_without_loss() {
    let rt = seed_runtime();
    let dir = persist_dir("mig-crash-src");
    persist_legacy(&rt, &dir);
    let want = log_ids(&rt);
    let want_fp = rt.fold().fingerprint();
    for step in [
        store::MigrateStep::Segments,
        store::MigrateStep::Verified,
        store::MigrateStep::WalRemoved,
    ] {
        let run = persist_dir(&format!("mig-crash-{step:?}"));
        copy_store(&dir, &run);
        let err = store::migrate_with_failpoint(&run, step).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::Other);
        assert!(
            err.to_string().contains("failpoint"),
            "migrate crash must be the injected failpoint; got {err}"
        );
        let opened = store::open(&run).unwrap();
        assert_eq!(log_ids(&opened), want, "loss after {step:?}");
        assert_eq!(opened.fold().fingerprint(), want_fp);
        match step {
            store::MigrateStep::Segments | store::MigrateStep::Verified => {
                assert!(
                    run.join("events.wal").exists(),
                    "wal must survive until after verify ({step:?})"
                );
            }
            store::MigrateStep::WalRemoved => {
                assert!(
                    !run.join("events.wal").exists(),
                    "wal gone after WalRemoved"
                );
            }
        }
        let _ = fs::remove_dir_all(&run);
    }
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn append_after_migrate_resumes() {
    let mut rt = seed_runtime();
    let dir = persist_dir("mig-append");
    persist_legacy(&rt, &dir);
    store::migrate(&dir).unwrap();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_one_more(&mut rt, T_NEW);
    p.append(&rt).unwrap();
    let opened = store::open(&dir).unwrap();
    assert_eq!(log_ids(&opened), log_ids(&rt));
    assert!(dir.join(store::MANIFEST_REL).exists());
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn export_jsonl_roundtrips_and_is_never_read_on_open() {
    let rt = seed_runtime();
    let dir = persist_dir("export");
    persist_expect_segmented(&rt, &dir);
    assert!(!dir.join("events.jsonl").exists());
    store::export_jsonl(&dir).unwrap();
    assert!(dir.join("events.jsonl").exists());
    let exported = read_jsonl_events(&dir.join("events.jsonl"));
    assert_eq!(
        exported.iter().map(|e| e.id).collect::<Vec<_>>(),
        store::encoded_events(&rt)
            .iter()
            .map(|e| e.id)
            .collect::<Vec<_>>()
    );
    fs::write(dir.join("events.jsonl"), b"{not-json\n").unwrap();
    expect_round_trip(&dir, &rt);
    assert_eq!(
        fs::read(dir.join("events.jsonl")).unwrap(),
        b"{not-json\n",
        "open must not rewrite events.jsonl when a manifest exists"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn mutate_jsonl_with_manifest_open_is_ok() {
    let rt = seed_runtime();
    let dir = persist_dir("mut-jsonl");
    persist_expect_segmented(&rt, &dir);
    fs::write(dir.join("events.jsonl"), b"garbage\n").unwrap();
    expect_round_trip(&dir, &rt);
    assert_eq!(
        fs::read(dir.join("events.jsonl")).unwrap(),
        b"garbage\n",
        "open must not rewrite events.jsonl when a manifest exists"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn legacy_dir_without_migrate_stays_legacy_after_persist() {
    let rt = seed_runtime();
    let dir = persist_dir("stay-leg");
    persist_legacy(&rt, &dir);
    store::persist(&rt, &dir).unwrap();
    assert!(
        !dir.join(store::MANIFEST_REL).exists(),
        "persist must not auto-convert a legacy dir"
    );
    assert!(dir.join("events.jsonl").exists());
    expect_round_trip(&dir, &rt);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn migrate_verify_fault_returns_invalid_data_and_keeps_legacy() {
    let rt = seed_runtime();
    let dir = persist_dir("mig-verify-fault");
    persist_legacy(&rt, &dir);
    let want = log_ids(&rt);
    let _g = sealed_writer_limits();
    let err = store::migrate_with_verify_fault(&dir, store::MigrateVerifyFault::FlipSealedByte)
        .expect_err("verify must see the corrupted sealed file");
    assert_eq!(err.kind(), ErrorKind::InvalidData);
    assert!(
        err.to_string().contains("CRC mismatch"),
        "verify fault must name the CRC check; got {err}"
    );
    assert!(
        dir.join("events.jsonl").exists(),
        "jsonl must survive a failed verify"
    );
    assert!(
        dir.join("events.wal").exists(),
        "wal must survive a failed verify"
    );
    assert!(
        !dir.join(store::MANIFEST_REL).exists(),
        "failed verify must leave no manifest (legacy intact)"
    );
    let opened = store::open(&dir).unwrap();
    assert_eq!(log_ids(&opened), want);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn migrate_verify_failure_after_manifest_flip_unflips() {
    let rt = seed_runtime();
    let dir = persist_dir("mig-unflip");
    persist_legacy(&rt, &dir);
    let _g = sealed_writer_limits();
    store::migrate_with_verify_fault(&dir, store::MigrateVerifyFault::FlipSealedByte)
        .expect_err("verify after persist_log (manifest already written) must fail");
    assert!(
        !dir.join(store::MANIFEST_REL).exists(),
        "manifest flip must be unflipped when verify fails"
    );
    assert!(dir.join("events.jsonl").exists());
    assert!(dir.join("events.wal").exists());
    expect_round_trip(&dir, &rt);
    let _ = fs::remove_dir_all(&dir);
}

fn persist_many_sealed(rt: &Runtime, dir: &Path, max_events: usize) {
    store::set_test_segment_limits(Some(10_000_000), Some(max_events));
    let _g = LimitGuard;
    persist_expect_segmented(rt, dir);
}

#[test]
fn swapped_sealed_files_with_matching_meta_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("swap-files");
    persist_with_sealed(&rt, &dir);
    assert!(sealed_len(&dir) >= 2, "need two sealed files");
    let p0 = sealed_path(&dir, 0);
    let p1 = sealed_path(&dir, 1);
    let a = fs::read(&p0).unwrap();
    let b = fs::read(&p1).unwrap();
    fs::write(&p0, &b).unwrap();
    fs::write(&p1, &a).unwrap();
    retarget_manifest_sealed_from_file(&dir, 0, &p0);
    retarget_manifest_sealed_from_file(&dir, 1, &p1);
    assert_invalid_data(&dir, "chain link");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn replaced_sealed_from_other_store_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("repl-sealed");
    persist_with_sealed(&rt, &dir);
    let mut other = seed_runtime();
    other.intern("other-store-term");
    let dir_b = persist_dir("repl-sealed-b");
    persist_with_sealed(&other, &dir_b);
    let src = sealed_path(&dir_b, 0);
    let dst = sealed_path(&dir, 1);
    fs::copy(&src, &dst).unwrap();
    retarget_manifest_sealed_from_file(&dir, 1, &dst);
    assert_invalid_data(&dir, "chain link");
    let _ = fs::remove_dir_all(&dir);
    let _ = fs::remove_dir_all(&dir_b);
}

#[test]
fn deleted_middle_sealed_with_contiguous_manifest_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("del-mid");
    persist_many_sealed(&rt, &dir, 3);
    let n = sealed_len(&dir);
    assert!(n >= 3, "need three sealed files");
    let keep: Vec<Vec<u8>> = (0..n)
        .filter(|&i| i != 1)
        .map(|i| fs::read(sealed_path(&dir, i)).unwrap())
        .collect();
    let active_bytes = fs::read(active_path(&dir)).unwrap();
    for i in 0..n {
        fs::remove_file(sealed_path(&dir, i)).unwrap();
    }
    fs::remove_file(active_path(&dir)).unwrap();
    for (i, bytes) in keep.iter().enumerate() {
        fs::write(dir.join(format!("seg-{:08}.log", i as u32 + 1)), bytes).unwrap();
    }
    let active_name = format!("seg-{:08}.log", keep.len() as u32 + 1);
    fs::write(dir.join(&active_name), &active_bytes).unwrap();
    let mut man: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(manifest_path(&dir)).unwrap()).unwrap();
    let old = man["sealed"].as_array().unwrap().clone();
    let mut rebuilt = Vec::new();
    let mut seq = 1u32;
    for (i, entry) in old.iter().enumerate() {
        if i == 1 {
            continue;
        }
        let mut e = entry.clone();
        e["name"] = serde_json::Value::String(format!("seg-{:08}.log", seq));
        rebuilt.push(e);
        seq = seq.saturating_add(1);
    }
    man["sealed"] = serde_json::Value::Array(rebuilt);
    man["active"] = serde_json::Value::String(active_name);
    fs::write(manifest_path(&dir), serde_json::to_vec(&man).unwrap()).unwrap();
    for i in 0..keep.len() {
        retarget_manifest_sealed_from_file(
            &dir,
            i,
            &dir.join(format!("seg-{:08}.log", i as u32 + 1)),
        );
    }
    assert_invalid_data(&dir, "chain link");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn reordered_manifest_sealed_entries_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("reord-man");
    persist_with_sealed(&rt, &dir);
    assert!(sealed_len(&dir) >= 2);
    let mut man: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(manifest_path(&dir)).unwrap()).unwrap();
    let a = man["sealed"][0].clone();
    let b = man["sealed"][1].clone();
    man["sealed"][0] = b;
    man["sealed"][1] = a;
    fs::write(manifest_path(&dir), serde_json::to_vec(&man).unwrap()).unwrap();
    assert_invalid_data(&dir, "not contiguous");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn active_segment_from_other_store_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("act-other");
    persist_with_sealed(&rt, &dir);
    let mut other = seed_runtime();
    other.intern("active-from-elsewhere");
    let dir_b = persist_dir("act-other-b");
    persist_with_sealed(&other, &dir_b);
    fs::copy(active_path(&dir_b), active_path(&dir)).unwrap();
    assert_invalid_data(&dir, "chain link");
    let _ = fs::remove_dir_all(&dir);
    let _ = fs::remove_dir_all(&dir_b);
}
