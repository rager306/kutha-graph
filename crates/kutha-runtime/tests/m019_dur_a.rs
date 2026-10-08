//! M019 S05 / Phase 60 durability residuals A. RED on current code is a bug.
//! D-05: FF5/FF6/M010–M017/`m002_*` assertions unchanged. Lock file is ignored
//! in directory listings only.

use kutha_common::{Event, EventId, Op};
use kutha_runtime::{Runtime, store};
use std::fs::{self, OpenOptions};
use std::io::ErrorKind;
use std::os::unix::fs::MetadataExt;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;

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

fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xffff_ffffu32;
    for &b in data {
        crc ^= u32::from(b);
        for _ in 0..8 {
            crc = if crc & 1 == 1 {
                (crc >> 1) ^ 0xedb8_8320
            } else {
                crc >> 1
            };
        }
    }
    !crc
}

const T_OLD: u64 = 2015;
const T_NEW: u64 = 2021;
const VF_FIRST: u64 = 2010;
const VF_SECOND: u64 = 2020;

fn persist_dir(label: &str) -> PathBuf {
    // PID + nanos: workspace runs spawn many binaries that hit tmp/ in parallel.
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .join(format!(
            "p60-store-{label}-{}-{nanos}",
            std::process::id()
        ));
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
    rt
}

fn log_ids(rt: &Runtime) -> Vec<EventId> {
    rt.log()
        .iter()
        .filter(|e| !matches!(e.op, Op::Define { .. }))
        .map(|e| e.id)
        .collect()
}

fn emit_product_assert(rt: &mut Runtime, subject: &str, object: &str) {
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

fn crash_omit_last(p: &mut store::Persister, rt: &Runtime) {
    let n = store::pending_append_len(p, rt).unwrap();
    assert!(
        n >= 2,
        "pending batch must include payload and QuantumOutcome; n={n}"
    );
    store::append_with_event_failpoint(p, rt, n - 1)
        .expect_err("event failpoint must inject after N-1 events");
}

fn listing_names(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n != store::STORE_LOCK_REL)
        .collect();
    names.sort();
    names
}

fn assert_writer_lock_error(err: &std::io::Error) {
    assert_eq!(
        err.kind(),
        ErrorKind::WouldBlock,
        "second writer must fail closed with WouldBlock; got {err}"
    );
    let msg = err.to_string();
    assert!(
        msg.contains(store::STORE_LOCK_REL),
        "error must name the lock file {}; got {msg}",
        store::STORE_LOCK_REL
    );
}

fn persist_legacy(rt: &Runtime, dir: &Path) {
    store::ensure_legacy_dir(dir).unwrap();
    store::persist(rt, dir).unwrap();
}

fn persist_segmented(rt: &Runtime, dir: &Path) {
    store::persist(rt, dir).unwrap();
    assert!(
        dir.join(store::MANIFEST_REL).exists(),
        "empty dir must persist as segments"
    );
}

fn overlay_full_active(rt: &Runtime, dir: &Path) {
    let events = store::encoded_events(rt);
    let n = events.len();
    assert!(n >= 6, "need a large active for rollover; n={n}");
    store::persist(rt, dir).unwrap();
    store::write_test_segmented(dir, &events, &[2], n - 2).unwrap();
}

fn active_name(dir: &Path) -> String {
    let v: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(dir.join(store::MANIFEST_REL)).unwrap()).unwrap();
    v["active"].as_str().expect("manifest.active").to_string()
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
        if serde_json::from_slice::<serde_json::Value>(&bytes[off + 8..end])
            .ok()
            .and_then(|v| v.get("k").and_then(|k| k.as_str()).map(|s| s == "seal"))
            .unwrap_or(false)
        {
            break;
        }
        out.push((off, end));
        off = end;
    }
    out
}

fn last_event_from_segment(bytes: &[u8]) -> Event {
    let (start, end) = *event_frame_ranges(bytes)
        .last()
        .expect("active must have an event frame");
    serde_json::from_slice(&bytes[start + 8..end]).expect("last Event")
}

