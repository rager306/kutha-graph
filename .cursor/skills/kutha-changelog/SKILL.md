---
name: kutha-changelog
description: >-
  Writes dated CHANGELOG.md entries on product / process / Trajectory planes.
  Does not bump versions, tag, or publish GitHub Releases. Hands commit off to
  ce-commit. Use when the user asks for a changelog, dated history, запись в
  CHANGELOG, "сделай changelog", or says release / bump version / GitHub Release
  / 发布 (to refuse SemVer and write history instead).
---

# Kutha changelog (not a release bumper)

Rewrite of [baoyu `release-skills`](https://github.com/JimLiu/baoyu-skills/blob/main/.claude/skills/release-skills/SKILL.md) for this repo.

Baoyu owns SemVer, annotated tags, GitHub Releases, and `chore: release vX`. This repo forbids that until `.kutha/STATE.md` names a release. `git_path_implies` is coupling, not a bumper (`.kutha/META.md`). Commit skill is **ce-commit**; ship/PR skill is **ce-commit-push-pr**. This skill only authors `CHANGELOG.md`.

## Hard stop

Do **not**:

- Edit `Cargo.toml` / `pyproject.toml` versions (stay `0.0.0`)
- Set `publish = true`
- `git tag`, `gh release`, crates.io / `cargo publish`
- Add `.releaserc.yml`, `CHANGELOG.zh.md`, or a Keep a Changelog `[Unreleased]` SemVer bucket
- Invent a second commit protocol (no baoyu “commit each module then release commit”)
- Call **ce-commit-push-pr**, `git push`, or open a PR unless the user explicitly asked to ship/PR
- Treat governor green, honeycomb restage, or a changelog date as ADR **Accepted**

If the user said “release” / “bump” / “GitHub Release” / “发布”: say so in Russian, keep `0.0.0`, and continue with a dated history entry only if they still want the changelog.

## Neighbor skills

| Ask | Skill | This skill does |
|-----|--------|-----------------|
| changelog / dated history | **this** | Write `CHANGELOG.md`, run governor checks |
| commit / save | **ce-commit** | After the entry exists; include `CHANGELOG.md` in the named file list |
| commit + push + PR | **ce-commit-push-pr** | Out of scope unless the user asked to ship |

Read ce-commit when handing off. Do not copy its gather/branch/`-F` steps here.

**docs-coupling vs ce-commit splits:** baoyu commits per module then a release commit. Here `CHANGELOG.md` must be in the **same** commit as plane-bearing paths (`crates/**/*.rs`, harness dicts/scripts, `docs/ADR/**`). Prefer **one** commit for one wave. If ce-commit splits 2–3 concerns, every group that matches those globs must still list `CHANGELOG.md` (extend the dated section before that commit). ADR groups must also list `.kutha/dictionaries/honeycomb.yaml`.

## Planes

| Paths | CHANGELOG home | Also |
|-------|----------------|------|
| `crates/**` | `### Product` | |
| `scripts/kutha_gov/**`, `scripts/tests/**`, `.kutha/**`, `docs/process/**` | `### Process` | |
| `docs/ADR/**`, honeycomb restage | `### Trajectory` and/or H2 `Architecture` | `.kutha/dictionaries/honeycomb.yaml` |

Do not collapse Trajectory into “the product shipped”. Governor green ≠ Accepted ≠ capability.

Language: `CHANGELOG.md` **English**. Chat with the user **Russian**. Commits **English** (ce-commit).

## Workflow

Copy and track:

```text
- [ ] Read STATE freeze + CHANGELOG legend
- [ ] Classify the diff by plane
- [ ] Draft dated English section
- [ ] Insert below the legend, above the previous H2
- [ ] Restage honeycomb.yaml if ADRs changed
- [ ] uv run kutha-gov precommit --check changelog-planes
- [ ] uv run kutha-gov precommit --check docs-coupling  (if plane-bearing paths are in the tree)
- [ ] Hand off to ce-commit only if the user asked to commit
```

### 1. Context

Read `.kutha/STATE.md` (lease + freeze) and the `CHANGELOG.md` legend (must keep needles `not GitHub Releases`, `product`, `process`, `Trajectory`).

Window: working tree + commits since the last `CHANGELOG.md` H2, or the current uncommitted wave. Do not scan “since last git tag” as a version source.

### 2. Classify

Conventional-commit types (`feat`/`fix`/…) are hints for `Added` / `Changed` / `Fixed`, not for SemVer. Skip pure `chore`/`style` noise unless it changes a plane contract.

H2 title:

- One plane → `## YYYY-MM-DD — {Product|Process|Architecture}: {short title}`
- Cross-plane wave → `## YYYY-MM-DD — {title}` plus H3 `Product` / `Process` / `Trajectory`

Date: the session date (`YYYY-MM-DD`). Do not mint “Wave N” unless STATE/ROADMAP already names it.

Always add a **Trajectory** H3 when ADRs, lease, freeze, or named FF tests moved; otherwise one line that lease/cells are unchanged.

Keep a Changelog groups (`Added` / `Changed` / `Fixed`) **inside** plane headings. Omit empty groups.

### 3. Insert

Place the new H2 immediately after the legend, before older dated sections. Do not rewrite prior entries. Do not leave `[Unreleased]` as a version waiting room; date it before handoff to ce-commit.

If ADRs changed, restage `.kutha/dictionaries/honeycomb.yaml` (`uv run kutha-gov map` to sanity-check). Do not mark cells Accepted.

### 4. Verify

```text
uv run kutha-gov precommit --check changelog-planes
uv run kutha-gov precommit --check version-freeze
```

Add `docs-coupling` when crates, harness, or ADRs are in the same tree. Fix HIGHs before any commit handoff.

### 5. Handoff

- Changelog only → stop. Report the H2 title in Russian.
- User asked to commit → **ce-commit**. Named files include `CHANGELOG.md` (and `honeycomb.yaml` when ADRs moved). Message states the outcome, not “update changelog”. Not `chore: release v…`.
- User asked to push/PR → **ce-commit-push-pr** after the changelog exists; still no tag/release/version bump.

## Dry-run

If the user wants a preview only: print the proposed H2+body, the plane classification, and “no files written”. Do not edit, commit, or push.

## Example

```markdown
## 2026-09-14 — Process: L_map compact honeycomb index

### Added

- Compact L_map index `.kutha/dictionaries/honeycomb.yaml` plus `kutha-gov map`.

### Process

- Governor validates shape and links; it does not accept cells or treat the index as a backlog.

### Trajectory

- Cells remain **Proposed**. Lease stays H3; next thin slice is still H4.
```
