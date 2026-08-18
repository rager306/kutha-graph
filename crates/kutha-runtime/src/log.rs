use kutha_common::Event;

/// Append-only semantic log (SoT). Not Rocks WAL.
#[derive(Clone, Debug, Default)]
pub struct EventLog {
    events: Vec<Event>,
}

impl EventLog {
    pub fn append(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Event> {
        self.events.iter()
    }

    pub fn as_slice(&self) -> &[Event] {
        &self.events
    }

    pub fn prefix(&self, n: usize) -> Self {
        Self {
            events: self.events.get(..n).unwrap_or(&self.events).to_vec(),
        }
    }

    pub fn from_events(events: Vec<Event>) -> Self {
        Self { events }
    }
}
