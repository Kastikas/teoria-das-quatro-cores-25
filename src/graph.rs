use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub const VERTS: usize = 32;
pub const DEG: usize = 14;
pub const EDGES: usize = 72;
pub const MAXRING: usize = 16;

pub type ConfMat = [[usize; DEG]; VERTS];

#[derive(Clone, Debug)]
pub struct Configuration {
    pub id: usize,
    pub name: String,
    pub mat: ConfMat,
    pub contract_edges: Vec<(usize, usize)>,
}

impl Configuration {
    pub fn new(id: usize, verts: usize, ring: usize, extent_claim: usize) -> Self {
        let mut mat = [[0usize; DEG]; VERTS];
        mat[0][0] = verts;
        mat[0][1] = ring;
        mat[0][2] = extent_claim;
        Self {
            id,
            name: id.to_string(),
            mat,
            contract_edges: Vec::new(),
        }
    }

    pub fn verts(&self) -> usize {
        self.mat[0][0]
    }

    pub fn ring(&self) -> usize {
        self.mat[0][1]
    }

    pub fn extent_claim(&self) -> usize {
        self.mat[0][2]
    }

    pub fn max_cons_subset(&self) -> usize {
        self.mat[0][3]
    }

    pub fn set_max_cons_subset(&mut self, val: usize) {
        self.mat[0][3] = val;
    }

    pub fn set_vertex(&mut self, v: usize, neighbors: &[usize]) {
        if v >= VERTS {
            return;
        }
        let len = std::cmp::min(neighbors.len(), DEG - 1);
        self.mat[v][0] = len;
        for (i, &nb) in neighbors.iter().take(len).enumerate() {
            self.mat[v][i + 1] = nb;
        }
    }

    /// Verifies the structural and geometric conditions of Robertson, Sanders, Seymour, Thomas (1997)
    /// (Conditions 1-7 of RSST ReadConf) plus the internal radius <= 2 bound for cartwheel subconfigurations.
    ///
    /// This function acts as a microsecond fast-fail filter before expensive Kempe / Birkhoff live coloring
    /// or contract synthesis searches. Any configuration that fails this check will be unconditionally
    /// rejected by RSST discharge verification engines (discharge.c / discharge_universal.c).
    pub fn check_rsst_admissibility(&self) -> Result<(), &'static str> {
        let n = self.verts();
        let r = self.ring();

        // Condition (1): 2 <= r < n and n < VERTS
        if r < 2 || n <= r || n >= VERTS {
            return Err("Condition 1 violated: invalid vertex count or ring size");
        }

        // Condition (2): Degree bounds
        // Ring vertices: 3 <= deg < n, deg < DEG
        for i in 1..=r {
            let d = self.mat[i][0];
            if d < 3 || d >= n || d >= DEG {
                return Err("Condition 2 violated: ring vertex degree out of bounds");
            }
        }
        // Internal vertices: 5 <= deg < n, deg < DEG
        for i in (r + 1)..=n {
            let d = self.mat[i][0];
            if d < 5 || d >= n || d >= DEG {
                return Err("Condition 2 violated: internal vertex degree < 5 or >= n");
            }
        }

        // Condition (3): Neighbor index bounds 1..=n
        for i in 1..=n {
            let d = self.mat[i][0];
            for j in 1..=d {
                let nb = self.mat[i][j];
                if nb < 1 || nb > n {
                    return Err("Condition 3 violated: neighbor out of range 1..=n");
                }
            }
        }

        // Condition (4): Chordless ring boundary in clockwise order
        for i in 1..=r {
            let d = self.mat[i][0];
            let expected_next = if i == r { 1 } else { i + 1 };
            let expected_prev = if i == 1 { r } else { i - 1 };
            if self.mat[i][1] != expected_next {
                return Err("Condition 4 violated: ring vertex first neighbor is not clockwise next");
            }
            if self.mat[i][d] != expected_prev {
                return Err("Condition 4 violated: ring vertex last neighbor is not counterclockwise prev");
            }
            // Intermediate neighbors must be strictly internal (no chords across the ring)
            for j in 2..d {
                let nb = self.mat[i][j];
                if nb <= r || nb > n {
                    return Err("Condition 4 violated: chord detected on boundary cycle");
                }
            }
        }

