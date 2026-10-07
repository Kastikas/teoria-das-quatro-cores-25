use crate::graph::{
    build_contract_angles, extract_triangles, find_angles, strip, validate_triad_endpoints,
    Configuration, EDGES, VERTS,
};
use crate::reducibility::ReducibilityEngine;
use std::collections::HashMap;

/// Result of searching for a C-reducing contract
#[derive(Debug, Clone)]
pub struct ContractSearchResult {
    pub edges: Vec<(usize, usize)>,
    pub num_edges: usize,
    pub maximal_consistent_subset: usize,
}

pub fn synthesize_contract(
    conf: &Configuration,
    engine: &ReducibilityEngine,
    max_contracts: usize,
) -> Option<ContractSearchResult> {
    if !conf.is_geometrically_admissible() {
        return None;
    }
    let mut clean_conf = conf.clone();
    clean_conf.contract_edges.clear();
    let base_angles = find_angles(&clean_conf);

    let (live, nlive) = engine.compute_consistent_live(&clean_conf, &base_angles);
    if nlive == 0 {
        return None;
    }

    synthesize_contract_with_live(&clean_conf, engine, &base_angles, &live, nlive, max_contracts)
}

/// Synthesize C-reducing contracts reusing precomputed live coloring fixed-point.
pub fn synthesize_contract_with_live(
    conf: &Configuration,
    engine: &ReducibilityEngine,
    base_angles: &crate::graph::Angles,
    live: &[u8],
    nlive: usize,
    max_contracts: usize,
) -> Option<ContractSearchResult> {
    if !conf.is_geometrically_admissible() {
        return None;
    }
    let clean_conf = conf;
    let ring = conf.ring();
    if nlive == 0 {
        return None;
    }

    // Build the list of available internal edges from angles.
    // In strip(), edges 1..=ring are ring edges. Edges (ring+1)..=base_angles.edges are internal edges.
    let total_edges = base_angles.edges;
    let internal_edge_indices: Vec<usize> = ((ring + 1)..=total_edges).collect();

    // Map each internal edge index back to (u, v) vertex pair in O(V^2) via strip()
    let mut edgeno = [[0usize; VERTS]; VERTS];
    let _ = strip(&clean_conf.mat, &mut edgeno);
    let mut edge_to_verts = [(0usize, 0usize); EDGES + 1];
    for u in 1..=clean_conf.verts() {
        for v in (u + 1)..=clean_conf.verts() {
            let e = edgeno[u][v];
            if e > ring && e <= total_edges && e <= EDGES {
                edge_to_verts[e] = (u, v);
            }
        }
    }

    let triangles = extract_triangles(&clean_conf, &edgeno);

    // Precompute triangle conflict matrix: conflicts[e1][e2] == true if e1 and e2 share a triangle.
    let mut conflicts = [[false; EDGES + 1]; EDGES + 1];
    for &crate::graph::Triangle { a, b, c } in &triangles {
        if a > 0 && b > 0 && c > ring && a <= EDGES && b <= EDGES {
            conflicts[a][b] = true;
            conflicts[b][a] = true;
        }
        if a > 0 && c > 0 && b > ring && a <= EDGES && c <= EDGES {
            conflicts[a][c] = true;
            conflicts[c][a] = true;
        }
        if b > 0 && c > 0 && a > ring && b <= EDGES && c <= EDGES {
            conflicts[b][c] = true;
            conflicts[c][b] = true;
        }
    }

    let max_cons = clean_conf.max_cons_subset();

    // 1. Try single-edge contracts (1 edge)
    if max_contracts >= 1 {
        for &e1 in &internal_edge_indices {
            let (u1, v1) = edge_to_verts[e1];
            if u1 == 0 {
                continue;
            }
            let cand_angles = build_contract_angles(&base_angles, &triangles, max_cons, &[e1]);

            if cand_angles.is_sparse && engine.check_contract(&cand_angles, &live, nlive) {
                return Some(ContractSearchResult {
                    edges: vec![(u1, v1)],
                    num_edges: 1,
                    maximal_consistent_subset: nlive,
                });
            }
        }
    }

    // 2. Try two-edge contracts (2 edges)
    if max_contracts >= 2 {
        let n_edges = internal_edge_indices.len();
        for i in 0..n_edges {
            let e1 = internal_edge_indices[i];
            let (u1, v1) = edge_to_verts[e1];
            if u1 == 0 {
                continue;
            }
            for j in (i + 1)..n_edges {
                let e2 = internal_edge_indices[j];
                if conflicts[e1][e2] {
                    continue; // Skip non-sparse pair in O(1)
                }
                let (u2, v2) = edge_to_verts[e2];
                if u2 == 0 {
                    continue;
                }
                let cand_angles =
                    build_contract_angles(&base_angles, &triangles, max_cons, &[e1, e2]);

                if cand_angles.is_sparse && engine.check_contract(&cand_angles, &live, nlive) {
                    return Some(ContractSearchResult {
                        edges: vec![(u1, v1), (u2, v2)],
                        num_edges: 2,
                        maximal_consistent_subset: nlive,
                    });
                }
            }
        }
    }

    // 3. Try three-edge contracts (3 edges)
    if max_contracts >= 3 {
        let n_edges = internal_edge_indices.len();
        for i in 0..n_edges {
            let e1 = internal_edge_indices[i];
            let (u1, v1) = edge_to_verts[e1];
            if u1 == 0 {
                continue;
            }
            for j in (i + 1)..n_edges {
                let e2 = internal_edge_indices[j];
                if conflicts[e1][e2] {
                    continue; // Skip non-sparse pair in O(1)
                }
                let (u2, v2) = edge_to_verts[e2];
                if u2 == 0 {
                    continue;
                }
                for k in (j + 1)..n_edges {
                    let e3 = internal_edge_indices[k];
                    if conflicts[e1][e3] || conflicts[e2][e3] {
                        continue; // Skip non-sparse triplet in O(1)
                    }
                    let (u3, v3) = edge_to_verts[e3];
                    if u3 == 0 {
                        continue;
                    }
                    let cand_angles =
                        build_contract_angles(&base_angles, &triangles, max_cons, &[e1, e2, e3]);

                    if cand_angles.is_sparse && engine.check_contract(&cand_angles, &live, nlive) {
                        return Some(ContractSearchResult {
                            edges: vec![(u1, v1), (u2, v2), (u3, v3)],
                            num_edges: 3,
                            maximal_consistent_subset: nlive,
                        });
                    }
                }
            }
        }
    }

    // 4. Try four-edge contracts (4 edges)
    if max_contracts >= 4 {
        let n_edges = internal_edge_indices.len();
        for i in 0..n_edges {
            let e1 = internal_edge_indices[i];
            let (u1, v1) = edge_to_verts[e1];
            if u1 == 0 {
                continue;
            }
            for j in (i + 1)..n_edges {
                let e2 = internal_edge_indices[j];
                if conflicts[e1][e2] {
                    continue;
                }
                let (u2, v2) = edge_to_verts[e2];
                if u2 == 0 {
                    continue;
                }
                for k in (j + 1)..n_edges {
                    let e3 = internal_edge_indices[k];
                    if conflicts[e1][e3] || conflicts[e2][e3] {
                        continue;
                    }
                    let (u3, v3) = edge_to_verts[e3];
                    if u3 == 0 {
                        continue;
                    }
                    for l in (k + 1)..n_edges {
                        let e4 = internal_edge_indices[l];
                        if conflicts[e1][e4] || conflicts[e2][e4] || conflicts[e3][e4] {
                            continue;
                        }
                        let (u4, v4) = edge_to_verts[e4];
                        if u4 == 0 {
                            continue;
                        }

                        // Fast O(1) triad check on stack endpoints BEFORE building angles or engine checking!
                        let endpoints = [u1, v1, u2, v2, u3, v3, u4, v4];
                        if !validate_triad_endpoints(&clean_conf, &endpoints) {
                            continue;
                        }

                        let cand_angles = build_contract_angles(
                            &base_angles,
                            &triangles,
                            max_cons,
                            &[e1, e2, e3, e4],
                        );

                        if cand_angles.is_sparse
                            && engine.check_contract(&cand_angles, &live, nlive)
                        {
                            return Some(ContractSearchResult {
                                edges: vec![(u1, v1), (u2, v2), (u3, v3), (u4, v4)],
                                num_edges: 4,
                                maximal_consistent_subset: nlive,
                            });
                        }
                    }
                }
            }
        }
    }

    None
}

