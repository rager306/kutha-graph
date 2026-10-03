//! Incremental framed append (M002 S04 / APPEND-01). Prefix bytes stay; one WAL `sync_data` commits.

use crate::hydrate::outcomes_from_events;
use crate::quantum::Runtime;
use crate::store::{
    encoded_log, json_err, replace_snapshot, sync_dir, write_justifications, write_outcomes,
    write_terms, EVENTS_JSONL, EVENTS_WAL,
};
use crate::wal;
use kutha_common::{Event, EventId, Op};
use std::cell::Cell;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

thread_local! {
    static ABORT_CHECKPOINT_SNAPSHOT_RENAME: Cell<bool> = const { Cell::new(false) };
}

/// Automatic sidecar/snapshot refresh after this many appended events (D-06).
pub const CHECKPOINT_EVERY: usize = 4096;

/// Named append/checkpoint failpoints (D-08). Stop *after* the named step.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppendStep {
    Jsonl,
    WalWrite,
    WalSync,
    CheckpointJsonl,
    CheckpointTerms,
    CheckpointSnapshot,
    CheckpointOutcomes,
    CheckpointJustifications,
}

/// Incremental framed append after a full [`crate::store::persist`] or [`crate::store::open`] (D-02).
pub struct Persister {
    dir: PathBuf,
    durable_count: usize,
    durable_graph_count: usize,
    dict_len: usize,
    jsonl_len: u64,
    wal_len: u64,
    last_id: EventId,
    last_graph_id: Option<EventId>,
    last_frame_len: u64,
    events_since_checkpoint: usize,
}

impl Persister {
    /// Attach to `dir` after a full persist or open.
    pub fn attach(runtime: &Runtime, dir: &Path) -> std::io::Result<Self> {
        let jsonl_path = dir.join(EVENTS_JSONL);
        let wal_path = dir.join(EVENTS_WAL);
        let jsonl_len = fs::metadata(&jsonl_path)?.len();
        let wal_len = fs::metadata(&wal_path)?.len();
        let encoded = encoded_log(runtime);
        if encoded.is_empty() {
            return Err(invalid(
                "attach requires a persisted encoded log (dictionary or events)",
            ));
        }
        let last_event = last_jsonl_event(&jsonl_path, jsonl_len)?;
        let last_id = last_event.id;
        let last_frame_len = wal::frame_bytes(&last_event)?.len() as u64;
        let wid = wal::last_event_id_at_end(&wal_path, wal_len, last_frame_len)?;
        if last_id != wid {
            return Err(invalid(
                "jsonl/WAL last EventId disagree (recover with full persist)",
            ));
        }
        let graph: Vec<&Event> = encoded
            .iter()
            .filter(|e| !matches!(e.op, Op::Define { .. }))
            .collect();
        Ok(Self {
            dir: dir.to_path_buf(),
            durable_count: encoded.len(),
            durable_graph_count: graph.len(),
            dict_len: runtime.dictionary().strings().len(),
            jsonl_len,
            wal_len,
            last_id,
            last_graph_id: graph.last().map(|e| e.id),
            last_frame_len,
            events_since_checkpoint: 0,
        })
    }

    pub fn append(&mut self, runtime: &Runtime) -> std::io::Result<()> {
        self.append_until(runtime, None, None)
    }

    pub fn checkpoint(&mut self, runtime: &Runtime) -> std::io::Result<()> {
        self.checkpoint_until(runtime, None)
    }

    pub fn close(mut self, runtime: &Runtime) -> std::io::Result<()> {
        self.checkpoint(runtime)
    }

    fn append_until(
        &mut self,
        runtime: &Runtime,
        stop_after: Option<AppendStep>,
        stop_after_events: Option<usize>,
    ) -> std::io::Result<()> {
        let new_events = collect_new(self, runtime)?;
        if new_events.is_empty() {
            return Ok(());
        }
        self.tail_guard()?;

        let write_n = stop_after_events.unwrap_or(new_events.len());
        if write_n == 0 {
            return Err(injected_event_failpoint());
        }
        let n = write_n.min(new_events.len());
        let batch = &new_events[..n];

        let jsonl_path = self.dir.join(EVENTS_JSONL);
        let wal_path = self.dir.join(EVENTS_WAL);

        {
            let mut f = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&jsonl_path)?;
            for e in batch {
                serde_json::to_writer(&mut f, e).map_err(json_err)?;
                f.write_all(b"\n")?;
            }
        }
        after_step(stop_after, AppendStep::Jsonl)?;

