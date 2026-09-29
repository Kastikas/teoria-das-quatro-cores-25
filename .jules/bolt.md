## 2023-10-01 - Avoid HashMap on Graph Hot Paths
**Learning:** In the `quatro_cores` project, configurations represent very small subgraphs (max vertices=32, max degree=14). Using heap-allocated structures like `HashMap` and `Vec` inside inner loops (`O(N^2)` combinatorial matching algorithms) creates a massive overhead.
**Action:** Always replace `HashMap` frequency counters with small, stack-allocated fixed arrays (`[u8; 16]`) when checking graph degrees. Defer `Vec` allocations until they are absolutely necessary, and prefer `sort_unstable` over `sort` for primitives.
