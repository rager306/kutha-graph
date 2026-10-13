//! Explicit Op wire migrate (M062+). Not a serde alias on [`Op`].
//!
//! `store::open` / `Op` deserialize stay fail-open for removed variants.
//! Callers pass an intern callback so TermIds are never guessed.

use crate::{
    policy_version_hash, Event, EventId, Op, RuleAst, SupportPolarity, TermId, KUTHA_ACTION,
    KUTHA_ADMISSION, KUTHA_ALLOWS, KUTHA_POLICY, KUTHA_RULE,
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
    /// RecordAdmission justification_id was empty.
    EmptyJustificationId,
    /// RecordAdmission policy_version was empty.
    EmptyPolicyVersion,
    /// RecordAction target_claim was empty.
    EmptyTargetClaim,
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
            Self::EmptyJustificationId => {
                write!(f, "RecordAdmission justification_id must be non-empty")
            }
            Self::EmptyPolicyVersion => {
                write!(f, "RecordAdmission policy_version must be non-empty")
            }
            Self::EmptyTargetClaim => {
                write!(f, "RecordAction target_claim must be non-empty")
            }
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

#[derive(Deserialize)]
struct RecordAdmissionBody {
    justification_id: String,
    admitted: bool,
    policy_version: String,
    valid_from: u64,
    #[serde(default)]
    valid_to: Option<u64>,
}

#[derive(Deserialize)]
struct RecordActionBody {
    justification_id: String,
    target_claim: String,
    #[serde(default)]
    #[allow(dead_code)]
    source_claim_ids: Vec<String>,
    #[serde(default)]
    #[allow(dead_code)]
    source_event_ids: Vec<String>,
    #[serde(default)]
    #[allow(dead_code)]
    rule_version: String,
    admitted: bool,
    #[serde(default)]
    #[allow(dead_code)]
    policy_version: String,
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

/// Build modern `kutha:admission` Assert (same shape as runtime `record_justification`).
pub fn record_admission_to_assert(
    justification_id: impl Into<String>,
    admitted: bool,
    policy_version: impl Into<String>,
    valid_from: u64,
    valid_to: Option<u64>,
    mut intern: impl FnMut(&str) -> TermId,
) -> Result<Op, OpMigrateError> {
    let justification_id = justification_id.into();
    if justification_id.is_empty() {
        return Err(OpMigrateError::EmptyJustificationId);
    }
    let policy_version = policy_version.into();
    if policy_version.is_empty() {
        return Err(OpMigrateError::EmptyPolicyVersion);
    }
    let subject = intern(&justification_id);
    let relation = intern(KUTHA_ADMISSION);
    let object = intern(if admitted { "true" } else { "false" });
    let polarity = Some(if admitted {
        SupportPolarity::Positive
    } else {
        SupportPolarity::Negative
    });
    Ok(Op::Assert {
        subject,
        relation,
        object,
        valid_from,
        valid_to,
        claim: None,
        delivery_key: Some(policy_version),
        polarity,
    })
}

/// Build modern `kutha:action` Assert (same shape as runtime `record_action`).
///
/// Legacy cite fields (`source_*`, `rule_version`, `policy_version`) are accepted
/// for shape checks but live on `JustificationCite` / admission in the modern model.
pub fn record_action_to_assert(
    justification_id: impl Into<String>,
    target_claim: impl Into<String>,
    admitted: bool,
    valid_from: u64,
    valid_to: Option<u64>,
    mut intern: impl FnMut(&str) -> TermId,
) -> Result<Op, OpMigrateError> {
    let justification_id = justification_id.into();
    if justification_id.is_empty() {
        return Err(OpMigrateError::EmptyJustificationId);
    }
    let target_claim = target_claim.into();
    if target_claim.is_empty() {
        return Err(OpMigrateError::EmptyTargetClaim);
    }
    let subject = intern(&justification_id);
    let relation = intern(KUTHA_ACTION);
    let object = intern(&target_claim);
    let polarity = Some(if admitted {
        SupportPolarity::Positive
    } else {
        SupportPolarity::Negative
    });
    Ok(Op::Assert {
        subject,
        relation,
        object,
        valid_from,
        valid_to,
        claim: None,
        delivery_key: Some(justification_id),
        polarity,
    })
}

/// Remap one legacy dictionary meta-op JSON value into one or more modern [`Op`]s.
///
/// Leased: `AllowRelation` (M062), `RegisterRule` (M063), `PinPolicy` (M064),
/// `RecordAdmission` (M065), `RecordAction` (M066). Does not touch `Op`'s serde.
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
        "RecordAdmission" => {
            let parsed: RecordAdmissionBody = serde_json::from_value(body.clone())
                .map_err(|_| OpMigrateError::InvalidShape("RecordAdmission body"))?;
            Ok(vec![record_admission_to_assert(
                parsed.justification_id,
                parsed.admitted,
                parsed.policy_version,
                parsed.valid_from,
                parsed.valid_to,
                intern,
            )?])
        }
        "RecordAction" => {
            let parsed: RecordActionBody = serde_json::from_value(body.clone())
                .map_err(|_| OpMigrateError::InvalidShape("RecordAction body"))?;
            Ok(vec![record_action_to_assert(
                parsed.justification_id,
                parsed.target_claim,
                parsed.admitted,
                parsed.valid_from,
                parsed.valid_to,
                intern,
            )?])
        }
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
/// Single-op remaps (`AllowRelation`, `RecordAdmission`, `RecordAction`): one
/// event, legacy id kept. Multi-op remaps (`RegisterRule`, `PinPolicy`):
/// `StoreBlob` gets a fresh id; Assert keeps the legacy id. `object_ids` are
/// recomputed.
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

/// Rewrite one events.jsonl line for directory Op-wire migrate (M067).
///
/// Modern [`Event`] JSON is kept; `Define` names sync into `dict`. Legacy
/// meta-op envelopes remap via [`remap_legacy_events`], with `Op::Define` rows
/// inserted for newly interned names so reopen reconstructs the same TermIds.
pub fn rewrite_event_json_line(
    value: &serde_json::Value,
    dict: &mut crate::TermDictionary,
) -> Result<Vec<Event>, OpMigrateError> {
    if let Ok(event) = serde_json::from_value::<Event>(value.clone()) {
        if let Op::Define { name } = &event.op {
            dict.intern(name);
        }
        return Ok(vec![event]);
    }
    let before = dict.len();
    let remapped = remap_legacy_events(value, |s| dict.intern(s))?;
    let ingested_at = remapped.first().map(|e| e.ingested_at).unwrap_or(0);
    let mut out = Vec::with_capacity(remapped.len() + (dict.len() - before));
    for name in dict.strings()[before..].iter() {
        out.push(Event::new(
            Op::Define {
                name: name.clone(),
            },
            ingested_at,
        ));
    }
    out.extend(remapped);
    Ok(out)
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
    fn record_admission_remaps_like_record_justification() {
        let mut dict = TermDictionary::default();
        let raw = serde_json::json!({
            "RecordAdmission": {
                "justification_id": "j",
                "admitted": true,
                "policy_version": "p",
                "valid_from": 2017,
                "valid_to": null
            }
        });
        let op = remap_legacy_op_value(&raw, |s| dict.intern(s)).expect("remap");
        match op {
            Op::Assert {
                subject,
                relation,
                object,
                valid_from,
                valid_to,
                delivery_key,
                polarity,
                ..
            } => {
                assert_eq!(dict.lookup(subject), Some("j"));
                assert_eq!(dict.lookup(relation), Some(KUTHA_ADMISSION));
                assert_eq!(dict.lookup(object), Some("true"));
                assert_eq!(valid_from, 2017);
                assert_eq!(valid_to, None);
                assert_eq!(delivery_key.as_deref(), Some("p"));
                assert_eq!(polarity, Some(SupportPolarity::Positive));
            }
            other => panic!("expected Assert, got {other:?}"),
        }
    }

    #[test]
    fn denied_record_admission_uses_negative_polarity() {
        let mut dict = TermDictionary::default();
        let raw = serde_json::json!({
            "RecordAdmission": {
                "justification_id": "j-deny",
                "admitted": false,
                "policy_version": "pin",
                "valid_from": 0,
                "valid_to": null
            }
        });
        let op = remap_legacy_op_value(&raw, |s| dict.intern(s)).expect("remap");
        match op {
            Op::Assert {
                object,
                polarity,
                ..
            } => {
                assert_eq!(dict.lookup(object), Some("false"));
                assert_eq!(polarity, Some(SupportPolarity::Negative));
            }
            other => panic!("expected Assert, got {other:?}"),
        }
    }

    #[test]
    fn empty_record_admission_fields_are_rejected() {
        let mut dict = TermDictionary::default();
        let empty_jid = serde_json::json!({
            "RecordAdmission": {
                "justification_id": "",
                "admitted": true,
                "policy_version": "p",
                "valid_from": 0,
                "valid_to": null
            }
        });
        assert_eq!(
            remap_legacy_ops(&empty_jid, |s| dict.intern(s)).unwrap_err(),
            OpMigrateError::EmptyJustificationId
        );
        let empty_pin = serde_json::json!({
            "RecordAdmission": {
                "justification_id": "j",
                "admitted": true,
                "policy_version": "",
                "valid_from": 0,
                "valid_to": null
            }
        });
        assert_eq!(
            remap_legacy_ops(&empty_pin, |s| dict.intern(s)).unwrap_err(),
            OpMigrateError::EmptyPolicyVersion
        );
    }

    #[test]
    fn record_action_remaps_like_record_action_api() {
        let mut dict = TermDictionary::default();
        let claim = "00000000-0000-0000-0000-000000000001";
        let raw = serde_json::json!({
            "RecordAction": {
                "justification_id": "j",
                "target_claim": claim,
                "source_claim_ids": [],
                "source_event_ids": [],
                "rule_version": "r",
                "admitted": true,
                "policy_version": "p",
                "valid_from": 2017,
                "valid_to": null
            }
        });
        let op = remap_legacy_op_value(&raw, |s| dict.intern(s)).expect("remap");
        match op {
            Op::Assert {
                subject,
                relation,
                object,
                delivery_key,
                polarity,
                valid_from,
                ..
            } => {
                assert_eq!(dict.lookup(subject), Some("j"));
                assert_eq!(dict.lookup(relation), Some(KUTHA_ACTION));
                assert_eq!(dict.lookup(object), Some(claim));
                assert_eq!(delivery_key.as_deref(), Some("j"));
                assert_eq!(polarity, Some(SupportPolarity::Positive));
                assert_eq!(valid_from, 2017);
            }
            other => panic!("expected Assert, got {other:?}"),
        }
    }

    #[test]
    fn rewrite_line_inserts_defines_before_remapped_assert() {
        let mut dict = TermDictionary::default();
        let id = EventId::nil();
        let raw = serde_json::json!({
            "id": id,
            "op": {
                "AllowRelation": {
                    "name": "relatedTo",
                    "valid_from": 0,
                    "valid_to": null
                }
            },
            "ingested_at": 3,
            "object_ids": []
        });
        let events = rewrite_event_json_line(&raw, &mut dict).expect("rewrite");
        assert!(events.len() >= 2);
        assert!(matches!(&events[0].op, Op::Define { name } if name == "relatedTo"));
        assert!(matches!(
            &events.last().unwrap().op,
            Op::Assert { delivery_key: Some(k), .. } if k == "relatedTo"
        ));
        assert_eq!(events.last().unwrap().id, id);
    }

    #[test]
    fn empty_record_action_fields_are_rejected() {
        let mut dict = TermDictionary::default();
        let empty_jid = serde_json::json!({
            "RecordAction": {
                "justification_id": "",
                "target_claim": "00000000-0000-0000-0000-000000000001",
                "admitted": true,
                "valid_from": 0,
                "valid_to": null
            }
        });
        assert_eq!(
            remap_legacy_ops(&empty_jid, |s| dict.intern(s)).unwrap_err(),
            OpMigrateError::EmptyJustificationId
        );
        let empty_claim = serde_json::json!({
            "RecordAction": {
                "justification_id": "j",
                "target_claim": "",
                "admitted": false,
                "valid_from": 0,
                "valid_to": null
            }
        });
        assert_eq!(
            remap_legacy_ops(&empty_claim, |s| dict.intern(s)).unwrap_err(),
            OpMigrateError::EmptyTargetClaim
        );
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
