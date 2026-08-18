# ADR-000: Kutha Hybrid Architecture — Research Foundation

## Status

**Proposed** (foundation ADR — D1–D10 locked; honeycomb Proposed; P0 heartbeat+snapshot spike exists; not Accepted product runtime)

## Date

2026-08-15

## Context

### Проблема

Нужен графовый движок, в котором:

1. **агенты и GenAI-обогащение живут внутри runtime**, а не как надстройка над storage;
2. **время и аудит** — first-class (bi-temporal, lineage, point-in-time, fork);
3. **производительность** на multi-hop / vector / analytics сопоставима с state-first graph-vector СУБД (ориентир: Samyama / FalkorDB-класс);
4. **расширяемость** — плагины с обратимыми эффектами, без переписывания ядра.

Классические варианты не закрывают всё сразу:

| Подход | Сильная сторона | Слабость для цели |
|--------|-----------------|-------------------|
| State-first graph DB (Samyama-like) | Throughput, CSR, MVCC, HNSW | Agents / full history / fork — надстройка |
| Чистый event-sourced reactive graph (ActiveGraph) | Lineage, replay, behaviors | Write amp, latency multi-hop, storage bloat |
| Memory frameworks (Graphiti/Zep, Mem0) | Temporal agent memory | Не СУБД; зависят от внешнего графа |
| Document + in-DB GenAI (RavenDB) | Agents inside DB | Слабый native graph |

### Источники синтеза (research inputs)

- **ActiveGraph**: event log = SoT; graph = deterministic projection; behaviors.
- **Cordis**: Everything is a Plugin; spatiotemporal composability; reversible effects.
- **Cui («背包问题九讲»)**: generalized items → max-convolution / composition of local transition functions.
- **Samyama**: RocksDB, arenas/MVCC, CSR, vectorized executor, cost planner, enrichment.
- **FalkorDB**: sparse matrix / GraphBLAS-like hot path, low-latency traversal + vector.
- **Graphiti / Zep**: bi-temporal edges, fact invalidation, episodic ingestion.
- **RavenDB GenAI**: continuous GenAI tasks + server-side agents with tools.
- **ruVector / RVF**: HNSW+GNN, agent memory types, COW, witness chain, portable cognitive containers.
- **HelixDB**: indexes as access paths; tiered storage.
- **Tarantool**: compute-close-to-data; hot in-mem / cold disk.

### Целевые вертикали (порядок приоритета)

1. Legal temporal agents (bi-temporal norms, deterministic audit)
2. Regulated finance / compliance
3. Scientific / biomedical knowledge evolution
4. Horizontal agent runtime (shared temporal memory)
5. Enterprise knowledge + security plane

### Именование

Рабочее имя продукта/репо: **Kutha** / **kutkha** (камчатский Ворон-творец; tech-пространство относительно чистое). Домены `.com/.io/.ai` заняты; `.dev` — кандидат. Финальный бренд — отдельный ADR.

---

## Decision

**Принятая исследовательская гипотеза (working architecture):**

> Kutha — гибридный AI-native graph engine на Rust: **event-sourced reactive core** (control / agent / lineage plane) + **pluggable high-performance materializations** (data plane) + **meta-prompt + external dictionaries** как универсальный управляющий слой агентов.

Это **не** «Samyama поверх ActiveGraph» и **не** чистая замена Samyama. Это дифференцированный продукт в квадранте *high temporal × high hybrid × in-DB agents*.

### D1. Двухплоскостная модель

```
Client / Cypher + Hybrid + Temporal AS OF + Agent API
                    │
        Behavior & Agent Runtime (Cordis-style)
        • Behaviors / Relation Behaviors
        • GenAI Enrichment (RavenDB-style)
        • Meta-prompt + Dictionaries (control)
        • Spatiotemporal composition + reversible effects
                    │ emit / react
        Append-only Event Log (SoT, lineage, fork)
                    │ project
        Materialized Projections (pluggable, reversible)
        • Hot CSR / sparse / GraphBLAS-like
        • HNSW (+ optional GNN)
        • Bi-temporal views
        • Agentic memory views
        • Secondary / FTS / property columns
                    │
        Storage: RocksDB / object store + WAL (+ optional Raft)
```

| Плоскость | SoT / роль | Не является |
|-----------|------------|-------------|
| **Control** | Event log + behaviors | Hot adjacency layout |
| **Data** | Materializations | Источник истины |
| **Agent** | Meta-prompt + dictionaries + memory packs | Свободный LLM без валидации |

### D2. Формальная модель реактивности

\[
L = [e_1,\ldots,e_n],\quad
e_i=\langle id, type, payload, causedBy\rangle
\]

