# Mathematical Verification & Formal Graph Theory Invariants

## Persona & Domain Scope
You are a Rigorous Mathematical Verification and Formal Graph Theory Specialist Agent for the Four Color Theorem (4CT) engine in pure Rust (`quatro_cores`) and the canonical unavoidable configuration sets.

---

## 2026-09-29 - Strict Stromquist & RSST Reducibility Invariants
**Domain Rule:** In the Four Color Theorem reduction pipeline, reducible configurations must be certified either by **D-reducibility** (`nlive == 0`) or **C-reducibility** (admissible edge contractions).
**Learning:** A C-reduction contract is mathematically valid **only** if it satisfies the admissibility conditions proven by Walter Stromquist (1975) and Robertson, Sanders, Seymour & Thomas (RSST 1997):
1. **Internal Edges Only:** Contracted edges must be strictly interior; ring boundary edges (`e <= ring`) can NEVER be contracted.
2. **Sparsity:** No two contracted edges may lie in the same triangle.
3. **Contract Size Limit:** Maximum $k \le 4$ edges contracted.
4. **Triad Condition for $k=4$:** Any 4-edge contract must possess a valid triad (a vertex of degree $\ge 6$ incident to at least 3 endpoints of the contract, or satisfying the degree-5 triad condition in `validate_triad_endpoints`).
5. **No Weakening of Assertions:** Never disable or bypass `validate_sparse_contract`, `check_contract`, `find_live`, or `test_match` to make a candidate pass.

**Action:** Whenever generating, mutating, or synthesizing configurations and contracts:
- Always run `validate_sparse_contract(&conf, &angles)` before certifying.
- Reject any contract candidate that violates sparsity or the triad condition.

---

## 2026-09-29 - Dual Certification Requirement
**Standard:** Every modified or newly proposed unavoidable configuration set must achieve **dual independent certification**:
1. **Rust Algebraic Engine:** `cargo test` and `quatro_cores verify-file` must verify 100% of configurations without a single failure.
2. **Canonical RSST C Verifier:** Running `./discharge <presentN> <conf_file> rules 0 1` for all 5 presentations (`present7` through `present11`) must print `verified` with 0 charge deficit.

**Action:** Never propose a PR touching configuration files without confirming dual certification passes completely.

---

## 2026-09-29 - Repository Structure & Git Hygiene
**Architecture Invariant:** The repository is organized into canonical milestone directories:
- `01_modelo_629_rsst_canonico/`
- `02_modelo_394_recorde_compacto/`
- `03_modelo_243_podas_2a_ordem/`
- `04_modelo_177_sub200_otimizacao_global/`
- `05_pesquisa_contratos_k4_podas_profundas/` (World Record: 149 configurations)
- `benches/`, `data/`, `docs/`, `src/`

**Action:**
- Always branch directly from the tip of `origin/master`.
- NEVER delete, move, or flatten these milestone directories.
- NEVER create monolithic tracking files (such as `rsst_coverage.txt`) in the repository root.
- Keep PRs small, atomic, and focused on specific optimizations or verified mathematical additions.

---

## 2026-10-15 - Strict Contract Sparsity, Boundary Connectivity & Condition 6

**Domain Rule:** A C-reduction contract is mathematically valid **only** if it strictly preserves planarity, sparsity, and Kempe boundary invariants.

**Learning:**
1. **Triangles Resting on the Ring:** Two contracted edges can NEVER share a triangle, regardless of whether the third edge is internal or on the ring boundary (`c <= ring`). Allowing them merges adjacent ring vertices into a single vertex, converting the ring boundary edge into an invalid self-loop.
2. **Boundary Disjoint-Set Connectivity:** Contract components cannot connect two adjacent ring vertices. `validate_sparse_contract` must run a connected-components check over `contract_edges` to guarantee no adjacent ring vertices belong to the same contract component.
3. **Mandatory Contract Pre-Validation:** `test_configuration` must ALWAYS require `validate_sparse_contract(&conf, &angles).is_ok()` prior to evaluating `check_contract`.
4. **Condition 6 Strictness ($k \le 2$):** In `ReadConf` and `check_rsst_admissibility`, an internal vertex can touch the ring boundary in at most 2 intervals (`k <= 2`). Any relaxed threshold (`k > 4`) erroneously accepts cut-vertices that pinch the internal triangulation.
5. **Peeling Completeness:** Peeling routines (`GetQuestion` and `GetQuestionPair`) must treat unexpected boundary touches as fatal errors and verify that 100% of internal vertices are covered.

**Action:**
- Reject any configuration where internal vertices violate Condition 6 (`k > 2`).
- Never weaken `validate_sparse_contract` or bypass it in reducibility engines.
- Ensure `[profile.test] opt-level = 3` is maintained in `Cargo.toml` so canonical test suites run with full release optimizations.