        // Condition (5): Euler triangulation formula
        // Sum of degrees must equal 6*(n - 1) - 2*r
        let mut degree_sum = 0;
        for i in 1..=n {
            degree_sum += self.mat[i][0];
        }
        let expected_sum = 6 * (n - 1) - 2 * r;
        if degree_sum != expected_sum {
            return Err("Condition 5 violated: Euler triangulation degree sum mismatch");
        }

        // Condition (6): Boundary contact bound
        // For each internal vertex i > r, the number of ring transitions k <= 4
        for i in (r + 1)..=n {
            let mut k = 0;
            let d = self.mat[i][0];
            for j in 1..=d {
                let n1 = self.mat[i][j];
                let n2 = self.mat[i][if j < d { j + 1 } else { 1 }];
                if n1 > r && n2 <= r {
                    k += 1;
                    let n3 = self.mat[i][if j < d - 1 { j + 2 } else { j + 2 - d }];
                    if n3 <= r {
                        k += 1;
                    }
                }
            }
            if k > 4 {
                return Err("Condition 6 violated: internal vertex has boundary contact > 4");
            }
        }

        // Condition (7): Planar orientability / facial consistency
        // If 'a' follows 'k' in i's cyclic list, then 'i' must follow 'a' in k's cyclic list.
        for i in 1..=n {
            let d_i = self.mat[i][0];
            for j in 1..=d_i {
                let a = if j == d_i {
                    if i <= r {
                        continue;
                    }
                    self.mat[i][1]
                } else {
                    self.mat[i][j + 1]
                };
                let k = self.mat[i][j];
                let d_k = self.mat[k][0];
                let mut p = 1;
                while p < d_k {
                    if a == self.mat[k][p] && i == self.mat[k][p + 1] {
                        break;
                    }
                    p += 1;
                }
                if p == d_k && (a != self.mat[k][p] || i != self.mat[k][1]) {
                    return Err("Condition 7 violated: inconsistent planar face orientation");
                }
            }
        }

        // Radius <= 2 check (from RSST Radius())
        // At least one internal vertex reaches all other internal vertices in <= 2 steps
        let mut has_valid_center = false;
        for u in (r + 1)..=n {
            let mut reached = [false; VERTS];
            reached[u] = true;
            let d_u = self.mat[u][0];
            for i in 1..=d_u {
                let v = self.mat[u][i];
                reached[v] = true;
                if v > r {
                    let d_v = self.mat[v][0];
                    for j in 1..=d_v {
                        reached[self.mat[v][j]] = true;
                    }
                }
            }
            let mut all_reached = true;
            for v in (r + 1)..=n {
                if !reached[v] {
                    all_reached = false;
                    break;
                }
            }
            if all_reached {
                has_valid_center = true;
                break;
            }
        }
        if !has_valid_center {
            return Err("Radius bound violated: no internal vertex reaches all others within distance 2");
        }

        Ok(())
    }

    /// Fast-fail boolean check for RSST geometric admissibility.
    #[inline]
    pub fn is_geometrically_admissible(&self) -> bool {
        self.check_rsst_admissibility().is_ok()
    }
}

