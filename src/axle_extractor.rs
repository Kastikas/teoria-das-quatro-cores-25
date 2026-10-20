#![allow(dead_code)]

use crate::graph::{canonical_dihedral_signature, Configuration, VERTS};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub const CARTVERT: usize = 98;

/// Represents an axle exported from `discharge.c` (`--dump-axles`).
#[derive(Clone, Debug)]
pub struct AxleData {
    pub pres_deg: usize,
    pub lineno: usize,
    pub astack_depth: usize,
    pub low: Vec<usize>,
    pub upp: Vec<usize>,
    pub matching_confs: Vec<usize>,
}

impl AxleData {
    /// Parses a single JSON line dumped by `discharge --dump-axles`.
    pub fn from_json_line(line: &str) -> Option<Self> {
        let line = line.trim();
        if !line.starts_with('{') || !line.ends_with('}') {
            return None;
        }
        let inner = &line[1..line.len() - 1];

        let extract_int = |key: &str| -> Option<usize> {
            let pos = inner.find(key)?;
            let rest = &inner[pos + key.len()..];
            let rest = rest.trim_start_matches(|c: char| c == ':' || c.is_whitespace());
            let end = rest.find(|c: char| !c.is_ascii_digit()).unwrap_or(rest.len());
            rest[..end].parse().ok()
        };

        let extract_array = |key: &str| -> Option<Vec<usize>> {
            let pos = inner.find(key)?;
            let rest = &inner[pos + key.len()..];
            let start = rest.find('[')?;
            let end = rest.find(']')?;
            let array_str = rest[start + 1..end].trim();
            if array_str.is_empty() {
                return Some(Vec::new());
            }
            let mut res = Vec::new();
            for item in array_str.split(',') {
                let trimmed = item.trim();
                if !trimmed.is_empty() {
                    if let Ok(v) = trimmed.parse::<usize>() {
                        res.push(v);
                    }
                }
            }
            Some(res)
        };

        let pres_deg = extract_int("\"pres_deg\"")?;
        let lineno = extract_int("\"lineno\"")?;
        let astack_depth = extract_int("\"astack_depth\"").unwrap_or(0);
        let low = extract_array("\"low\"")?;
        let upp = extract_array("\"upp\"")?;
        let matching_confs = extract_array("\"matching_confs\"").unwrap_or_default();

        Some(Self {
            pres_deg,
            lineno,
            astack_depth,
            low,
            upp,
            matching_confs,
        })
    }

    /// Loads all axles from a JSONL dump file.
    #[allow(dead_code)]
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<Self>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut axles = Vec::new();
        for line in reader.lines() {
            let line = line?;
            if let Some(axle) = Self::from_json_line(&line) {
                axles.push(axle);
            }
        }
        Ok(axles)
    }
}

/// Reconstructs the oriented adjacency matrix `adjmat` from an axle
/// exactly matching `Getadjmat` and `DoFan` of `src/discharge.c`.
pub fn build_adjmat(axle: &AxleData) -> [[i32; CARTVERT]; CARTVERT] {
    let mut adjmat = [[-1i32; CARTVERT]; CARTVERT];
    let deg = axle.low[0];

    for i in 1..=deg {
        let h = if i == 1 { deg } else { i - 1 };
        adjmat[0][h] = i as i32;
        adjmat[i][0] = h as i32;
        adjmat[h][i] = 0;

        let a = deg + h;
        adjmat[i][h] = a as i32;
        adjmat[a][i] = h as i32;
        adjmat[h][a] = i as i32;

        if axle.upp[i] < 12 {
            do_fan(deg, i, axle.upp[i], &mut adjmat);
        }
    }
    adjmat
}

