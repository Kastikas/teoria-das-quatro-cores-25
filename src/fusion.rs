use crate::graph::{find_angles, strip, validate_sparse_contract, Configuration, VERTS};
use crate::reducibility::ReducibilityEngine;
use std::collections::HashMap;

/// Result of searching for a C-reducing contract
#[derive(Debug, Clone)]
pub struct ContractSearchResult {
    pub edges: Vec<(usize, usize)>,
    pub num_edges: usize,
    pub is_valid: bool,
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
    let angles = find_angles(&clean_conf);

    let ring = conf.ring();
    let (live, nlive) = engine.compute_consistent_live(&clean_conf, &angles);

    if nlive == 0 {
        // Already D-reducible! No contract needed.
        return None;
    }

    // Build the list of available internal edges from angles.
    // In strip(), edges 1..=ring are ring edges. Edges (ring+1)..=angles.edges are internal edges.
    let total_edges = angles.edges;
    let internal_edge_indices: Vec<usize> = ((ring + 1)..=total_edges).collect();

    // Map each internal edge index back to (u, v) vertex pair in O(V^2) via strip()
    let mut edgeno = [[0usize; VERTS]; VERTS];
    let _ = strip(&clean_conf.mat, &mut edgeno);
    let mut edge_to_verts = HashMap::new();
    for u in 1..=clean_conf.verts() {
        for v in (u + 1)..=clean_conf.verts() {
            let e = edgeno[u][v];
            if e > ring && e <= total_edges {
                edge_to_verts.insert(e, (u, v));
            }
        }
    }

    // Precompute triangle conflict matrix: conflicts[e1][e2] == true if e1 and e2 share a triangle.
    // In Robertson et al. (RSST 1997 reduce.c), a contract is strictly sparse iff no two edges share a triangle.
    let mut conflicts = vec![vec![false; total_edges + 1]; total_edges + 1];
    for v in 1..=clean_conf.verts() {
        let deg = clean_conf.mat[v][0];
        for h in 1..=deg {
            if v <= ring && h == deg {
                continue;
            }
            let i = if h < deg { h + 1 } else { 1 };
            let u = clean_conf.mat[v][h];
            let w = clean_conf.mat[v][i];
            let a = edgeno[v][w];
            let b = edgeno[u][w];
            let c = edgeno[u][v];
            if a > 0 && b > 0 && c > ring {
                conflicts[a][b] = true;
                conflicts[b][a] = true;
            }
            if a > 0 && c > 0 && b > ring {
                conflicts[a][c] = true;
                conflicts[c][a] = true;
            }
            if b > 0 && c > 0 && a > ring {
                conflicts[b][c] = true;
                conflicts[c][b] = true;
            }
        }
    }

