---
id: harvey-lab-firm-knowledge
source: harvey
axes: [Query, Agent, Space, Time, Verify]
usefulness: high
optimality: low
demand: high
confidence: code
layer5: "Legal practice overlay: cross-matter retrieval over a firm DMS (matters, correspondence, deal files) — not statutory RAG and not Kutha SoT"
status: closed
channels_failed: []
---

# Firm knowledge is a matter graph — not statutes and not grep-as-SoT

User URL: https://github.com/harveyai/harvey-labs/tree/main/tasks/firm-knowledge (cloned `/tmp/user-url-scout/harvey-labs`, **do not vendor** ~63k files). Harvey LAB: legal-agent benchmark (tasks + filesystem harness + LLM-judge rubrics). This card is the **firm-knowledge** slice only. Read: `tasks/firm-knowledge/tasks/001/task.json` — instructions to pull every antitrust matter that drew an HSR second request, with document evidence; rubric criteria name matter IDs (`1003-00001` …) and a **precision** criterion (C-007: do not assert any qualifying matter outside a closed set); `docs_dir: ../../dms`; `dms/matters/` — 266 synthetic matters, 250 tasks sharing one DMS; `docs/architecture.md` — no database: agent tools are bash/read/write/glob/grep. Distinct from `paper-statutory-temporal-qa` / `paper-sat-graph-legal-rag` / `paper-legalsearch-r1-temporal-agent` (public norms / RL statute search), `law-nexus-kb-ontology-catalog` (YAML normative AST), `dify-workflow-rag-orchestration` (RAG canvas).

## 1. Raw idea

A law firm’s private store: **matters** with correspondence, memos, deal files. The hard query is cross-matter (“every HSR second-request we handled”) with **recall and precision** against a gold matter-ID set. LAB evaluates agents that grep a synthetic DMS. That is the *benchmark*, not the engine.

## 2. STCA applicability

Space 090 practice overlay: matters are named-graph / tenant-slice cousins (counsel–client). Query: this is MATCH over a **firm knowledge graph** (matter–document–party–event), not cosine chunk RAG and not statute AS-OF. Time: correspondence dates and closing emails are valid-time on *practice facts*; governing law still comes from the statutory pack. Agent: LAB’s grep harness is the anti-pattern; Kutha should compile the instruction to AS-OF Cypher over the DMS fold. Verify: rubric precision (no extra matters) is a why-not / receipt demand. Do not ingest the synthetic DMS as SoT.

## 3. Quality / cost

Usefulness high: ADR-090 is empty without practice overlay. Optimality low for the harness (filesystem grep, LLM judge). Cost: use LAB firm-knowledge as an **eval pack** (task + gold matter IDs); do not vendor harvey-labs; other practice-area folders stay skip-unless a second eval card.

## 4. Demand

Counsel will ask “show every deal where we got a second request” and punish extra matters. Engine demand: matter-typed fold + fail-closed precision, not glob.

## 5. Niche → effect

Legal practice overlay: cross-matter retrieval over a firm DMS (matters, correspondence, deal files) — not statutory RAG and not Kutha SoT