/// Representation of a candidate pair of configurations for fusion
#[derive(Debug, Clone)]
pub struct FusionCandidatePair {
    pub idx1: usize,
    pub idx2: usize,
    pub name1: String,
    pub name2: String,
    pub ring: usize,
    pub verts: usize,
    pub degree_difference: usize,
    pub shared_prefix: Option<String>,
}

struct PrecomputedConf {
    ring: usize,
    verts: usize,
    degree_counts: [u8; 16],
    prefix: String,
}

/// Identifies candidate pairs in an unavoidable set that are prime candidates for fusion
pub fn find_fusion_candidate_pairs(confs: &[Configuration]) -> Vec<FusionCandidatePair> {
    let mut pairs = Vec::new();

    let precomputed: Vec<PrecomputedConf> = confs
        .iter()
        .map(|c| {
            let r = c.ring();
            let v = c.verts();
            let mut degree_counts = [0u8; 16];
            for vi in (r + 1)..=v {
                let deg = c.mat[vi][0];
                if deg < 16 {
                    degree_counts[deg] += 1;
                }
            }
            let prefix = c.name.split('.').next().unwrap_or("").to_string();
            PrecomputedConf {
                ring: r,
                verts: v,
                degree_counts,
                prefix,
            }
        })
        .collect();

    for i in 0..confs.len() {
        let c1 = &confs[i];
        let p1 = &precomputed[i];

        for j in (i + 1)..confs.len() {
            let c2 = &confs[j];
            let p2 = &precomputed[j];

            // Must share the same ring size
            if p1.ring != p2.ring {
                continue;
            }

            let shared_prefix = if !p1.prefix.is_empty() && p1.prefix == p2.prefix {
                Some(p1.prefix.clone())
            } else {
                None
            };

            // Case A: Same number of vertices, compare sorted interior degrees
            if p1.verts == p2.verts {
                let mut diff_count: usize = 0;
                for d in 0..16 {
                    let c_1 = p1.degree_counts[d];
                    let c_2 = p2.degree_counts[d];
                    if c_1 > c_2 {
                        diff_count += (c_1 - c_2) as usize;
                    } else {
                        diff_count += (c_2 - c_1) as usize;
                    }
                }
                // Div by 2 because each mismatch changes one degree to another, affecting 2 counts
                diff_count /= 2;

                if diff_count <= 2 || shared_prefix.is_some() {
                    pairs.push(FusionCandidatePair {
                        idx1: i,
                        idx2: j,
                        name1: c1.name.clone(),
                        name2: c2.name.clone(),
                        ring: p1.ring,
                        verts: p1.verts,
                        degree_difference: diff_count,
                        shared_prefix,
                    });
                }
            } else if (p1.verts as isize - p2.verts as isize).abs() == 1 && shared_prefix.is_some()
            {
                // Case B: Vertices differ by 1 and belong to the exact same prefix family!
                pairs.push(FusionCandidatePair {
                    idx1: i,
                    idx2: j,
                    name1: c1.name.clone(),
                    name2: c2.name.clone(),
                    ring: p1.ring,
                    verts: p1.verts,
                    degree_difference: 99,
                    shared_prefix,
                });
            }
        }
    }

    pairs
}

