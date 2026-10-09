use crate::quantum::{
    Justification, PersistedQuantumOutcome, Runtime, RuntimeError, cascade_limit,
};
use crate::snapshot::Snapshot;
use crate::wal;
use kutha_common::{Event, EventId, Op};
use serde::de::DeserializeOwned;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::path::Path;

const TERMS_REL: &str = "terms.jsonl";
/// Outcomes sidecar lease. SoT is `Op::QuantumOutcome` on the event log (LOG-01 / D-02).
pub const OUTCOMES_REL: &str = "quantum_outcomes.jsonl";
/// Justifications sidecar lease. SoT is `Op::JustificationCite` on the event log (LOG-02 / D-02).
pub const JUSTIFICATIONS_REL: &str = "justifications.jsonl";

/// Named persist steps for the crash-matrix failpoint (D-07). Stop *after* the
/// named step completes (WAL, jsonl, terms, snapshot, outcomes, justifications).
#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PersistStep {
    Wal,
    Jsonl,
    Terms,
    Snapshot,
    Outcomes,
    Justifications,
}

/// Advisory single-writer lock file inside the store directory (WRITER-01).
/// Local filesystem only; not multi-host / NFS coordination.
pub const STORE_LOCK_REL: &str = "kutha.lock";

thread_local! {
    static PERSIST_DIR_FSYNC_COUNT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static REPLACE_TMP_FILE_SYNC_COUNT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// Test-visible count of directory `sync_all` calls from [`sync_dir`] on the
/// current thread (exact under parallel tests).
#[doc(hidden)]
pub fn persist_dir_fsync_count() -> usize {
    PERSIST_DIR_FSYNC_COUNT.with(|c| c.get())
}

/// Reset the directory fsync counter. Tests that assert a delta should prefer
/// [`persist_dir_fsync_count`] before/after a single `persist` instead.
#[doc(hidden)]
pub fn reset_persist_dir_fsync_count() {
    PERSIST_DIR_FSYNC_COUNT.with(|c| c.set(0));
}

/// Test-visible count of tmp-file `sync_all` calls from [`replace_via_tmp`].
#[doc(hidden)]
pub fn replace_via_tmp_file_sync_count() -> usize {
    REPLACE_TMP_FILE_SYNC_COUNT.with(|c| c.get())
}

/// Reset the tmp-file `sync_all` counter (thread-local).
#[doc(hidden)]
pub fn reset_replace_via_tmp_file_sync_count() {
    REPLACE_TMP_FILE_SYNC_COUNT.with(|c| c.set(0));
}

/// Exclusive advisory lock on [`STORE_LOCK_REL`]. Fail-closed: `WouldBlock` names the file.
pub fn acquire_store_write_lock(dir: &Path) -> std::io::Result<File> {
    fs::create_dir_all(dir)?;
    let path = dir.join(STORE_LOCK_REL);
    let file = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(&path)?;
    match file.try_lock() {
        Ok(()) => Ok(file),
        Err(std::fs::TryLockError::WouldBlock) => Err(std::io::Error::new(
            ErrorKind::WouldBlock,
            format!("store writer lock held ({STORE_LOCK_REL})"),
        )),
        Err(std::fs::TryLockError::Error(e)) => Err(std::io::Error::new(
            e.kind(),
            format!("store writer lock failed on {STORE_LOCK_REL}: {e}"),
        )),
    }
}

/// Opens `dir` and calls `sync_all` so a rename is durable on the parent.
///
/// On platforms where opening a directory fails (non-Unix), returns `Ok`
/// without syncing — directory file descriptors are a Unix feature.
pub(crate) fn sync_dir(dir: &Path) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        let f = File::open(dir)?;
        f.sync_all()?;
        PERSIST_DIR_FSYNC_COUNT.with(|c| c.set(c.get() + 1));
        Ok(())
    }
    #[cfg(not(unix))]
    {
        match File::open(dir) {
            Ok(f) => {
                f.sync_all()?;
                PERSIST_DIR_FSYNC_COUNT.with(|c| c.set(c.get() + 1));
                Ok(())
            }
            Err(_) => Ok(()),
        }
    }
}

