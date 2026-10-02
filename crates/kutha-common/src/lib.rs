//! Shared encoding for the P0 heartbeat (ADR-011).
//! Intern map ≠ agent control dictionaries (ADR-050).

mod event;
mod intern;
mod rule;

pub use event::{
    blob_digest_matches, policy_version_hash, rule_definition_hash, sha256_hex, Event, EventId, Op,
    OutcomeDisposition, SupportPolarity, TermId, TimeScale, TransactionTime, Tt, ValidTime, Vt,
    WallTime, KUTHA_ACTION, KUTHA_ADMISSION, KUTHA_ALLOWS, KUTHA_POLICY, KUTHA_RESERVED_RELS,
    KUTHA_RULE, TRANSACTION_TIME_SCALE, VALID_TIME_SCALE,
};
pub use intern::TermDictionary;
pub use rule::{is_rule_ident, rule_body_hash, RuleAst, RuleAtom, RuleTerm};
