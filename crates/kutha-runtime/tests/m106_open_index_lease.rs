//! Open must restore the snapshot fold-index lease, not only the mode flag.
//! A Lazy snapshot opened while the env says eager must still scan. An Eager
//! snapshot opened while the env says lazy must keep the warm range.

use kutha_common::{Op, ValidTime};
use kutha_runtime::{store, FoldIndexMode, Runtime};
use std::fs;
use std::sync::Mutex;

static FOLD_INDEX_ENV: Mutex<()> = Mutex::new(());

struct RestoreEnv {
    key: &'static str,
    prev: Option<String>,
}

impl Drop for RestoreEnv {
    fn drop(&mut self) {
        match &self.prev {
            Some(v) => unsafe { std::env::set_var(self.key, v) },
            None => unsafe { std::env::remove_var(self.key) },
        }
    }
}

fn set_env(key: &'static str, value: &str) -> RestoreEnv {
    let prev = std::env::var(key).ok();
    unsafe { std::env::set_var(key, value) };
    RestoreEnv { key, prev }
}

fn fill_two_cohorts(rt: &mut Runtime) {
    let rel = rt.intern("relatedTo");
    let obj = rt.intern("o");
    for i in 0..20 {
        let s = rt.intern(&format!("early{i}"));
        rt.emit(Op::Assert {
            subject: s,
            relation: rel,
            object: obj,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
    }
    for i in 0..20 {
        let s = rt.intern(&format!("late{i}"));
        rt.emit(Op::Assert {
            subject: s,
            relation: rel,
            object: obj,
            valid_from: 2030,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
    }
}

fn as_of_examines(rt: &Runtime) -> u64 {
    rt.fold().reset_hot_examine_count();
    let _ = rt.fold().as_of(ValidTime::from(2010));
    rt.fold().hot_examine_count()
}

#[test]
fn open_keeps_lazy_scan_when_env_is_eager() {
    let _lock = FOLD_INDEX_ENV.lock().unwrap_or_else(|e| e.into_inner());
    let mut rt = Runtime::with_index_mode(32, FoldIndexMode::Lazy);
    fill_two_cohorts(&mut rt);
    let scanned = as_of_examines(&rt);
    let dir = std::env::temp_dir().join(format!("kutha-m106-lazy-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    store::persist(&rt, &dir).unwrap();

    let _restore = set_env("KUTHA_FOLD_INDEX_MODE", "eager");
    let opened = store::open(&dir).expect("open lazy snapshot");
    assert_eq!(opened.index_mode(), FoldIndexMode::Lazy);
    let after = as_of_examines(&opened);
    assert_eq!(
        after, scanned,
        "Lazy snapshot must keep the cold scan after open; env eager must not warm the lease"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn open_keeps_eager_range_when_env_is_lazy() {
    let _lock = FOLD_INDEX_ENV.lock().unwrap_or_else(|e| e.into_inner());
    let mut rt = Runtime::with_index_mode(32, FoldIndexMode::Eager);
    fill_two_cohorts(&mut rt);
    let ranged = as_of_examines(&rt);
    let dir = std::env::temp_dir().join(format!("kutha-m106-eager-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    store::persist(&rt, &dir).unwrap();

    let _restore = set_env("KUTHA_FOLD_INDEX_MODE", "lazy");
    let opened = store::open(&dir).expect("open eager snapshot");
    assert_eq!(opened.index_mode(), FoldIndexMode::Eager);
    let after = as_of_examines(&opened);
    assert_eq!(
        after, ranged,
        "Eager snapshot must keep the warm range after open; env lazy must not force a full scan"
    );
    let _ = fs::remove_dir_all(&dir);
}