fn injected_persist_failpoint() -> std::io::Error {
    std::io::Error::new(
        std::io::ErrorKind::Other,
        "injected fault: persist failpoint",
    )
}

fn after_persist_step(stop_after: Option<PersistStep>, step: PersistStep) -> std::io::Result<()> {
    if stop_after == Some(step) {
        Err(injected_persist_failpoint())
    } else {
        Ok(())
    }
}

fn injected_migrate_failpoint() -> std::io::Error {
    std::io::Error::new(
        std::io::ErrorKind::Other,
        "injected fault: migrate failpoint",
    )
}

fn after_migrate_step(stop_after: Option<MigrateStep>, step: MigrateStep) -> std::io::Result<()> {
    if stop_after == Some(step) {
        Err(injected_migrate_failpoint())
    } else {
        Ok(())
    }
}

fn io_invalid(msg: &str) -> std::io::Error {
    std::io::Error::new(ErrorKind::InvalidData, msg)
}

/// File-backed semantic log + snapshot lease. WAL file is durability cousin, not Rocks-as-SoT.
/// `terms.jsonl` is a derived intern picture. Durable term meanings ride `Op::Define` (persist prefix + live intern, M010 S02–S03).
///
/// Layout (54-02): an **empty** dir (no `manifest.json`, no `events.jsonl`, no `events.wal`)
/// and an already-segmented dir (manifest present) write `KUTHASEG` + manifest. A **legacy**
/// dir (`events.jsonl` or `events.wal` present, no manifest) keeps jsonl+WAL. Persist never
/// auto-migrates. Pin tests that inspect jsonl tails with [`ensure_legacy_dir`].
///
/// Persist order (legacy): WAL, jsonl, terms, snapshot, outcomes, justifications.
/// Persist order (segmented): segments+manifest, terms, snapshot, outcomes, justifications.
pub fn persist(runtime: &Runtime, dir: &Path) -> std::io::Result<()> {
    persist_until(runtime, dir, None)
}

/// Uncommitted suffix events skipped by [`migrate`] (COMMIT-01).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MigrateReport {
    pub dropped_uncommitted: usize,
}

/// Crash-injection points after each migrate step (DUR-01). `events.wal` is
/// removed only after a verified reopen of the segmented store.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MigrateStep {
    Segments,
    Verified,
    WalRemoved,
}

/// Convert a legacy jsonl+WAL directory to `manifest.json` + segments.
///
/// Loads via [`load_events_d04`] then [`committed_prefix`]: only the committed
/// prefix is written; the uncommitted suffix is dropped and counted in
/// [`MigrateReport::dropped_uncommitted`]. `events.jsonl` is left as an export
/// and is never read by [`open`] once the manifest exists. `events.wal` is
/// removed only after a verified reopen (ids + fold fingerprint). Idempotent
/// when `manifest.json` already exists. Never auto-invoked from [`persist`].
pub fn migrate(dir: &Path) -> std::io::Result<MigrateReport> {
    migrate_until(dir, None, None)
}

/// Crash-injection migrate: steps through `stop_after` inclusive, then an injected fault.
pub fn migrate_with_failpoint(
    dir: &Path,
    stop_after: MigrateStep,
) -> std::io::Result<MigrateReport> {
    migrate_until(dir, Some(stop_after), None)
}

/// Fault injected after segments+manifest are on disk and before verify-reopen.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MigrateVerifyFault {
    /// XOR one payload byte in the first sealed file (CRC must fail verify).
    FlipSealedByte,
}

/// Migrate, then corrupt the freshly written store before verify-reopen.
pub fn migrate_with_verify_fault(
    dir: &Path,
    fault: MigrateVerifyFault,
) -> std::io::Result<MigrateReport> {
    migrate_until(dir, None, Some(fault))
}

