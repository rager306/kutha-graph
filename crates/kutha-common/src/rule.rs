//! Bounded positive rule AST and canonical encoding (M016 S02 / RULE-01).
//! Evaluation is Phase 38. No negation, no aggregates, no JSON-as-SoT.

use sha2::{Digest, Sha256};

const TAG_NAMED: u8 = 0x01;
const TAG_HORN: u8 = 0x02;
const TAG_VAR: u8 = 0x00;
const TAG_CONST: u8 = 0x01;
const MAX_IDENT: usize = 64;
const MAX_PREMISES: usize = 8;
const MAX_VAR: u8 = 15;

/// Bounded positive rule body. Hash of [`RuleAst::encode`] is `rule_version`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuleAst {
    Named {
        name: String,
    },
    Horn {
        head: RuleAtom,
        premises: Vec<RuleAtom>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuleAtom {
    pub relation: String,
    pub subject: RuleTerm,
    pub object: RuleTerm,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuleTerm {
    Var(u8),
    Const(String),
}

/// Identifier: `[A-Za-z][A-Za-z0-9_-]{0,63}`.
pub fn is_rule_ident(s: &str) -> bool {
    let b = s.as_bytes();
    if b.is_empty() || b.len() > MAX_IDENT {
        return false;
    }
    if !b[0].is_ascii_alphabetic() {
        return false;
    }
    b[1..]
        .iter()
        .all(|c| c.is_ascii_alphanumeric() || *c == b'_' || *c == b'-')
}

fn hex32(digest: [u8; 32]) -> String {
    let mut out = String::with_capacity(64);
    for b in digest {
        out.push_str(&format!("{b:02x}"));
    }
    out
}

/// SHA-256 of domain `kutha-rule-def` then canonical body bytes.
pub fn rule_body_hash(canonical: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(b"kutha-rule-def");
    h.update(canonical);
    hex32(h.finalize().into())
}

impl RuleTerm {
    pub fn var(i: u8) -> Option<Self> {
        (i <= MAX_VAR).then_some(Self::Var(i))
    }

    pub fn constant(s: impl Into<String>) -> Option<Self> {
        let s = s.into();
        is_rule_ident(&s).then_some(Self::Const(s))
    }

    fn valid(&self) -> bool {
        match self {
            RuleTerm::Var(i) => *i <= MAX_VAR,
            RuleTerm::Const(s) => is_rule_ident(s),
        }
    }
}

impl RuleAtom {
    pub fn new(relation: impl Into<String>, subject: RuleTerm, object: RuleTerm) -> Option<Self> {
        let relation = relation.into();
        if !is_rule_ident(&relation) || !subject.valid() || !object.valid() {
            return None;
        }
        Some(Self {
            relation,
            subject,
            object,
        })
    }
}

impl RuleAst {
    pub fn named(name: impl Into<String>) -> Option<Self> {
        let name = name.into();
        is_rule_ident(&name).then_some(Self::Named { name })
    }

    pub fn horn(head: RuleAtom, premises: Vec<RuleAtom>) -> Option<Self> {
        if premises.is_empty() || premises.len() > MAX_PREMISES {
            return None;
        }
        if !head.relation.is_empty()
            && is_rule_ident(&head.relation)
            && head.subject.valid()
            && head.object.valid()
            && premises
                .iter()
                .all(|a| is_rule_ident(&a.relation) && a.subject.valid() && a.object.valid())
        {
            Some(Self::Horn { head, premises })
        } else {
            None
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        match self {
            RuleAst::Named { name } => {
                out.push(TAG_NAMED);
                put_ident(&mut out, name);
            }
            RuleAst::Horn { head, premises } => {
                out.push(TAG_HORN);
                put_atom(&mut out, head);
                out.push(premises.len() as u8);
                for p in premises {
                    put_atom(&mut out, p);
                }
            }
        }
        out
    }

    /// Exact-consume decode. Trailing bytes, unknown tags, and invalid idents fail.
    pub fn from_canonical(bytes: &[u8]) -> Option<Self> {
        let mut rest = bytes;
        let ast = parse_ast(&mut rest)?;
        rest.is_empty().then_some(ast)
    }

    pub fn hash(&self) -> String {
        rule_body_hash(&self.encode())
    }
}

fn put_ident(out: &mut Vec<u8>, s: &str) {
    let n = u16::try_from(s.len()).unwrap_or(0);
    out.extend_from_slice(&n.to_le_bytes());
    out.extend_from_slice(s.as_bytes());
}

fn put_atom(out: &mut Vec<u8>, atom: &RuleAtom) {
    put_ident(out, &atom.relation);
    put_term(out, &atom.subject);
    put_term(out, &atom.object);
}

fn put_term(out: &mut Vec<u8>, term: &RuleTerm) {
    match term {
        RuleTerm::Var(i) => {
            out.push(TAG_VAR);
            out.push(*i);
        }
        RuleTerm::Const(s) => {
            out.push(TAG_CONST);
            put_ident(out, s);
        }
    }
}

fn take_u8(rest: &mut &[u8]) -> Option<u8> {
    let (b, r) = rest.split_first()?;
    *rest = r;
    Some(*b)
}

fn take_u16(rest: &mut &[u8]) -> Option<u16> {
    if rest.len() < 2 {
        return None;
    }
    let v = u16::from_le_bytes([rest[0], rest[1]]);
    *rest = &rest[2..];
    Some(v)
}

fn take_bytes<'a>(rest: &mut &'a [u8], n: usize) -> Option<&'a [u8]> {
    if rest.len() < n {
        return None;
    }
    let (h, t) = rest.split_at(n);
    *rest = t;
    Some(h)
}

fn take_ident(rest: &mut &[u8]) -> Option<String> {
    let n = take_u16(rest)? as usize;
    let b = take_bytes(rest, n)?;
    let s = std::str::from_utf8(b).ok()?;
    is_rule_ident(s).then(|| s.to_string())
}

fn parse_term(rest: &mut &[u8]) -> Option<RuleTerm> {
    match take_u8(rest)? {
        TAG_VAR => RuleTerm::var(take_u8(rest)?),
        TAG_CONST => RuleTerm::constant(take_ident(rest)?),
        _ => None,
    }
}

fn parse_atom(rest: &mut &[u8]) -> Option<RuleAtom> {
    let relation = take_ident(rest)?;
    let subject = parse_term(rest)?;
    let object = parse_term(rest)?;
    RuleAtom::new(relation, subject, object)
}

fn parse_ast(rest: &mut &[u8]) -> Option<RuleAst> {
    match take_u8(rest)? {
        TAG_NAMED => RuleAst::named(take_ident(rest)?),
        TAG_HORN => {
            let head = parse_atom(rest)?;
            let n = take_u8(rest)? as usize;
            if n == 0 || n > MAX_PREMISES {
                return None;
            }
            let mut premises = Vec::with_capacity(n);
            for _ in 0..n {
                premises.push(parse_atom(rest)?);
            }
            RuleAst::horn(head, premises)
        }
        _ => None,
    }
}

#[cfg(test)]
mod hash_lock {
    use super::*;

    #[test]
    fn named_hash_is_not_utf8_domain_hash() {
        let ast = RuleAst::named("derive_pq").unwrap();
        let pin = ast.hash();
        let mut h = Sha256::new();
        h.update(b"kutha-rule-def");
        h.update(b"derive_pq");
        let utf8 = hex32(h.finalize().into());
        assert_ne!(pin, utf8);
        assert_eq!(pin, rule_body_hash(&ast.encode()));
    }
}
