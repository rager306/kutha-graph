//! Explicit Op wire migrate (M062). Not a serde alias on [`Op`].
//!
//! `store::open` / `Op` deserialize stay fail-open for removed variants.
//! Callers pass an intern callback so TermIds are never guessed.

use crate::{Event, EventId, Op, TermId, KUTHA_ALLOWS};
use serde::Deserialize;

/// Why a legacy Op JSON could not be remapped.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OpMigrateError {
    /// JSON root was not a single-key object (external tag).
    NotTaggedObject,
    /// Variant not handled in this migrate wave (still deferred).
    DeferredVariant(&'static str),
    /// Unknown / unsupported tag.
    UnknownVariant(String),
    /// Tagged body failed shape checks.
    InvalidShape(&'static str),
    /// AllowRelation name was empty.
    EmptyName,
}

impl std::fmt::Display for OpMigrateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotTaggedObject => write!(f, "legacy Op JSON must be a single-key object"),
            Self::DeferredVariant(v) => write!(f, "legacy Op variant {v} migrate not leased yet"),
            Self::UnknownVariant(v) => write!(f, "unknown legacy Op variant {v}"),
            Self::InvalidShape(msg) => write!(f, "invalid legacy Op shape: {msg}"),
            Self::EmptyName => write!(f, "AllowRelation name must be non-empty"),
        }
    }
}

impl std::error::Error for OpMigrateError {}

#[derive(Deserialize)]
struct AllowRelationBody {
    name: String,
    valid_from: u64,
    #[serde(default)]
    valid_to: Option<u64>,
}

/// Build the modern `kutha:allows` Assert (same TermId shape as runtime `emit_allow`).
pub fn allow_relation_to_assert(
    name: impl Into<String>,
    valid_from: u64,
    valid_to: Option<u64>,
    mut intern: impl FnMut(&str) -> TermId,
) -> Result<Op, OpMigrateError> {
    let name = name.into();
    if name.is_empty() {
        return Err(OpMigrateError::EmptyName);
    }
    let subject = intern(&name);
    let relation = intern(KUTHA_ALLOWS);
    Ok(Op::Assert {
        subject,
        relation,
        object: subject,
        valid_from,
        valid_to,
        claim: None,
        delivery_key: Some(name),
        polarity: None,
    })
}

/// Remap one legacy dictionary meta-op JSON value into a modern [`Op`].
///
/// Leased in M062-S01: `AllowRelation` only. Other four meta-ops return
/// [`OpMigrateError::DeferredVariant`]. Does not touch `Op`'s serde.
pub fn remap_legacy_op_value(
    value: &serde_json::Value,
    intern: impl FnMut(&str) -> TermId,
) -> Result<Op, OpMigrateError> {
    let obj = value
        .as_object()
        .ok_or(OpMigrateError::NotTaggedObject)?;
    if obj.len() != 1 {
        return Err(OpMigrateError::NotTaggedObject);
    }
    let (tag, body) = obj.iter().next().expect("len == 1");
    match tag.as_str() {
        "AllowRelation" => {
            let parsed: AllowRelationBody = serde_json::from_value(body.clone())
                .map_err(|_| OpMigrateError::InvalidShape("AllowRelation body"))?;
            allow_relation_to_assert(parsed.name, parsed.valid_from, parsed.valid_to, intern)
        }
        "RegisterRule" => Err(OpMigrateError::DeferredVariant("RegisterRule")),
        "PinPolicy" => Err(OpMigrateError::DeferredVariant("PinPolicy")),
        "RecordAdmission" => Err(OpMigrateError::DeferredVariant("RecordAdmission")),
        "RecordAction" => Err(OpMigrateError::DeferredVariant("RecordAction")),
        other => Err(OpMigrateError::UnknownVariant(other.to_string())),
    }
}

/// Remap a legacy Event JSON whose `op` is a removed meta-op tag.
/// Recomputes `object_ids` from the remapped op (legacy ids are not trusted).
pub fn remap_legacy_event_value(
    value: &serde_json::Value,
    intern: impl FnMut(&str) -> TermId,
) -> Result<Event, OpMigrateError> {
    #[derive(Deserialize)]
    struct RawEvent {
        id: EventId,
        op: serde_json::Value,
        ingested_at: u64,
    }
    let raw: RawEvent = serde_json::from_value(value.clone())
        .map_err(|_| OpMigrateError::InvalidShape("Event envelope"))?;
    let op = remap_legacy_op_value(&raw.op, intern)?;
    let mut event = Event::new(op, raw.ingested_at);
    event.id = raw.id;
    Ok(event)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TermDictionary;

    #[test]
    fn allow_relation_remaps_like_emit_allow() {
        let mut dict = TermDictionary::default();
        let raw = serde_json::json!({
            "AllowRelation": {
                "name": "relatedTo",
                "valid_from": 0,
                "valid_to": null
            }
        });
        let op = remap_legacy_op_value(&raw, |s| dict.intern(s)).expect("remap");
        let Op::Assert {
            subject,
            relation,
            object,
            valid_from,
            valid_to,
            delivery_key,
            ..
        } = op
        else {
            panic!("expected Assert, got {op:?}");
        };
        assert_eq!(subject, object);
        assert_eq!(dict.lookup(subject), Some("relatedTo"));
        assert_eq!(dict.lookup(relation), Some(KUTHA_ALLOWS));
        assert_eq!(valid_from, 0);
        assert_eq!(valid_to, None);
        assert_eq!(delivery_key.as_deref(), Some("relatedTo"));
    }

    #[test]
    fn empty_allow_name_is_rejected() {
        let mut dict = TermDictionary::default();
        let raw = serde_json::json!({
            "AllowRelation": { "name": "", "valid_from": 0, "valid_to": null }
        });
        let err = remap_legacy_op_value(&raw, |s| dict.intern(s)).unwrap_err();
        assert_eq!(err, OpMigrateError::EmptyName);
    }

    #[test]
    fn register_rule_is_deferred() {
        let mut dict = TermDictionary::default();
        let raw = serde_json::json!({
            "RegisterRule": { "definition": "R", "valid_from": 0, "valid_to": null }
        });
        let err = remap_legacy_op_value(&raw, |s| dict.intern(s)).unwrap_err();
        assert_eq!(err, OpMigrateError::DeferredVariant("RegisterRule"));
    }

    #[test]
    fn op_serde_still_rejects_allow_relation() {
        let raw = r#"{"AllowRelation":{"name":"relatedTo","valid_from":0,"valid_to":null}}"#;
        assert!(
            serde_json::from_str::<Op>(raw).is_err(),
            "serde aliases must stay forbidden"
        );
    }
}