fn migrate_until(
    dir: &Path,
    stop_after: Option<MigrateStep>,
    fault: Option<MigrateVerifyFault>,
) -> std::io::Result<MigrateReport> {
    fs::create_dir_all(dir)?;
    let _lock = acquire_store_write_lock(dir)?;
    if dir.join(crate::segment::MANIFEST_REL).exists() {
        let wal = dir.join(EVENTS_WAL);
        if wal.exists() {
            fs::remove_file(&wal)?;
        }
        return Ok(MigrateReport {
            dropped_uncommitted: 0,
        });
    }
    if !writes_legacy_layout(dir) {
        return Err(io_invalid(
            "migrate requires a legacy store (events.jsonl or events.wal, no manifest)",
        ));
    }
    let raw = load_events_d04(dir)?;
    let before = raw.len();
    let events = committed_prefix(raw);
    let dropped_uncommitted = before.saturating_sub(events.len());
    crate::segment::persist_log(dir, &events)?;
    after_migrate_step(stop_after, MigrateStep::Segments)?;
    if let Some(fault) = fault {
        apply_migrate_verify_fault(dir, fault)?;
    }
    if let Err(e) = migrate_verify_reopen(dir, &events) {
        let man = dir.join(crate::segment::MANIFEST_REL);
        if man.exists() {
            fs::remove_file(&man)?;
        }
        return Err(e);
    }
    after_migrate_step(stop_after, MigrateStep::Verified)?;
    let wal = dir.join(EVENTS_WAL);
    if wal.exists() {
        fs::remove_file(&wal)?;
    }
    after_migrate_step(stop_after, MigrateStep::WalRemoved)?;
    Ok(MigrateReport {
        dropped_uncommitted,
    })
}

fn apply_migrate_verify_fault(dir: &Path, fault: MigrateVerifyFault) -> std::io::Result<()> {
    match fault {
        MigrateVerifyFault::FlipSealedByte => {
            let manifest = crate::segment::read_manifest(dir)?;
            let name = manifest
                .sealed
                .first()
                .map(|s| s.name.clone())
                .unwrap_or(manifest.active);
            let path = dir.join(name);
            let mut bytes = fs::read(&path)?;
            let off = crate::segment::HEADER_LEN as usize + 8;
            if off >= bytes.len() {
                return Err(io_invalid("migrate verify fault: sealed file too short"));
            }
            bytes[off] ^= 1;
            fs::write(&path, bytes)
        }
    }
}

/// Reopen the segmented store and require ids + fold fingerprint to match the
/// committed prefix just written. Do not skip: a crash after this point may
/// drop `events.wal`.
fn migrate_verify_reopen(dir: &Path, events: &[Event]) -> std::io::Result<()> {
    let reopened = crate::segment::load_events(dir)?;
    if event_ids(&reopened) != event_ids(events) {
        return Err(io_invalid(
            "migrate verify: reopened segment ids disagree with committed prefix",
        ));
    }
    let opened = open(dir)?;
    opened.replay_check().map_err(runtime_err)?;
    let want = crate::fold::GraphFold::replay(events).fingerprint();
    if opened.fold().fingerprint() != want {
        return Err(io_invalid(
            "migrate verify: fold fingerprint disagrees with committed prefix",
        ));
    }
    Ok(())
}

/// Write `events.jsonl` from the on-disk log via `replace_via_tmp`.
/// Never consulted by [`open`] when `manifest.json` exists.
pub fn export_jsonl(dir: &Path) -> std::io::Result<()> {
    let _lock = acquire_store_write_lock(dir)?;
    let events = load_log_events(dir)?;
    replace_events_jsonl(dir, &events)
}

/// Touch `events.jsonl` so [`persist`] keeps the legacy jsonl+WAL writer.
/// Empty-dir detection is: no manifest, no `events.jsonl`, no `events.wal`.
#[doc(hidden)]
pub fn ensure_legacy_dir(dir: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dir)?;
    let jsonl = dir.join(EVENTS_JSONL);
    if !jsonl.exists() {
        File::create(&jsonl)?;
    }
    Ok(())
}

/// Test seam (D-07): run `persist` and return an injected I/O error after `stop_after` completes.
#[doc(hidden)]
pub fn persist_with_failpoint(
    runtime: &Runtime,
    dir: &Path,
    stop_after: PersistStep,
) -> std::io::Result<()> {
    persist_until(runtime, dir, Some(stop_after))
}

