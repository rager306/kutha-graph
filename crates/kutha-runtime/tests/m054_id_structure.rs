//! M054 S05 / Phase 121 — shrink SHA reuse + SEM helper split (CE #20 #12).

mod common;

use common::sem::save_shrunk_to;
use kutha_common::{sha256_hex, Event};
use std::path::PathBuf;

fn tmp_dir(label: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .join(format!(
            "m054-id-structure-{label}-{}-{}",
            std::process::id(),
            nanos
        ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// #20 — shrink corpus filename sha12 is canonical SHA-256, not a local FNV.
#[test]
fn shrink_corpus_filename_uses_sha256_hex() {
    let dir = tmp_dir("shrink");
    let events: Vec<Event> = Vec::new();
    let path = save_shrunk_to(&dir, 7, &events);
    let bytes = std::fs::read(&path).expect("shrunk bytes");
    let digest = sha256_hex(&bytes);
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .expect("utf8 name");
    let sha12 = &digest[..12];
    assert!(
        name.contains(sha12),
        "save_shrunk_to must name seed-{{seed}}-{{sha12}}.json from sha256_hex, got {name} want {sha12}"
    );
    assert_eq!(name, format!("seed-7-{sha12}.json"));
    let _ = std::fs::remove_dir_all(&dir);
}

/// #12 — SEM helper is generator / oracles / shrink / persistence, not a flat catch-all.
#[test]
fn sem_helper_is_split_into_named_modules() {
    let common = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/common");
    let sem = common.join("sem");
    assert!(
        !common.join("sem.rs").is_file(),
        "flat tests/common/sem.rs must be replaced by tests/common/sem/"
    );
    for name in ["mod.rs", "generator.rs", "oracles.rs", "shrink.rs", "persistence.rs"] {
        let path = sem.join(name);
        assert!(path.is_file(), "missing SEM module {}", path.display());
    }
}