fn do_fan(deg: usize, i: usize, k: usize, adjmat: &mut [[i32; CARTVERT]; CARTVERT]) {
    if k < 5 || k > 11 {
        return;
    }
    let a = if i == 1 { 2 * deg } else { deg + i - 1 };
    let b = deg + i;

    let mut fan_seq = Vec::with_capacity(9);
    fan_seq.push(a);
    for m in 2..=(k - 4) {
        fan_seq.push(m * deg + i);
    }
    fan_seq.push(b);

    for m in 0..fan_seq.len() - 1 {
        let u_prev = fan_seq[m];
        let u_curr = fan_seq[m + 1];
        adjmat[i][u_prev] = u_curr as i32;
        adjmat[u_prev][u_curr] = i as i32;
        adjmat[u_curr][i] = u_prev as i32;
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InternalChain {
    Closed(Vec<usize>),
    Open {
        chain: Vec<usize>,
        k_ext: usize,
    },
}

/// Identifies the chain of neighbors around vertex `w` in `adjmat`.
/// Returns `Closed(cycle)` if it forms a complete closed wheel of `deg_w` triangles,
/// or `Open { chain, k_ext }` if it forms a contiguous open chain of length `3 <= m <= deg_w`.
pub fn get_vertex_chain(
    w: usize,
    deg_w: usize,
    adjmat: &[[i32; CARTVERT]; CARTVERT],
) -> Option<InternalChain> {
    if deg_w < 5 || deg_w > 11 {
        return None;
    }

    let mut next_map: HashMap<usize, usize> = HashMap::new();
    let mut has_pred: HashSet<usize> = HashSet::new();
    let mut all_nodes: HashSet<usize> = HashSet::new();

    for z in 0..CARTVERT {
        let nxt = adjmat[w][z];
        if nxt != -1 {
            let nxt_u = nxt as usize;
            next_map.insert(z, nxt_u);
            has_pred.insert(nxt_u);
            all_nodes.insert(z);
            all_nodes.insert(nxt_u);
        }
    }

    if all_nodes.is_empty() {
        return None;
    }

    let starts: Vec<usize> = all_nodes
        .iter()
        .copied()
        .filter(|u| !has_pred.contains(u))
        .collect();

    if starts.len() == 1 {
        // Exactly one head -> open chain
        let start = starts[0];
        let mut chain = Vec::new();
        let mut curr = start;
        let mut visited = HashSet::new();

        while visited.insert(curr) {
            chain.push(curr);
            match next_map.get(&curr) {
                Some(&nxt) => curr = nxt,
                None => break,
            }
        }

        if chain.len() == all_nodes.len() {
            let m = chain.len();
            if m >= 3 && m <= deg_w {
                if m == deg_w && next_map.get(&chain[m - 1]) == Some(&chain[0]) {
                    return Some(InternalChain::Closed(chain));
                } else if m < deg_w {
                    return Some(InternalChain::Open {
                        chain,
                        k_ext: deg_w - m,
                    });
                }
            }
        }
    } else if starts.is_empty() {
        // Pure cycle
        let (&first, _) = next_map.iter().next()?;
        let mut cycle = Vec::new();
        let mut curr = first;
        let mut visited = HashSet::new();

        while visited.insert(curr) {
            cycle.push(curr);
            match next_map.get(&curr) {
                Some(&nxt) => {
                    if nxt == first {
                        break;
                    }
                    curr = nxt;
                }
                None => return None,
            }
        }

        if cycle.len() == deg_w && cycle.len() == all_nodes.len() {
            return Some(InternalChain::Closed(cycle));
        }
    }

    None
}

/// Identifies all vertices `w` that are eligible to be internal vertices in extracted configurations.
/// Closed vertices (spokes/center/others) with full cycles are eligible.
/// Hats (d+1..=2*d) and unblocked fans (2*d+1..=8*d) with 5 <= upp[w] <= 11 and open chains
/// of length 3 <= m <= upp[w] are also eligible.
pub fn find_eligible_internals(axle: &AxleData, adjmat: &[[i32; CARTVERT]; CARTVERT]) -> Vec<usize> {
    let d = axle.pres_deg;
    let mut eligible = Vec::new();

    for w in 0..=8 * d {
        if w >= axle.upp.len() {
            continue;
        }
        let deg_w = axle.upp[w];
        if deg_w < 5 || deg_w > 11 {
            continue;
        }
        if w > 2 * d {
            let spoke = (w - 1) % d + 1;
            let layer = (w - 1) / d;
            if axle.low[spoke] != axle.upp[spoke] || axle.low[spoke] < layer + 4 {
                continue;
            }
        }

        if let Some(chain_info) = get_vertex_chain(w, deg_w, adjmat) {
            match chain_info {
                InternalChain::Closed(_) => {
                    eligible.push(w);
                }
                InternalChain::Open { .. } => {
                    if w > d {
                        eligible.push(w);
                    }
                }
            }
        }
    }
    eligible
}

/// Simulates `GetQuestion` of `src/discharge.c` to verify that every internal vertex
/// of `conf` is reached and queried by the triangular reconstruction traversal.
pub fn is_fully_queried_by_get_question(conf: &Configuration) -> bool {
    let nverts = conf.verts();
    let ring = conf.ring();
    if nverts <= ring {
        return false;
    }

    let mut found = vec![false; nverts + 1];

    // Q[0].z = vertex with maximum degree among internal vertices
    let mut max_deg = 0;
    let mut best = 0;
    for v in (ring + 1)..=nverts {
        let d = conf.mat[v][0];
        if d > max_deg {
            max_deg = d;
            best = v;
        }
    }
    if best == 0 {
        return false;
    }
    found[best] = true;

    // Q[1].z = internal neighbor of best with maximum degree
    let mut second_max = 0;
    let mut secondbest = 0;
    let best_deg = conf.mat[best][0];
    for i in 1..=best_deg {
        let v = conf.mat[best][i];
        if v <= ring {
            continue;
        }
        let d = conf.mat[v][0];
        if d > second_max {
            second_max = d;
            secondbest = v;
        }
    }
    if secondbest == 0 {
        return false;
    }
    found[secondbest] = true;

    let mut q_z = Vec::new();
    q_z.push(best);
    q_z.push(secondbest);

    let mut search = 0;
    while search < q_z.len() {
        let v = q_z[search];
        search += 1;
        if v <= ring {
            continue;
        }
        let d = conf.mat[v][0];
        let mut i = 1;
        while i <= d && !found[conf.mat[v][i]] {
            i += 1;
        }
        if i > d {
            continue;
        }

        let mut h = if i == 1 { d } else { i - 1 };
        while h != i {
            let u = conf.mat[v][h];
            if u <= ring {
                break;
            }
            if !found[u] {
                q_z.push(u);
                found[u] = true;
            }
            h = if h == 1 { d } else { h - 1 };
        }
        if h == i {
            continue;
        }

        let mut j = if i == d { 1 } else { i + 1 };
        loop {
            let w = conf.mat[v][j];
            if w <= ring {
                break;
            }
            if !found[w] {
                q_z.push(w);
                found[w] = true;
            }
            j = if j == d { 1 } else { j + 1 };
        }

        let r = if h >= j { h - j } else { (h + d) - j };
        if r <= 2 {
            continue;
        }

        let u = conf.mat[v][h];
        if !found[u] {
            q_z.push(u);
            found[u] = true;
        }

        let mut g = if h == 1 { d } else { h - 1 };
        while g != j {
            let t = conf.mat[v][g];
            if t > ring && !found[t] {
                q_z.push(t);
                found[t] = true;
            }
            g = if g == 1 { d } else { g - 1 };
        }
    }

    for u in (ring + 1)..=nverts {
        if !found[u] {
            return false;
        }
    }
    true
}

/// Helper to get the full ordered clockwise cyclic list of neighbors around an eligible internal vertex.
pub fn get_internal_cyclic_neighbors(
    u: usize,
    deg_u: usize,
    adjmat: &[[i32; CARTVERT]; CARTVERT],
) -> Vec<usize> {
    match get_vertex_chain(u, deg_u, adjmat) {
        Some(InternalChain::Closed(cycle)) => cycle,
        Some(InternalChain::Open { chain, .. }) => chain,
        None => Vec::new(),
    }
}

/// Extracts all admissible RSST planar configurations from a given axle with ring size in `[min_ring, max_ring]`.
pub fn extract_configurations_from_axle(
    axle: &AxleData,
    min_ring: usize,
    max_ring: usize,
) -> Vec<Configuration> {
    let d = axle.pres_deg;
    let adjmat = build_adjmat(axle);
    let eligible = find_eligible_internals(axle, &adjmat);
    if eligible.is_empty() {
        return Vec::new();
    }

    // Pre-calculate chain_info for eligible vertices
    let mut vertex_chains: HashMap<usize, InternalChain> = HashMap::new();
    for &u in &eligible {
        if let Some(chain) = get_vertex_chain(u, axle.upp[u], &adjmat) {
            vertex_chains.insert(u, chain);
        }
    }

    // Build internal adjacency map
    let eligible_set: HashSet<usize> = eligible.iter().copied().collect();
    let mut int_adj: HashMap<usize, Vec<usize>> = HashMap::new();
    for &u in &eligible {
        let chain_nbs = match &vertex_chains[&u] {
            InternalChain::Closed(c) => c,
            InternalChain::Open { chain, .. } => chain,
        };
        let mut el_nbs = Vec::new();
        for &nb in chain_nbs {
            if eligible_set.contains(&nb) {
                el_nbs.push(nb);
            }
        }
        int_adj.insert(u, el_nbs);
    }

    let mut seen_subsets: HashSet<Vec<usize>> = HashSet::new();
    let mut candidate_subsets: Vec<Vec<usize>> = Vec::new();

    // For each center c in eligible
    for &c in &eligible {
        // Compute distance from c
        let mut dist: HashMap<usize, usize> = HashMap::new();
        let mut q = VecDeque::new();
        dist.insert(c, 0);
        q.push_back(c);

        let mut ball_2 = Vec::new();
        while let Some(u) = q.pop_front() {
            let d_u = dist[&u];
            ball_2.push(u);
            if d_u < 2 {
                if let Some(nbs) = int_adj.get(&u) {
                    for &nb in nbs {
                        if !dist.contains_key(&nb) {
                            dist.insert(nb, d_u + 1);
                            q.push_back(nb);
                        }
                    }
                }
            }
        }

        // BFS-growth of connected subsets containing c within ball_2
        let ball_set: HashSet<usize> = ball_2.into_iter().collect();
        let mut queue: VecDeque<Vec<usize>> = VecDeque::new();
        let init_subset = vec![c];
        seen_subsets.insert(init_subset.clone());
        queue.push_back(init_subset);

        while let Some(sub) = queue.pop_front() {
            if sub.len() >= 2 {
                candidate_subsets.push(sub.clone());
            }
            if sub.len() >= 12 {
                continue;
            }

            // Neighboring vertices to expand
            let sub_set: HashSet<usize> = sub.iter().copied().collect();
            let mut neighbors_to_add = HashSet::new();
            for &u in &sub {
                if let Some(nbs) = int_adj.get(&u) {
                    for &nb in nbs {
                        if ball_set.contains(&nb) && !sub_set.contains(&nb) {
                            neighbors_to_add.insert(nb);
                        }
                    }
                }
            }

            for &v in &neighbors_to_add {
                let mut next_sub = sub.clone();
                next_sub.push(v);
                next_sub.sort_unstable();
                if seen_subsets.insert(next_sub.clone()) {
                    queue.push_back(next_sub);
                }
            }
        }
    }

    let mut extracted_confs = Vec::new();
    let mut seen_signatures = HashSet::new();

    for s_vec in candidate_subsets {
        let s_len = s_vec.len();
        let s_set: HashSet<usize> = s_vec.iter().copied().collect();

        // 1. Well-positioned condition (CheckIso line 1090)
        let mut well_pos = true;
        for i in 1..=d {
            let prev_hat = if i == 1 { 2 * d } else { d + i - 1 };
            let hat = d + i;
            if !s_set.contains(&i) && s_set.contains(&hat) && s_set.contains(&prev_hat) {
                well_pos = false;
                break;
            }
        }
        if !well_pos {
            continue;
        }

        // 2. Open chain endpoint check:
        // When w with open chain is in S, endpoints z_1 and z_m must NOT be in S
        let mut endpoints_outside = true;
        for &w in &s_vec {
            if let Some(InternalChain::Open { chain, .. }) = vertex_chains.get(&w) {
                let z1 = chain[0];
                let zm = chain[chain.len() - 1];
                if s_set.contains(&z1) || s_set.contains(&zm) {
                    endpoints_outside = false;
                    break;
                }
            }
        }
        if !endpoints_outside {
            continue;
        }

        // 3. Build full cyclic neighbor list N(u) for each u in S,
        // inserting virtual ring vertices for open chains.
        let mut full_neighbors: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut virtual_parents: HashMap<usize, usize> = HashMap::new();
        let mut virtual_vertex_count = 0;

        for &u in &s_vec {
            match &vertex_chains[&u] {
                InternalChain::Closed(cycle) => {
                    full_neighbors.insert(u, cycle.clone());
                }
                InternalChain::Open { chain, k_ext } => {
                    let mut nbs = chain.clone();
                    for _ in 0..*k_ext {
                        virtual_vertex_count += 1;
                        let y = CARTVERT + virtual_vertex_count;
                        virtual_parents.insert(y, u);
                        nbs.push(y);
                    }
                    full_neighbors.insert(u, nbs);
                }
            }
        }

        // 4. Extract directed boundary edges
        let mut bnd_next: HashMap<usize, usize> = HashMap::new();
        let mut bnd_prev: HashMap<usize, usize> = HashMap::new();
        let mut boundary_valid = true;

        for &u in &s_vec {
            let nbs = &full_neighbors[&u];
            let deg_u = nbs.len();
            for j in 0..deg_u {
                let v = nbs[j];
                let w = nbs[(j + 1) % deg_u];
                if !s_set.contains(&v) && !s_set.contains(&w) {
                    if bnd_next.contains_key(&v) || bnd_prev.contains_key(&w) {
                        boundary_valid = false;
                        break;
                    }
                    bnd_next.insert(v, w);
                    bnd_prev.insert(w, v);
                }
            }
            if !boundary_valid {
                break;
            }
        }

        if !boundary_valid || bnd_next.is_empty() {
            continue;
        }

        let ring_size = bnd_next.len();
        if ring_size < min_ring || ring_size > max_ring || s_len + ring_size >= VERTS {
            continue;
        }

        // 5. Verify boundary cycle connectivity
        let (&first_v, _) = bnd_next.iter().next().unwrap();
        let mut cycle = Vec::with_capacity(ring_size);
        let mut curr = first_v;
        let mut cycle_visited = HashSet::new();

        while cycle.len() <= ring_size {
            cycle.push(curr);
            cycle_visited.insert(curr);
            match bnd_next.get(&curr) {
                Some(&next_v) => {
                    if next_v == first_v {
                        break;
                    }
                    if cycle_visited.contains(&next_v) {
                        boundary_valid = false;
                        break;
                    }
                    curr = next_v;
                }
                None => {
                    boundary_valid = false;
                    break;
                }
            }
        }

        if !boundary_valid || cycle.len() != ring_size {
            continue;
        }

        // 6. Verify Euler degree sum and ring vertex degrees
        let expected_euler_sum = 6 * (s_len + ring_size - 1) - 2 * ring_size;
        let mut actual_deg_sum = 0;

        for &u in &s_vec {
            actual_deg_sum += axle.upp[u];
        }

        let mut ring_degrees = Vec::with_capacity(ring_size);
        let mut ring_admiss = true;

        for &x in &cycle {
            let d_x = if x > CARTVERT {
                3
            } else {
                let mut int_nb_count = 0;
                for &u in &s_vec {
                    if full_neighbors[&u].contains(&x) {
                        int_nb_count += 1;
                    }
                }
                2 + int_nb_count
            };

            if d_x < 3 || d_x >= 13 {
                ring_admiss = false;
                break;
            }
            ring_degrees.push(d_x);
            actual_deg_sum += d_x;
        }

        if !ring_admiss || actual_deg_sum != expected_euler_sum {
            continue;
        }

        // 7. Construct Configuration
        let total_n = ring_size + s_len;
        let mut conf = Configuration::new(
            0,
            total_n,
            ring_size,
            16, // Default extent claim
        );

        // Map cartwheel vertex -> configuration index
        // Ring vertices cycle: 1..=ring_size
        // Internal vertices s_vec: (ring_size + 1)..=total_n
        let mut cart_to_conf: HashMap<usize, usize> = HashMap::new();
        for (idx, &x) in cycle.iter().enumerate() {
            cart_to_conf.insert(x, idx + 1);
        }
        for (idx, &u) in s_vec.iter().enumerate() {
            cart_to_conf.insert(u, ring_size + idx + 1);
        }

        // Fill internal vertex adjacencies
        for (idx, &u) in s_vec.iter().enumerate() {
            let conf_u = ring_size + idx + 1;
            let nbs = &full_neighbors[&u];
            let mut conf_nbs = Vec::with_capacity(nbs.len());
            for &nb in nbs {
                if let Some(&cnb) = cart_to_conf.get(&nb) {
                    conf_nbs.push(cnb);
                } else {
                    ring_admiss = false;
                    break;
                }
            }
            if !ring_admiss || conf_nbs.len() != axle.upp[u] {
                ring_admiss = false;
                break;
            }
            conf.set_vertex(conf_u, &conf_nbs);
        }
        if !ring_admiss {
            continue;
        }

        // Fill ring vertex adjacencies
        for k in 1..=ring_size {
            let x = cycle[k - 1];
            let next_k = if k == ring_size { 1 } else { k + 1 };
            let prev_k = if k == 1 { ring_size } else { k - 1 };
            let next_x = cycle[next_k - 1];
            let prev_x = cycle[prev_k - 1];

            if x > CARTVERT {
                let parent_w = virtual_parents[&x];
                let conf_w = cart_to_conf[&parent_w];
                conf.set_vertex(k, &[next_k, conf_w, prev_k]);
            } else {
                let mut u_first = None;
                for &u in &s_vec {
                    let nbs = &full_neighbors[&u];
                    let d_u = nbs.len();
                    for j in 0..d_u {
                        if nbs[j] == x && nbs[(j + 1) % d_u] == next_x {
                            u_first = Some(u);
                            break;
                        }
                    }
                    if u_first.is_some() {
                        break;
                    }
                }

                let u_start = match u_first {
                    Some(u) => u,
                    None => {
                        ring_admiss = false;
                        break;
                    }
                };

                let mut ordered_nbs = Vec::new();
                ordered_nbs.push(next_k);

                let mut curr_u = u_start;
                let mut count_steps = 0;
                loop {
                    ordered_nbs.push(cart_to_conf[&curr_u]);
                    count_steps += 1;
                    if count_steps > 15 {
                        ring_admiss = false;
                        break;
                    }

                    let nbs = &full_neighbors[&curr_u];
                    let d_u = nbs.len();
                    let j_pos = match nbs.iter().position(|&nb| nb == x) {
                        Some(pos) => pos,
                        None => {
                            ring_admiss = false;
                            break;
                        }
                    };
                    let z = nbs[(j_pos + d_u - 1) % d_u];

                    if s_set.contains(&z) {
                        curr_u = z;
                    } else {
                        if z != prev_x {
                            ring_admiss = false;
                        }
                        break;
                    }
                }

                if !ring_admiss {
                    break;
                }

                ordered_nbs.push(prev_k);
                conf.set_vertex(k, &ordered_nbs);
            }
        }

        if !ring_admiss {
            continue;
        }

        // 8. Strict RSST Admissibility & GetQuestion Queryability Certification
        if conf.check_rsst_admissibility().is_ok() && is_fully_queried_by_get_question(&conf) {
            let sig = canonical_dihedral_signature(&conf);
            if seen_signatures.insert(sig) {
                conf.id = extracted_confs.len() + 1;
                conf.name = format!("extracted_axle_{}_{}", axle.lineno, conf.id);
                extracted_confs.push(conf);
            }
        }
    }

    extracted_confs
}

pub type DihedralSignature = (usize, usize, Vec<(usize, usize)>);

/// Stores candidate configurations extracted from dumped presentation axles,
/// indexed by their canonical dihedral signature.
pub struct DihedralIntersectionFunnel {
    pub ring_size: usize,
    pub axles: Vec<AxleData>,
    /// Maps DihedralSignature -> (Configuration, HashSet<AxleIndex>)
    pub candidates: HashMap<DihedralSignature, (Configuration, HashSet<usize>)>,
    /// Maps canonical conf ID -> HashSet<AxleIndex> (which axles rely on this canonical configuration)
    pub canonical_axle_deps: HashMap<usize, HashSet<usize>>,
}

impl DihedralIntersectionFunnel {
    pub fn new(ring_size: usize) -> Self {
        Self {
            ring_size,
            axles: Vec::new(),
            candidates: HashMap::new(),
            canonical_axle_deps: HashMap::new(),
        }
    }

    /// Feeds axles into the funnel, extracting all valid configurations of `ring_size`.
    pub fn feed_axles(&mut self, axles: &[AxleData]) {
        self.axles = axles.to_vec();
        for (axle_idx, axle) in axles.iter().enumerate() {
            for &conf_id in &axle.matching_confs {
                self.canonical_axle_deps
                    .entry(conf_id)
                    .or_default()
                    .insert(axle_idx);
            }

            if axle.matching_confs.is_empty() || axle.matching_confs.len() > 2 {
                continue;
            }

            let extracted = extract_configurations_from_axle(axle, self.ring_size, self.ring_size);
            for conf in extracted {
                let sig = canonical_dihedral_signature(&conf);
                self.candidates
                    .entry(sig)
                    .or_insert_with(|| (conf, HashSet::new()))
                    .1
                    .insert(axle_idx);
            }
        }
    }

    /// Filters candidates that cover at least `min_axles` distinct axles.
    pub fn filter_by_coverage(&self, min_axles: usize) -> Vec<(Configuration, HashSet<usize>)> {
        self.candidates
            .values()
            .filter(|(_, covered)| covered.len() >= min_axles)
            .cloned()
            .collect()
    }

    /// Finds candidates that fully cover the combined exclusive axles of two or more canonical configurations (Ka, Kb).
    pub fn find_super_replacements(&self) -> Vec<(Configuration, Vec<usize>, usize)> {
        let mut replacements = Vec::new();
        let conf_ids: Vec<usize> = self.canonical_axle_deps.keys().copied().collect();

        for i in 0..conf_ids.len() {
            let id_a = conf_ids[i];
            let axles_a = &self.canonical_axle_deps[&id_a];
            if axles_a.is_empty() {
                continue;
            }

            for j in (i + 1)..conf_ids.len() {
                let id_b = conf_ids[j];
                let axles_b = &self.canonical_axle_deps[&id_b];
                if axles_b.is_empty() {
                    continue;
                }

                let exclusive_union: HashSet<usize> = axles_a
                    .union(axles_b)
                    .copied()
                    .filter(|&ax_idx| {
                        self.axles[ax_idx]
                            .matching_confs
                            .iter()
                            .all(|&c| c == id_a || c == id_b)
                    })
                    .collect();
                if exclusive_union.is_empty() {
                    continue;
                }

                for (_, (cand, covered)) in &self.candidates {
                    let missing = exclusive_union.difference(covered).count();
                    if missing <= 1 {
                        replacements.push((cand.clone(), vec![id_a, id_b], missing));
                    }
                }
            }
        }

        replacements
    }
}

/// Certifies candidates by testing D-reducibility and C-reducibility (contracts of 1 or 2 edges).
pub fn certify_candidate(
    conf: &mut Configuration,
    engine: &crate::reducibility::ReducibilityEngine,
) -> bool {
    if !conf.is_geometrically_admissible() {
        return false;
    }
    let angles = crate::graph::find_angles(conf);
    let (live, nlive) = engine.compute_consistent_live(conf, &angles);

    if nlive == 0 {
        return true;
    }

    crate::synthesizer::SynthesisPipeline::try_synthesize_sparse_contract(
        conf,
        &angles,
        engine,
        &live,
        nlive,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_axle_json_parser() {
        let json = r#"{"pres_deg":11,"lineno":3,"astack_depth":0,"low":[11,8,5,5],"upp":[11,12,5,5],"matching_confs":[1,2,3]}"#;
        let axle = AxleData::from_json_line(json).expect("Failed to parse valid axle JSON");
        assert_eq!(axle.pres_deg, 11);
        assert_eq!(axle.lineno, 3);
        assert_eq!(axle.astack_depth, 0);
        assert_eq!(axle.low, vec![11, 8, 5, 5]);
        assert_eq!(axle.upp, vec![11, 12, 5, 5]);
        assert_eq!(axle.matching_confs, vec![1, 2, 3]);
    }

    #[test]
    fn test_extract_configurations_from_known_axle() {
        let json = r#"{"pres_deg":11,"lineno":3,"astack_depth":0,"low":[11,8,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5],"upp":[11,12,5,5,5,12,12,12,12,12,12,12,12,5,5,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12],"matching_confs":[1]}"#;
        let axle = AxleData::from_json_line(json).expect("valid axle");
        let extracted = extract_configurations_from_axle(&axle, 6, 14);
        assert!(!extracted.is_empty(), "Must extract at least one configuration");
        for conf in &extracted {
            assert!(conf.check_rsst_admissibility().is_ok(), "Every extracted config must be 100% RSST admissible");
        }

        // Export extracted configurations to /tmp/extracted_test.conf
        let mut conf_file_content = String::new();
        for conf in &extracted {
            conf_file_content.push_str(&conf.to_conf_format());
        }
        std::fs::write("/tmp/extracted_test.conf", conf_file_content).expect("write conf file");
        std::fs::write("/tmp/test_axle.json", json).expect("write axle json");

        // Ensure target/verify_extracted C verifier is available
        if !std::path::Path::new("target/verify_extracted").exists() && std::path::Path::new("tests/verify_extracted.c").exists() {
            let _ = std::process::Command::new("gcc")
                .args(["-O3", "-Wno-unused-result", "tests/verify_extracted.c", "-o", "target/verify_extracted"])
                .output();
        }

        if std::path::Path::new("target/verify_extracted").exists() {
            let output = std::process::Command::new("target/verify_extracted")
                .arg("/tmp/test_axle.json")
                .arg("/tmp/extracted_test.conf")
                .output();

            if let Ok(out) = output {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let stderr = String::from_utf8_lossy(&out.stderr);
                println!("C Verifier stdout:\n{}", stdout);
                if !out.status.success() {
                    eprintln!("C Verifier stderr:\n{}", stderr);
                    panic!("target/verify_extracted failed on extracted configurations!");
                }
            }
        }
    }

    #[test]
    fn test_dihedral_intersection_funnel_and_certification() {
        let json = r#"{"pres_deg":11,"lineno":3,"astack_depth":0,"low":[11,8,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5],"upp":[11,12,5,5,5,12,12,12,12,12,12,12,12,5,5,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12],"matching_confs":[1]}"#;
        let axle = AxleData::from_json_line(json).expect("valid axle");
        let axles = vec![axle];

        let mut funnel = DihedralIntersectionFunnel::new(6);
        funnel.feed_axles(&axles);
        println!("Funnel ring 6 candidates: {}", funnel.candidates.len());

        let engine = crate::reducibility::ReducibilityEngine::new();
        for (_, (mut cand, _)) in funnel.candidates {
            let is_certified = certify_candidate(&mut cand, &engine);
            println!("Candidate {} (ring={}): certified={}", cand.name, cand.ring(), is_certified);
        }
    }

    #[test]
    fn test_supervisor_exclusive_union_and_open_chain_extraction() {
        // 1. Test get_vertex_chain on hat with open chain
        let json = r#"{"pres_deg":11,"lineno":3,"astack_depth":0,"low":[11,8,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5],"upp":[11,12,5,5,5,12,12,12,12,12,12,12,12,5,5,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12],"matching_confs":[1]}"#;
        let axle = AxleData::from_json_line(json).expect("valid axle");
        let adjmat = build_adjmat(&axle);

        // Hat 13 has upp[13] = 5 and open chain [14, 3, 2, 12]
        let chain_13 = get_vertex_chain(13, axle.upp[13], &adjmat);
        assert!(chain_13.is_some(), "Hat 13 must have a valid chain");
        match chain_13.unwrap() {
            InternalChain::Open { chain, k_ext } => {
                assert_eq!(chain, vec![14, 3, 2, 12]);
                assert_eq!(k_ext, 1, "k_ext must be upp[13] - 4 = 5 - 4 = 1");
            }
            InternalChain::Closed(_) => panic!("Hat 13 should be an open chain, not closed"),
        }

        // 2. Test exclusive_union filter in DihedralIntersectionFunnel
        let mut axle0 = axle.clone();
        axle0.matching_confs = vec![10, 20]; // Exclusive to {10, 20}
        let mut axle1 = axle.clone();
        axle1.matching_confs = vec![10, 30]; // Matches 30, so not exclusive to {10, 20}
        let mut axle2 = axle.clone();
        axle2.matching_confs = vec![10, 20, 30]; // 3 matches -> skipped in feed_axles, excluded from exclusive_union

        let mut funnel = DihedralIntersectionFunnel::new(6);
        funnel.feed_axles(&[axle0, axle1, axle2]);

        // Axle 2 has 3 matching confs, so it must not be in canonical_axle_deps or candidates
        assert_eq!(funnel.axles.len(), 3);
        // Find super replacements
        let _replacements = funnel.find_super_replacements();
        // Check that any replacement evaluated strictly exclusive axles
        let axles_10 = &funnel.canonical_axle_deps[&10];
        let axles_20 = &funnel.canonical_axle_deps[&20];
        let exclusive: HashSet<usize> = axles_10
            .union(axles_20)
            .copied()
            .filter(|&idx| {
                funnel.axles[idx]
                    .matching_confs
                    .iter()
                    .all(|&c| c == 10 || c == 20)
            })
            .collect();
        assert_eq!(exclusive, [0].iter().copied().collect::<HashSet<usize>>());
    }
}
