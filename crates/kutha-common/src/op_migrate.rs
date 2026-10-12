//! Explicit Op wire migrate (M062+). Not a serde alias on [`Op`].
//!
//! `store::open` / `Op` deserialize stay fail-open for removed variants.
//! Callers pass an intern callback so TermIds are never guessed.

use crate::{
    policy_version_hash, Event, EventId, Op, RuleAst, TermId, KUTHA_ALLOWS, KUTHA_POLICY,
    KUTHA_RULE,
};
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
    /// RegisterRule definition is not a valid named rule ident.
    InvalidRuleDefinition,
    /// PinPolicy definition was empty.
    EmptyPolicyDefinition,
    /// Caller asked for a single Op/Event but remap expands to multiple.
    MultiOpRequired {
        count: usize,
    },
}

impl std::fmt::Display for OpMigrateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotTaggedObject => write!(f, "legacy Op JSON must be a single-key object"),
            Self::DeferredVariant(v) => write!(f, "legacy Op variant {v} migrate not leased yet"),
            Self::UnknownVariant(v) => write!(f, "unknown legacy Op variant {v}"),
            Self::InvalidShape(msg) => write!(f, "invalid legacy Op shape: {msg}"),
            Self::EmptyName => write!(f, "AllowRelation name must be non-empty"),
            Self::InvalidRuleDefinition => {
                write!(f, "RegisterRule definition is not a valid named rule ident")
            }
            Self::EmptyPolicyDefinition => write!(f, "PinPolicy definition must be non-empty"),
            Self::MultiOpRequired { count } => {
                write!(f, "legacy remap expands to {count} ops; use remap_legacy_ops")
            }
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

#[derive(Deserialize)]
struct RegisterRuleBody {
    definition: String,
    valid_from: u64,
    #[serde(default)]
    valid_to: Option<u64>,
}

#[derive(Deserialize)]
struct PinPolicyBody {
    definition: String,
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

/// Build modern `StoreBlob` + `kutha:rule` Assert (same shape as runtime `emit_rule`).
pub fn register_rule_to_ops(
    definition: impl Into<String>,
    valid_from: u64,
    valid_to: Option<u64>,
    mut intern: impl FnMut(&str) -> TermId,
) -> Result<Vec<Op>, OpMigrateError> {
    let definition = definition.into();
    let Some(ast) = RuleAst::named(&definition) else {
        return Err(OpMigrateError::InvalidRuleDefinition);
    };
    let bytes = ast.encode();
    let digest = ast.hash();
    let subject = intern(&digest);
    let relation = intern(KUTHA_RULE);
    Ok(vec![
        Op::StoreBlob {
            digest: digest.clone(),
            bytes,
        },
        Op::Assert {
            subject,
            relation,
            object: subject,
            valid_from,
            valid_to,
            claim: None,
            delivery_key: Some(digest),
            polarity: None,
        },
    ])
}

/// Build modern `StoreBlob` + `kutha:policy` Assert (same shape as runtime `emit_policy`).
pub fn pin_policy_to_ops(
    definition: impl Into<String>,
    valid_from: u64,
    valid_to: Option<u64>,
    mut intern: impl FnMut(&str) -> TermId,
) -> Result<Vec<Op>, OpMigrateError> {
    let definition = definition.into();
    if definition.is_empty() {
        return Err(OpMigrateError::EmptyPolicyDefinition);
    }
    let digest = policy_version_hash(&definition);
    let bytes = definition.into_bytes();
    let subject = intern(&digest);
    let relation = intern(KUTHA_POLICY);
    Ok(vec![
        Op::StoreBlob {
            digest: digest.clone(),
            bytes,
        },
        Op::Assert {
            subject,
            relation,
            object: subject,
            valid_from,
            valid_to,
            claim: None,
            delivery_key: Some(digest),
            polarity: None,
        },
    ])
}

/// Remap one legacy dictionary meta-op JSON value into one or more modern [`Op`]s.
///
/// Leased: `AllowRelation` (M062), `RegisterRule` (M063), `PinPolicy` (M064).
/// Remaining meta-ops return [`OpMigrateError::DeferredVariant`]. Does not touch `Op`'s serde.
pub fn remap_legacy_ops(
    value: &serde_json::Value,
    intern: impl FnMut(&str) -> TermId,
) -> Result<Vec<Op>, OpMigrateError> {
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
            Ok(vec![allow_relation_to_assert(
                parsed.name,
                parsed.valid_from,
                parsed.valid_to,
                intern,
            )?])
        }
        "RegisterRule" => {
            let parsed: RegisterRuleBody = serde_json::from_value(body.clone())
                .map_err(|_| OpMigrateError::InvalidShape("RegisterRule body"))?;
            register_rule_to_ops(parsed.definition, parsed.valid_from, parsed.valid_to, intern)
        }
        "PinPolicy" => {
            let parsed: PinPolicyBody = serde_json::from_value(body.clone())
                .map_err(|_| OpMigrateError::InvalidShape("PinPolicy body"))?;
            pin_policy_to_ops(parsed.definition, parsed.valid_from, parsed.valid_to, intern)
        }
        "RecordAdmission" => Err(OpMigrateError::DeferredVariant("RecordAdmission")),
        "RecordAction" => Err(OpMigrateError::DeferredVariant("RecordAction")),
        other => Err(OpMigrateError::UnknownVariant(other.to_string())),
    }
}

