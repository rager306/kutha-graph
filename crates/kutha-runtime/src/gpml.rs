//! Read-only GPML/Cypher subset parser into the existing typed [`QueryIr`].
//!
//! M003a scope is deliberately narrow: fixed-hop node/relationship patterns,
//! a start-name property filter, `AS OF`, optional `TT`, `RETURN`, and
//! `LIMIT`. Writes, aggregation, path functions, and arbitrary property
//! filters are out of scope and fail closed. Vocabulary binding is behind
//! [`VocabBinder`]; the runtime binding uses dictionary lookup plus the FF6
//! relation allowlist and never interns.

use crate::quantum::Runtime;
use crate::query_ir::QueryIr;
use kutha_common::{TermId, TransactionTime, ValidTime};

/// Read-only parse/bind failure. The variants are intentionally narrow.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GpmlError {
    Parse { at: usize, reason: String },
    Unsupported { reason: String },
    UnknownTerm { name: String },
    UnknownRelation { name: String },
}

impl std::fmt::Display for GpmlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse { at, reason } => write!(f, "GPML parse error at byte {at}: {reason}"),
            Self::Unsupported { reason } => write!(f, "unsupported GPML construct: {reason}"),
            Self::UnknownTerm { name } => write!(f, "unknown GPML term {name}"),
            Self::UnknownRelation { name } => write!(f, "unknown GPML relation {name}"),
        }
    }
}

impl std::error::Error for GpmlError {}

/// Vocabulary binding for parsed names. Implementations must never intern.
pub trait VocabBinder {
    fn term_id(&self, name: &str) -> Result<TermId, GpmlError>;
    fn relation_id(&self, name: &str) -> Result<TermId, GpmlError>;
}

/// Intern-map + FF6 allowlist binding. Lookup only; no `intern`.
pub struct RuntimeBinder<'a> {
    rt: &'a Runtime,
}

impl<'a> RuntimeBinder<'a> {
    pub fn new(rt: &'a Runtime) -> Self {
        Self { rt }
    }
}

impl VocabBinder for RuntimeBinder<'_> {
    fn term_id(&self, name: &str) -> Result<TermId, GpmlError> {
        self.rt
            .dictionary()
            .id(name)
            .ok_or_else(|| GpmlError::UnknownTerm {
                name: name.to_string(),
            })
    }

    fn relation_id(&self, name: &str) -> Result<TermId, GpmlError> {
        // FF6 stays the fail-closed relation surface. A dictionary hit outside
        // the allowlist is still refused.
        if !crate::allow::load_allowed_names().contains(name) {
            return Err(GpmlError::UnknownRelation {
                name: name.to_string(),
            });
        }
        self.rt
            .dictionary()
            .id(name)
            .ok_or_else(|| GpmlError::UnknownRelation {
                name: name.to_string(),
            })
    }
}

#[derive(Clone, Debug)]
struct NodePattern {
    variable: Option<String>,
    label: Option<String>,
    name: Option<String>,
}

#[derive(Clone, Debug)]
struct HopPattern {
    relation: String,
    hops: u32,
}

#[derive(Clone, Debug)]
struct Pattern {
    start: NodePattern,
    hops: Vec<HopPattern>,
    last: NodePattern,
}

impl Pattern {
    fn start_name(&self) -> Option<&str> {
        self.start.name.as_deref()
    }

    fn is_chain(&self) -> bool {
        !self.hops.is_empty()
    }
}