fn persist_until(
    runtime: &Runtime,
    dir: &Path,
    stop_after: Option<PersistStep>,
) -> std::io::Result<()> {
    fs::create_dir_all(dir)?;
    let _lock = acquire_store_write_lock(dir)?;
    persist_unlocked(runtime, dir, stop_after)
}

fn persist_unlocked(
    runtime: &Runtime,
    dir: &Path,
    stop_after: Option<PersistStep>,
) -> std::io::Result<()> {
    let events = encoded_log(runtime);
    if writes_legacy_layout(dir) {
        replace_events_wal(dir, &events, true)?;
        after_persist_step(stop_after, PersistStep::Wal)?;
        replace_events_jsonl(dir, &events)?;
        after_persist_step(stop_after, PersistStep::Jsonl)?;
    } else {
        crate::segment::persist_log(dir, &events)?;
        after_persist_step(stop_after, PersistStep::Wal)?;
        after_persist_step(stop_after, PersistStep::Jsonl)?;
    }
    drop(events);
    write_terms(dir, runtime.dictionary().strings())?;
    after_persist_step(stop_after, PersistStep::Terms)?;
    replace_snapshot(dir, &runtime.snapshot(), true)?;
    after_persist_step(stop_after, PersistStep::Snapshot)?;
    // Outcomes sidecar is a lease of log-native rows (D-05); never persist Typed-only buffers.
    write_outcomes(
        dir,
        &crate::hydrate::outcomes_from_events(runtime.log().iter()),
    )?;
    after_persist_step(stop_after, PersistStep::Outcomes)?;
    // Justifications last so a crash cannot leave cites without matching events;
    // missing file on open is empty vec (cannot admit).
    write_justifications(dir, &runtime.justification_records())?;
    after_persist_step(stop_after, PersistStep::Justifications)?;
    if stop_after.is_none() {
        sync_dir(dir)?;
    }
    Ok(())
}

/// Legacy writer: `events.jsonl` or `events.wal` present and no `manifest.json`.
/// Empty dir (none of the three) and segmented dirs write KUTHASEG.
fn writes_legacy_layout(dir: &Path) -> bool {
    !dir.join(crate::segment::MANIFEST_REL).exists()
        && (dir.join(EVENTS_JSONL).exists() || dir.join(EVENTS_WAL).exists())
}

pub(crate) const EVENTS_JSONL: &str = "events.jsonl";
const EVENTS_JSONL_TMP: &str = "events.jsonl.tmp";
pub(crate) const EVENTS_WAL: &str = "events.wal";
const EVENTS_WAL_TMP: &str = "events.wal.tmp";
const SNAPSHOT_JSON: &str = "snapshot.json";
const SNAPSHOT_JSON_TMP: &str = "snapshot.json.tmp";
const TERMS_TMP: &str = "terms.jsonl.tmp";
const OUTCOMES_TMP: &str = "quantum_outcomes.jsonl.tmp";
const JUSTIFICATIONS_TMP: &str = "justifications.jsonl.tmp";

fn abort_before_rename() -> std::io::Error {
    std::io::Error::new(
        std::io::ErrorKind::Other,
        "injected fault: abort before rename",
    )
}

pub(crate) fn replace_via_tmp(
    dest: &Path,
    tmp: &Path,
    commit: bool,
    write: impl FnOnce(&mut File) -> std::io::Result<()>,
) -> std::io::Result<()> {
    {
        let mut f = File::create(tmp)?;
        write(&mut f)?;
        f.sync_all()?;
        REPLACE_TMP_FILE_SYNC_COUNT.with(|c| c.set(c.get() + 1));
    }
    if !commit {
        return Err(abort_before_rename());
    }
    fs::rename(tmp, dest)?;
    if let Some(parent) = dest.parent() {
        if !parent.as_os_str().is_empty() {
            sync_dir(parent)?;
        }
    }
    Ok(())
}

/// Replace `events.jsonl` by writing a same-directory temp file, syncing, then rename.
/// `persist` returns Ok only after this rename (DUR-02).
fn replace_events_jsonl(dir: &Path, events: &[Event]) -> std::io::Result<()> {
    write_events_jsonl_temp(dir, events, true)
}