/// Remap when the legacy variant expands to exactly one modern [`Op`].
pub fn remap_legacy_op_value(
    value: &serde_json::Value,
    intern: impl FnMut(&str) -> TermId,
) -> Result<Op, OpMigrateError> {
    let mut ops = remap_legacy_ops(value, intern)?;
    if ops.len() != 1 {
        return Err(OpMigrateError::MultiOpRequired { count: ops.len() });
    }
    Ok(ops.pop().expect("len == 1"))
}

/// Remap a legacy Event JSON whose `op` is a removed meta-op tag.
///
/// `AllowRelation`: one event, legacy id kept. Multi-op remaps (`RegisterRule`,
/// `PinPolicy`): `StoreBlob` gets a fresh id; Assert keeps the legacy id.
/// `object_ids` are recomputed.
pub fn remap_legacy_events(
    value: &serde_json::Value,
    intern: impl FnMut(&str) -> TermId,
) -> Result<Vec<Event>, OpMigrateError> {
    #[derive(Deserialize)]
    struct RawEvent {
        id: EventId,
        op: serde_json::Value,
        ingested_at: u64,
    }
    let raw: RawEvent = serde_json::from_value(value.clone())
        .map_err(|_| OpMigrateError::InvalidShape("Event envelope"))?;
    let ops = remap_legacy_ops(&raw.op, intern)?;
    if ops.is_empty() {
        return Err(OpMigrateError::InvalidShape("empty remap"));
    }
    let mut out = Vec::with_capacity(ops.len());
    let last = ops.len() - 1;
    for (i, op) in ops.into_iter().enumerate() {
        let mut event = Event::new(op, raw.ingested_at);
        if i == last {
            event.id = raw.id;
        }
        out.push(event);
    }
    Ok(out)
}