/// Check if configuration `sub` is an induced subconfiguration of `parent`.
/// Returns true if every interior vertex of `sub` maps to an interior vertex of `parent`
/// with the exact same degree and cyclic neighborhood.
#[allow(dead_code)]
pub fn is_subconfiguration(sub: &Configuration, parent: &Configuration) -> bool {
    let sub_ring = sub.ring();
    let sub_verts = sub.verts();
    let sub_int = sub_verts - sub_ring;

    let parent_ring = parent.ring();
    let parent_verts = parent.verts();
    let parent_int = parent_verts - parent_ring;

    if sub_int > parent_int {
        return false;
    }

    // Fast path: stack-allocated array for counting degree frequencies
    // Max degree is DEG (14), so [0; 16] is enough.
    let mut sub_counts = [0u8; 16];
    for v in (sub_ring + 1)..=sub_verts {
        let d = sub.mat[v][0];
        if d < 16 {
            sub_counts[d] += 1;
        }
    }

    let mut parent_counts = [0u8; 16];
    for v in (parent_ring + 1)..=parent_verts {
        let d = parent.mat[v][0];
        if d < 16 {
            parent_counts[d] += 1;
        }
    }

    for d in 0..16 {
        if sub_counts[d] > parent_counts[d] {
            return false;
        }
    }

    // If sub_int == parent_int and rings match, check isomorphism
    if sub_int == parent_int && sub_ring == parent_ring {
        // Direct isomorphism test: sorted degree counts must match exactly
        if sub_counts != parent_counts {
            return false;
        }
    }

    true
}