\[
G(L)=\mathrm{foldl}(\mathrm{apply\_event}, G_0, L)
\]

\[
B:(e,G)\to \mathrm{Option}(Patch)
\]

- Прямой mutate графа запрещён; только patch → validate → `patch.proposed` / `patch.applied`.
- **Relation Behaviors** инкапсулируют координацию на рёбрах.
- Модуль/pack ≈ **обобщённый предмет Cui** \(h_i\); глобальная логика — композиция локальных функций (max-convolution / аналог).
- Runtime quantum: emit → log+project → trigger \(B\)/\(R_B\) → каскад до стабилизации.
- Идентификаторы: **UUID v7** (монотонность + индексность).

### D3. Управление агентами: meta-prompt + dictionaries (не hard FSM)

Отклонён жёсткий FSM как единственная модель. Принят универсальный слой:

1. **Meta-prompt** — версионируемая «конституция» (режимы, tool policy, temporal semantics, conflict handling); сама temporal.
2. **External dictionaries** (first-class temporal graph entities):
   - Controlled vocabulary
   - Action dictionary
   - Relation dictionary
   - State / mode dictionary
   - Policy dictionary
   - Domain ontology / schema

**Цикл:** request → load meta-prompt@T → load dictionaries@T → propose → validate(dicts+security) → execute → event log.

FSM / statecharts **выводимы** из State+Action dictionaries (для аудита/визуализации), не захардкожены в коде.

Enterprise: ABAC + temporal policies + capability sandbox агентов; policy-as-graph.

### D4. Bi-temporal как семантика фактов

Каждый факт/ребро: `valid_from`, `valid_to`, `ingested_at`, `invalidated_at` (+ provenance).  
Invalidation — behavior, не silent overwrite. Query: `AS OF`, temporal filters — native.

### D5. Materializations as plugins

CSR/HNSW/temporal slices/memory views:

- инкрементально поддерживаются behaviors;
- **reversible** (unload → откат побочных эффектов);
- допускают конкурирующие проекции.

RVF-подобные контейнеры: portable agent memory, sealed knowledge units, checkpoint/fork с witness chain — **transport / packaging layer**, не замена hot RocksDB/CSR path.

### D6. Язык и runtime ядра

| Слой | Выбор | Почему |
|------|--------|--------|
| Core engine | **Rust + Tokio + RocksDB (event store)** | No GC, workspaces=границы, serde, channels |
| Agent I/O / early composition | TS/Cordis-like SDK допустим | I/O-bound LLM; не materializer |
| Java / C# | Обвязка enterprise apps | Не ядро движка |

Compile-time hexagon: crates + `pub(crate)`. Ошибки behavior → `behavior.failed` (`Result`), не panic. Content-addressed cache внешних tool/LLM вызовов — обязателен для cheap replay/fork.

### D7. Профили нагрузки (один продукт — три режима)

| Профиль | RAM | CPU | Disk | Следствие для дизайна |
|---------|-----|-----|------|------------------------|
| **A OLTP** | низкая (агрегаты) | serde / outbox | **WAL IOPS** | lean events, hot entity cache |
| **B Temporal KG** | **критична** (граф in-mem) | fold / Cypher / as-of | read-heavy + temporal idx | CSR projection residency |
| **C Agents** | средняя | низкая на хосте | **объём lineage** | tiered archive, compression |

### D8. Обязательные антипаттерн-контроллеры

1. **Replay tax \(O(N)\)** → snapshots + log segmentation.  
2. **Single-writer bottleneck** → RocksDB/SQLite WAL, batching.  
3. **Fat events** → lean deltas + ids; blobs через object store / read ports.

### D9. Тестирование как first-class capability

| Класс | Назначение |
|-------|------------|
| Sociable unit + in-memory ports | Use-cases без мок-зоопарка |
| Strict replay | `ReplayDivergenceError` |
| Fork & diff | Counterfactual |
| Adapter + Testcontainers | SQLx/Postgres и т.д. |

### D10. Позиционирование (value prop)

> Event-sourced bi-temporal graph engine on Rust where agents are first-class citizens, governed by meta-prompts and external dictionaries, with performance delivered by pluggable materializations — not agents bolted onto storage.

White space vs Graphiti/Zep (memory layer), FalkorDB (perf, weak agents/temporal), RavenDB (agents, weak graph), Samyama (perf + enrichment as add-on).

---

## Consequences

### Становится проще

- Audit, PITR, fork, legal/finance explainability.
- Добавление indexes / enrichment / solvers как packs.
- Replay/fork-тесты вместо тяжёлых стендов.
- Универсальный agent control across verticals (меняются словари, не runtime).

### Становится сложнее