/// Computes the canonical edge signature of a planar configuration modulo the dihedral
/// group D_{2R} of boundary symmetries (all rotations and reflections of the ring cycle).
///
/// Mathematical Invariant:
/// Two configurations C1 and C2 represent the exact same planar triangulation of the disk
/// (differing only by cyclic shift, reflection, or internal vertex relabeling) if and only if
/// their canonical dihedral signatures are equal.
pub fn canonical_dihedral_signature(conf: &Configuration) -> (usize, usize, Vec<(usize, usize)>) {
    let verts = conf.verts();
    let ring = conf.ring();
    if verts == 0 || ring < 3 || verts >= VERTS {
        return (ring, verts, Vec::new());
    }

    let mut best_edges: Option<Vec<(usize, usize)>> = None;

    // Test all 2 * ring symmetries of the boundary cycle (dihedral group D_{2R}):
    // - R rotations: shift in 0..ring
    // - R reflections: shift in 0..ring with inverted orientation
    for reflection in [false, true] {
        for shift in 0..ring {
            let mut pi = [0usize; VERTS];
            let mut inv_pi = [0usize; VERTS];

            for v in 1..=ring {
                let mapped = if !reflection {
                    ((v - 1 + shift) % ring) + 1
                } else {
                    ((shift + ring - ((v - 1) % ring)) % ring) + 1
                };
                pi[v] = mapped;
                if mapped < VERTS {
                    inv_pi[mapped] = v;
                }
            }

            // Start BFS queue with ring vertices ordered by their mapped label 1..=ring
            let mut queue = [0usize; VERTS];
            let mut q_tail = 0;
            for lbl in 1..=ring {
                queue[q_tail] = inv_pi[lbl];
                q_tail += 1;
            }

            let mut q_head = 0;
            let mut next_label = ring + 1;

            while q_head < q_tail && next_label <= verts + 1 {
                let u = queue[q_head];
                q_head += 1;
                let deg = conf.mat[u][0];

                if !reflection {
                    for i in 1..=deg {
                        let v = conf.mat[u][i];
                        if v > ring && v <= verts && pi[v] == 0 {
                            pi[v] = next_label;
                            next_label += 1;
                            if q_tail < VERTS {
                                queue[q_tail] = v;
                                q_tail += 1;
                            }
                        }
                    }
                } else {
                    for i in (1..=deg).rev() {
                        let v = conf.mat[u][i];
                        if v > ring && v <= verts && pi[v] == 0 {
                            pi[v] = next_label;
                            next_label += 1;
                            if q_tail < VERTS {
                                queue[q_tail] = v;
                                q_tail += 1;
                            }
                        }
                    }
                }
            }

            if next_label == verts + 1 {
                let mut edges = Vec::with_capacity(EDGES);
                for u in 1..=verts {
                    for h in 1..=conf.mat[u][0] {
                        let v = conf.mat[u][h];
                        if u < v {
                            let (m_u, m_v) = (pi[u], pi[v]);
                            edges.push((m_u.min(m_v), m_u.max(m_v)));
                        }
                    }
                }
                edges.sort_unstable();

                match &best_edges {
                    None => best_edges = Some(edges),
                    Some(cur) => {
                        if &edges < cur {
                            best_edges = Some(edges);
                        }
                    }
                }
            }
        }
    }

    if let Some(best) = best_edges {
        (ring, verts, best)
    } else {
        // Fallback: standard sorted edge list
        let mut edges = Vec::new();
        for u in 1..=verts {
            for h in 1..=conf.mat[u][0] {
                let v = conf.mat[u][h];
                if u < v {
                    edges.push((u, v));
                }
            }
        }
        edges.sort_unstable();
        (ring, verts, edges)
    }
}

pub struct Angles {
    pub ring: usize,
    pub edges: usize,
    pub angle: [[usize; 5]; EDGES],
    pub diffangle: [[usize; 5]; EDGES],
    pub sameangle: [[usize; 5]; EDGES],
    pub contract: [usize; EDGES + 1],
    pub is_sparse: bool,
}

pub fn read_configurations<P: AsRef<Path>>(path: P) -> io::Result<Vec<Configuration>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut configs = Vec::new();

    while let Some(line) = lines.next() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let name = trimmed.to_string();
        let id: usize = name.parse().unwrap_or(configs.len() + 1);

        let header_line = match lines.next() {
            Some(l) => l?,
            None => break,
        };
        let h_tokens: Vec<usize> = header_line
            .split_whitespace()
            .filter_map(|t| t.parse().ok())
            .collect();
        if h_tokens.len() < 3 {
            break;
        }
        let verts = h_tokens[0];
        let ring = h_tokens[1];
        let extent_claim = h_tokens[2];
        let max_cons = if h_tokens.len() >= 4 { h_tokens[3] } else { 0 };
        let mut conf = Configuration::new(id, verts, ring, extent_claim);
        conf.name = name;
        conf.set_max_cons_subset(max_cons);

        // Contract line
        let contract_line = match lines.next() {
            Some(l) => l?,
            None => break,
        };
        let c_tokens: Vec<usize> = contract_line
            .split_whitespace()
            .filter_map(|t| t.parse().ok())
            .collect();
        if !c_tokens.is_empty() {
            let n_contracts = c_tokens[0];
            let mut idx = 1;
            for _ in 0..n_contracts {
                if idx + 1 < c_tokens.len() {
                    conf.contract_edges.push((c_tokens[idx], c_tokens[idx + 1]));
                    idx += 2;
                }
            }
        }

        // Adjacency for v in 1..=verts
        let mut vertices_read = 0;
        for _ in 1..=verts {
            let adj_line = match lines.next() {
                Some(l) => l?,
                None => break,
            };
            let a_tokens: Vec<usize> = adj_line
                .split_whitespace()
                .filter_map(|t| t.parse().ok())
                .collect();
            if a_tokens.len() >= 2 {
                let v = a_tokens[0];
                let deg = a_tokens[1];
                let mut nbs = Vec::new();
                for i in 0..deg {
                    if 2 + i < a_tokens.len() {
                        nbs.push(a_tokens[2 + i]);
                    }
                }
                conf.set_vertex(v, &nbs);
                vertices_read += 1;
            }
        }

        if vertices_read < verts {
            break;
        }

        // Coordinates: skip all lines until we reach an empty line or EOF
        for peek_line in lines.by_ref() {
            let pl = peek_line?;
            if pl.trim().is_empty() {
                break;
            }
        }

        configs.push(conf);
    }

    Ok(configs)
}