/// Parse GPML into the existing typed IR using `binder`.
///
/// The accepted shape is:
/// `MATCH <pattern> AS OF <vt> [TT <tt>] RETURN ... [LIMIT <n>]`.
pub fn parse_gpml(text: &str, binder: &impl VocabBinder) -> Result<QueryIr, GpmlError> {
    let mut p = Parser::new(text)?;
    p.expect_keyword("MATCH")?;
    let pattern = p.parse_pattern(binder)?;
    p.expect_keyword("AS")?;
    p.expect_keyword("OF")?;
    let vt = ValidTime::from(p.parse_u64("AS OF valid time")?);
    let tt = if p.peek_keyword("TT") {
        p.expect_keyword("TT")?;
        TransactionTime::from(p.parse_u64("TT transaction time")?)
    } else {
        TransactionTime::MAX
    };
    p.expect_keyword("RETURN")?;
    p.parse_return_vars()?;
    let limit = if p.peek_keyword("LIMIT") {
        p.expect_keyword("LIMIT")?;
        p.parse_u64("LIMIT")? as usize
    } else {
        0
    };
    p.expect_end()?;

    if !pattern.is_chain() {
        let Some(name) = pattern.start_name() else {
            return Err(GpmlError::Unsupported {
                reason: "a single-node pattern needs a {name: \"...\"} start filter".to_string(),
            });
        };
        let start = binder.term_id(name)?;
        return Ok(QueryIr::Hops {
            start,
            rels: Vec::new(),
            k: 0,
            tt,
            vt,
            limit,
        });
    }

    let Some(name) = pattern.start_name() else {
        return Err(GpmlError::Unsupported {
            reason: "a chained pattern needs a {name: \"...\"} start filter".to_string(),
        });
    };
    let start = binder.term_id(name)?;
    if pattern.last.name.is_some() {
        return Err(GpmlError::Unsupported {
            reason: "property filters on non-start nodes are outside M003a".to_string(),
        });
    }
    let mut rel_ids = Vec::with_capacity(pattern.hops.len());
    let mut k = 0u32;
    for hop in pattern.hops {
        let id = binder.relation_id(&hop.relation)?;
        for _ in 0..hop.hops {
            // A later distinct hop appends per-hop ids below; repeated same-rel
            // uses `QueryIr`'s compact `len == 1` form.
            k = k.saturating_add(1);
        }
        rel_ids.push(id);
    }
    if rel_ids.iter().all(|id| *id == rel_ids[0]) {
        rel_ids.truncate(1);
    }
    if k == 0 {
        return Err(GpmlError::Unsupported {
            reason: "a chained pattern must contain at least one hop".to_string(),
        });
    }
    Ok(QueryIr::Hops {
        start,
        rels: rel_ids,
        k,
        tt,
        vt,
        limit,
    })
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Tok {
    Ident(String),
    Quoted(String),
    Int(u64),
    Punct(char),
    Arrow,
}

struct Parser<'a> {
    src: &'a str,
    bytes: &'a [u8],
    pos: usize,
    toks: Vec<(usize, Tok)>,
    idx: usize,
}

impl<'a> Parser<'a> {
    fn new(src: &'a str) -> Result<Self, GpmlError> {
        let mut p = Self {
            src,
            bytes: src.as_bytes(),
            pos: 0,
            toks: Vec::new(),
            idx: 0,
        };
        p.tokenize()?;
        Ok(p)
    }

