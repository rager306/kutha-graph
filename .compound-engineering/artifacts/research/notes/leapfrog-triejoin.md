# Leapfrog Triejoin (arXiv:1210.0481v5)

- **Paper:** Todd L. Veldhuizen, *Leapfrog Triejoin: A Simple, Worst-Case Optimal Join Algorithm* (ICDT 2014 / LogicBlox).
- **URL:** https://arxiv.org/html/1210.0481v5
- **Result:** Full conjunctive queries in \(O(Q^*\log M)\) (AGM bound \(Q^*\); \(M\) = largest relation). Hash-table variant reaches \(O(Q^*)\).
- **Kutha:** baseline for hot data-plane join (CSR + Cypher-style multi-hop). Does not change event-log SoT (D1/D2). P0 has `leapfrog_intersect` on sorted rows, not full variable-ordered MATCH.