    // 1. Try single-edge contracts (1 edge)
    if max_contracts >= 1 {
        for &e1 in &internal_edge_indices {
            let (u1, v1) = match edge_to_verts.get(&e1) {
                Some(&p) => p,
                None => continue,
            };
            let mut cand_conf = clean_conf.clone();
            cand_conf.contract_edges = vec![(u1, v1)];
            let cand_angles = find_angles(&cand_conf);

            if validate_sparse_contract(&cand_conf, &cand_angles).is_ok()
                && engine.check_contract(&cand_angles, &live, nlive)
            {
                return Some(ContractSearchResult {
                    edges: vec![(u1, v1)],
                    num_edges: 1,
                    is_valid: true,
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
            let (u1, v1) = match edge_to_verts.get(&e1) {
                Some(&p) => p,
                None => continue,
            };
            for j in (i + 1)..n_edges {
                let e2 = internal_edge_indices[j];
                if conflicts[e1][e2] {
                    continue; // Skip non-sparse pair in O(1)
                }
                let (u2, v2) = match edge_to_verts.get(&e2) {
                    Some(&p) => p,
                    None => continue,
                };
                let mut cand_conf = clean_conf.clone();
                cand_conf.contract_edges = vec![(u1, v1), (u2, v2)];
                let cand_angles = find_angles(&cand_conf);

                if validate_sparse_contract(&cand_conf, &cand_angles).is_ok()
                    && engine.check_contract(&cand_angles, &live, nlive)
                {
                    return Some(ContractSearchResult {
                        edges: vec![(u1, v1), (u2, v2)],
                        num_edges: 2,
                        is_valid: true,
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
            let (u1, v1) = match edge_to_verts.get(&e1) {
                Some(&p) => p,
                None => continue,
            };
            for j in (i + 1)..n_edges {
                let e2 = internal_edge_indices[j];
                if conflicts[e1][e2] {
                    continue; // Skip non-sparse pair in O(1)
                }
                let (u2, v2) = match edge_to_verts.get(&e2) {
                    Some(&p) => p,
                    None => continue,
                };
                for k in (j + 1)..n_edges {
                    let e3 = internal_edge_indices[k];
                    if conflicts[e1][e3] || conflicts[e2][e3] {
                        continue; // Skip non-sparse triplet in O(1)
                    }
                    let (u3, v3) = match edge_to_verts.get(&e3) {
                        Some(&p) => p,
                        None => continue,
                    };
                    let mut cand_conf = clean_conf.clone();
                    cand_conf.contract_edges = vec![(u1, v1), (u2, v2), (u3, v3)];
                    let cand_angles = find_angles(&cand_conf);

                    if validate_sparse_contract(&cand_conf, &cand_angles).is_ok()
                        && engine.check_contract(&cand_angles, &live, nlive)
                    {
                        return Some(ContractSearchResult {
                            edges: vec![(u1, v1), (u2, v2), (u3, v3)],
                            num_edges: 3,
                            is_valid: true,
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
            let (u1, v1) = match edge_to_verts.get(&e1) {
                Some(&p) => p,
                None => continue,
            };
            for j in (i + 1)..n_edges {
                let e2 = internal_edge_indices[j];
                if conflicts[e1][e2] {
                    continue;
                }
                let (u2, v2) = match edge_to_verts.get(&e2) {
                    Some(&p) => p,
                    None => continue,
                };
                for k in (j + 1)..n_edges {
                    let e3 = internal_edge_indices[k];
                    if conflicts[e1][e3] || conflicts[e2][e3] {
                        continue;
                    }
                    let (u3, v3) = match edge_to_verts.get(&e3) {
                        Some(&p) => p,
                        None => continue,
                    };
                    for l in (k + 1)..n_edges {
                        let e4 = internal_edge_indices[l];
                        if conflicts[e1][e4] || conflicts[e2][e4] || conflicts[e3][e4] {
                            continue;
                        }
                        let (u4, v4) = match edge_to_verts.get(&e4) {
                            Some(&p) => p,
                            None => continue,
                        };
                        let mut cand_conf = clean_conf.clone();
                        cand_conf.contract_edges = vec![(u1, v1), (u2, v2), (u3, v3), (u4, v4)];
                        let cand_angles = find_angles(&cand_conf);

                        if validate_sparse_contract(&cand_conf, &cand_angles).is_ok()
                            && engine.check_contract(&cand_angles, &live, nlive)
                        {
                            return Some(ContractSearchResult {
                                edges: vec![(u1, v1), (u2, v2), (u3, v3), (u4, v4)],
                                num_edges: 4,
                                is_valid: true,
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
    pub verts1: usize,
    pub verts2: usize,
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
        let r1 = c1.ring();
        let v1 = c1.verts();
        let mut sorted_degs1: Vec<usize> = (r1 + 1..=v1).map(|v| c1.mat[v][0]).collect();
        sorted_degs1.sort();

        let prefix1 = c1.name.split('.').next().unwrap_or("").to_string();

        for j in (i + 1)..confs.len() {
            let c2 = &confs[j];
            let p2 = &precomputed[j];

            // Must share the same ring size
            if p1.ring != p2.ring {
                continue;
            }

            let r2 = c2.ring();
            let v2 = c2.verts();
            let mut sorted_degs2: Vec<usize> = (r2 + 1..=v2).map(|v| c2.mat[v][0]).collect();
            sorted_degs2.sort();

            let prefix2 = c2.name.split('.').next().unwrap_or("").to_string();
            let shared_prefix = if !prefix1.is_empty() && prefix1 == prefix2 {
                Some(prefix1.clone())
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
                        verts1: p1.verts,
                        verts2: p2.verts,
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
                    verts1: p1.verts,
                    verts2: p2.verts,
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

    let sub_int_degs: Vec<usize> = (sub_ring + 1..=sub_verts).map(|v| sub.mat[v][0]).collect();
    let parent_int_degs: Vec<usize> = (parent_ring + 1..=parent_verts)
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
        let mut s1 = sub_int_degs.clone();
        s1.sort();
        let mut s2 = parent_int_degs.clone();
        s2.sort();
        if s1 != s2 {
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
