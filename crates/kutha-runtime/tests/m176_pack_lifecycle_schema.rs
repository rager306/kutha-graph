//! M176 phase 259: D021 lifecycle log schema, admission, and compatibility.

use kutha_common::{Event, EventId, Op};
use kutha_runtime::{Runtime, RuntimeError};

fn install(pack_id: &str, version: &str) -> Op {
    Op::PackInstall {
        pack_id: pack_id.to_string(),
        name: format!("pack-{pack_id}"),
        version: version.to_string(),
        manifest_digest: None,
    }
}

fn activate(pack_id: &str, version: &str) -> Op {
    Op::PackActivate {
        pack_id: pack_id.to_string(),
        version: version.to_string(),
    }
}

fn retire(pack_id: &str, version: &str) -> Op {
    Op::PackRetire {
        pack_id: pack_id.to_string(),
        version: version.to_string(),
    }
}

#[test]
fn lifecycle_ops_are_meta_and_do_not_create_graph_facts() {
    let mut rt = Runtime::default();
    let before_fp = rt.fold().fingerprint();
    let before_facts = rt.fold().facts().len();
    rt.emit_pack_install("core", "pack-core", "1.0.0")
        .expect("install");
    rt.emit_pack_activate("core", "1.0.0").expect("activate");
    rt.emit_pack_retire("core", "1.0.0").expect("retire");
    assert_eq!(rt.fold().facts().len(), before_facts);
    assert_eq!(rt.fold().fingerprint(), before_fp);
    let tail = &rt.log().as_slice()[rt.log().len() - 3..];
    assert!(tail.iter().all(|event| matches!(
        event.op,
        Op::PackInstall { .. } | Op::PackActivate { .. } | Op::PackRetire { .. }
    )));
}

#[test]
fn lifecycle_ops_reject_invalid_ordering() {
    let mut rt = Runtime::default();
    assert!(matches!(
        rt.emit_pack_activate("missing", "1.0.0"),
        Err(RuntimeError::PackLifecycleRejected { .. })
    ));
    rt.emit_pack_install("missing", "pack-missing", "1.0.0")
        .expect("install");
    assert!(matches!(
        rt.emit_pack_install("missing", "pack-missing", "1.1.0"),
        Err(RuntimeError::PackLifecycleRejected { .. })
    ));
    assert!(matches!(
        rt.emit_pack_retire("missing", "9.9.9"),
        Err(RuntimeError::PackLifecycleRejected { .. })
    ));
    rt.emit_pack_activate("missing", "1.0.0").expect("activate");
    assert!(matches!(
        rt.emit_pack_activate("missing", "1.0.0"),
        Err(RuntimeError::PackLifecycleRejected { .. })
    ));
    rt.emit_pack_retire("missing", "1.0.0").expect("retire");
    assert!(matches!(
        rt.emit_pack_retire("missing", "1.0.0"),
        Err(RuntimeError::PackLifecycleRejected { .. })
    ));
}

#[test]
fn lifecycle_ops_survive_serde_roundtrip() {
    for op in [
        install("serde", "1.2.3"),
        activate("serde", "1.2.3"),
        retire("serde", "1.2.3"),
    ] {
        let event = Event::new(op, 42);
        let raw = serde_json::to_vec(&event).expect("serialize");
        let decoded: Event = serde_json::from_slice(&raw).expect("deserialize");
        assert_eq!(decoded, event);
    }
}

#[test]
fn lifecycle_rejects_empty_identifiers_and_user_forgery() {
    let mut rt = Runtime::default();
    assert!(matches!(
        rt.emit_pack_install("", "", ""),
        Err(RuntimeError::PackLifecycleRejected { .. })
    ));
    assert!(matches!(
        rt.emit_pack_install("core", "", "1.0.0"),
        Err(RuntimeError::PackLifecycleRejected { .. })
    ));
    assert!(matches!(
        rt.emit(install("core", "1.0.0")),
        Err(RuntimeError::MetaOpRejected)
    ));
}

#[test]
fn pack_digest_separates_length_collisions() {
    let id = EventId::nil();
    // Use explicit ops for the collision shape; install() prefixes the name.
    let a = Event {
        id,
        op: Op::PackInstall {
            pack_id: "ab".to_string(),
            name: "c".to_string(),
            version: "1".to_string(),
            manifest_digest: None,
        },
        ingested_at: 1,
        object_ids: vec![],
    };
    let b = Event {
        id,
        op: Op::PackInstall {
            pack_id: "a".to_string(),
            name: "bc".to_string(),
            version: "1".to_string(),
            manifest_digest: None,
        },
        ingested_at: 1,
        object_ids: vec![],
    };
    assert_ne!(a.digest_bytes(), b.digest_bytes());

    let activate = |pack_id: &str, version: &str| Event {
        id,
        op: Op::PackActivate {
            pack_id: pack_id.to_string(),
            version: version.to_string(),
        },
        ingested_at: 1,
        object_ids: vec![],
    };
    assert_ne!(
        activate("ab", "c").digest_bytes(),
        activate("a", "bc").digest_bytes()
    );
    let retire = |pack_id: &str, version: &str| Event {
        id,
        op: Op::PackRetire {
            pack_id: pack_id.to_string(),
            version: version.to_string(),
        },
        ingested_at: 1,
        object_ids: vec![],
    };
    assert_ne!(
        retire("ab", "c").digest_bytes(),
        retire("a", "bc").digest_bytes()
    );
}
