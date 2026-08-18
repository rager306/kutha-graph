# Руководство: Spatiotemporal Compositional Architecture (STCA) на Rust

**Статус:** рабочий манифест-спецификация для Kutha / kutha-graph  
**Связь:** нормативная опора **ADR-002 (STCA)**; видение продукта — **ADR-001**; фундамент D1–D10 — **ADR-000**; детализация — соты ADR-010+  
**Дата:** 2026-08-16

Каждая концепция заземлена на исходные материалы. Ссылки на авторов — в тексте; номера пассажей из исходного синтеза помечены как `[i]` там, где нужна трассировка к Studio-источнику.

---

## 1. Синтетическая архитектурная парадигма (STCA)

Парадигма **Spatiotemporal Compositional Architecture (STCA)** объединяет:

1. **Пространственный контур** — статическое доменное разделение (вертикальные срезы, порты).
2. **Временной контур** — событийно-порождённый граф (append-only event log → детерминированная проекция).
3. **Композиционный мост** — математика обобщённых предметов и max-свёртки (Cui).

```
  [ ПРОСТРАНСТВО (Space) ]              [ ВРЕМЯ (Temporal) ]
  Вертикальные доменные срезы            Append-only Event Log
  Cockburn / Palermo / Martin            ActiveGraph (Nakajima)
  + Vertical Slices (Drotbohm)           arXiv:2605.21997
           │                                      │
           ▼                                      ▼
  Свободный домен + Ports               Детерминированный fold
  (адаптеры снаружи)                    (граф = проекция лога)
           │                                      │
           └──────────────────┬───────────────────┘
                              ▼
                   [ ЯДРО STCA ]
              Spatiotemporal Context
                              ▲
                              │
                   [ Маппинг и свёртка ]
                 Cui «背包问题九讲» (max-convolution)
```

**Формула Kutha:** STCA-ядро + pluggable hot materializations (CSR/HNSW/…) + dict-first агенты + нативная bi-temporal семантика (референс Graphiti, без Graphiti runtime).

### 1.1. Пространственная изоляция (Space)

- **Классика:** гексагональная архитектура (Alistair Cockburn, 2005), onion (Jeffrey Palermo, 2008), clean architecture (Robert C. Martin, 2012) — зависимости направлены к чистому домену.
- **Эволюция 2025–2026:** техническая нарезка на папки `ports` / `adapters` / `domain` даёт низкую cohesion при смене требований. По Oliver Drotbohm (Spring I/O / CodeCrafts 2026) — **функциональная декомпозиция на вертикальные срезы (Vertical Slices / Sliced Onion)**. Гексагон живёт *внутри* доменного модуля; модули общаются только через **Ports**.
- **Метафреймворк:** spatiotemporal composability (Cordis / DeepSeek Harness) — компоненты как автономные плагины (Pack), взаимодействие через контекст; эффекты обратимы.

### 1.2. Временная изоляция (Time)

- Состояние **не** перезаписывается на месте (не State-on-Write).
- По ActiveGraph (Yohei Nakajima, 2026, “The Log is the Agent”, arXiv:2605.21997): **единственный SoT — упорядоченный append-only Event Log**.
- Граф \(G_t = \mathrm{foldl}(\mathrm{apply}, G_0, L_t)\) — детерминированная проекция.
- **Behaviors** и **Relation Behaviors** реагируют на топологию/события, предлагают **Patches**; рантайм валидирует и пишет в лог.
- Принцип: «Граф — мир. Поведения — физика. Лог — доказательство» (Nakajima, 2026).

### 1.3. Алгоритмический мост (Cui)

- Модуль \(M_i\) ≈ **обобщённый предмет**: локальная \(h_i(v)\) при бюджете ресурса \(v\).
- Глобальное состояние — **max-convolution** локальных функций (Tianyi Cui, «Nine Lectures on the Knapsack Problem», лекция 8).
- В Kutha: распределение cascade-бюджетов, лимитов агентов, конкуренции плагинов материализаций — без жёсткого workflow-engine.

---

## 2. Математическая и алгоритмическая спецификация

### Алгоритм 1. Детерминированный fold (проекция графа)