fn ininterval(grav: &[usize; DEG], done: &[usize; VERTS]) -> usize {
    let d = grav[0];
    if d == 0 {
        return 0;
    }

    let mut first = 1;
    while first < d && done[grav[first]] == 0 {
        first += 1;
    }
    if first == d {
        return done[grav[d]];
    }

    let mut last = first;
    while last < d && done[grav[last + 1]] != 0 {
        last += 1;
    }
    let length = last - first + 1;
    if last == d {
        return length;
    }
    if first > 1 {
        for j in (last + 2)..=d {
            if done[grav[j]] != 0 {
                return 0;
            }
        }
        return length;
    }

    let mut worried = 0;
    let mut total_length = length;
    for j in (last + 2)..=d {
        if done[grav[j]] != 0 {
            total_length += 1;
            worried = 1;
        } else if worried != 0 {
            return 0;
        }
    }
    total_length
}

pub fn strip(graph: &ConfMat, edgeno: &mut [[usize; VERTS]; VERTS]) -> usize {
    let verts = graph[0][0];
    let ring = graph[0][1];

    for u in 1..VERTS {
        for v in 1..VERTS {
            edgeno[u][v] = 0;
        }
    }

    for v in 1..=ring {
        let u = if v > 1 { v - 1 } else { ring };
        edgeno[u][v] = v;
        edgeno[v][u] = v;
    }

    let mut done = [0usize; VERTS];
    let mut term = 3 * (verts - 1) - ring;

    let mut max_arr = [0usize; VERTS];

    for _x in (ring + 1)..=verts {
        let mut maxint = 0;
        let mut maxes = 0;

        for v in (ring + 1)..=verts {
            if done[v] != 0 {
                continue;
            }
            let inter = ininterval(&graph[v], &done);
            if inter > maxint {
                maxint = inter;
                maxes = 1;
                max_arr[1] = v;
            } else if inter == maxint {
                maxes += 1;
                max_arr[maxes] = v;
            }
        }

        let mut maxdeg = 0;
        let mut best = max_arr[1];
        for h in 1..=maxes {
            let d = graph[max_arr[h]][0];
            if d > maxdeg {
                maxdeg = d;
                best = max_arr[h];
            }
        }

        let grav = &graph[best];
        let d = grav[0];
        let mut first = 1;
        let mut previous = done[grav[d]];
        while previous != 0 || done[grav[first]] == 0 {
            previous = done[grav[first]];
            first += 1;
            if first > d {
                first = 1;
                break;
            }
        }

        let mut h = first;
        while done[grav[h]] != 0 {
            edgeno[best][grav[h]] = term;
            edgeno[grav[h]][best] = term;
            term -= 1;
            if h == d {
                if first == 1 {
                    break;
                }
                h = 0;
            }
            h += 1;
        }
        done[best] = 1;
    }

    for _x in 1..=ring {
        let mut maxint = 0;
        let mut best = 1;
        for v in 1..=ring {
            if done[v] != 0 {
                continue;
            }
            let u = if v > 1 { v - 1 } else { ring };
            let w = if v < ring { v + 1 } else { 1 };
            let inter = 3 * graph[v][0] + 4 * (done[u] + done[w]);
            if inter > maxint {
                maxint = inter;
                best = v;
            }
        }

        let grav = &graph[best];
        let u = if best > 1 { best - 1 } else { ring };
        if done[u] != 0 {
            for h in (2..grav[0]).rev() {
                edgeno[best][grav[h]] = term;
                edgeno[grav[h]][best] = term;
                term -= 1;
            }
        } else {
            for h in 2..grav[0] {
                edgeno[best][grav[h]] = term;
                edgeno[grav[h]][best] = term;
                term -= 1;
            }
        }
        done[best] = 1;
    }

    3 * (verts - 1) - ring
}

