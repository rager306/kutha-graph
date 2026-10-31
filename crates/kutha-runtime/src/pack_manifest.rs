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
            if name.trim().is_empty() {
                return Err(PackManifestError::EmptyOperator);
            }
            // Exact spelling is the declared name; whitespace is not normalized
            // away, so a padded name cannot alias a clean one.
            if !valid_operator_name(name) {
                return Err(PackManifestError::InvalidOperator { name: name.clone() });
            }
            if !seen.insert(name.as_str()) {
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
    // `StoreBlob` admission also accepts rule-body and policy digests; a pack
    // manifest is stricter on purpose: it must be addressed by raw SHA-256 of
    // its canonical bytes, so one blob cannot be read under two identities.
    if sha256_hex(bytes) != digest {
        return Err(PackManifestError::DigestMismatch {
            digest: digest.to_string(),
        });
    }
    PackManifest::parse(bytes)
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

/// Operators declared by one install, for incremental replay.
pub fn declared_operators_for(
    blobs: &HashMap<String, Vec<u8>>,
    install: &Op,
) -> Result<BTreeSet<String>, PackManifestError> {
    let Op::PackInstall {
        pack_id,
        name,
        version,
        manifest_digest: Some(digest),
    } = install
    else {
        return Ok(BTreeSet::new());
    };
    let manifest = load_manifest(blobs, digest)?;
    manifest.validate_identity(pack_id, name, version)?;
    Ok(manifest.operators.into_iter().collect())
}

/// Operator → owning pack index used by admission and replay.
///
/// Replaces the per-install scan of every declared operator set. Each operator
/// maps to the pack that declared it, so collision detection is O(1) per name
/// and a retire drops that pack's names in one pass.
#[derive(Clone, Debug, Default)]
pub struct OperatorIndex {
    owner: BTreeMap<String, String>,
    by_pack: BTreeMap<String, BTreeSet<String>>,
}

impl OperatorIndex {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a pack's operators. Fails on a name already owned elsewhere.
    pub fn declare(
        &mut self,
        pack_id: &str,
        operators: impl IntoIterator<Item = String>,
    ) -> Result<(), PackManifestError> {
        let mut owned = BTreeSet::new();
        for operator in operators {
            if let Some(existing) = self.owner.get(&operator) {
                if existing != pack_id {
                    return Err(PackManifestError::OperatorCollision {
                        name: operator,
                        pack_id: existing.clone(),
                    });
                }
            }
            owned.insert(operator);
        }
        // Re-declaring replaces the previous set: drop names the pack no longer
        // claims so ownership cannot go stale.
        self.retire(pack_id);
        for operator in &owned {
            self.owner.insert(operator.clone(), pack_id.to_string());
        }
        self.by_pack.insert(pack_id.to_string(), owned);
        Ok(())
    }

    /// Release every name a retired pack owned.
    pub fn retire(&mut self, pack_id: &str) {
        if let Some(names) = self.by_pack.remove(pack_id) {
            for name in names {
                self.owner.remove(&name);
            }
        }
    }

    pub fn owner_of(&self, operator: &str) -> Option<&str> {
        self.owner.get(operator).map(String::as_str)
    }

    /// Number of owned operator names. Used by tests and diagnostics.
    pub fn len(&self) -> usize {
        self.owner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.owner.is_empty()
    }
}

/// Validate an install against a blob map and an [`OperatorIndex`].
///
/// Collision detection is a single lookup per declared operator, so replay is
/// linear in packs and operators.
pub fn validate_install_indexed(
    blobs: &HashMap<String, Vec<u8>>,
    install: &Op,
    index: &OperatorIndex,
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
    for operator in &manifest.operators {
        if let Some(owner) = index.owner_of(operator) {
            if owner != pack_id {
                return Err(PackManifestError::OperatorCollision {
                    name: operator.clone(),
                    pack_id: owner.to_string(),
                });
            }
        }
    }
    Ok(())
}
