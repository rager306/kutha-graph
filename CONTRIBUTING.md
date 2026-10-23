# Contributing

Kutha is a research engine. Read [`README.md`](README.md), then [`AGENTS.md`](AGENTS.md) and [`.kutha/STATE.md`](.kutha/STATE.md), before changing code. [`.kutha/STATE.md`](.kutha/STATE.md) is the lease. [`.planning/STATE.md`](.planning/STATE.md) is session focus and does not thaw a frozen item.

## What a change may do

- One slice at a time. Do not mark an ADR **Accepted**. A cell stays **Proposed** until that cell is in the running engine.
- Do not bump `0.0.0`, add a git tag, or publish a GitHub Release unless STATE says so.
- Do not add an SPDX `LICENSE` file. ADR-092 D092-2 leaves the license undecided. Until a human chooses one, this repository grants no license to copy, modify, or redistribute.
- Docs, identifiers, and commit messages are English. Chat with the owner is Russian.

## Checks

```text
cargo test --workspace
cp .env.example .env   # once
uv run kutha-gov ci
uv run kutha-gov precommit
```

Python is **3.13** via **uv**, not system `python3`. Product tests that touch the store use the repo-local `tmp/` helper and remove it. Format Rust with `rustfmt` edition 2024 (`rustfmt.toml`).

GitHub Actions runs `uv run kutha-gov ci --jobs 1` on `main`, `work/main-line`, and pull requests. See [`docs/process/repository.md`](docs/process/repository.md).