pub fn find_angles(conf: &Configuration) -> Angles {
    let mut edgeno = [[0usize; VERTS]; VERTS];
    let total_edges = strip(&conf.mat, &mut edgeno);

    let mut is_sparse = true;

    let mut angles = Angles {
        ring: conf.ring(),
        edges: total_edges,
        angle: [[0; 5]; EDGES],
        diffangle: [[0; 5]; EDGES],
        sameangle: [[0; 5]; EDGES],
        contract: [0; EDGES + 1],
        is_sparse: true,
    };

    angles.contract[0] = conf.contract_edges.len();
    angles.contract[EDGES] = conf.max_cons_subset();

    for &(u, v) in &conf.contract_edges {
        if u < VERTS && v < VERTS {
            let e = edgeno[u][v];
            if e > 0 && e <= EDGES {
                angles.contract[e] = 1;
            }
        }
    }

    for i in 1..=angles.ring {
        if angles.contract[i] != 0 {
            is_sparse = false;
        }
    }

    for v in 1..=conf.verts() {
        let deg = conf.mat[v][0];
        for h in 1..=deg {
            if v <= conf.ring() && h == deg {
                continue;
            }
            let i = if h < deg { h + 1 } else { 1 };
            let u = conf.mat[v][h];
            let w = conf.mat[v][i];
            let a = edgeno[v][w];
            let b = edgeno[u][w];
            let c = edgeno[u][v];

            if a > 0
                && b > 0
                && angles.contract[a] != 0
                && angles.contract[b] != 0
                && c > conf.ring()
            {
                is_sparse = false;
            }

            if a > c {
                if angles.angle[c][0] < 4 {
                    angles.angle[c][0] += 1;
                    let cnt = angles.angle[c][0];
                    angles.angle[c][cnt] = a;
                }

                if angles.contract[a] == 0 && angles.contract[b] == 0 && angles.contract[c] == 0 {
                    if angles.diffangle[c][0] < 4 {
                        angles.diffangle[c][0] += 1;
                        let dc = angles.diffangle[c][0];
                        angles.diffangle[c][dc] = a;
                    }
                }
                if angles.contract[b] != 0 {
                    if angles.sameangle[c][0] < 4 {
                        angles.sameangle[c][0] += 1;
                        let sc = angles.sameangle[c][0];
                        angles.sameangle[c][sc] = a;
                    }
                }
            }
            if b > c {
                if angles.angle[c][0] < 4 {
                    angles.angle[c][0] += 1;
                    let cnt = angles.angle[c][0];
                    angles.angle[c][cnt] = b;
                }

                if angles.contract[a] == 0 && angles.contract[b] == 0 && angles.contract[c] == 0 {
                    if angles.diffangle[c][0] < 4 {
                        angles.diffangle[c][0] += 1;
                        let dc = angles.diffangle[c][0];
                        angles.diffangle[c][dc] = b;
                    }
                }
                if angles.contract[a] != 0 {
                    if angles.sameangle[c][0] < 4 {
                        angles.sameangle[c][0] += 1;
                        let sc = angles.sameangle[c][0];
                        angles.sameangle[c][sc] = b;
                    }
                }
            }
        }
    }

    angles.is_sparse = is_sparse;
    angles
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

pub fn extract_triangles(conf: &Configuration, edgeno: &[[usize; VERTS]; VERTS]) -> Vec<Triangle> {
    let mut triangles = Vec::new();
    let ring = conf.ring();
    for v in 1..=conf.verts() {
        let deg = conf.mat[v][0];
        for h in 1..=deg {
            if v <= ring && h == deg {
                continue;
            }
            let i = if h < deg { h + 1 } else { 1 };
            let u = conf.mat[v][h];
            let w = conf.mat[v][i];
            let a = edgeno[v][w];
            let b = edgeno[u][w];
            let c = edgeno[u][v];
            triangles.push(Triangle { a, b, c });
        }
    }
    triangles
}

pub fn build_contract_angles(
    base: &Angles,
    triangles: &[Triangle],
    max_cons_subset: usize,
    contract_edges: &[usize],
) -> Angles {
    let mut angles = Angles {
        ring: base.ring,
        edges: base.edges,
        angle: base.angle,
        diffangle: [[0; 5]; EDGES],
        sameangle: [[0; 5]; EDGES],
        contract: [0; EDGES + 1],
        is_sparse: true,
    };

    angles.contract[0] = contract_edges.len();
    angles.contract[EDGES] = max_cons_subset;

    for &e in contract_edges {
        if e <= EDGES {
            angles.contract[e] = 1;
            if e <= angles.ring {
                angles.is_sparse = false;
            }
        }
    }

    for &Triangle { a, b, c } in triangles {
        if a > 0 && b > 0 && angles.contract[a] != 0 && angles.contract[b] != 0 && c > angles.ring {
            angles.is_sparse = false;
        }

        if a > c {
            if angles.contract[a] == 0 && angles.contract[b] == 0 && angles.contract[c] == 0 {
                if angles.diffangle[c][0] < 4 {
                    angles.diffangle[c][0] += 1;
                    let dc = angles.diffangle[c][0];
                    angles.diffangle[c][dc] = a;
                }
            }
            if angles.contract[b] != 0 {
                if angles.sameangle[c][0] < 4 {
                    angles.sameangle[c][0] += 1;
                    let sc = angles.sameangle[c][0];
                    angles.sameangle[c][sc] = a;
                }
            }
        }
        if b > c {
            if angles.contract[a] == 0 && angles.contract[b] == 0 && angles.contract[c] == 0 {
                if angles.diffangle[c][0] < 4 {
                    angles.diffangle[c][0] += 1;
                    let dc = angles.diffangle[c][0];
                    angles.diffangle[c][dc] = b;
                }
            }
            if angles.contract[a] != 0 {
                if angles.sameangle[c][0] < 4 {
                    angles.sameangle[c][0] += 1;
                    let sc = angles.sameangle[c][0];
                    angles.sameangle[c][sc] = b;
                }
            }
        }
    }

    angles
}

#[allow(dead_code)]
pub fn validate_triad_endpoints(conf: &Configuration, endpoints: &[usize]) -> bool {
    if endpoints.len() < 8 {
        return true;
    }
    let verts = conf.verts();
    let ring = conf.ring();

    for v in (ring + 1)..=verts {
        let deg = conf.mat[v][0];
        let mut a = 0;
        for i in 1..=deg {
            let u = conf.mat[v][i];
            if endpoints.contains(&u) {
                a += 1;
            }
        }
        if a < 3 {
            continue;
        }
        if deg >= 6 {
            return true;
        }

        let mut is_neighbour = [false; VERTS];
        for i in 1..=deg {
            let u = conf.mat[v][i];
            if u < VERTS {
                is_neighbour[u] = true;
            }
        }
        for &ep in endpoints {
            if ep < VERTS && !is_neighbour[ep] {
                return true;
            }
        }
    }
    false
}

#[allow(dead_code)]
pub fn validate_triad(conf: &Configuration) -> bool {
    if conf.contract_edges.len() < 4 {
        return true;
    }
    let mut endpoints = [0usize; 8];
    for (i, &(u, v)) in conf.contract_edges.iter().take(4).enumerate() {
        endpoints[2 * i] = u;
        endpoints[2 * i + 1] = v;
    }
    validate_triad_endpoints(conf, &endpoints)
}

#[allow(dead_code)]
pub fn validate_sparse_contract(conf: &Configuration, angles: &Angles) -> Result<(), &'static str> {
    let n = conf.contract_edges.len();
    if n == 0 {
        return Ok(());
    }
    if n > 4 {
        return Err("Contract has more than 4 edges");
    }
    if !angles.is_sparse {
        return Err(
            "Contract is not sparse (contains ring edge or multiple edges in same triangle)",
        );
    }
    if n == 4 && !validate_triad(conf) {
        return Err("Contract has no triad");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_diamond() -> Configuration {
        let mut conf = Configuration::new(1, 10, 6, 4);
        conf.set_vertex(1, &[2, 7, 10, 6]);
        conf.set_vertex(2, &[3, 8, 7, 1]);
        conf.set_vertex(3, &[4, 8, 2]);
        conf.set_vertex(4, &[5, 9, 8, 3]);
        conf.set_vertex(5, &[6, 10, 9, 4]);
        conf.set_vertex(6, &[1, 10, 5]);
        conf.set_vertex(7, &[2, 8, 9, 10, 1]);
        conf.set_vertex(8, &[2, 3, 4, 9, 7]);
        conf.set_vertex(9, &[8, 4, 5, 10, 7]);
        conf.set_vertex(10, &[9, 5, 6, 1, 7]);
        conf
    }

    #[test]
    fn test_canonical_dihedral_signature_self_consistent() {
        let conf = make_test_diamond();
        let sig1 = canonical_dihedral_signature(&conf);
        let sig2 = canonical_dihedral_signature(&conf);
        assert_eq!(sig1, sig2);
        assert_eq!(sig1.0, 6);
        assert_eq!(sig1.1, 10);
        assert!(!sig1.2.is_empty());
    }

    #[test]
    fn test_canonical_dihedral_signature_distinct() {
        let conf1 = make_test_diamond();
        let mut conf2 = make_test_diamond();
        conf2.set_vertex(3, &[4, 2]);
        let sig1 = canonical_dihedral_signature(&conf1);
        let sig2 = canonical_dihedral_signature(&conf2);
        assert_ne!(sig1, sig2);
    }

    #[test]
    fn test_rsst_admissibility_valid() {
        let conf = make_test_diamond();
        assert!(conf.is_geometrically_admissible());
        assert_eq!(conf.check_rsst_admissibility(), Ok(()));
    }

    #[test]
    fn test_rsst_admissibility_condition_2_ring_degree() {
        let mut conf = make_test_diamond();
        // Ring vertex degree < 3 should be rejected
        conf.mat[3][0] = 2;
        let res = conf.check_rsst_admissibility();
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("Condition 2 violated"));
    }

    #[test]
    fn test_rsst_admissibility_condition_2_internal_degree() {
        let mut conf = make_test_diamond();
        // Internal vertex degree < 5 should be rejected
        conf.mat[7][0] = 4;
        let res = conf.check_rsst_admissibility();
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("Condition 2 violated"));
    }

    #[test]
    fn test_rsst_admissibility_condition_4_chord() {
        let mut conf = make_test_diamond();
        // Add a chord between ring vertex 1 and ring vertex 3
        conf.mat[1][2] = 3; // Intermediate neighbor is on ring (<= r)
        let res = conf.check_rsst_admissibility();
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("Condition 4 violated"));
    }

    #[test]
    fn test_rsst_admissibility_condition_5_euler() {
        let mut conf = make_test_diamond();
        // Modify internal vertex 7 to have degree 6 with valid neighbors in 1..=10
        // Conditions 1-4 pass, but Euler degree sum is 43 != 42
        conf.set_vertex(7, &[2, 8, 9, 10, 1, 8]);
        let res = conf.check_rsst_admissibility();
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("Condition 5 violated"));
    }

    #[test]
    fn test_rsst_admissibility_unavoidable_25() {
        let path = "08_pesquisa_2flips/unavoidable_25.conf";
        if let Ok(configs) = read_configurations(path) {
            assert_eq!(configs.len(), 25);
            for conf in &configs {
                assert!(
                    conf.is_geometrically_admissible(),
                    "Canonical unavoidable config {} failed RSST admissibility: {:?}",
                    conf.name,
                    conf.check_rsst_admissibility()
                );
            }
        }
    }

    #[test]
    fn test_rsst_admissibility_raw_candidates_filter_exact_16() {
        let path = "08_pesquisa_2flips/candidates_2flips_raw.conf";
        if let Ok(configs) = read_configurations(path) {
            assert_eq!(configs.len(), 1572);
            let mut failed_count = 0;
            let mut passed_count = 0;
            for conf in &configs {
                if conf.is_geometrically_admissible() {
                    passed_count += 1;
                } else {
                    failed_count += 1;
                }
            }
            assert_eq!(failed_count, 16, "Must reject exactly 16 defective configurations");
            assert_eq!(passed_count, 1556, "Must accept exactly 1556 valid configurations");
        }
    }
}

