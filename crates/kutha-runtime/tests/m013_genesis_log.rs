//! M013 S04 / Phase 25: genesis-log RED oracles (YAML HashSet fallback).
//! Constructors still load YAML; 25-02 wires genesis. Do not intern extra YAML names.

use kutha_common::Op;
use kutha_runtime::{store, Runtime, RuntimeError};
use std::sync::Mutex;

/// Serialize tests that read or mutate `KUTHA_RELATIONS_PATH` (process-global).
static RELATIONS_ENV: Mutex<()> = Mutex::new(());

struct RestoreRelationsPath(Option<String>);

impl Drop for RestoreRelationsPath {
    fn drop(&mut self) {
        match &self.0 {
            Some(v) => std::env::set_var("KUTHA_RELATIONS_PATH", v),
            None => std::env::remove_var("KUTHA_RELATIONS_PATH"),
        }
    }
}

#[test]
fn retract_seeded_related_to_fails_closed_after() {
    let _env = RELATIONS_ENV.lock().unwrap_or_else(|e| e.into_inner());
    let mut rt = Runtime::default();
    let cut = rt.log().len();

    let retract_id = rt.log().iter().find_map(|e| match &e.op {
        Op::AllowRelation { name, .. } if name == "relatedTo" => Some(e.id),
        _ => None,
    });
    if let Some(id) = retract_id {
        rt.emit(Op::Retract { event_id: id }).unwrap();
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
    std::env::set_var("KUTHA_RELATIONS_PATH", &missing);

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
