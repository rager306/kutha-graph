---
phase: 270-direct-falsifiers-for-manifest-and-payload-digests
plan: 01
status: complete
requirements: [DIG-01]
---

# Summary — Direct falsifiers for manifest and payload digests

- Built blobs addressed by `policy_version_hash` that pass the generic `StoreBlob` gate but are not raw SHA-256.
- Both the manifest and enrichment checks reject them; mutation-checked by disabling the raw check, which fails both tests.
- Raw-addressed manifests still pass and still register their operator.

- Tests: digest falsifiers 5/5; workspace 816 passed / 0 failed; `kutha-gov ci --jobs 1` exit 0.
