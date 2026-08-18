use crate::fold::GraphFold;
use kutha_common::TermId;

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
}