- Две плоскости (log + materializers) — больше семантики consistency.
- Incremental materialization и conflict detection — R&D cost.
- In-DB agents → жёсткий security plane (иначе unshippable for enterprise).
- Найм/скорость разработки на Rust выше по людям, ниже по infra TCO.

### Оценки fit (research)

| Направление | Оценка |
|-------------|--------|
| Чистая замена Samyama по сырому throughput | 5–6/10 |
| AI-native / agentic KG + GraphRAG + multi-agent memory | 8.5–9.5/10 |
| Temporal / auditable / regulated | 9/10 |
| Гибрид (этот ADR) | 8–9/10 |

---

## Alternatives Considered

| Альтернатива | Вердикт |
|--------------|---------|
| Чистый state-first Samyama + bolted agents | Отвергнуто как продукт; заимствовать materializers |
| Чистый ActiveGraph без hot projections | Отвергнуто для B/C scale |
| Hard FSM как единственный agent control | Отвергнуто; FSM выводим из dictionaries |
| TS/Node как ядро графа | Отвергнуто для B; допустим для agent I/O |
| Внешний ETL + внешний agent orchestrator | Отвергнуто как целевая идеология; ingestion = reactive packs |
| RVF как primary storage | Отвергнуто; RVF = portable/cognitive packaging |

---

## Open Research Questions

Вопросы, которые **обязаны** породить follow-on ADR или spike до Accepted статуса соответствующих решений.

### R1. Consistency & concurrency

- Какая модель между log append и projection apply? (single-writer serial / sharded logs / causal)
- Как детектировать конфликты patch при параллельных behaviors?
- Эквивалент MVCC на versioned events vs arena-MVCC только в projection?

### R2. Materialization protocol

- API reversible plugin: mount / apply_delta / snapshot / unload / rollback.
- Инкрементальный CSR/HNSW update cost vs periodic rebuild.
- Несколько конкурирующих projections одного типа — семантика query routing.

### R3. Bi-temporal query & storage

- Физическая раскладка: edge versions vs separate temporal index.
- AS OF + multi-hop cost model.
- Invalidation cascade bounds (не взрывать reactive fan-out).

### R4. Agent control plane

- Схема сущностей: `MetaPrompt`, `Dictionary`, `DictionaryEntry`, `AgentInstance`.
- Кто главный: LLM propose vs dictionary validate (fail-closed).
- Capability model vs RBAC; policy-as-graph schema.
- Связь meta-prompt@version с witness/provenance решения.

### R5. Event schema & storage

- Lean event taxonomy (create/patch/invalidate/behavior.* / llm.*).
- Snapshot format; segment compaction; tiered cold storage.
- Content-addressed LLM/tool cache keys and eviction.

### R6. Query surface

- Cypher subset + temporal extensions + hybrid (vector+graph+FTS) composition.
- Planner: cost across projections; late materialization borrow from Samyama ADR-012.

### R7. Security & multi-tenancy

- ABAC attributes on node/edge/temporal slice/agent action/container.
- Sandbox для in-DB agents (WASM? process? crate-level?).
- Shared public knowledge (law corpus) + private tenant data.

### R8. RVF / portable units

- Какой subset сегментов нужен в v1 (WITNESS, VEC, GRAPH, META, WASM)?
- Mapping: graph fork ↔ RVCOW; event segment ↔ RVF file.

### R9. Benchmarks & success metrics

- Legal PIT: correctness + latency for norm@date.
- Agent replay: divergence rate; fork cost.
- Graph: hops/s and ingest vs Samyama/FalkorDB baselines (не цель обогнать всегда — цель «близко на hot path»).

### R10. Branding & licensing

- kutkha vs kutha vs alternatives; trademark pass.
- License for core vs agent packs vs legal ontology packs.

---

## Research & Detailing Backlog

Порядок детализации (каждое → отдельный ADR или spike report).

| Phase | Тема | Выход | Depends |
|-------|------|-------|---------|
| **P0** | Event log + apply + behavior quantum (Rust spike) | Working mini-runtime; **ADR-010** (+ Vision/STCA ADR-001/002) | — |
| **P0** | Lean event schema + UUID v7 + caused_by | **ADR-011** | P0 runtime |
| **P0** | Snapshot + segment strategy | **ADR-012** | P0 log |
| **P1** | Bi-temporal edge model + invalidation behavior | **ADR-013** | P0 |
| **P1** | CSR materialization plugin (reversible stub) | **ADR-040** | P0 |
| **P1** | Meta-prompt + dictionary entity model | **ADR-050** | P0 |
| **P2** | HNSW plugin + hybrid retrieve stub | ADR Hybrid Query | P1 CSR |
| **P2** | GenAI enrichment behavior (RavenDB-pattern) | ADR Enrichment | P1 agent |
| **P2** | Security plane sketch (ABAC + capabilities) | ADR Security | P1 agent |
| **P3** | Legal reference vertical (norm@date MVP) | Case study + API | P1 temporal, P1 agent |
| **P3** | RVF packaging subset | ADR Portable Containers | P0–P2 |
| **P3** | Benchmark harness vs baselines | BENCHMARKS.md | P1–P2 |
| **P4** | Raft / multi-node (optional) | ADR Distribution | storage mature |
| **P4** | GNN learned index (ruVector-inspired) | ADR Learned Indexes | P2 HNSW |

