//! Shared log-backed fixtures for integration tests (D-04). Not a `[[test]]` crate.
#![allow(dead_code)]

pub mod sem;

use kutha_common::Op;
use kutha_runtime::Runtime;
use std::ffi::OsStr;
use std::fs;
use std::ops::Deref;
use std::path::{Path, PathBuf};

/// RAII persist directory under repo-local gitignored `tmp/` (AGENTS.md).
/// Removes the directory on drop, including panic unwind.
pub struct PersistScratch {
    path: PathBuf,
}

impl PersistScratch {
    pub fn new(label: &str) -> Self {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("tmp")
            .join(format!("{label}-{nanos}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).unwrap();
        Self { path }
    }
}

impl Deref for PersistScratch {
    type Target = Path;
    fn deref(&self) -> &Path {
        &self.path
    }
}

impl AsRef<Path> for PersistScratch {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

impl AsRef<OsStr> for PersistScratch {
    fn as_ref(&self) -> &OsStr {
        self.path.as_os_str()
    }
}

impl Drop for PersistScratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

const W1: u64 = 2014;
const W2: u64 = 2020;
const VF: u64 = 2010;

pub fn seed_statute_between_anchors(
    rt: &mut Runtime,
) -> (
    kutha_common::TermId,
    kutha_common::TermId,
    kutha_common::TermId,
) {
    let article = rt.intern("art-12-speed");
    let in_force = rt.intern("inForceAs");
    let limit_50 = rt.intern("50-kmh");
    rt.emit_tt_anchor(W1).unwrap();
    rt.emit(Op::Assert {
        subject: article,
        relation: in_force,
        object: limit_50,
        valid_from: VF,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit_tt_anchor(W2).unwrap();
    (article, in_force, limit_50)
}
