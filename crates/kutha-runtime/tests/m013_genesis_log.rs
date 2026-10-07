//! M013 S04 / Phase 25: genesis-log oracles (YAML only on new-log create).
//! Do not intern extra YAML names. FF6 / m012_allowlist_facts asserts stay unchanged.

use kutha_common::{KUTHA_ALLOWS, Op};
use kutha_runtime::{Runtime, RuntimeError, store};
use std::sync::Mutex;

/// Serialize tests that read or mutate `KUTHA_RELATIONS_PATH` (process-global).
static RELATIONS_ENV: Mutex<()> = Mutex::new(());

struct RestoreRelationsPath(Option<String>);

impl Drop for RestoreRelationsPath {
    fn drop(&mut self) {
        match &self.0 {
            // SAFETY: test-only env mutation; callers serialize through the file's env mutex or run before other threads read the variable.
            Some(v) => unsafe { std::env::set_var("KUTHA_RELATIONS_PATH", v) },
            // SAFETY: test-only env mutation; callers serialize through the file's env mutex or run before other threads read the variable.
            None => unsafe { std::env::remove_var("KUTHA_RELATIONS_PATH") },
        }
    }
}

#[test]
fn retract_seeded_related_to_fails_closed_after() {
    let _env = RELATIONS_ENV.lock().unwrap_or_else(|e| e.into_inner());
    let mut rt = Runtime::default();
    let cut = rt.log().len();

    let retract_id = rt.log().iter().find_map(|e| match &e.op {
        Op::Assert {
            delivery_key: Some(name),
            relation,
            ..
        } if name == "relatedTo" && rt.dictionary().lookup(*relation) == Some(KUTHA_ALLOWS) => {
            Some(e.id)
        }
        _ => None,
    });
    if let Some(id) = retract_id {
        rt.emit_retract_dict(id).unwrap();
    }

    let relation = rt.intern("relatedTo");
    let subject = rt.intern("alice-seed");
    let object = rt.intern("bob-seed");
    let n = rt.log().len();

    let err = rt
        .emit(Op::Assert {
            subject,
            relation,
            object,
            valid_from: 2017,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap_err();
    assert!(
        matches!(err, RuntimeError::UnknownRelation { ref name } if name == "relatedTo"),
        "{err:?}"
    );
    assert_eq!(n, rt.log().len(), "fail-closed: log must not grow");

    let mut prefix = rt.fork_at(cut);
    let rel = prefix.intern("relatedTo");
    let s = prefix.intern("alice-seed");
    let o = prefix.intern("bob-seed");
    prefix
        .emit(Op::Assert {
            subject: s,
            relation: rel,
            object: o,
            valid_from: 2017,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
}

#[test]
fn open_after_relations_path_missing_still_emits() {
    let _env = RELATIONS_ENV.lock().unwrap_or_else(|e| e.into_inner());
    let mut rt = Runtime::default();
    let article = rt.intern("art-12-speed");
    let in_force = rt.intern("inForceAs");
    let limit = rt.intern("50-kmh");
    rt.emit(Op::Assert {
        subject: article,
        relation: in_force,
        object: limit,
        valid_from: 2017,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();

    let dir = std::env::temp_dir().join(format!(
        "kutha-m013-s04-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    store::persist(&rt, &dir).unwrap();

    let missing = dir.join("missing-relations.yaml");
    let prev = std::env::var("KUTHA_RELATIONS_PATH").ok();
    let _restore = RestoreRelationsPath(prev);
    // SAFETY: test-only env mutation; callers serialize through the file's env mutex or run before other threads read the variable.
    unsafe { std::env::set_var("KUTHA_RELATIONS_PATH", &missing) };

    let mut opened = store::open(&dir).unwrap();
    let article = opened.intern("art-12-speed");
    let in_force = opened.intern("inForceAs");
    let limit = opened.intern("50-kmh");
    opened
        .emit(Op::Assert {
            subject: article,
            relation: in_force,
            object: limit,
            valid_from: 2017,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .expect("open must still emit inForceAs when the YAML path is missing");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn stripped_genesis_rejects_product_relation() {
    let _env = RELATIONS_ENV.lock().unwrap_or_else(|e| e.into_inner());
    let rt = Runtime::default();
    let dict = rt.dictionary().strings().to_vec();
    let events: Vec<_> = rt
        .log()
        .as_slice()
        .iter()
        .filter(|e| {
            !matches!(
                &e.op,
                Op::Assert {
                    relation,
                    delivery_key: Some(_),
                    ..
                } if rt.dictionary().lookup(*relation) == Some(KUTHA_ALLOWS)
            )
        })
        .cloned()
        .collect();
    let mut stripped = Runtime::from_dict_and_events(dict, events, rt.max_cascade).unwrap();
    let relation = stripped.intern("relatedTo");
    let subject = stripped.intern("alice-stripped");
    let object = stripped.intern("bob-stripped");
    let n = stripped.log().len();
    let err = stripped
        .emit(Op::Assert {
            subject,
            relation,
            object,
            valid_from: 2017,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap_err();
    assert!(
        matches!(err, RuntimeError::UnknownRelation { ref name } if name == "relatedTo"),
        "{err:?}"
    );
    assert_eq!(n, stripped.log().len(), "fail-closed: log must not grow");
}

#[test]
fn persist_open_replay_keeps_genesis() {
    let _env = RELATIONS_ENV.lock().unwrap_or_else(|e| e.into_inner());
    let rt = Runtime::default();
    const YAML_NAMES: [&str; 7] = [
        "knows",
        "knownBy",
        "relatedTo",
        "inForceAs",
        "runStatus",
        "observed",
        "processAllows",
    ];
    for name in YAML_NAMES {
        assert!(
            rt.log().iter().any(|e| matches!(
                &e.op,
                Op::Assert {
                    delivery_key: Some(n),
                    relation,
                    ..
                } if n == name && rt.dictionary().lookup(*relation) == Some(KUTHA_ALLOWS)
            )),
            "genesis missing {name}"
        );
    }

    let dir = std::env::temp_dir().join(format!(
        "kutha-m013-s04-keep-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    store::persist(&rt, &dir).unwrap();
    let opened = store::open(&dir).unwrap();
    opened.replay_check().unwrap();
    for name in YAML_NAMES {
        assert!(
            opened.log().iter().any(|e| matches!(
                &e.op,
                Op::Assert {
                    delivery_key: Some(n),
                    relation,
                    ..
                } if n == name && opened.dictionary().lookup(*relation) == Some(KUTHA_ALLOWS)
            )),
            "opened log missing genesis {name}"
        );
    }
    let expected_prov = opened.provenance_fingerprint();
    opened.provenance_check(expected_prov).unwrap();

    let mut tampered_events = opened.log().as_slice().to_vec();
    let mut flipped = false;
    for e in &mut tampered_events {
        if let Op::Assert {
            delivery_key: Some(name),
            relation,
            ..
        } = &mut e.op
        {
            if opened.dictionary().lookup(*relation) == Some(KUTHA_ALLOWS) {
                *name = format!("{name}-tampered");
                flipped = true;
                break;
            }
        }
    }
    assert!(flipped, "expected a genesis AllowRelation to tamper");
    let tampered = Runtime::from_dict_and_events(
        opened.dictionary().strings().to_vec(),
        tampered_events,
        opened.max_cascade,
    )
    .unwrap();
    let err = tampered.provenance_check(expected_prov).unwrap_err();
    assert!(
        matches!(err, RuntimeError::ProvenanceMismatch { .. }),
        "{err:?}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}
