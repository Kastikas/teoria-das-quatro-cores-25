use crate::graph::Configuration;
use std::collections::HashSet;

pub struct FlippedConfig {
    pub conf: Configuration,
    pub flipped_edge: (usize, usize),
    pub new_edge: (usize, usize),
}

pub fn generate_internal_flips(conf: &Configuration) -> Vec<FlippedConfig> {
    let verts = conf.verts();
    let ring = conf.ring();
    let mut results = Vec::new();

    let mut existing_edges = HashSet::new();
    for u in 1..=verts {
        for h in 1..=conf.mat[u][0] {
            let v = conf.mat[u][h];
            existing_edges.insert((u.min(v), u.max(v)));
        }
    }

    // Inspect each internal edge (u, v)
    for u in 1..=verts {
        let deg_u = conf.mat[u][0];
        for h_u in 1..=deg_u {
            let v = conf.mat[u][h_u];
            if u >= v {
                continue;
            }
            let is_ring_edge = u <= ring && v <= ring && (v == u + 1 || (u == 1 && v == ring));
            if is_ring_edge {
                continue;
            }

            // In cyclic order around u, v is at index h_u.
            // Find w and z: the neighbors immediately before and after v in u's list
            let prev_u = if h_u == 1 { deg_u } else { h_u - 1 };
            let next_u = if h_u == deg_u { 1 } else { h_u + 1 };
            let w = conf.mat[u][prev_u];
            let z = conf.mat[u][next_u];

            // Verify that in v's cyclic list, u is between z and w
            let deg_v = conf.mat[v][0];
            let mut h_v = 0;
            for i in 1..=deg_v {
                if conf.mat[v][i] == u {
                    h_v = i;
                    break;
                }
            }
            if h_v == 0 {
                continue;
            }

            let prev_v = if h_v == 1 { deg_v } else { h_v - 1 };
            let next_v = if h_v == deg_v { 1 } else { h_v + 1 };
            let w_cand = conf.mat[v][next_v];
            let z_cand = conf.mat[v][prev_v];

            // For a valid planar triangulation face, (u, v, w) and (u, z, v) are the two triangles
            if !((w_cand == w && z_cand == z) || (w_cand == z && z_cand == w)) {
                continue;
            }

            // Cannot flip if (w, z) is already an edge
            if existing_edges.contains(&(w.min(z), w.max(z))) {
                continue;
            }

            // A configuration ring must be chordless (cannot add a chord between two ring vertices)
            if w <= ring && z <= ring {
                continue;
            }

            // Ensure degrees of internal vertices don't drop below 5
            if u > ring && deg_u <= 5 {
                continue;
            }
            if v > ring && deg_v <= 5 {
                continue;
            }

            // Construct new configuration with updated cyclic orders
            let mut new_conf = Configuration::new(9999, verts, ring, 0);

            for i in 1..=verts {
                let deg_i = conf.mat[i][0];
                let mut nbs: Vec<usize> = (1..=deg_i).map(|idx| conf.mat[i][idx]).collect();

                if i == u {
                    // Remove v from u's list
                    nbs.retain(|&x| x != v);
                } else if i == v {
                    // Remove u from v's list
                    nbs.retain(|&x| x != u);
                } else if i == w {
                    // Insert z between u and v (or v and u)
                    let mut new_list = Vec::new();
                    for &item in &nbs {
                        new_list.push(item);
                        if (item == u && nbs.contains(&v)) || (item == v && nbs.contains(&u)) {
                            // Find where to insert z
                        }
                    }
                    // Insert z right next to u or v where they meet
                    if let Some(pos_u) = nbs.iter().position(|&x| x == u) {
                        if let Some(pos_v) = nbs.iter().position(|&x| x == v) {
                            let insert_pos = pos_u.max(pos_v);
                            nbs.insert(insert_pos, z);
                        }
                    }
                } else if i == z {
                    if let Some(pos_u) = nbs.iter().position(|&x| x == u) {
                        if let Some(pos_v) = nbs.iter().position(|&x| x == v) {
                            let insert_pos = pos_u.max(pos_v);
                            nbs.insert(insert_pos, w);
                        }
                    }
                }

                new_conf.set_vertex(i, &nbs);
            }

            results.push(FlippedConfig {
                conf: new_conf,
                flipped_edge: (u, v),
                new_edge: (w, z),
            });
        }
    }

    results
}
