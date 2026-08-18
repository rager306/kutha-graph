use crate::TermId;
use std::collections::HashMap;

/// Reversible string↔id map. Droppable; not SoT; not ADR-050 control dictionaries.
#[derive(Clone, Debug, Default)]
pub struct TermDictionary {
    to_id: HashMap<String, TermId>,
    to_str: Vec<String>,
}

impl TermDictionary {
    pub fn intern(&mut self, s: &str) -> TermId {
        if let Some(id) = self.to_id.get(s) {
            return *id;
        }
        let id = self.to_str.len() as TermId;
        self.to_str.push(s.to_string());
        self.to_id.insert(s.to_string(), id);
        id
    }

    pub fn lookup(&self, id: TermId) -> Option<&str> {
        self.to_str.get(id as usize).map(|s| s.as_str())
    }

    pub fn len(&self) -> usize {
        self.to_str.len()
    }

    pub fn strings(&self) -> &[String] {
        &self.to_str
    }

    pub fn from_strings(to_str: Vec<String>) -> Self {
        let mut to_id = HashMap::new();
        for (i, s) in to_str.iter().enumerate() {
            to_id.insert(s.clone(), i as TermId);
        }
        Self { to_id, to_str }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intern_is_stable_and_dense() {
        let mut d = TermDictionary::default();
        let a = d.intern("Alice");
        let r = d.intern("knows");
        let a2 = d.intern("Alice");
        assert_eq!(a, a2);
        assert_ne!(a, r);
        assert_eq!(d.lookup(a), Some("Alice"));
    }
}