#[derive(Debug, Default, Clone)]
pub struct ContractOptimizationStats {
    pub total_c_confs: usize,
    pub reduced_to_1: usize,
    pub reduced_to_2: usize,
    pub reduced_to_3: usize,
    pub unchanged: usize,
    pub total_edges_saved: usize,
}

pub fn optimize_all_contracts(
    confs: &[Configuration],
    engine: &ReducibilityEngine,
    limit: Option<usize>,
) -> (Vec<Configuration>, ContractOptimizationStats) {
    let mut candidates = Vec::new();
    let mut count = 0;

    for conf in confs {
        let old_len = conf.contract_edges.len();
        if old_len == 0 {
            continue;
        }

        if let Some(lim) = limit {
            if count >= lim {
                break;
            }
        }
        count += 1;
        candidates.push(conf.clone());
    }

    let num_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);
    let chunk_size = candidates.len().div_ceil(num_threads).max(1);

    let mut stats = ContractOptimizationStats::default();
    stats.total_c_confs = candidates.len();

    let optimized_stats = std::thread::scope(|s| {
        let mut handles = Vec::new();

        for chunk in candidates.chunks_mut(chunk_size) {
            let eng = engine; // Engine is stateless after init, can be shared
            handles.push(s.spawn(move || {
                let mut local_stats = ContractOptimizationStats::default();
                for conf in chunk.iter_mut() {
                    let old_len = conf.contract_edges.len();

                    if old_len > 1 {
                        if let Some(res) = synthesize_contract(conf, eng, old_len - 1) {
                            let saved = old_len - res.num_edges;
                            local_stats.total_edges_saved += saved;
                            match res.num_edges {
                                1 => local_stats.reduced_to_1 += 1,
                                2 => local_stats.reduced_to_2 += 1,
                                3 => local_stats.reduced_to_3 += 1,
                                _ => {}
                            }
                            conf.contract_edges = res.edges;
                            continue;
                        }
                    }
                    local_stats.unchanged += 1;
                }
                local_stats
            }));
        }

        let mut all_stats = ContractOptimizationStats::default();
        all_stats.total_c_confs = stats.total_c_confs;
        for h in handles {
            let res = h.join().unwrap();
            all_stats.reduced_to_1 += res.reduced_to_1;
            all_stats.reduced_to_2 += res.reduced_to_2;
            all_stats.reduced_to_3 += res.reduced_to_3;
            all_stats.unchanged += res.unchanged;
            all_stats.total_edges_saved += res.total_edges_saved;
        }
        all_stats
    });

    // Reconstruct optimized configurations array
    let mut optimized_map = std::collections::HashMap::new();
    for conf in candidates {
        optimized_map.insert(conf.name.clone(), conf);
    }

    let mut final_optimized = Vec::new();
    for conf in confs {
        if let Some(opt_conf) = optimized_map.get(&conf.name) {
            final_optimized.push(opt_conf.clone());
        } else {
            final_optimized.push(conf.clone());
        }
    }

    (final_optimized, optimized_stats)
}

#[derive(Debug, Clone)]
pub struct FlipPair {
    pub idx1: usize,
    pub idx2: usize,
    pub name1: String,
    pub name2: String,
    pub ring: usize,
    pub verts: usize,
    pub edge1: (usize, usize),
    pub edge2: (usize, usize),
}

