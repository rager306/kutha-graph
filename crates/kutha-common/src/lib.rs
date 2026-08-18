//! Shared encoding for the P0 heartbeat (ADR-011).
//! Intern map ≠ agent control dictionaries (ADR-050).

mod event;
mod intern;

pub use event::{Event, EventId, Op, TermId, TransactionTime, ValidTime};
pub use intern::TermDictionary;
