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
            }
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
