//! Shared encoding for the P0 heartbeat (ADR-011).
//! Intern map ≠ agent control dictionaries (ADR-050).

mod event;
mod intern;
mod op_migrate;
mod pin_migrate;
mod rule;

pub use event::{
    Event, EventId, KUTHA_ACTION, KUTHA_ADMISSION, KUTHA_ALLOWS, KUTHA_POLICY, KUTHA_RESERVED_RELS,
    KUTHA_RULE, Op, OutcomeDisposition, SupportPolarity, TRANSACTION_TIME_SCALE, TermId, TimeScale,
    TransactionTime, Tt, VALID_TIME_SCALE, ValidTime, Vt, WallTime, blob_digest_matches,
    policy_version_hash, rule_definition_hash, sha256_hex,
};
pub use intern::TermDictionary;
pub use op_migrate::{
    allow_relation_to_assert, pin_policy_to_ops, record_action_to_assert,
    record_admission_to_assert, register_rule_to_ops, remap_legacy_event_value, remap_legacy_events,
    remap_legacy_op_value, remap_legacy_ops, rewrite_event_json_line, OpMigrateError,
};
pub use pin_migrate::{
    legacy_ident_rule_pin, remap_behavior_rule_version, remap_event_pins,
};
pub use rule::{RuleAst, RuleAtom, RuleTerm, is_rule_ident, rule_body_hash};