#[test]
fn checkpoint_omitted_qo_then_open_for_append_fails_closed() {
    let mut rt = seed_runtime();
    let dir = persist_dir("ckpt-omit-qo");
    persist_segmented(&rt, &dir);
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_product_assert(&mut rt, "p60-omit-article", "p60-omit-80");
    crash_omit_last(&mut p, &rt);
    p.checkpoint(&rt).expect("checkpoint after omitted QO");
    drop(p);
    let err = match store::open_for_append(&dir) {
        Ok(_) => panic!("must fail closed, not heal"),
        Err(e) => e,
    };
    assert_eq!(err.kind(), ErrorKind::InvalidData);
    assert!(
        err.to_string().contains("snapshot log_offset is ahead"),
        "needle must stay snapshot-ahead; got {err}"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn jsonl_inode_changes_on_persist_rename() {
    let mut rt = seed_runtime();
    let dir = persist_dir("jsonl-ino");
    persist_legacy(&rt, &dir);
    let path = dir.join("events.jsonl");
    let ino1 = fs::metadata(&path).unwrap().ino();
    emit_product_assert(&mut rt, "p60-ino-article", "p60-ino-80");
    store::persist(&rt, &dir).unwrap();
    let ino2 = fs::metadata(&path).unwrap().ino();
    assert_ne!(
        ino1, ino2,
        "jsonl must be replaced via tmp+rename (inode must change)"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn seg_manifest_tmp_sync_all_before_rename() {
    let rt = seed_runtime();
    let dir = persist_dir("man-sync");
    store::reset_replace_via_tmp_file_sync_count();
    let before = store::replace_via_tmp_file_sync_count();
    store::persist_with_failpoint(&rt, &dir, store::PersistStep::Wal)
        .expect_err("stop after persist_log / write_manifest");
    let delta = store::replace_via_tmp_file_sync_count().saturating_sub(before);
    assert_eq!(
        delta, 2,
        "persist_log: one active segment + manifest each replace_via_tmp; tmp sync_all count={delta}"
    );
    assert!(dir.join(store::MANIFEST_REL).exists());
    assert!(!dir.join("manifest.json.tmp").exists());
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn seg_final_sync_dir_after_persist_log() {
    let rt = seed_runtime();
    let fail_dir = persist_dir("final-sync-fp");
    let ok_dir = persist_dir("final-sync-ok");
    let b1 = store::persist_dir_fsync_count();
    store::persist_with_failpoint(&rt, &fail_dir, store::PersistStep::Justifications)
        .expect_err("stop after justifications");
    let fail_delta = store::persist_dir_fsync_count().saturating_sub(b1);
    let b2 = store::persist_dir_fsync_count();
    store::persist(&rt, &ok_dir).unwrap();
    let ok_delta = store::persist_dir_fsync_count().saturating_sub(b2);
    assert_eq!(
        ok_delta,
        fail_delta + 1,
        "complete segmented persist must add the final sync_dir; ok={ok_delta} fail={fail_delta}"
    );
    let _ = fs::remove_dir_all(&fail_dir);
    let _ = fs::remove_dir_all(&ok_dir);
}

#[test]
fn mixed_typed_dict_partial_group_is_visible() {
    let mut rt = seed_runtime();
    let dir = persist_dir("mixed-group");
    persist_segmented(&rt, &dir);
    let committed = log_ids(&rt);
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    rt.emit_allow("p60-mixed-rel", VF_FIRST, None).unwrap();
    let n = store::pending_append_len(&p, &rt).unwrap();
    assert!(
        n >= 2,
        "dictionary group must be more than one encoded event; n={n}"
    );
    store::append_with_event_failpoint(&mut p, &rt, 1)
        .expect_err("crash after the first SelfCommit of the group");
    let opened = store::open(&dir).expect("partial SelfCommit group stays visible");
    assert_eq!(log_ids(&opened), committed, "graph ids stay at the last QO");
    assert!(
        opened.dictionary().id("p60-mixed-rel").is_some(),
        "characterization: a crashed Typed/Dictionary group can expose its Define prefix"
    );
    opened.replay_check().unwrap();
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn migrate_tail_guard_rejects_same_length_last_frame_swap() {
    let mut rt = seed_runtime();
    let dir = persist_dir("mig-guard");
    persist_legacy(&rt, &dir);
    store::migrate(&dir).unwrap();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    let active = dir.join(active_name(&dir));
    let bytes = fs::read(&active).unwrap();
    let mut last = last_event_from_segment(&bytes);
    let (start, end) = *event_frame_ranges(&bytes)
        .last()
        .expect("active event frame");
    last.id = EventId::from_u128(last.id.as_u128().wrapping_add(1));
    let frame = store::event_frame_bytes(&last).unwrap();
    assert_eq!(frame.len(), end - start, "swap must keep last-frame length");
    let mut tampered = bytes.clone();
    tampered[start..end].copy_from_slice(&frame);
    assert_eq!(tampered.len(), bytes.len());
    fs::write(&active, &tampered).unwrap();
    rt.emit_tt_anchor(T_NEW).unwrap();
    match p.append(&rt) {
        Ok(()) => panic!("tail guard after migrate must reject a same-length last-frame swap"),
        Err(e) => assert_eq!(e.kind(), ErrorKind::InvalidData),
    }
    assert_eq!(
        fs::read(&active).unwrap(),
        tampered,
        "rejected append must write nothing"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn active_truncate_at_exact_frame_end_keeps_prefix() {
    let mut rt = seed_runtime();
    let dir = persist_dir("exact-end");
    persist_segmented(&rt, &dir);
    let prefix = log_ids(&rt);
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    rt.emit_tt_anchor(T_NEW).unwrap();
    p.append(&rt).unwrap();
    drop(p);
    let active = dir.join(active_name(&dir));
    let bytes = fs::read(&active).unwrap();
    let ranges = event_frame_ranges(&bytes);
    let cut = ranges[ranges.len() - 2].1;
    assert!(
        cut < bytes.len(),
        "must drop the last complete frame, not tear mid-frame"
    );
    {
        let f = OpenOptions::new().write(true).open(&active).unwrap();
        f.set_len(cut as u64).unwrap();
    }
    let opened = store::open(&dir).expect("exact frame-end truncate keeps the prefix");
    assert_eq!(log_ids(&opened), prefix);
    opened.replay_check().unwrap();
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn wrong_role_active_names_sealed_file_fails_closed() {
    let _limits = tiny_limits();
    let rt = seed_runtime();
    let dir = persist_dir("wrong-role");
    persist_segmented(&rt, &dir);
    let man_path = dir.join(store::MANIFEST_REL);
    let mut man: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&man_path).unwrap()).unwrap();
    let sealed0 = man["sealed"][0]["name"]
        .as_str()
        .expect("need a sealed file")
        .to_string();
    man["active"] = serde_json::Value::String(sealed0);
    fs::write(&man_path, serde_json::to_vec(&man).unwrap()).unwrap();
    match store::open(&dir) {
        Ok(_) => panic!("active naming a sealed file must fail closed"),
        Err(e) => assert_eq!(e.kind(), ErrorKind::InvalidData),
    }
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn two_writer_attach_fails_closed() {
    let rt = seed_runtime();
    let dir = persist_dir("two-writer");
    persist_segmented(&rt, &dir);
    let _first = store::Persister::attach(&rt, &dir).unwrap();
    let err = match store::Persister::attach(&rt, &dir) {
        Ok(_) => panic!("second attach must fail closed"),
        Err(e) => e,
    };
    assert_writer_lock_error(&err);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn second_writer_names_kutha_lock() {
    let rt = seed_runtime();
    let dir = persist_dir("lock-msg");
    persist_legacy(&rt, &dir);
    let p = store::Persister::attach(&rt, &dir).unwrap();
    let err = store::persist(&rt, &dir).expect_err("persist while attach holds");
    assert_writer_lock_error(&err);
    drop(p);
    store::persist(&rt, &dir).expect("persist after drop");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn reader_open_while_writer_holds_lock() {
    let rt = seed_runtime();
    let dir = persist_dir("reader");
    persist_segmented(&rt, &dir);
    let want = log_ids(&rt);
    let _p = store::Persister::attach(&rt, &dir).unwrap();
    let opened = store::open(&dir).expect("reader takes no lock");
    assert_eq!(log_ids(&opened), want);
    opened.replay_check().unwrap();
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn dead_process_stale_lock_does_not_block() {
    if let Ok(dir) = std::env::var("KUTHA_P60_HOLD_LOCK") {
        let path = PathBuf::from(&dir).join(store::STORE_LOCK_REL);
        let f = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(&path)
            .unwrap();
        f.try_lock().expect("child must take the advisory lock");
        fs::write(PathBuf::from(&dir).join("p60-lock-ready"), b"1").unwrap();
        std::mem::forget(f);
        loop {
            std::thread::park();
        }
    }
    let rt = seed_runtime();
    let dir = persist_dir("dead-lock");
    persist_segmented(&rt, &dir);
    let exe = std::env::current_exe().expect("test binary");
    let mut child = Command::new(&exe)
        .arg("dead_process_stale_lock_does_not_block")
        .arg("--exact")
        .env("KUTHA_P60_HOLD_LOCK", dir.display().to_string())
        .spawn()
        .expect("spawn lock holder");
    let ready = dir.join("p60-lock-ready");
    while !ready.exists() {
        if let Ok(Some(st)) = child.try_wait() {
            panic!("lock-holder child exited before ready: {st}");
        }
        std::hint::spin_loop();
    }
    child.kill().unwrap();
    let _ = child.wait();
    let _p =
        store::Persister::attach(&rt, &dir).expect("advisory lock must die with the child process");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn panic_releases_writer_lock() {
    let rt = seed_runtime();
    let dir = persist_dir("panic-lock");
    persist_segmented(&rt, &dir);
    let panicked = catch_unwind(AssertUnwindSafe(|| {
        let _p = store::Persister::attach(&rt, &dir).unwrap();
        panic!("p60-release-lock");
    }));
    assert!(panicked.is_err(), "inner panic must run");
    store::Persister::attach(&rt, &dir).expect("Drop on unwind must release kutha.lock");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn lock_file_ignored_by_dir_listings() {
    let rt = seed_runtime();
    let dir = persist_dir("lock-list");
    persist_segmented(&rt, &dir);
    let _p = store::Persister::attach(&rt, &dir).unwrap();
    assert!(
        dir.join(store::STORE_LOCK_REL).exists(),
        "writer must create kutha.lock"
    );
    let tmp: Vec<_> = listing_names(&dir)
        .into_iter()
        .filter(|n| n.ends_with(".tmp"))
        .collect();
    assert!(
        tmp.is_empty(),
        "listings that skip the lock must see no .tmp; {tmp:?}"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn chain_active_prev_hash_flip_fails_closed() {
    let rt = seed_runtime();
    let dir = persist_dir("active-prev");
    persist_segmented(&rt, &dir);
    let active = dir.join(active_name(&dir));
    let mut bytes = fs::read(&active).unwrap();
    bytes[12] ^= 0xff;
    fs::write(&active, &bytes).unwrap();
    match store::open(&dir) {
        Ok(_) => panic!("flipped active header prev-hash must fail closed"),
        Err(e) => {
            assert_eq!(e.kind(), ErrorKind::InvalidData);
            assert!(
                e.to_string().contains("chain link"),
                "must name the chain-link check; got {e}"
            );
        }
    }
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn manifest_pre_rename_second_open_keeps_prefix() {
    let _limits = tiny_limits();
    let mut rt = seed_runtime();
    let dir = persist_dir("pre-rename");
    overlay_full_active(&rt, &dir);
    let committed = log_ids(&rt);
    let old_active = active_name(&dir);
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    rt.emit_tt_anchor(T_NEW).unwrap();
    store::append_with_failpoint(&mut p, &rt, store::AppendStep::RolloverNewHeader)
        .expect_err("crash after new header, before manifest rename");
    let man: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(dir.join(store::MANIFEST_REL)).unwrap()).unwrap();
    assert_eq!(
        man["active"].as_str(),
        Some(old_active.as_str()),
        "manifest must still name the old active"
    );
    let opened = store::open(&dir).expect("second open after pre-rename crash");
    assert_eq!(log_ids(&opened), committed);
    opened.replay_check().unwrap();
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn sealed_trailer_bytes_lie_fails_closed() {
    let _limits = tiny_limits();
    let rt = seed_runtime();
    let dir = persist_dir("trailer-lie");
    persist_segmented(&rt, &dir);
    let man_path = dir.join(store::MANIFEST_REL);
    let mut man: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&man_path).unwrap()).unwrap();
    let sealed_name = man["sealed"][0]["name"].as_str().unwrap().to_string();
    let sealed = dir.join(&sealed_name);
    let bytes = fs::read(&sealed).unwrap();
    let ranges = event_frame_ranges(&bytes);
    assert!(
        ranges.len() >= 2,
        "need a frame to drop from the sealed file"
    );
    let keep_end = ranges[ranges.len() - 2].1;
    let trailer_off = ranges.last().unwrap().1;
    let mut lied = bytes[..keep_end].to_vec();
    lied.extend_from_slice(&bytes[trailer_off..]);
    let new_len = lied.len() as u64;
    let t_start = keep_end + 8;
    let mut trailer: serde_json::Value = serde_json::from_slice(&lied[t_start..]).unwrap();
    trailer["bytes"] = serde_json::json!(new_len);
    let payload = serde_json::to_vec(&trailer).unwrap();
    let old_payload_len = lied.len() - t_start;
    assert_eq!(
        payload.len(),
        old_payload_len,
        "bytes-only lie must keep trailer payload length"
    );
    lied[t_start..].copy_from_slice(&payload);
    lied[keep_end + 4..keep_end + 8].copy_from_slice(&crc32(&payload).to_le_bytes());
    fs::write(&sealed, &lied).unwrap();
    man["sealed"][0]["bytes"] = serde_json::json!(new_len);
    fs::write(&man_path, serde_json::to_vec(&man).unwrap()).unwrap();
    match store::open(&dir) {
        Ok(_) => panic!("trailer.bytes rewritten to match a truncated sealed file must fail"),
        Err(e) => assert_eq!(e.kind(), ErrorKind::InvalidData),
    }
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn persist_migrate_race_serialized_or_fails_closed() {
    let rt = seed_runtime();
    let dir = persist_dir("race");
    persist_legacy(&rt, &dir);
    let p = store::Persister::attach(&rt, &dir).unwrap();
    let err = store::migrate(&dir).expect_err("migrate while a writer holds the lock");
    assert_writer_lock_error(&err);
    drop(p);

    let dir2 = persist_dir("race-threads");
    persist_legacy(&rt, &dir2);
    let dir2 = Arc::new(dir2);
    let rt = Arc::new(rt);
    std::thread::scope(|s| {
        let d_p = Arc::clone(&dir2);
        let d_m = Arc::clone(&dir2);
        let rt_p = Arc::clone(&rt);
        let hp = s.spawn(move || store::persist(rt_p.as_ref(), d_p.as_path()));
        let hm = s.spawn(move || store::migrate(d_m.as_path()));
        let rp = hp.join().expect("persist thread");
        let rm = hm.join().expect("migrate thread");
        match (rp, rm) {
            (Ok(()), Ok(_)) | (Ok(()), Err(_)) | (Err(_), Ok(_)) => {}
            (Err(a), Err(b)) => {
                assert!(
                    a.kind() == ErrorKind::WouldBlock || b.kind() == ErrorKind::WouldBlock,
                    "both writers failed without WouldBlock: {a}; {b}"
                );
            }
        }
    });
    store::open(dir2.as_path()).expect("race must not leave an unopenable store");
    let _ = fs::remove_dir_all(&dir);
    let _ = fs::remove_dir_all(dir2.as_path());
}

#[test]
fn persister_is_poisoned_after_failed_append_and_cannot_write_without_lock() {
    let mut rt = seed_runtime();
    let dir = persist_dir("poison");
    persist_segmented(&rt, &dir);
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    let active = dir.join(active_name(&dir));
    let bytes = fs::read(&active).unwrap();
    let mut last = last_event_from_segment(&bytes);
    let (start, end) = *event_frame_ranges(&bytes)
        .last()
        .expect("active event frame");
    last.id = EventId::from_u128(last.id.as_u128().wrapping_add(1));
    let frame = store::event_frame_bytes(&last).unwrap();
    assert_eq!(frame.len(), end - start);
    let mut tampered = bytes.clone();
    tampered[start..end].copy_from_slice(&frame);
    fs::write(&active, &tampered).unwrap();
    rt.emit_tt_anchor(T_NEW).unwrap();
    let first = p.append(&rt).expect_err("tail guard must fail closed");
    assert_eq!(first.kind(), ErrorKind::InvalidData);

    fs::write(&active, &bytes).unwrap();
    let opened = store::open(&dir).expect("restored prefix must open");
    let second_writer = store::Persister::attach(&opened, &dir)
        .expect("fail-closed append must release kutha.lock");
    drop(second_writer);
    let after_heal = listing_bytes(&dir);

    let again = p
        .append(&rt)
        .expect_err("poisoned Persister must not append");
    assert_eq!(
        again.kind(),
        ErrorKind::InvalidData,
        "poison returns the same fail-closed kind as the tail-guard append"
    );
    assert!(
        again.to_string().contains("poisoned"),
        "poison message must name the poison; got {again}"
    );
    assert_eq!(
        listing_bytes(&dir),
        after_heal,
        "poisoned append must write nothing"
    );

    let ck = p
        .checkpoint(&rt)
        .expect_err("poisoned Persister must not checkpoint");
    assert_eq!(ck.kind(), ErrorKind::InvalidData);
    assert_eq!(listing_bytes(&dir), after_heal);

    let cl = p
        .close(&rt)
        .expect_err("poisoned Persister must not close-write");
    assert_eq!(cl.kind(), ErrorKind::InvalidData);
    assert_eq!(listing_bytes(&dir), after_heal);

    let (healed, mut fresh) =
        store::open_for_append(&dir).expect("heal is a new open_for_append that re-takes the lock");
    fresh.append(&healed).expect("fresh writer after heal");
    drop(fresh);
    let _ = fs::remove_dir_all(&dir);
}

fn listing_bytes(dir: &Path) -> Vec<(String, Vec<u8>)> {
    listing_names(dir)
        .into_iter()
        .map(|n| {
            let b = fs::read(dir.join(&n)).unwrap_or_default();
            (n, b)
        })
        .collect()
}