        {
            let mut f = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&wal_path)?;
            wal::write_frames(&mut f, batch)?;
            after_step(stop_after, AppendStep::WalWrite)?;
            f.sync_data()?;
            wal::note_log_file_sync();
        }
        after_step(stop_after, AppendStep::WalSync)?;

        if stop_after_events.is_some_and(|want| want < new_events.len()) {
            return Err(injected_event_failpoint());
        }

        remember_appended(self, runtime, batch, &jsonl_path, &wal_path)?;

        if self.events_since_checkpoint >= CHECKPOINT_EVERY {
            self.checkpoint_until(runtime, stop_after)?;
        }
        Ok(())
    }

    fn checkpoint_until(
        &mut self,
        runtime: &Runtime,
        stop_after: Option<AppendStep>,
    ) -> std::io::Result<()> {
        {
            let f = OpenOptions::new()
                .write(true)
                .open(self.dir.join(EVENTS_JSONL))?;
            f.sync_data()?;
            wal::note_log_file_sync();
        }
        after_step(stop_after, AppendStep::CheckpointJsonl)?;

        let commit_snapshot = !ABORT_CHECKPOINT_SNAPSHOT_RENAME.with(|c| c.replace(false));
        replace_snapshot(&self.dir, &runtime.snapshot(), commit_snapshot)?;
        after_step(stop_after, AppendStep::CheckpointSnapshot)?;

        write_terms(&self.dir, runtime.dictionary().strings())?;
        after_step(stop_after, AppendStep::CheckpointTerms)?;

        write_outcomes(&self.dir, &outcomes_from_events(runtime.log().iter()))?;
        after_step(stop_after, AppendStep::CheckpointOutcomes)?;

        write_justifications(&self.dir, &runtime.justification_records())?;
        after_step(stop_after, AppendStep::CheckpointJustifications)?;

        if stop_after.is_none() {
            sync_dir(&self.dir)?;
        }
        self.events_since_checkpoint = 0;
        Ok(())
    }

    fn tail_guard(&self) -> std::io::Result<()> {
        let jsonl_path = self.dir.join(EVENTS_JSONL);
        let wal_path = self.dir.join(EVENTS_WAL);
        let jsonl_len = fs::metadata(&jsonl_path)?.len();
        let wal_len = fs::metadata(&wal_path)?.len();
        if jsonl_len != self.jsonl_len || wal_len != self.wal_len {
            return Err(invalid(
                "log file length mismatch (recover with full persist)",
            ));
        }
        let jid = last_jsonl_event_id(&jsonl_path, jsonl_len)?;
        let wid = wal::last_event_id_at_end(&wal_path, wal_len, self.last_frame_len)?;
        if jid != self.last_id || wid != self.last_id {
            return Err(invalid(
                "log tail EventId mismatch (recover with full persist)",
            ));
        }
        Ok(())
    }
}

fn remember_appended(
    p: &mut Persister,
    runtime: &Runtime,
    new_events: &[Event],
    jsonl_path: &Path,
    wal_path: &Path,
) -> std::io::Result<()> {
    let last = new_events
        .last()
        .ok_or_else(|| invalid("append produced no events"))?;
    p.last_frame_len = wal::frame_bytes(last)?.len() as u64;
    p.last_id = last.id;
    for e in new_events.iter().rev() {
        if !matches!(e.op, Op::Define { .. }) {
            p.last_graph_id = Some(e.id);
            break;
        }
    }
    p.durable_count += new_events.len();
    p.durable_graph_count += new_events
        .iter()
        .filter(|e| !matches!(e.op, Op::Define { .. }))
        .count();
    p.dict_len = runtime.dictionary().strings().len();
    p.jsonl_len = fs::metadata(jsonl_path)?.len();
    p.wal_len = fs::metadata(wal_path)?.len();
    p.events_since_checkpoint += new_events.len();
    Ok(())
}

fn collect_new(p: &Persister, runtime: &Runtime) -> std::io::Result<Vec<Event>> {
    let strings = runtime.dictionary().strings();
    if strings.len() < p.dict_len {
        return Err(invalid("runtime dictionary shorter than durable dict"));
    }
    let encoded = encoded_log(runtime);
    if encoded.len() < p.durable_count && strings.len() == p.dict_len {
        return Err(invalid("runtime log shorter than durable_count"));
    }
    let graph: Vec<Event> = encoded
        .into_iter()
        .filter(|e| !matches!(e.op, Op::Define { .. }))
        .collect();
    if graph.len() < p.durable_graph_count {
        return Err(invalid("runtime graph shorter than durable graph"));
    }
    if p.durable_graph_count > 0 {
        let expected = p
            .last_graph_id
            .ok_or_else(|| invalid("durable graph tail missing last_graph_id"))?;
        if graph[p.durable_graph_count - 1].id != expected {
            return Err(invalid(
                "runtime event at durable graph tail has another id",
            ));
        }
    }
    let mut out = Vec::new();
    for name in &strings[p.dict_len..] {
        out.push(Event::stable_define(name.clone(), 0));
    }
    out.extend(graph[p.durable_graph_count..].iter().cloned());
    Ok(out)
}

