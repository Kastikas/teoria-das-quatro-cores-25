## 2026-09-29 - [Missing Array Bounds Verification in Fixed-Size Matrices]
**Vulnerability:** User-provided configurations could supply vertices or degrees outside the pre-allocated fixed-size arrays (`VERTS` and `DEG`), causing panics on runtime due to out-of-bounds accesses in Rust.
**Learning:** Because the matrices are stack-allocated and fixed-size (e.g. `[[usize; DEG]; VERTS]`), input limits must be strictly validated *before* indices are used, else it acts as a Denial of Service attack vector through unexpected crashes during file parsing.
**Prevention:** Implement bounds validation and clamp large arrays/slices before writing to static dimensions.
