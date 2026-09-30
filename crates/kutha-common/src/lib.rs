//! Shared encoding for the P0 heartbeat (ADR-011).
//! Intern map ≠ agent control dictionaries (ADR-050).

mod event;
mod intern;

pub use event::{
    Event, EventId, Op, OutcomeDisposition, SupportPolarity, TermId, TimeScale, TransactionTime,
    ValidTime, TRANSACTION_TIME_SCALE, VALID_TIME_SCALE,
};
pub use intern::TermDictionary;