fn last_jsonl_event(path: &Path, file_len: u64) -> std::io::Result<Event> {
    if file_len == 0 {
        return Err(invalid("empty events.jsonl"));
    }
    let mut f = File::open(path)?;
    f.seek(SeekFrom::Start(file_len - 1))?;
    let mut one = [0u8; 1];
    f.read_exact(&mut one)?;
    if one[0] != b'\n' {
        return Err(invalid("events.jsonl tail is not newline-terminated"));
    }
    let mut line_start = 0u64;
    let mut pos = file_len - 1;
    while pos > 0 {
        pos -= 1;
        f.seek(SeekFrom::Start(pos))?;
        f.read_exact(&mut one)?;
        if one[0] == b'\n' {
            line_start = pos + 1;
            break;
        }
    }
    let line_len = (file_len - 1).saturating_sub(line_start);
    f.seek(SeekFrom::Start(line_start))?;
    let mut buf = vec![0u8; line_len as usize];
    f.read_exact(&mut buf)?;
    let text = std::str::from_utf8(&buf)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    serde_json::from_str(text).map_err(json_err)
}

fn last_jsonl_event_id(path: &Path, file_len: u64) -> std::io::Result<EventId> {
    Ok(last_jsonl_event(path, file_len)?.id)
}

fn invalid(msg: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, msg)
}

fn injected_event_failpoint() -> std::io::Error {
    std::io::Error::new(
        std::io::ErrorKind::Other,
        "injected fault: append event failpoint",
    )
}

fn after_step(stop_after: Option<AppendStep>, step: AppendStep) -> std::io::Result<()> {
    if stop_after == Some(step) {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "injected fault: append failpoint",
        ))
    } else {
        Ok(())
    }
}

/// Test-visible count of `sync_data`/`sync_all` on log files (D-07).
#[doc(hidden)]
pub fn persist_log_sync_count() -> usize {
    wal::log_file_sync_count()
}

/// Reset the log-file sync counter.
#[doc(hidden)]
pub fn reset_persist_log_sync_count() {
    wal::reset_log_file_sync_count();
}

/// Test seam (D-08): `append` and return an injected I/O error after `stop_after`.
#[doc(hidden)]
pub fn append_with_failpoint(
    persister: &mut Persister,
    runtime: &Runtime,
    stop_after: AppendStep,
) -> std::io::Result<()> {
    persister.append_until(runtime, Some(stop_after), None)
}

/// Test seam (COMMIT-01): write the first `stop_after_events` of the pending batch,
/// then return an injected I/O error without updating persister counters.
#[doc(hidden)]
pub fn append_with_event_failpoint(
    persister: &mut Persister,
    runtime: &Runtime,
    stop_after_events: usize,
) -> std::io::Result<()> {
    persister.append_until(runtime, None, Some(stop_after_events))
}

/// Test seam (COMMIT-01): how many encoded events `append` would write.
#[doc(hidden)]
pub fn pending_append_len(persister: &Persister, runtime: &Runtime) -> std::io::Result<usize> {
    collect_new(persister, runtime).map(|v| v.len())
}

/// Test seam (D-08): `checkpoint` and return an injected I/O error after `stop_after`.
#[doc(hidden)]
pub fn checkpoint_with_failpoint(
    persister: &mut Persister,
    runtime: &Runtime,
    stop_after: AppendStep,
) -> std::io::Result<()> {
    persister.checkpoint_until(runtime, Some(stop_after))
}

/// Test seam: checkpoint jsonl-sync then abort snapshot replace before rename.
#[doc(hidden)]
pub fn abort_checkpoint_snapshot_before_rename(
    persister: &mut Persister,
    runtime: &Runtime,
) -> std::io::Result<()> {
    ABORT_CHECKPOINT_SNAPSHOT_RENAME.with(|c| c.set(true));
    let result = persister.checkpoint(runtime);
    ABORT_CHECKPOINT_SNAPSHOT_RENAME.with(|c| c.set(false));
    result
}
