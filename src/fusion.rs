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

/// Synthesize C-reducing contracts for a configuration whose D-reducibility failed (nlive > 0).
pub fn synthesize_contract(
    conf: &Configuration,
    engine: &ReducibilityEngine,
    max_contracts: usize,
) -> Option<ContractSearchResult> {
    let mut clean_conf = conf.clone();
    clean_conf.contract_edges.clear();
    let base_angles = find_angles(&clean_conf);

    let ring = conf.ring();
    let (live, nlive) = engine.compute_consistent_live(&clean_conf, &base_angles);

    if nlive == 0 {
        // Already D-reducible! No contract needed.
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
    sorted_degs: Vec<usize>,
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
            let mut sorted_degs: Vec<usize> = (r + 1..=v).map(|vi| c.mat[vi][0]).collect();
            sorted_degs.sort();
            let prefix = c.name.split('.').next().unwrap_or("").to_string();
            PrecomputedConf {
                ring: r,
                verts: v,
                sorted_degs,
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
                let diff_count = p1
                    .sorted_degs
                    .iter()
                    .zip(p2.sorted_degs.iter())
                    .filter(|&(a, b)| a != b)
                    .count();

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

    let mut sub_int_degs: Vec<usize> = (sub_ring + 1..=sub_verts).map(|v| sub.mat[v][0]).collect();
    let mut parent_int_degs: Vec<usize> = (parent_ring + 1..=parent_verts)
        .map(|v| parent.mat[v][0])
        .collect();

    let mut sub_map = HashMap::new();
    for &d in &sub_int_degs {
        *sub_map.entry(d).or_insert(0) += 1;
    }
    let mut parent_map = HashMap::new();
    for &d in &parent_int_degs {
        *parent_map.entry(d).or_insert(0) += 1;
    }

    for (k, v) in sub_map {
        if parent_map.get(&k).copied().unwrap_or(0) < v {
            return false;
        }
    }

    // If sub_int == parent_int and rings match, check isomorphism
    if sub_int == parent_int && sub_ring == parent_ring {
        // Direct isomorphism test: sorted degree sequence must match exactly
        sub_int_degs.sort();
        parent_int_degs.sort();
        if sub_int_degs != parent_int_degs {
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
    let mut optimized = confs.to_vec();
    let mut stats = ContractOptimizationStats::default();

    let mut count = 0;
    for conf in optimized.iter_mut() {
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
        stats.total_c_confs += 1;

        if old_len > 1 {
            // Try to find a contract with strictly fewer edges
            if let Some(res) = synthesize_contract(conf, engine, old_len - 1) {
                let saved = old_len - res.num_edges;
                stats.total_edges_saved += saved;
                match res.num_edges {
                    1 => stats.reduced_to_1 += 1,
                    2 => stats.reduced_to_2 += 1,
                    3 => stats.reduced_to_3 += 1,
                    _ => {}
                }
                conf.contract_edges = res.edges;
                continue;
            }
        }

        stats.unchanged += 1;
    }

    (optimized, stats)
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

    // Precompute edge sets for each configuration
    let mut edge_sets = Vec::with_capacity(confs.len());
    for conf in confs {
        let mut edges = std::collections::HashSet::new();
        for u in 1..=conf.verts() {
            for h in 1..=conf.mat[u][0] {
                let v = conf.mat[u][h];
                edges.insert((u.min(v), u.max(v)));
            }
        }
        edge_sets.push(edges);
    }

    for i in 0..confs.len() {
        let c1 = &confs[i];
        for j in (i + 1)..confs.len() {
            let c2 = &confs[j];
            if c1.ring() != c2.ring() || c1.verts() != c2.verts() {
                continue;
            }

            let e1 = &edge_sets[i];
            let e2 = &edge_sets[j];

            let mut diff1 = Vec::new();
            for e in e1 {
                if !e2.contains(e) {
                    diff1.push(*e);
                }
            }
            if diff1.len() != 1 {
                continue;
            }

            let mut diff2 = Vec::new();
            for e in e2 {
                if !e1.contains(e) {
                    diff2.push(*e);
                }
            }
            if diff2.len() != 1 {
                continue;
            }

            flip_pairs.push(FlipPair {
                idx1: i,
                idx2: j,
                name1: c1.name.clone(),
                name2: c2.name.clone(),
                ring: c1.ring(),
                verts: c1.verts(),
                edge1: diff1[0],
                edge2: diff2[0],
            });
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
}
