//! M093 S01 — CE #9: kutha-migrate CLI help/extra-arg contract.

use std::path::PathBuf;
use std::process::Command;

fn migrate_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_kutha-migrate"))
}

#[test]
fn kutha_migrate_help_exits_zero_on_stdout() {
    for flag in ["--help", "-h"] {
        let output = Command::new(migrate_bin())
            .arg(flag)
            .output()
            .expect("spawn");
        assert_eq!(
            output.status.code(),
            Some(0),
            "{flag} must exit 0, got {:?}",
            output.status.code()
        );
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("usage: kutha-migrate"),
            "{flag} usage missing on stdout: {stdout}"
        );
        assert!(
            stdout.contains("must already exist") || stdout.contains("existing"),
            "{flag} must document that store-dir must exist: {stdout}"
        );
        assert!(
            output.stderr.is_empty(),
            "{flag} must not print usage on stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn kutha_migrate_rejects_extra_positionals() {
    let output = Command::new(migrate_bin())
        .args(["/tmp/some-store", "extra"])
        .output()
        .expect("spawn");
    assert_eq!(output.status.code(), Some(2));
    let err = String::from_utf8_lossy(&output.stderr);
    assert!(
        err.contains("usage: kutha-migrate") || err.contains("extra"),
        "extra args must surface usage/reject: {err}"
    );
}