/// Test seam (DUR-02): write the temp sibling then return Err without renaming dest.
pub fn abort_events_jsonl_replace_before_rename(
    dir: &Path,
    events: &[Event],
) -> std::io::Result<()> {
    write_events_jsonl_temp(dir, events, false)
}

/// Test seam (REPL-02): WAL temp write, dest left byte-identical.
pub fn abort_wal_replace_before_rename(dir: &Path, runtime: &Runtime) -> std::io::Result<()> {
    replace_events_wal(dir, &encoded_log(runtime), false)
}

/// Test seam (REPL-02): snapshot temp write, dest left byte-identical.
pub fn abort_snapshot_replace_before_rename(dir: &Path, runtime: &Runtime) -> std::io::Result<()> {
    replace_snapshot(dir, &runtime.snapshot(), false)
}

fn write_events_jsonl_temp(dir: &Path, events: &[Event], commit: bool) -> std::io::Result<()> {
    replace_via_tmp(
        &dir.join(EVENTS_JSONL),
        &dir.join(EVENTS_JSONL_TMP),
        commit,
        |f| {
            for e in events {
                serde_json::to_writer(&mut *f, e).map_err(json_err)?;
                f.write_all(b"\n")?;
            }
            Ok(())
        },
    )
}

fn replace_events_wal(dir: &Path, events: &[Event], commit: bool) -> std::io::Result<()> {
    replace_via_tmp(
        &dir.join(EVENTS_WAL),
        &dir.join(EVENTS_WAL_TMP),
        commit,
        |f| wal::write_wal(f, events),
    )
}

pub(crate) fn replace_snapshot(dir: &Path, snap: &Snapshot, commit: bool) -> std::io::Result<()> {
    replace_via_tmp(
        &dir.join(SNAPSHOT_JSON),
        &dir.join(SNAPSHOT_JSON_TMP),
        commit,
        |f| {
            serde_json::to_writer(&mut *f, snap).map_err(json_err)?;
            Ok(())
        },
    )
}

fn read_jsonl_events(path: &Path) -> std::io::Result<Vec<Event>> {
    let bytes = fs::read(path)?;
    let mut v = Vec::new();
    let mut start = 0usize;
    for (i, b) in bytes.iter().enumerate() {
        if *b != b'\n' {
            continue;
        }
        let line = &bytes[start..i];
        start = i + 1;
        if line.iter().all(|c| c.is_ascii_whitespace()) {
            continue;
        }
        let text = std::str::from_utf8(line)
            .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))?;
        v.push(serde_json::from_str(text).map_err(json_err)?);
    }
    // D-05: bytes after the last newline are a torn tail and are dropped.
    Ok(v)
}

fn event_ids(events: &[Event]) -> Vec<EventId> {
    events.iter().map(|e| e.id).collect()
}

/// `candidate` covers `covered` when ids match in order and `candidate` may be longer (D-04).
fn ids_cover(candidate: &[Event], covered: &[Event]) -> bool {
    let a = event_ids(candidate);
    let b = event_ids(covered);
    a.len() >= b.len() && a[..b.len()] == b[..]
}