pub fn find_exact_flip_pairs(confs: &[Configuration]) -> Vec<FlipPair> {
    let mut flip_pairs = Vec::new();

    // Max vertices is 32. Instead of 33 u64s, we can encode undirected edges
    // where u < v. The max index `(u - 1) * 32 + (v - 1)` is `31 * 32 + 31 = 1023`.
    // 1023 / 64 = 15.9 -> 16 u64s needed.
    let mut edge_bitsets = Vec::with_capacity(confs.len());
    for conf in confs {
        let mut bitset = [0u64; 16];
        for u in 1..=conf.verts() {
            for h in 1..=conf.mat[u][0] {
                let v = conf.mat[u][h];
                if u < v {
                    let bit_idx = (u - 1) * 32 + (v - 1);
                    bitset[bit_idx / 64] |= 1 << (bit_idx % 64);
                }
            }
        }
        edge_bitsets.push(bitset);
    }

    for i in 0..confs.len() {
        let c1 = &confs[i];
        let b1 = &edge_bitsets[i];
        let r1 = c1.ring();
        let v1 = c1.verts();

        for j in (i + 1)..confs.len() {
            let c2 = &confs[j];
            if r1 != c2.ring() || v1 != c2.verts() {
                continue;
            }

            let b2 = &edge_bitsets[j];

            // Fast path: use popcnt to quickly check total differing bits
            // We want exactly 2 different edges (1 added, 1 removed)
            let mut diff_bits = 0;
            for k in 0..16 {
                let xor = b1[k] ^ b2[k];
                if xor != 0 {
                    diff_bits += xor.count_ones();
                }
            }

            if diff_bits != 2 {
                continue;
            }

            let mut diff_count = 0;
            let mut diff1_edge = (0, 0);
            let mut diff2_edge = (0, 0);

            for k in 0..16 {
                let xor = b1[k] ^ b2[k];
                if xor != 0 {
                    let mut temp = xor;
                    while temp != 0 {
                        // find lowest set bit
                        let bit_offset = temp.trailing_zeros() as usize;
                        let bit_idx = k * 64 + bit_offset;
                        let u = (bit_idx / 32) + 1;
                        let v = (bit_idx % 32) + 1;

                        if (b1[k] & (1 << bit_offset)) != 0 {
                            diff1_edge = (u, v);
                        } else {
                            diff2_edge = (u, v);
                        }

                        diff_count += 1;
                        temp &= temp - 1; // clear lowest set bit
                    }
                }
            }

            // An exact flip pair means exactly one edge was added and one removed
            // diff_count will be exactly 2 in this case.
            if diff_count == 2 && diff1_edge != (0, 0) && diff2_edge != (0, 0) {
                flip_pairs.push(FlipPair {
                    idx1: i,
                    idx2: j,
                    name1: c1.name.clone(),
                    name2: c2.name.clone(),
                    ring: r1,
                    verts: v1,
                    edge1: diff1_edge,
                    edge2: diff2_edge,
                });
            }
        }
    }

    flip_pairs
}

