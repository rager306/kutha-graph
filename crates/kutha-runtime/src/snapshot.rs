use crate::fold::GraphFold;
use kutha_common::{Event, TermId};

/// Fold picture at a log offset. Droppable; not SoT (ADR-012).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Snapshot {
    pub log_offset: usize,
    pub next_tt: u64,
    pub max_cascade: usize,
    pub fold: GraphFold,
    pub dict_strings: Vec<String>,
    pub knows: TermId,
    pub known_by: TermId,
    /// Prefix `Op::Behavior` records compacted out of the log (exact Horn
    /// lineage: head id, `caused_by`, `rule_version`). Covered by the checkpoint
    /// blob digest. Empty is omitted; a no-Horn payload is marked by
    /// `behaviors_exact`, not by omitting this field.
    /// Not fingerprint input (fingerprint walks `fold.facts`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub behaviors: Vec<Event>,
    /// Checkpoint lineage mark (M163). Missing deserializes as false. Always
    /// serialized. Receipt rejects false after digest / fact_count / fingerprint
    /// so a vintage blob cannot open as an empty Horn lineage.
    #[serde(default)]
    pub behaviors_exact: bool,
}