    fn tokenize(&mut self) -> Result<(), GpmlError> {
        while self.pos < self.bytes.len() {
            let start = self.pos;
            let b = self.bytes[self.pos];
            match b {
                b' ' | b'\t' | b'\r' | b'\n' => {
                    self.pos += 1;
                    continue;
                }
                b'(' | b')' | b'{' | b'}' | b'[' | b']' | b':' | b',' | b'.' | b'*' => {
                    self.pos += 1;
                    self.toks.push((start, Tok::Punct(b as char)));
                }
                b'\'' | b'"' => {
                    self.pos += 1;
                    let value_start = self.pos;
                    while self.pos < self.bytes.len() && self.bytes[self.pos] != b {
                        self.pos += 1;
                    }
                    let at = self.pos;
                    if self.pos >= self.bytes.len() {
                        return Err(GpmlError::Parse {
                            at,
                            reason: "unterminated string".to_string(),
                        });
                    }
                    let value = self.src[value_start..self.pos].to_string();
                    self.pos += 1;
                    self.toks.push((start, Tok::Quoted(value)));
                }
                b'-' => {
                    if self.bytes.get(self.pos + 1) == Some(&b'>') {
                        self.pos += 2;
                        self.toks.push((start, Tok::Arrow));
                    } else {
                        self.pos += 1;
                        self.toks.push((start, Tok::Punct('-')));
                    }
                }
                b'0'..=b'9' => {
                    while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_digit() {
                        self.pos += 1;
                    }
                    let raw = &self.src[start..self.pos];
                    let value = raw.parse::<u64>().map_err(|_| GpmlError::Parse {
                        at: start,
                        reason: format!("invalid integer {raw}"),
                    })?;
                    self.toks.push((start, Tok::Int(value)));
                }
                _ if b.is_ascii_alphanumeric() || b == b'_' => {
                    while self.pos < self.bytes.len() {
                        let c = self.bytes[self.pos];
                        if c.is_ascii_alphanumeric() || c == b'_' {
                            self.pos += 1;
                        } else {
                            break;
                        }
                    }
                    self.toks
                        .push((start, Tok::Ident(self.src[start..self.pos].to_string())));
                }
                _ => return Err(self.err_here("unexpected character")),
            }
        }
        Ok(())
    }

    fn err_here(&self, reason: &str) -> GpmlError {
        GpmlError::Parse {
            at: self.pos,
            reason: reason.to_string(),
        }
    }

    fn peek(&self) -> Option<&(usize, Tok)> {
        self.toks.get(self.idx)
    }

    fn next(&mut self) -> Result<(usize, Tok), GpmlError> {
        let tok = self.toks.get(self.idx).cloned();
        if let Some(tok) = tok {
            self.idx += 1;
            Ok(tok)
        } else {
            Err(self.err_here("unexpected end of query"))
        }
    }

    fn peek_keyword(&self, word: &str) -> bool {
        matches!(self.peek(), Some((_, Tok::Ident(value))) if value.eq_ignore_ascii_case(word))
    }

    fn expect_keyword(&mut self, word: &str) -> Result<(), GpmlError> {
        let (at, tok) = self.next()?;
        match tok {
            Tok::Ident(value) if value.eq_ignore_ascii_case(word) => Ok(()),
            other => Err(GpmlError::Parse {
                at,
                reason: format!("expected {word}, found {other:?}"),
            }),
        }
    }

    fn expect_punct(&mut self, ch: char) -> Result<(), GpmlError> {
        let (at, tok) = self.next()?;
        match tok {
            Tok::Punct(value) if value == ch => Ok(()),
            other => Err(GpmlError::Parse {
                at,
                reason: format!("expected '{ch}', found {other:?}"),
            }),
        }
    }

    fn parse_u64(&mut self, what: &str) -> Result<u64, GpmlError> {
        let (at, tok) = self.next()?;
        match tok {
            Tok::Int(value) => Ok(value),
            other => Err(GpmlError::Parse {
                at,
                reason: format!("expected integer for {what}, found {other:?}"),
            }),
        }
    }

    fn parse_ident(&mut self, what: &str) -> Result<String, GpmlError> {
        let (at, tok) = self.next()?;
        match tok {
            Tok::Ident(value) => Ok(value),
            other => Err(GpmlError::Parse {
                at,
                reason: format!("expected {what}, found {other:?}"),
            }),
        }
    }