/// D-04 / D-06: jsonl is SoT **for legacy dirs** (no `manifest.json`). WAL is a
/// lease. Prefer WAL only when recover yields magic+v1 and recovered ids cover
/// jsonl ids in order. Header-only / magic-only / empty / short /
/// recover-err(`InvalidData`) + readable jsonl → jsonl. A WAL `Err` of any
/// other kind is returned (disk failure is not masked). Both parse and neither
/// covers the other → InvalidData. No jsonl and invalid WAL → InvalidData.
///
/// Segmented dirs (`manifest.json` present) never consult this path: [`open`]
/// and [`export_jsonl`] go through `load_log_events` → `segment::load_events`.
/// [`migrate`] is the only conversion; it loads here, then `committed_prefix`,
/// and writes segments. `events.jsonl` left after migrate is an export, unread
/// on open.
pub(crate) fn load_events_d04(dir: &Path) -> std::io::Result<Vec<Event>> {
    let wal_path = dir.join(EVENTS_WAL);
    let jsonl_path = dir.join(EVENTS_JSONL);
    let jsonl = if jsonl_path.exists() {
        Some(read_jsonl_events(&jsonl_path)?)
    } else {
        None
    };
    let wal = if wal_path.exists() {
        Some(wal::recover_events(&wal_path))
    } else {
        None
    };
    match (wal, jsonl) {
        (Some(Ok(w)), Some(j)) => {
            if ids_cover(&w, &j) {
                Ok(w)
            } else if ids_cover(&j, &w) {
                Ok(j)
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "WAL and events.jsonl ids disagree (neither sequence covers the other)",
                ))
            }
        }
        (Some(Ok(w)), None) => Ok(w),
        (Some(Err(e)), Some(j)) => {
            if e.kind() == std::io::ErrorKind::InvalidData {
                Ok(j)
            } else {
                Err(e)
            }
        }
        (Some(Err(e)), None) => {
            if e.kind() == std::io::ErrorKind::InvalidData {
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("invalid WAL and no events.jsonl: {e}"),
                ))
            } else {
                Err(e)
            }
        }
        (None, Some(j)) => Ok(j),
        (None, None) => Ok(Vec::new()),
    }
}

/// How many log events `open` hid as an uncommitted suffix (COMMIT-01).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpenReport {
    pub dropped_uncommitted: usize,
}

/// Trim a loaded log to the committed prefix (COMMIT-01 option A).
pub fn committed_prefix(events: Vec<Event>) -> Vec<Event> {
    crate::commit::committed_prefix(events)
}

pub fn op_commit_class(op: &Op) -> crate::commit::CommitClass {
    crate::commit::op_commit_class(op)
}

pub use crate::commit::CommitClass;

fn load_log_events(dir: &Path) -> std::io::Result<Vec<Event>> {
    if dir.join(crate::segment::MANIFEST_REL).exists() {
        crate::segment::load_events(dir)
    } else {
        load_events_d04(dir)
    }
}

/// Load `dir` and report how many events `committed_prefix` hid.
pub fn open_report(dir: &Path) -> std::io::Result<(Runtime, OpenReport)> {
    let raw = load_log_events(dir)?;
    let before = raw.len();
    let events = committed_prefix(raw);
    let dropped_uncommitted = before.saturating_sub(events.len());
    let rt = open_hydrated(dir, events)?;
    Ok((
        rt,
        OpenReport {
            dropped_uncommitted,
        },
    ))
}

/// Rewrite files when the opened prefix needs heal, then attach.
///
/// Heal runs when `committed_prefix` hid events, or when a legacy store's
/// jsonl tip is ahead of WAL so attach would fail despite `dropped==0`
/// (CE review #30).
pub fn open_for_append(dir: &Path) -> std::io::Result<(Runtime, Persister)> {
    open_for_append_until(dir, None)
}

/// Test seam: crash the heal persist after `stop_after`.
#[doc(hidden)]
pub fn open_for_append_with_failpoint(
    dir: &Path,
    stop_after: PersistStep,
) -> std::io::Result<(Runtime, Persister)> {
    open_for_append_until(dir, Some(stop_after))
}

fn open_for_append_until(
    dir: &Path,
    stop_after: Option<PersistStep>,
) -> std::io::Result<(Runtime, Persister)> {
    fs::create_dir_all(dir)?;
    let lock = acquire_store_write_lock(dir)?;
    let (rt, report) = open_report(dir)?;
    // Heal when COMMIT-01 hid a suffix, or when jsonl tip is ahead of WAL so
    // attach would fail despite dropped_uncommitted==0 (CE review #30).
    let needs_heal = report.dropped_uncommitted > 0
        || crate::persist_append::legacy_attach_needs_rewrite(dir, &rt)?;
    if needs_heal {
        persist_unlocked(&rt, dir, stop_after)?;
    }
    let persister = Persister::attach_holding(&rt, dir, Some(lock))?;
    Ok((rt, persister))
}

