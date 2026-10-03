## 2023-10-01 - Avoid HashMap on Graph Hot Paths
**Learning:** In the `quatro_cores` project, configurations represent very small subgraphs (max vertices=32, max degree=14). Using heap-allocated structures like `HashMap` and `Vec` inside inner loops (`O(N^2)` combinatorial matching algorithms) creates a massive overhead.
**Action:** Always replace `HashMap` frequency counters with small, stack-allocated fixed arrays (`[u8; 16]`) when checking graph degrees. Defer `Vec` allocations until they are absolutely necessary, and prefer `sort_unstable` over `sort` for primitives.

## $(date +%Y-%m-%d) - Array Bitset Decode Optimization & Bounds Safety
**Learning:** When packing small variable bounds (like graph edges up to 32 vertices) into fixed-size bitsets on the stack, decoding index modulo `u = bit_idx / 32` vs `v = bit_idx % 32` causes 1-indexed maximum limits (like vertex 32) to modulo decode improperly to 0 (since `32 % 32 == 0`).
**Action:** Always allocate index widths capable of representing `N+1` when dealing with 1-indexed mathematical limits. For `N=32`, a bit width of `64` ensures `32 % 64` resolves safely, preventing false decoding.

## 2023-11-20 - Fast Path Graph Edge Diffing using popcnt
**Learning:** In combinatorial search algorithms like `find_exact_flip_pairs` where graph edge bitsets are compared in an $O(N^2)$ loop, eagerly decoding the differing bit indices to reconstruct the edges is extremely expensive. Most comparisons yield differences > 2. By using a hardware-accelerated `count_ones()` (`popcnt`) to pre-calculate total differing bits, we can instantly bypass the expensive edge decoding logic for the vast majority of pairs.
**Action:** Always implement a fast-path bit counting phase (`count_ones()`) before entering complex loop constructs that extract specific bit indices, especially when you only care about exact difference thresholds (like exactly 2 differing edges).
