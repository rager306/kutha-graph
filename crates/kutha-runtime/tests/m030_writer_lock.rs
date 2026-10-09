//! M030 S01 / Phase 90 — non-poison append errors must keep kutha.lock
//! (CE review P1 #2). Fail-closed InvalidData still poisons and releases so a
//! heal writer can attach (see m019 poison oracle).

use kutha_common::Op;
use kutha_runtime::{Runtime, store};
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

fn persist_dir(label: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .join(format!(
            "m030-writer-lock-{label}-{}-{}",
            std::process::id(),
            nanos
        ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn seed_runtime() -> Runtime {
    let mut rt = Runtime::default();
    let s = rt.intern("m030-s");
    let r = rt.intern("relatedTo");
    let o = rt.intern("m030-o");
    rt.emit(Op::Assert {
        subject: s,
        relation: r,
        object: o,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit_tt_anchor(2015).unwrap();
    rt
}

fn assert_writer_lock_error(err: &std::io::Error) {
    assert_eq!(
        err.kind(),
        ErrorKind::WouldBlock,
        "second writer must fail closed with WouldBlock; got {err}"
    );
    assert!(
        err.to_string().contains(store::STORE_LOCK_REL),
        "error must name the lock file; got {err}"
    );
}

#[test]
fn failpoint_append_keeps_lock_and_writer_usable() {
    let mut rt = seed_runtime();
    let dir = persist_dir("fp-append");
    store::persist(&rt, &dir).unwrap();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    rt.emit_tt_anchor(2021).unwrap();
    let err = store::append_with_failpoint(&mut p, &rt, store::AppendStep::WalSync)
        .expect_err("failpoint must inject");
    assert_eq!(err.kind(), ErrorKind::Other);

    let opened = store::open(&dir).expect("reader open");
    let second = match store::Persister::attach(&opened, &dir) {
        Ok(_) => panic!(
            "non-poison failpoint must keep kutha.lock so a second writer cannot attach"
        ),
        Err(e) => e,
    };
    assert_writer_lock_error(&second);

    // Same Persister stays usable (not poisoned) and still holds the lock.
    p.append(&rt).expect("retry append after non-poison failpoint");
    let after = store::open(&dir).unwrap();
    assert!(
        after.log().as_slice().len() > opened.log().as_slice().len(),
        "retry append must land new events"
    );
    drop(p);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn failpoint_then_checkpoint_still_holds_lock() {
    let mut rt = seed_runtime();
    let dir = persist_dir("fp-then-ckpt");
    store::persist(&rt, &dir).unwrap();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    rt.emit_tt_anchor(2021).unwrap();
    store::append_with_failpoint(&mut p, &rt, store::AppendStep::WalWrite)
        .expect_err("failpoint must inject");
    p.append(&rt).expect("retry append");
    p.checkpoint(&rt).expect("checkpoint after non-poison recovery");

    let opened = store::open(&dir).unwrap();
    let second = match store::Persister::attach(&opened, &dir) {
        Ok(_) => panic!("writer must still hold kutha.lock after checkpoint"),
        Err(e) => e,
    };
    assert_writer_lock_error(&second);
    drop(p);
    let _ = fs::remove_dir_all(&dir);
}