fn fold_affecting_count(events: &[Event]) -> usize {
    events
        .iter()
        .filter(|e| crate::quantum::op_affects_fold(&e.op))
        .count()
}

pub fn open(dir: &Path) -> std::io::Result<Runtime> {
    open_report(dir).map(|(rt, _)| rt)
}

fn open_hydrated(dir: &Path, events: Vec<Event>) -> std::io::Result<Runtime> {
    let snap_path = dir.join(SNAPSHOT_JSON);
    let terms_path = dir.join(TERMS_REL);
    let outcomes = load_outcomes(&dir.join(OUTCOMES_REL))?;
    let justifications = load_justifications(&dir.join(JUSTIFICATIONS_REL))?;
    if snap_path.exists() {
        let snap: Snapshot = serde_json::from_reader(File::open(&snap_path)?).map_err(json_err)?;
        if snap.log_offset > fold_affecting_count(&events) {
            return Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "snapshot log_offset is ahead of the committed prefix",
            ));
        }
        if snap.fold.tt_anchors_pinned {
            let graph: Vec<Event> = events
                .iter()
                .filter(|e| !matches!(e.op, Op::Define { .. }))
                .cloned()
                .collect();
            // Hydrate trap: compare snapshot-held walls before from_snapshot rebuilds from the log.
            Runtime::check_tt_anchors_range(&snap.fold.tt_anchors, &graph, Some(snap.next_tt))
                .map_err(runtime_err)?;
        }
        let rt = Runtime::from_snapshot(snap, events);
        rt.replay_check().map_err(runtime_err)?;
        return finish_open(rt, outcomes, justifications);
    }
    let defined = defined_names(&events);
    if !defined.is_empty() {
        let rt =
            Runtime::from_dict_and_events(defined, events, cascade_limit()).map_err(runtime_err)?;
        return finish_open(rt, outcomes, justifications);
    }
    if terms_path.exists() {
        if let Some(strings) = read_terms(&terms_path)? {
            let rt = Runtime::from_dict_and_events(strings, events, cascade_limit())
                .map_err(runtime_err)?;
            return finish_open(rt, outcomes, justifications);
        }
        // Unparseable last-resort terms.jsonl is absent (D-04).
    }
    if events.is_empty() {
        let rt = Runtime::default();
        return finish_open(rt, outcomes, justifications);
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "missing snapshot.json, terms.jsonl, and Op::Define events (cannot restore intern meanings)",
    ))
}

fn finish_open(
    mut rt: Runtime,
    outcomes: Option<Vec<PersistedQuantumOutcome>>,
    justifications: Option<Vec<Justification>>,
) -> std::io::Result<Runtime> {
    rt.hydrate_from_log();
    if let Some(rows) = outcomes {
        sidecar_or_stale_prefix(
            &rows,
            rt.outcome_records(),
            "outcomes sidecar disagrees with log QuantumOutcome rows",
        )?;
    }
    if let Some(rows) = justifications {
        sidecar_or_stale_prefix(
            &rows,
            &rt.justification_records(),
            "justifications sidecar disagrees with log JustificationCite rows",
        )?;
    }
    Ok(rt)
}

/// Intact sidecar must equal log-derived rows. A proper prefix (older persist) or
/// unparseable file is treated as absent by the loader; well-formed non-prefix fails.
fn sidecar_or_stale_prefix<T: PartialEq>(
    rows: &[T],
    log_rows: &[T],
    mismatch: &str,
) -> std::io::Result<()> {
    if log_rows.starts_with(rows) {
        Ok(())
    } else {
        Err(std::io::Error::new(ErrorKind::InvalidData, mismatch))
    }
}

/// Encoded log (Defines then graph events) that [`persist`] writes.
#[doc(hidden)]
pub fn encoded_events(runtime: &Runtime) -> Vec<Event> {
    encoded_log(runtime)
}

/// Framed event bytes (`len_le32 + crc32 + JSON`) for test tampers.
#[doc(hidden)]
pub fn event_frame_bytes(event: &Event) -> std::io::Result<Vec<u8>> {
    wal::frame_bytes(event)
}