**Вход:** \(G_0 = \emptyset\), \(L_t = [e_1,\ldots,e_t]\).  
**Выход:** \(G_t\).

1. \(G \leftarrow G_0\).
2. Для каждого \(e_k \in L_t\): применить `object.created` / `relation.created` / `object.patched` (JSON merge-patch) и т.д.
3. Вернуть \(G\).

### Алгоритм 2. Relation Behaviors

**Вход:** событие \(e\), граф \(G\), реестр реберных поведений.  
**Выход:** список порождённых событий.

Для каждого поведения с совпадающим trigger и типом ребра: если событие инцидентно концам ребра — `execute(rel, e, G)`.

### Алгоритм 3. Cui Max-Convolution

**Вход:** бюджет \(V\), плагины \(M_1..M_n\) с \(h_i(v)\).  
**Выход:** вектор \(H\), где \(H[V]\) — оптимальная композиция.

\[
H_{\mathrm{next}}[W] = \max_{0 \le v \le W}\{ h_i(v) + H[W-v] \}
\]

---

## 3. Структура в Rust (Cargo Workspace)

```
[workspace]
 ├── app-shell/          # Composition Root (единственный бинарь)
 ├── common/             # Event, Patch, UuidV7, …
 └── modules/           # Вертикальные срезы (packs)
      └── <domain>-module/
           ├── domain/      # чистый домен (без sqlx/ORM)
           ├── internal/    # pub(crate)
           ├── adapters/    # исходящие порты
           └── lib.rs       # публичные Ports (traits)
```

### Законы изоляции (AI-Compiler Gates)

1. **Domain Ignorance** — `domain/` без инфраструктуры.
2. **Private-by-Default** — наружу только Ports.
3. **Единственный Composition Root** — только `app-shell` знает конкретные адаптеры.

Для Kutha-engine: `common` + runtime-крейт + materializer packs + dictionary/agent packs; hot path без обязательных сетевых адаптеров (см. ideation идея-21).

---

## 4. Стратегия тестирования

### 4.1. Strict Replay

Проигрыш исторического лога; внешние I/O — content-addressed cache (`SHA256(prompt…)`); расхождение → `ReplayDivergenceError`.

### 4.2. Fork-and-Diff

`fork_at(event_id)` → изменить поведение → стабилизация → структурный diff графов.

### 4.3. Regimes Gated Loop (Nakajima, arXiv:2606.10241)

Diagnose → Route на Action Seam → Static / Sandbox / In-Sample / Held-Out gates; падение held-out → discard (анти-overfit).

---

## 5. Эталонный каркас (сводка)

Полный типизированный каркас ActiveRuntime (Event, Patch, MaterializedGraph::apply_event, BehaviorFn, RelationBehaviorFn, emit-цикл до стабилизации) — в исходном Studio-артефакте и в прототипных набросках ADR-000 §Prototype. Идентификаторы: **UUID v7**.

---

## 6. Протокол для кодового агента

1. Менять только на швах: Behavior / Relation Behavior / Port.
2. Жёсткая схема payload события в `common/`.
3. Домен — чистые unit-тесты без сети.
4. Для адаптеров — replay-фикстуры с CA-кэшем.

---

## 7. Связь с продуктовыми уточнениями Kutha (2026-08-16)

| Уточнение | Место в STCA |
|-----------|----------------|
| Самодостаточный Rust-движок, без обязательных внешних API | Space: адаптеры опциональны; Time: SoT локальный лог (RocksDB) |
| Bi-temporal семантика «как Graphiti», нативно | Time: поля фактов + invalidation behaviors; не Graphiti runtime |
| Quantum Receipt / cascade budgets | Time + Cui: квант emit и бюджеты свёртки |
| Dict-first агенты | Space pack + control plane поверх STCA |
| Hot CSR/HNSW | Data-plane materializations *поверх* fold (D5 ADR-000) |

---

*Документ очищен от «воды» для чтения архитектором и парсинга LLM-агентом. Полный эталонный Rust-листинг ActiveRuntime — при необходимости восстановить из Studio `stca-guide` / ADR-000 prototype appendix.*
