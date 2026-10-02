use sha2::{Digest, Sha256};
use uuid::Uuid;

/// UUID v7 event identifier (ADR-000 D2 / ADR-011).
pub type EventId = Uuid;

/// Interned term (hot-path integer; strings live in the dictionary lease).
pub type TermId = u32;

/// Declared scale for fixture clocks (TIME-01 / F7). Not a wall calendar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeScale {
    /// One integer unit is one Gregorian year CE. Fixture 2017 means year 2017 CE, not Unix seconds.
    YearCe,
    /// Log sequence assigned at ingest (`Runtime` next_tt), not wall-clock.
    LogSequence,
}

/// Valid-time scale used by fixtures: Gregorian year CE, not Unix epoch.
pub const VALID_TIME_SCALE: TimeScale = TimeScale::YearCe;

/// Transaction-time scale used by fixtures: log sequence, not wall-clock.
/// TT-to-wall mapping is out of M012a S05.
pub const TRANSACTION_TIME_SCALE: TimeScale = TimeScale::LogSequence;

macro_rules! define_clock {
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident;
    ) => {
        $(#[$attr])*
        #[derive(
            Copy,
            Clone,
            Debug,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            serde::Serialize,
            serde::Deserialize,
        )]
        #[serde(transparent)]
        $vis struct $name(u64);

        impl $name {
            pub const MAX: Self = Self(u64::MAX);

            #[inline]
            pub const fn as_u64(self) -> u64 {
                self.0
            }

            #[inline]
            pub const fn to_le_bytes(self) -> [u8; 8] {
                self.0.to_le_bytes()
            }
        }

        impl From<u64> for $name {
            fn from(value: u64) -> Self {
                Self(value)
            }
        }

        impl From<$name> for u64 {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl PartialEq<u64> for $name {
            fn eq(&self, other: &u64) -> bool {
                self.0 == *other
            }
        }
    };
}

define_clock! {
    /// Valid-time instant.
    ///
    /// Fixtures use [`VALID_TIME_SCALE`] (`TimeScale::YearCe`): one integer unit is
    /// one Gregorian year CE. `2017` means year 2017 CE, not Unix seconds.
    ///
    /// Distinct from [`TransactionTime`]: mixing clocks without an explicit
    /// conversion does not type-check.
    ///
    /// ```compile_fail
    /// use kutha_common::{TransactionTime, ValidTime};
    /// fn takes_valid_time(_vt: ValidTime) {}
    /// takes_valid_time(TransactionTime::from(0));
    /// ```
    pub struct ValidTime;
}

define_clock! {
    /// Transaction-time.
    ///
    /// Fixtures use [`TRANSACTION_TIME_SCALE`] (`TimeScale::LogSequence`): assigned
    /// at ingest (`Runtime` next_tt), not wall-clock. A TT-to-wall calendar map is
    /// out of M012a S05.
    pub struct TransactionTime;
}

define_clock! {
    /// Wall-clock instant (same shape as the other clocks). Unused on the query
    /// API until `as_known_on` (Phase 32). No `From` into [`ValidTime`] or
    /// [`TransactionTime`].
    pub struct WallTime;
}

/// Conceptual valid-time name from the M014 roadmap (CLK-01).
pub type Vt = ValidTime;
/// Conceptual transaction-time name from the M014 roadmap (CLK-01).
pub type Tt = TransactionTime;

/// Durable progress encoding for a quantum (OUT-01 / D-O2). Call `Ok` is not completion.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutcomeDisposition {
    Zero,
    Partial,
    Full,
    Resume,
}

/// Stored support polarity for conflict views (ING-03). `None` on Assert is unbucketed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportPolarity {
    Positive,
    Negative,
}

/// Reserved dictionary relation names (D-01). Not product graph labels.
pub const KUTHA_ALLOWS: &str = "kutha:allows";
pub const KUTHA_RULE: &str = "kutha:rule";
pub const KUTHA_POLICY: &str = "kutha:policy";
pub const KUTHA_ADMISSION: &str = "kutha:admission";
pub const KUTHA_ACTION: &str = "kutha:action";

pub const KUTHA_RESERVED_RELS: [&str; 5] = [
    KUTHA_ALLOWS,
    KUTHA_RULE,
    KUTHA_POLICY,
    KUTHA_ADMISSION,
    KUTHA_ACTION,
];