pub(crate) fn encoded_log(runtime: &Runtime) -> Vec<Event> {
    let mut out: Vec<Event> = runtime
        .dictionary()
        .strings()
        .iter()
        .map(|name| Event::stable_define(name.clone(), 0))
        .collect();
    for e in runtime.log().iter() {
        if !matches!(e.op, Op::Define { .. }) {
            out.push(e.clone());
        }
    }
    out
}

fn defined_names(events: &[Event]) -> Vec<String> {
    events
        .iter()
        .filter_map(|e| match &e.op {
            Op::Define { name } => Some(name.clone()),
            _ => None,
        })
        .collect()
}

pub(crate) fn write_terms(dir: &Path, strings: &[String]) -> std::io::Result<()> {
    replace_via_tmp(&dir.join(TERMS_REL), &dir.join(TERMS_TMP), true, |f| {
        for s in strings {
            serde_json::to_writer(&mut *f, s).map_err(json_err)?;
            f.write_all(b"\n")?;
        }
        Ok(())
    })
}

pub(crate) fn write_outcomes(dir: &Path, rows: &[PersistedQuantumOutcome]) -> std::io::Result<()> {
    replace_via_tmp(
        &dir.join(OUTCOMES_REL),
        &dir.join(OUTCOMES_TMP),
        true,
        |f| {
            for row in rows {
                serde_json::to_writer(&mut *f, row).map_err(json_err)?;
                f.write_all(b"\n")?;
            }
            Ok(())
        },
    )
}

/// Parse a JSONL sidecar. Missing, mid-line, or unparseable bytes are `None` (absent).
fn load_jsonl_sidecar<T: DeserializeOwned>(path: &Path) -> std::io::Result<Option<Vec<T>>> {
    if !path.exists() {
        return Ok(None);
    }
    let f = File::open(path)?;
    let mut out = Vec::new();
    for line in BufReader::new(f).lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) if e.kind() == ErrorKind::InvalidData => return Ok(None),
            Err(e) => return Err(e),
        };
        if line.trim().is_empty() {
            continue;
        }
        match serde_json::from_str(&line) {
            Ok(row) => out.push(row),
            Err(_) => return Ok(None),
        }
    }
    Ok(Some(out))
}

fn load_outcomes(path: &Path) -> std::io::Result<Option<Vec<PersistedQuantumOutcome>>> {
    load_jsonl_sidecar(path)
}

pub(crate) fn write_justifications(dir: &Path, rows: &[Justification]) -> std::io::Result<()> {
    replace_via_tmp(
        &dir.join(JUSTIFICATIONS_REL),
        &dir.join(JUSTIFICATIONS_TMP),
        true,
        |f| {
            for row in rows {
                serde_json::to_writer(&mut *f, row).map_err(json_err)?;
                f.write_all(b"\n")?;
            }
            Ok(())
        },
    )
}

fn load_justifications(path: &Path) -> std::io::Result<Option<Vec<Justification>>> {
    load_jsonl_sidecar(path)
}

/// Last-resort intern dictionary. Unparseable means absent (`None`), then open
/// falls through to the existing "cannot restore intern meanings" error.
fn read_terms(path: &Path) -> std::io::Result<Option<Vec<String>>> {
    load_jsonl_sidecar(path)
}

pub(crate) fn json_err(e: serde_json::Error) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, e)
}

fn runtime_err(e: RuntimeError) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
}

pub use crate::persist_append::{
    AppendStep, CHECKPOINT_EVERY, Persister, abort_checkpoint_snapshot_before_rename,
    append_with_event_failpoint, append_with_failpoint, checkpoint_with_failpoint,
    pending_append_len, persist_log_sync_count, reset_persist_log_sync_count,
};

#[doc(hidden)]
pub use crate::segment::{
    HEADER_LEN as SEGMENT_HEADER_LEN, MANIFEST_REL, Manifest, SEGMENT_MAX_BYTES,
    SEGMENT_MAX_EVENTS, abort_write_segment_before_rename, reset_test_segment_limits,
    rewrite_segment_with_seal_trailer, set_test_segment_limits, write_test_segmented,
};