    fn parse_pattern(&mut self, binder: &impl VocabBinder) -> Result<Pattern, GpmlError> {
        self.expect_punct('(')?;
        let start = self.parse_node()?;
        self.expect_punct(')')?;
        let mut hops = Vec::new();
        let mut last = start.clone();
        while matches!(self.peek(), Some((_, Tok::Punct('-')))) {
            self.expect_punct('-')?;
            self.expect_punct('[')?;
            self.expect_punct(':')?;
            let relation = self.parse_ident("relationship name")?;
            let mut hop_count = 1u32;
            if self.peek_punct('*') {
                self.expect_punct('*')?;
                hop_count = self.parse_u64("hop count")? as u32;
                if self.peek_punct('.') {
                    self.expect_punct('.')?;
                    self.expect_punct('.')?;
                    let upper = self.parse_u64("upper hop bound")? as u32;
                    if upper < hop_count {
                        return Err(GpmlError::Unsupported {
                            reason: "descending variable-length bounds".to_string(),
                        });
                    }
                    hop_count = upper;
                }
                if hop_count == 0 || hop_count > 8 {
                    return Err(GpmlError::Unsupported {
                        reason: format!("hop count {hop_count} is outside M003a (1..=8)"),
                    });
                }
            }
            self.expect_punct(']')?;
            self.expect_arrow()?;
            self.expect_punct('(')?;
            last = self.parse_node()?;
            self.expect_punct(')')?;
            hops.push(HopPattern {
                relation,
                hops: hop_count,
            });
        }
        let _ = binder;
        Ok(Pattern { start, hops, last })
    }

    fn peek_punct(&self, ch: char) -> bool {
        matches!(self.peek(), Some((_, Tok::Punct(value))) if *value == ch)
    }

    fn expect_arrow(&mut self) -> Result<(), GpmlError> {
        let (at, tok) = self.next()?;
        match tok {
            Tok::Arrow => Ok(()),
            other => Err(GpmlError::Parse {
                at,
                reason: format!("expected '->', found {other:?}"),
            }),
        }
    }

    fn parse_node(&mut self) -> Result<NodePattern, GpmlError> {
        let mut node = NodePattern {
            variable: None,
            label: None,
            name: None,
        };
        if self.peek_ident_before_punct() {
            node.variable = Some(self.parse_ident("node variable")?);
        }
        if self.peek_punct(':') {
            self.expect_punct(':')?;
            node.label = Some(self.parse_ident("node label")?);
        }
        if self.peek_punct('{') {
            self.expect_punct('{')?;
            loop {
                let key = self.parse_ident("property name")?;
                self.expect_punct(':')?;
                match self.next()? {
                    (_, Tok::Quoted(value)) => {
                        if key.eq_ignore_ascii_case("name") {
                            node.name = Some(value);
                        } else {
                            return Err(GpmlError::Unsupported {
                                reason: format!("property filter {key} is outside M003a"),
                            });
                        }
                    }
                    (_, Tok::Int(value)) => {
                        return Err(GpmlError::Unsupported {
                            reason: format!("non-name property {key}={value} is outside M003a"),
                        });
                    }
                    (_, other) => {
                        return Err(GpmlError::Parse {
                            at: 0,
                            reason: format!("expected quoted property value, found {other:?}"),
                        });
                    }
                }
                if self.peek_punct(',') {
                    self.expect_punct(',')?;
                    continue;
                }
                break;
            }
            self.expect_punct('}')?;
        }
        Ok(node)
    }

    fn peek_ident_before_punct(&self) -> bool {
        matches!(self.peek(), Some((_, Tok::Ident(_))))
    }

    fn parse_return_vars(&mut self) -> Result<(), GpmlError> {
        if self.peek_punct('*') {
            self.expect_punct('*')?;
        } else {
            self.parse_ident("RETURN variable")?;
            while self.peek_punct(',') {
                self.expect_punct(',')?;
                self.parse_ident("RETURN variable")?;
            }
        }
        Ok(())
    }

    fn expect_end(&mut self) -> Result<(), GpmlError> {
        if self.idx >= self.toks.len() {
            Ok(())
        } else {
            let (at, tok) = self.peek().cloned().expect("checked");
            Err(GpmlError::Parse {
                at,
                reason: format!("unexpected trailing token {tok:?}"),
            })
        }
    }
}