### Критерии выхода из Research → Proposed для этого ADR

- [x] Spike P0 runtime демонстрирует emit → cascade → stable graph + replay.
- [x] Зафиксированы lean event types и snapshot story. *(lean events: `kutha-common`; snapshot = fold+dict+offset lease, JSONL semantic log; Rocks WAL still crash cousin, not SoT)*
- [x] Написаны черновики ADR Temporal + Materialization API + Agent Control. *(honeycomb 010–014, 040–043, 050 Proposed)*
- [x] Явно измерены: replay time vs log length; cascade depth bounds. *(2026-08-18: `replay_check` log_len=200 elapsed_ms≈2.0; `max_cascade` abort covered)*
- [x] Security threats для in-DB agents перечислены и имеют mitigation sketch. *(ADR-051/080/081 Proposed — sketch, not kernel)*

---

## Follow-on ADR Skeleton (to open)

**Superseded numbering (2026-08-16):** the spine is now **ADR-001 Overall Vision** → **ADR-002 STCA**, then **honeycomb cells ADR-010+** (see `docs/ADR/README.md`). Historical working titles below are remapped; do not open new ADRs under the old 001–012 runtime list.

| Old sketch | New cell (working) |
|------------|--------------------|
| ADR-001 Event Log & Reactive Runtime Quantum | **ADR-010** (Time band) |
| ADR-002 Lean Event Schema & Lineage | **ADR-011** |
| ADR-003 Snapshots, Segments & Tiered Storage | **ADR-012** |
| ADR-004 Bi-Temporal Facts & Invalidation | **ADR-013** |
| ADR-005 Materialization Plugin Protocol | **ADR-040** (Data band) |
| ADR-006 Meta-Prompt & Dictionary Agent Control | **ADR-050** (Agent band) |
| ADR-007 Capability Security Plane | **ADR-080** / **ADR-051** |
| ADR-008 Hybrid Query Surface | **ADR-070** / **ADR-043** |
| ADR-009 GenAI Enrichment Behaviors | **ADR-052** |
| ADR-010 Portable Cognitive Containers (RVF) | **ADR-091** |
| ADR-011 Legal Vertical Reference Architecture | **ADR-090** |
| ADR-012 Naming, Branding & License | **ADR-092** |

Spine (open / Research):

| ID | Title |
|----|--------|
| [ADR-001](./ADR-001-kutha-overall-vision.md) | Kutha Overall Vision (STCA on top of idea stack) |
| [ADR-002](./ADR-002-stca-paradigm.md) | Spatiotemporal Compositional Architecture (STCA) |

---

## References

### Internal synthesis (session 2026-08-15)

- Grok dialogue: hybrid synthesis, RVF, naming (kutkha), competitors, legal/scientific GTM, FSM → meta-prompt+dictionaries.
- ActiveGraph + Cordis + Cui formalization + Rust prototype sketch.
- Resource profiles A/B/C; Rust/Java/C#/TS comparison.
- Rust reliability / async / bottlenecks / replay-fork testing / strangler scale.

### External systems (comparative)

Graphiti/Zep, FalkorDB, HelixDB, Samyama, ruVector/RVF, RavenDB GenAI, Tarantool, Mem0, Letta; architecture lists (awesome-software-architecture, system-design-primer) as general background only.

### Related Samyama ADRs (borrow / contrast)

- ADR-001 Rust, ADR-002 RocksDB, ADR-006 Tokio — align.
- ADR-012 Late materialization, ADR-015 planning, ADR-020 MVCC — contrast: SoT in Kutha is log, not arena state.
- ADR-016 distributed — defer until single-node hybrid proven.

---

## Related Decisions

- None yet (root research ADR).
- Supersedes: informal notes from Grok share «DeepSeek Harness: Plugin Paradigm for AI».

---

## One-liner

**Kutha = event log as truth + reversible projection plugins for speed + meta-prompt/dictionaries for in-DB agents — researched on Rust, validated by replay/fork, aimed at legal/regulated temporal agent workloads first.**