/// Remap when the legacy event expands to exactly one modern [`Event`].
pub fn remap_legacy_event_value(
    value: &serde_json::Value,
    intern: impl FnMut(&str) -> TermId,
) -> Result<Event, OpMigrateError> {
    let mut events = remap_legacy_events(value, intern)?;
    if events.len() != 1 {
        return Err(OpMigrateError::MultiOpRequired {
            count: events.len(),
        });
    }
    Ok(events.pop().expect("len == 1"))
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
    fn register_rule_remaps_to_blob_and_kutha_rule_assert() {
        let mut dict = TermDictionary::default();
        let raw = serde_json::json!({
            "RegisterRule": {
                "definition": "derive_pq",
                "valid_from": 2010,
                "valid_to": null
            }
        });
        let ops = remap_legacy_ops(&raw, |s| dict.intern(s)).expect("remap");
        assert_eq!(ops.len(), 2);
        let digest = RuleAst::named("derive_pq").unwrap().hash();
        match &ops[0] {
            Op::StoreBlob { digest: d, bytes } => {
                assert_eq!(d, &digest);
                assert_eq!(RuleAst::from_canonical(bytes), Some(RuleAst::named("derive_pq").unwrap()));
            }
            other => panic!("expected StoreBlob, got {other:?}"),
        }
        match &ops[1] {
            Op::Assert {
                subject,
                relation,
                object,
                delivery_key,
                valid_from,
                ..
            } => {
                assert_eq!(subject, object);
                assert_eq!(dict.lookup(*relation), Some(KUTHA_RULE));
                assert_eq!(delivery_key.as_deref(), Some(digest.as_str()));
                assert_eq!(*valid_from, 2010);
            }
            other => panic!("expected Assert, got {other:?}"),
        }
        let err = remap_legacy_op_value(&raw, |s| dict.intern(s)).unwrap_err();
        assert_eq!(err, OpMigrateError::MultiOpRequired { count: 2 });
    }

    #[test]
    fn invalid_register_rule_definition_is_rejected() {
        let mut dict = TermDictionary::default();
        let raw = serde_json::json!({
            "RegisterRule": { "definition": " derive_pq", "valid_from": 0, "valid_to": null }
        });
        let err = remap_legacy_ops(&raw, |s| dict.intern(s)).unwrap_err();
        assert_eq!(err, OpMigrateError::InvalidRuleDefinition);
    }

    #[test]
    fn pin_policy_remaps_to_blob_and_kutha_policy_assert() {
        let mut dict = TermDictionary::default();
        let raw = serde_json::json!({
            "PinPolicy": {
                "definition": "allow-all",
                "valid_from": 2010,
                "valid_to": null
            }
        });
        let ops = remap_legacy_ops(&raw, |s| dict.intern(s)).expect("remap");
        assert_eq!(ops.len(), 2);
        let digest = policy_version_hash("allow-all");
        match &ops[0] {
            Op::StoreBlob { digest: d, bytes } => {
                assert_eq!(d, &digest);
                assert_eq!(bytes, b"allow-all");
            }
            other => panic!("expected StoreBlob, got {other:?}"),
        }
        match &ops[1] {
            Op::Assert {
                subject,
                relation,
                object,
                delivery_key,
                valid_from,
                ..
            } => {
                assert_eq!(subject, object);
                assert_eq!(dict.lookup(*relation), Some(KUTHA_POLICY));
                assert_eq!(delivery_key.as_deref(), Some(digest.as_str()));
                assert_eq!(*valid_from, 2010);
            }
            other => panic!("expected Assert, got {other:?}"),
        }
    }

    #[test]
    fn empty_pin_policy_definition_is_rejected() {
        let mut dict = TermDictionary::default();
        let raw = serde_json::json!({
            "PinPolicy": { "definition": "", "valid_from": 0, "valid_to": null }
        });
        let err = remap_legacy_ops(&raw, |s| dict.intern(s)).unwrap_err();
        assert_eq!(err, OpMigrateError::EmptyPolicyDefinition);
    }

    #[test]
    fn record_admission_is_deferred() {
        let mut dict = TermDictionary::default();
        let raw = serde_json::json!({
            "RecordAdmission": {
                "justification_id": "j",
                "admitted": true,
                "policy_version": "p",
                "valid_from": 0,
                "valid_to": null
            }
        });
        let err = remap_legacy_ops(&raw, |s| dict.intern(s)).unwrap_err();
        assert_eq!(err, OpMigrateError::DeferredVariant("RecordAdmission"));
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
