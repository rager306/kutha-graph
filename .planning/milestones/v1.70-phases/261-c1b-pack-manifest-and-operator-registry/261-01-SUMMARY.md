---
phase: 261-c1b-pack-manifest-and-operator-registry
plan: 01
status: complete
requirements: [C1B-01]
---

# Summary — C1b pack manifest and operator registry

- `PackManifest` schema `kutha-pack-manifest/v1`, canonical JSON, raw SHA-256 address.
- `Op::PackInstall.manifest_digest: Option<String>` with `serde(default)`; legacy rows parse.
- `emit_pack_install_with_manifest` stores the blob then the install; no execution.
- Admission: blob present, digest re-verified, exact identity, unique valid read-only names, no collision with installed/active packs; retired packs release names; padded identifiers rejected.
- Tests: 10/10 manifest; workspace 778 passed / 0 failed.
