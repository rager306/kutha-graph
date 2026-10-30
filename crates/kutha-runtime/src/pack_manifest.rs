//! Bounded C1b pack manifest: content-addressed declaration metadata.
//!
//! A manifest is JSON data pinned by a `StoreBlob`; it names operators but
//! never carries or executes code. Admission validates identity, schema,
//! operator shape, and cross-pack collisions. This module does not load
//! dictionaries, rebuild leases, or run an operator.

use crate::pack_lifecycle::{PackLifecycleState, derive_pack_lifecycle};
use kutha_common::{Event, Op, sha256_hex};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt;

/// Manifest schema tag. Unknown tags fail closed.
pub const PACK_MANIFEST_SCHEMA: &str = "kutha-pack-manifest/v1";

/// Declared pack identity and read-only operator names.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PackManifest {
    pub schema: String,
    pub pack_id: String,
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub operators: Vec<String>,
}

/// Manifest rejection. Every variant is fail-closed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PackManifestError {
    Malformed,
    Schema { found: String },
    Identity { field: &'static str },
    EmptyOperator,
    InvalidOperator { name: String },
    DuplicateOperator { name: String },
    MissingBlob { digest: String },
    DigestMismatch { digest: String },
    OperatorCollision { name: String, pack_id: String },
}

impl fmt::Display for PackManifestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Malformed => write!(f, "pack manifest is not valid JSON"),
            Self::Schema { found } => {
                write!(
                    f,
                    "pack manifest schema {found:?} is not {PACK_MANIFEST_SCHEMA:?}"
                )
            }
            Self::Identity { field } => write!(f, "pack manifest {field} does not match install"),
            Self::EmptyOperator => write!(f, "pack manifest declares an empty operator"),
            Self::InvalidOperator { name } => {
                write!(
                    f,
                    "pack manifest operator {name:?} is not a valid read-only name"
                )
            }
            Self::DuplicateOperator { name } => {
                write!(f, "pack manifest declares operator {name:?} twice")
            }
            Self::MissingBlob { digest } => {
                write!(f, "pack manifest blob {digest} is missing from the log")
            }
            Self::DigestMismatch { digest } => {
                write!(f, "pack manifest bytes do not match digest {digest}")
            }
            Self::OperatorCollision { name, pack_id } => {
                write!(f, "operator {name:?} is already declared by pack {pack_id}")
            }
        }
    }
}

impl std::error::Error for PackManifestError {}

impl PackManifest {
    /// Build a schema-v1 manifest.
    pub fn new(
        pack_id: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
        operators: Vec<String>,
    ) -> Self {
        Self {
            schema: PACK_MANIFEST_SCHEMA.to_string(),
            pack_id: pack_id.into(),
            name: name.into(),
            version: version.into(),
            operators,
        }
    }

    /// Canonical JSON bytes. Struct field order is stable.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("PackManifest is always serializable")
    }

    /// Content address (`sha256_hex` of [`Self::canonical_bytes`]).
    pub fn digest(&self) -> String {
        sha256_hex(&self.canonical_bytes())
    }

    /// Parse and validate shape only. Identity and collisions need a log.
    pub fn parse(bytes: &[u8]) -> Result<Self, PackManifestError> {
        let manifest: Self =
            serde_json::from_slice(bytes).map_err(|_| PackManifestError::Malformed)?;
        manifest.validate_shape()?;
        Ok(manifest)
    }

    /// Reject unknown schema, empty/invalid/duplicate operator names.
    pub fn validate_shape(&self) -> Result<(), PackManifestError> {
        if self.schema != PACK_MANIFEST_SCHEMA {
            return Err(PackManifestError::Schema {
                found: self.schema.clone(),
            });
        }
        let mut seen = BTreeSet::new();
        for name in &self.operators {
            let trimmed = name.trim();
            if trimmed.is_empty() {
                return Err(PackManifestError::EmptyOperator);
            }
            if !valid_operator_name(trimmed) {
                return Err(PackManifestError::InvalidOperator { name: name.clone() });
            }
            if !seen.insert(trimmed) {
                return Err(PackManifestError::DuplicateOperator { name: name.clone() });
            }
        }
        Ok(())
    }

    /// Require the manifest to describe exactly the installing pack.
    pub fn validate_identity(
        &self,
        pack_id: &str,
        name: &str,
        version: &str,
    ) -> Result<(), PackManifestError> {
        if self.pack_id != pack_id {
            return Err(PackManifestError::Identity { field: "pack_id" });
        }
        if self.name != name {
            return Err(PackManifestError::Identity { field: "name" });
        }
        if self.version != version {
            return Err(PackManifestError::Identity { field: "version" });
        }
        Ok(())
    }
}

/// Read-only operator names: ASCII letters/digits plus `_`, `.`, `-`; no `kutha:`.
pub fn valid_operator_name(name: &str) -> bool {
    !name.is_empty()
        && !name.starts_with("kutha:")
        && name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'.' | b'-'))
}

/// Load and validate a manifest blob by digest. Digest must be a raw SHA-256 address.
pub fn load_manifest(
    blobs: &HashMap<String, Vec<u8>>,
    digest: &str,
) -> Result<PackManifest, PackManifestError> {
    let bytes = blobs
        .get(digest)
        .ok_or_else(|| PackManifestError::MissingBlob {
            digest: digest.to_string(),
        })?;
    if sha256_hex(bytes) != digest {
        return Err(PackManifestError::DigestMismatch {
            digest: digest.to_string(),
        });
    }
    PackManifest::parse(bytes)
}

/// Validate the manifest pinned by `install` against the replayed log.
///
/// Cross-pack collision is checked against operators declared by packs whose
/// replayed lifecycle is `Installed` or `Active`; a retired pack releases its
/// names. The installing pack is excluded from its own collision set.
pub fn validate_install(
    log: &[Event],
    blobs: &HashMap<String, Vec<u8>>,
    install: &Op,
) -> Result<(), PackManifestError> {
    let Op::PackInstall {
        pack_id,
        name,
        version,
        manifest_digest: Some(digest),
    } = install
    else {
        return Ok(());
    };
    let manifest = load_manifest(blobs, digest)?;
    manifest.validate_identity(pack_id, name, version)?;
    let declared = declared_operators(log, blobs)?;
    for operator in &manifest.operators {
        if let Some((owner, _)) = declared
            .iter()
            .find(|(other, ops)| *other != pack_id && ops.contains(operator))
        {
            return Err(PackManifestError::OperatorCollision {
                name: operator.clone(),
                pack_id: owner.clone(),
            });
        }
    }
    Ok(())
}

/// Operators declared by installed/active packs, keyed by pack id.
pub fn declared_operators(
    log: &[Event],
    blobs: &HashMap<String, Vec<u8>>,
) -> Result<BTreeMap<String, BTreeSet<String>>, PackManifestError> {
    let states: HashMap<String, PackLifecycleState> = derive_pack_lifecycle(log)
        .into_iter()
        .map(|record| (record.pack_id, record.state))
        .collect();
    let mut out: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for event in log {
        let Op::PackInstall {
            pack_id,
            name,
            version,
            manifest_digest: Some(digest),
        } = &event.op
        else {
            continue;
        };
        if states.get(pack_id) == Some(&PackLifecycleState::Retired) {
            continue;
        }
        let manifest = load_manifest(blobs, digest)?;
        manifest.validate_identity(pack_id, name, version)?;
        out.entry(pack_id.clone())
            .or_default()
            .extend(manifest.operators);
    }
    Ok(out)
}
