//! One relation allowlist (FF6 stub). Not ADR-050's six kinds. Fail-closed.

use std::collections::HashSet;
use std::path::PathBuf;

const SCHEMA: &str = "kutha-relations/v1";
const ENV_PATH: &str = "KUTHA_RELATIONS_PATH";

pub(crate) fn apply_dotenv() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        for _ in 0..5 {
            let candidate = dir.join(".env");
            if candidate.is_file() {
                if let Ok(text) = std::fs::read_to_string(&candidate) {
                    for raw in text.lines() {
                        let line = raw.trim();
                        if line.is_empty() || line.starts_with('#') || !line.contains('=') {
                            continue;
                        }
                        let Some((key, value)) = line.split_once('=') else {
                            continue;
                        };
                        let key = key.trim();
                        let value = value.trim().trim_matches('"').trim_matches('\'');
                        if std::env::var_os(key).is_none() {
                            std::env::set_var(key, value);
                        }
                    }
                }
                return;
            }
            if !dir.pop() {
                return;
            }
        }
    });
}

pub fn relations_path() -> PathBuf {
    apply_dotenv();
    if let Ok(raw) = std::env::var(ENV_PATH) {
        let trimmed = raw.trim();
        if !trimmed.is_empty() {
            return PathBuf::from(trimmed);
        }
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("dictionaries/relations.yaml")
}

pub fn load_allowed_names() -> HashSet<String> {
    let path = relations_path();
    match std::fs::read_to_string(&path) {
        Ok(text) => parse_relations(&text).unwrap_or_default(),
        Err(_) => HashSet::new(),
    }
}

/// Minimal YAML list parser. Unknown schema or empty list → empty set (fail-closed).
pub fn parse_relations(text: &str) -> Option<HashSet<String>> {
    let mut schema_ok = false;
    let mut in_list = false;
    let mut names = HashSet::new();
    for raw in text.lines() {
        let line = strip_comment(raw).trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema:") {
            schema_ok = rest.trim() == SCHEMA;
            continue;
        }
        if line == "relations:" {
            in_list = true;
            continue;
        }
        if in_list {
            if let Some(item) = line.strip_prefix("- ") {
                let name = item.trim().trim_matches('"');
                if !name.is_empty() {
                    names.insert(name.to_string());
                }
                continue;
            }
            in_list = false;
        }
    }
    if schema_ok && !names.is_empty() {
        Some(names)
    } else {
        None
    }
}

fn strip_comment(line: &str) -> &str {
    match line.find('#') {
        Some(i) => &line[..i],
        None => line,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rejects_unknown_schema_and_empty_list() {
        assert!(parse_relations("schema: other\nrelations:\n  - knows\n").is_none());
        assert!(parse_relations("schema: kutha-relations/v1\nrelations:\n").is_none());
    }

    #[test]
    fn parse_reads_allowlisted_names() {
        let names =
            parse_relations("schema: kutha-relations/v1\nrelations:\n  - knows\n  - inForceAs\n")
                .unwrap();
        assert!(names.contains("knows"));
        assert!(names.contains("inForceAs"));
        assert!(!names.contains("notALegalRelation"));
    }
}