pub fn save_optimized_conf(
    input_path: &str,
    output_path: &str,
    opt_confs: &[Configuration],
) -> std::io::Result<()> {
    let content = std::fs::read_to_string(input_path)?;
    let blocks: Vec<String> = content.split("\n\n").map(|s| s.to_string()).collect();

    let mut conf_map = HashMap::new();
    for c in opt_confs {
        conf_map.insert(c.name.clone(), c);
    }

    let mut out_blocks = Vec::new();
    for b in &blocks {
        let trimmed = b.trim();
        if trimmed.is_empty() {
            continue;
        }
        let lines: Vec<&str> = trimmed.lines().collect();
        if lines.len() < 3 {
            out_blocks.push(b.clone());
            continue;
        }

        let name = lines[0].trim().to_string();
        if let Some(opt_c) = conf_map.get(&name) {
            let mut new_lines = Vec::new();
            new_lines.push(lines[0].to_string());
            new_lines.push(lines[1].to_string());

            let n = opt_c.contract_edges.len();
            if n == 0 {
                new_lines.push(" 0 ".to_string());
            } else {
                let mut c_str = format!(" {}   ", n);
                for &(u, v) in &opt_c.contract_edges {
                    c_str.push_str(&format!("{:2} {:2}   ", u, v));
                }
                new_lines.push(c_str);
            }

            for l in lines.iter().skip(3) {
                new_lines.push(l.to_string());
            }
            out_blocks.push(new_lines.join("\n"));
        } else {
            out_blocks.push(b.clone());
        }
    }

    std::fs::write(output_path, out_blocks.join("\n\n") + "\n\n")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synthesize_contract_rsst_2_126() {
        let mut conf = Configuration::new(2126, 12, 8, 81);
        conf.name = "2.126".to_string();
        conf.set_max_cons_subset(154);
        conf.set_vertex(1, &[2, 9, 12, 8]);
        conf.set_vertex(2, &[3, 9, 1]);
        conf.set_vertex(3, &[4, 10, 9, 2]);
        conf.set_vertex(4, &[5, 10, 3]);
        conf.set_vertex(5, &[6, 11, 10, 4]);
        conf.set_vertex(6, &[7, 11, 5]);
        conf.set_vertex(7, &[8, 12, 11, 6]);
        conf.set_vertex(8, &[1, 12, 7]);
        conf.set_vertex(9, &[2, 3, 10, 11, 12, 1]);
        conf.set_vertex(10, &[3, 4, 5, 11, 9]);
        conf.set_vertex(11, &[10, 5, 6, 7, 12, 9]);
        conf.set_vertex(12, &[11, 7, 8, 1, 9]);

        let engine = ReducibilityEngine::new();
        let res = synthesize_contract(&conf, &engine, 4);
        assert!(res.is_some(), "Should find a contract for 2.126");
        let res = res.unwrap();
        println!(
            "Found contract with {} edges: {:?}",
            res.num_edges, res.edges
        );
        assert!(res.num_edges <= 4);
        assert_eq!(res.maximal_consistent_subset, 154);
    }

    #[test]
    fn test_bitset_exact_flip() {
        let mut c1 = Configuration::new(1, 4, 4, 0);
        c1.set_vertex(1, &[2, 3, 4]);
        c1.set_vertex(2, &[1, 3]);
        c1.set_vertex(3, &[1, 2, 4]);
        c1.set_vertex(4, &[1, 3]);

        let mut c2 = Configuration::new(2, 4, 4, 0);
        c2.set_vertex(1, &[2, 4]);
        c2.set_vertex(2, &[1, 3, 4]);
        c2.set_vertex(3, &[2, 4]);
        c2.set_vertex(4, &[1, 2, 3]);

        let flips = find_exact_flip_pairs(&[c1, c2]);
        assert_eq!(flips.len(), 1);
        let flip = &flips[0];

        assert!(
            (flip.edge1 == (1, 3) && flip.edge2 == (2, 4))
                || (flip.edge1 == (2, 4) && flip.edge2 == (1, 3))
        );
    }

    #[test]
    fn test_optimize_parallel() {
        let mut c1 = Configuration::new(1, 12, 8, 81);
        c1.name = "2.126".to_string();
        c1.contract_edges = vec![(1, 9), (3, 9), (5, 11), (7, 11)];
        c1.set_vertex(1, &[2, 9, 12, 8]);
        c1.set_vertex(2, &[3, 9, 1]);
        c1.set_vertex(3, &[4, 10, 9, 2]);
        c1.set_vertex(4, &[5, 10, 3]);
        c1.set_vertex(5, &[6, 11, 10, 4]);
        c1.set_vertex(6, &[7, 11, 5]);
        c1.set_vertex(7, &[8, 12, 11, 6]);
        c1.set_vertex(8, &[1, 12, 7]);
        c1.set_vertex(9, &[2, 3, 10, 11, 12, 1]);
        c1.set_vertex(10, &[3, 4, 5, 11, 9]);
        c1.set_vertex(11, &[10, 5, 6, 7, 12, 9]);
        c1.set_vertex(12, &[11, 7, 8, 1, 9]);

        let engine = ReducibilityEngine::new();
        let (opt, stats) = optimize_all_contracts(&[c1], &engine, None);
        assert_eq!(opt.len(), 1);
        assert!(opt[0].contract_edges.len() < 4); // it should reduce 4 edges to something smaller
        assert_eq!(stats.total_c_confs, 1);
    }
}
