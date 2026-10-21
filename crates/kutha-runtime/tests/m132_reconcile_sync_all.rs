//! M132 S01 / Phase 206 — after set_len in reconcile_uncommitted_tip, size
//! changes must be sync_all'd (CE residual: sync_data may not persist length).

fn reconcile_body() -> &'static str {
    let src = include_str!("../src/persist_append.rs");
    let start = src
        .find("fn reconcile_uncommitted_tip(")
        .expect("reconcile_uncommitted_tip");
    let rest = &src[start..];
    let end = rest[1..]
        .find("\n    fn ")
        .map(|i| i + 1)
        .unwrap_or(rest.len());
    &rest[..end]
}

#[test]
fn reconcile_truncation_uses_sync_all_after_set_len() {
    let body = reconcile_body();
    let mut set_lens = 0usize;
    let mut sync_alls = 0usize;
    let mut rest = body;
    while let Some(i) = rest.find("set_len(") {
        set_lens += 1;
        let after = &rest[i + "set_len(".len()..];
        let semi = after.find(';').expect("set_len must end with ;");
        let tail = after[semi + 1..].trim_start();
        assert!(
            !tail.starts_with("f.sync_data()"),
            "set_len must not be followed by sync_data (length may not persist)"
        );
        assert!(
            tail.starts_with("f.sync_all()"),
            "set_len must be followed by sync_all; got {:?}",
            tail.lines().next()
        );
        sync_alls += 1;
        rest = &after[semi + 1..];
    }
    assert!(set_lens >= 3, "expected ≥3 set_len sites, got {set_lens}");
    assert_eq!(sync_alls, set_lens);
}
