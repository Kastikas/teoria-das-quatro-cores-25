## 2026-09-29 - [Missing Array Bounds Verification in Fixed-Size Matrices]
**Vulnerability:** User-provided configurations could supply vertices or degrees outside the pre-allocated fixed-size arrays (`VERTS` and `DEG`), causing panics on runtime due to out-of-bounds accesses in Rust.
**Learning:** Because the matrices are stack-allocated and fixed-size (e.g. `[[usize; DEG]; VERTS]`), input limits must be strictly validated *before* indices are used, else it acts as a Denial of Service attack vector through unexpected crashes during file parsing.
**Prevention:** Implement bounds validation and clamp large arrays/slices before writing to static dimensions.

## 2024-05-18 - [Denial of Service via Configuration Integer Underflow/Out-of-Bounds]
**Vulnerability:** Parsing maliciously crafted or invalid `.conf` files caused integer overflow/underflow panics during memory allocation logic (e.g., `3 * (verts - 1) - ring`) or out-of-bounds indexing in algorithms due to missing validation on `verts`, `ring`, and array entries (`neighbors`).
**Learning:** Fixed-size static allocations in Rust are safe from arbitrary memory execution, but panic safely on out-of-bounds. However, unhandled panics on user input act as an application-level DoS attack vector. We must manually clamp or reject unfeasible topology dimensions.
**Prevention:** Always strictly validate `verts`, `ring`, and `nb` indices before casting or executing mathematical formulas based on input when configuring standard data structures.
