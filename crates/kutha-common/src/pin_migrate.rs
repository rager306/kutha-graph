//! Explicit pin-scheme migrate (M071). Not a silent remap of [`crate::rule_definition_hash`].
//!
//! Main-era UTF-8 domain pins for rule idents stay fail-closed under v1 admit until
//! [`remap_behavior_rule_version`] / directory `migrate_pin_wire` rewrites them.
//! See `docs/process/pin-compat.md`.

use crate::{
    is_rule_ident, rule_body_hash, rule_definition_hash, Event, Op, RuleAst,
};

/// Main-era / pre–RULE-01 pin for a rule **ident**: domain `kutha-rule-def` ‖ UTF-8 name.
///
/// Returns `None` when `name` is not a rule ident (non-idents already use domain+UTF-8 under v1).
pub fn legacy_ident_rule_pin(name: &str) -> Option<String> {
    if !is_rule_ident(name) {
        return None;
    }
    Some(rule_body_hash(name.as_bytes()))
}

/// If `pin` is the legacy UTF-8 ident pin for `name`, return pin-scheme v1 Named pin.
pub fn remap_behavior_rule_version(name: &str, pin: &str) -> Option<String> {
    let legacy = legacy_ident_rule_pin(name)?;
    let v1 = RuleAst::named(name)?.hash();
    debug_assert_eq!(v1, rule_definition_hash(name));
    if pin == legacy && pin != v1 {
        Some(v1)
    } else {
        None
    }
}

/// Remap [`Op::Behavior`] `rule_version` when it is a legacy ident pin.
/// Other ops are returned unchanged (`changed = false`).
pub fn remap_event_pins(mut event: Event) -> (Event, bool) {
    match &mut event.op {
        Op::Behavior {
            name,
            rule_version,
            ..
        } => {
            if let Some(new_pin) = remap_behavior_rule_version(name, rule_version) {
                *rule_version = new_pin;
                return (event, true);
            }
        }
        _ => {}
    }
    (event, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EventId;

    #[test]
    fn remap_derive_pq_legacy_utf8_to_v1() {
        let legacy = legacy_ident_rule_pin("derive_pq").expect("ident");
        assert_eq!(
            legacy,
            "7ce5d5ad5ca8d5148305d84d3eee3f1356540ad44da56fac9fdde748603436db"
        );
        let v1 = remap_behavior_rule_version("derive_pq", &legacy).expect("remap");
        assert_eq!(
            v1,
            "2dca8c1394bfd203753efd7c991030c23719b41b285aad78ef061103b316596d"
        );
        assert_eq!(v1, rule_definition_hash("derive_pq"));
        assert!(remap_behavior_rule_version("derive_pq", &v1).is_none());
    }

    #[test]
    fn nonident_pin_is_not_remapped() {
        let name = "not a rule ident!!!";
        assert!(legacy_ident_rule_pin(name).is_none());
        let pin = rule_definition_hash(name);
        assert!(remap_behavior_rule_version(name, &pin).is_none());
    }

    #[test]
    fn remap_event_pins_rewrites_behavior_only() {
        let legacy = legacy_ident_rule_pin("derive_pq").unwrap();
        let e = Event {
            id: EventId::nil(),
            op: Op::Behavior {
                name: "derive_pq".into(),
                caused_by: EventId::nil(),
                rule_version: legacy.clone(),
                subject: 0,
                relation: 0,
                object: 0,
                valid_from: 0,
                valid_to: None,
            },
            ingested_at: 1,
            object_ids: vec![0, 0, 0],
        };
        let (out, changed) = remap_event_pins(e);
        assert!(changed);
        match out.op {
            Op::Behavior { rule_version, .. } => {
                assert_eq!(rule_version, rule_definition_hash("derive_pq"));
                assert_ne!(rule_version, legacy);
            }
            other => panic!("expected Behavior, got {other:?}"),
        }
    }
}