/// Lean write operators (TGMS-shaped; ADR-010 D010-3).
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Op {
    /// N-ary objects: subject, relation, object (interned).
    /// Three identities (ING-02 / D-02): `claim` is proposition identity (`Fact.claim_id`);
    /// the minting `Event.id` / `Fact.event_id` is the support slot; `delivery_key` is retry
    /// identity only (ING-01) and is not a claim id.
    /// When `claim` is absent, the new event id becomes the claim id (ADR-011 / M011 S01).
    /// `polarity`: stored support polarity; default `None` (ING-03).
    Assert {
        subject: TermId,
        relation: TermId,
        object: TermId,
        valid_from: u64,
        valid_to: Option<u64>,
        #[serde(default)]
        claim: Option<EventId>,
        #[serde(default)]
        delivery_key: Option<String>,
        #[serde(default)]
        polarity: Option<SupportPolarity>,
    },
    Retract {
        event_id: EventId,
    },
    Correct {
        event_id: EventId,
        object: TermId,
        valid_from: u64,
        valid_to: Option<u64>,
    },
    /// Explicit interval patch (M011 S04). Does not change whole-version `Correct`.
    CorrectInterval {
        event_id: EventId,
        object: TermId,
        patch_from: u64,
        patch_to: Option<u64>,
    },
    /// Behavior-emitted follow-on (still a log event; never LLM narrative).
    Behavior {
        name: String,
        caused_by: EventId,
        /// Pinned rule identity for provenance (ADR-060 obligation 2). Fold ignores this.
        /// Missing JSON decodes to empty so legacy WAL/JSONL rows stay readable.
        #[serde(default)]
        rule_version: String,
        subject: TermId,
        relation: TermId,
        object: TermId,
        valid_from: u64,
        valid_to: Option<u64>,
    },
    /// Logged term definition (ADR-011 / M010 S02). Fold no-op; not a graph fact.
    Define {
        name: String,
    },
    /// Logged quantum outcome (M012a S01 / LOG-01). Fold no-op; not a graph fact.
    QuantumOutcome {
        quantum_id: String,
        disposition: OutcomeDisposition,
        aborted_on_budget: bool,
        events_in_quantum: usize,
        event_ids: Vec<EventId>,
        receipt_digest_hex: String,
        resume_of: Option<String>,
    },
    /// Logged justification cite (M012a S01 / LOG-02). Fold no-op; not a graph fact.
    JustificationCite {
        justification_id: String,
        target_claim: EventId,
        source_claim_ids: Vec<EventId>,
        source_event_ids: Vec<EventId>,
        rule_version: String,
        tt: u64,
        vt: u64,
    },
    /// Content-addressed body (D-02). Fold stores once; not a graph Fact.
    StoreBlob {
        digest: String,
        bytes: Vec<u8>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Event {
    pub id: EventId,
    pub op: Op,
    pub ingested_at: u64,
    pub object_ids: Vec<TermId>,
}

impl Event {
    pub fn new(op: Op, ingested_at: impl Into<TransactionTime>) -> Self {
        let ingested_at = ingested_at.into().as_u64();
        let object_ids = match &op {
            Op::Assert {
                subject,
                relation,
                object,
                ..
            }
            | Op::Behavior {
                subject,
                relation,
                object,
                ..
            } => vec![*subject, *relation, *object],
            Op::Retract { .. }
            | Op::Define { .. }
            | Op::QuantumOutcome { .. }
            | Op::JustificationCite { .. }
            | Op::StoreBlob { .. } => vec![],
            Op::Correct { object, .. } | Op::CorrectInterval { object, .. } => vec![*object],
        };
        Self {
            id: Uuid::now_v7(),
            op,
            ingested_at,
            object_ids,
        }
    }

    /// Name-stable `Op::Define` for persist encoding (DUR-03).
    ///
    /// Id is the first 16 bytes of SHA-256(`kutha-define-id` || term bytes).
    /// Live [`Event::new`] intern still mints UUID v7.
    pub fn stable_define(name: impl Into<String>, ingested_at: impl Into<TransactionTime>) -> Self {
        let ingested_at = ingested_at.into().as_u64();
        let name = name.into();
        let mut h = Sha256::new();
        h.update(b"kutha-define-id");
        h.update(name.as_bytes());
        let digest = h.finalize();
        let mut id_bytes = [0u8; 16];
        id_bytes.copy_from_slice(&digest[..16]);
        Self {
            id: Uuid::from_bytes(id_bytes),
            op: Op::Define { name },
            ingested_at,
            object_ids: vec![],
        }
    }

    pub fn digest_bytes(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update(self.id.as_bytes());
        h.update(self.ingested_at.to_le_bytes());
        match &self.op {
            Op::Assert {
                subject,
                relation,
                object,
                valid_from,
                valid_to,
                claim,
                delivery_key,
                polarity,
            } => {
                h.update(b"assert");
                h.update(subject.to_le_bytes());
                h.update(relation.to_le_bytes());
                h.update(object.to_le_bytes());
                h.update(valid_from.to_le_bytes());
                h.update(valid_to.unwrap_or(u64::MAX).to_le_bytes());
                if let Some(c) = claim {
                    h.update(b"claim");
                    h.update(c.as_bytes());
                }
                match delivery_key {
                    Some(k) => {
                        h.update(b"dkey");
                        h.update((k.len() as u64).to_le_bytes());
                        h.update(k.as_bytes());
                    }
                    None => h.update(b"no-dkey"),
                }
                h.update(polarity_tag(*polarity));
            }
            Op::Retract { event_id } => {
                h.update(b"retract");
                h.update(event_id.as_bytes());
            }
            Op::Correct {
                event_id,
                object,
                valid_from,
                valid_to,
            } => {
                h.update(b"correct");
                h.update(event_id.as_bytes());
                h.update(object.to_le_bytes());
                h.update(valid_from.to_le_bytes());
                h.update(valid_to.unwrap_or(u64::MAX).to_le_bytes());
            }
            Op::CorrectInterval {
                event_id,
                object,
                patch_from,
                patch_to,
            } => {
                h.update(b"correct-interval");
                h.update(event_id.as_bytes());
                h.update(object.to_le_bytes());
                h.update(patch_from.to_le_bytes());
                h.update(patch_to.unwrap_or(u64::MAX).to_le_bytes());
            }
            Op::Behavior {
                name,
                caused_by,
                rule_version,
                subject,
                relation,
                object,
                valid_from,
                valid_to,
            } => {
                h.update(b"behavior");
                h.update(name.as_bytes());
                h.update(caused_by.as_bytes());
                h.update(rule_version.as_bytes());
                h.update(subject.to_le_bytes());
                h.update(relation.to_le_bytes());
                h.update(object.to_le_bytes());
                h.update(valid_from.to_le_bytes());
                h.update(valid_to.unwrap_or(u64::MAX).to_le_bytes());
            }
            Op::Define { name } => {
                h.update(b"define");
                h.update(name.as_bytes());
            }
            Op::QuantumOutcome {
                quantum_id,
                disposition,
                aborted_on_budget,
                events_in_quantum,
                event_ids,
                receipt_digest_hex,
                resume_of,
            } => {
                h.update(b"quantum-outcome");
                h.update(quantum_id.as_bytes());
                h.update(disposition_tag(*disposition));
                h.update([u8::from(*aborted_on_budget)]);
                h.update((*events_in_quantum as u64).to_le_bytes());
                h.update((event_ids.len() as u64).to_le_bytes());
                for id in event_ids {
                    h.update(id.as_bytes());
                }
                h.update(receipt_digest_hex.as_bytes());
                match resume_of {
                    Some(s) => {
                        h.update(b"resume");
                        h.update(s.as_bytes());
                    }
                    None => h.update(b"no-resume"),
                }
            }
            Op::JustificationCite {
                justification_id,
                target_claim,
                source_claim_ids,
                source_event_ids,
                rule_version,
                tt,
                vt,
            } => {
                h.update(b"justification-cite");
                h.update(justification_id.as_bytes());
                h.update(target_claim.as_bytes());
                h.update((source_claim_ids.len() as u64).to_le_bytes());
                for id in source_claim_ids {
                    h.update(id.as_bytes());
                }
                h.update((source_event_ids.len() as u64).to_le_bytes());
                for id in source_event_ids {
                    h.update(id.as_bytes());
                }
                h.update(rule_version.as_bytes());
                h.update(tt.to_le_bytes());
                h.update(vt.to_le_bytes());
            }
            Op::StoreBlob { digest, bytes } => {
                h.update(b"store-blob");
                h.update(digest.as_bytes());
                h.update((bytes.len() as u64).to_le_bytes());
                h.update(bytes);
            }
        }
        h.finalize().into()
    }
}

fn hex32(digest: [u8; 32]) -> String {
    let mut out = String::with_capacity(64);
    for b in digest {
        out.push_str(&format!("{b:02x}"));
    }
    out
}

/// Raw SHA-256 hex of `bytes` (StoreBlob address).
pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex32(h.finalize().into())
}

/// SHA-256 of domain `kutha-rule-def` then UTF-8 definition bytes (RULE-01).
pub fn rule_definition_hash(definition: &str) -> String {
    let mut h = Sha256::new();
    h.update(b"kutha-rule-def");
    h.update(definition.as_bytes());
    hex32(h.finalize().into())
}

/// SHA-256 of domain `kutha-policy-def` then UTF-8 definition bytes (ADM-02).
pub fn policy_version_hash(definition: &str) -> String {
    let mut h = Sha256::new();
    h.update(b"kutha-policy-def");
    h.update(definition.as_bytes());
    hex32(h.finalize().into())
}

/// Admit StoreBlob: non-empty bytes and digest match raw SHA-256 or domain hashes (T-26-03).
pub fn blob_digest_matches(digest: &str, bytes: &[u8]) -> bool {
    if bytes.is_empty() || digest.is_empty() {
        return false;
    }
    if digest == sha256_hex(bytes) {
        return true;
    }
    if let Ok(s) = std::str::from_utf8(bytes) {
        digest == rule_definition_hash(s) || digest == policy_version_hash(s)
    } else {
        false
    }
}

fn polarity_tag(p: Option<SupportPolarity>) -> &'static [u8] {
    match p {
        None => b"pol-none",
        Some(SupportPolarity::Positive) => b"pol-pos",
        Some(SupportPolarity::Negative) => b"pol-neg",
    }
}

fn disposition_tag(d: OutcomeDisposition) -> &'static [u8] {
    match d {
        OutcomeDisposition::Zero => b"zero",
        OutcomeDisposition::Partial => b"partial",
        OutcomeDisposition::Full => b"full",
        OutcomeDisposition::Resume => b"resume",
    }
}
