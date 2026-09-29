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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Configuration;

    // Helper function to create a basic Configuration for testing
    // To satisfy internal flip conditions, internal vertices must have deg > 5.

    fn setup_valid_flip_config() -> Configuration {
        let mut conf = Configuration::new(1, 8, 4, 0); // 4 ring vertices, 4 internal

        // Ring: 1, 2, 3, 4
        // Internal: 5, 6, 7, 8
        // For a valid planar triangulation face, (u, v, w) and (u, z, v) are the two triangles.
        // Let u=5, v=6, w=7, z=8.

        conf.set_vertex(1, &[2, 6, 8, 4, 5]);
        conf.set_vertex(2, &[3, 7, 5, 1, 6]);
        conf.set_vertex(3, &[4, 8, 6, 2, 7]);
        conf.set_vertex(4, &[1, 5, 7, 3, 8]);

        conf.set_vertex(5, &[1, 4, 7, 6, 8, 2]); // Neighbors of 5
        conf.set_vertex(6, &[2, 5, 8, 7, 1, 3]); // Neighbors of 6
        conf.set_vertex(7, &[3, 4, 5, 6, 2]);
        conf.set_vertex(8, &[4, 1, 6, 5, 3]);

        conf
    }

    #[test]
    fn test_valid_flip() {
        let conf = setup_valid_flip_config();
        let flips = generate_internal_flips(&conf);

        // We expect exactly 1 valid flip: (4, 5) to (1, 7) based on our mock data.
        assert_eq!(flips.len(), 1);
        assert_eq!(flips[0].flipped_edge, (4, 5));
        assert_eq!(flips[0].new_edge, (1, 7));

        // Ensure the cyclic order in the new configuration reflects the flip
        // In the original, 4's neighbors were: [1, 5, 7, 3, 8]
        // 5's neighbors were: [1, 4, 7, 6, 8, 2]
        // The flipped edge is (4, 5), meaning 5 is removed from 4, 4 is removed from 5.
        // The new edge is (1, 7), so 7 is inserted in 1, and 1 is inserted in 7.
        let new_conf = &flips[0].conf;

        // Vertex 4 shouldn't have 5 in its list
        let mut nbs_4 = Vec::new();
        for i in 1..=new_conf.mat[4][0] {
            nbs_4.push(new_conf.mat[4][i]);
        }
        assert!(!nbs_4.contains(&5));

        // Vertex 5 shouldn't have 4 in its list
        let mut nbs_5 = Vec::new();
        for i in 1..=new_conf.mat[5][0] {
            nbs_5.push(new_conf.mat[5][i]);
        }
        assert!(!nbs_5.contains(&4));

        // Vertex 1 should now have 7 in its list
        let mut nbs_1 = Vec::new();
        for i in 1..=new_conf.mat[1][0] {
            nbs_1.push(new_conf.mat[1][i]);
        }
        assert!(nbs_1.contains(&7));

        // Vertex 7 should now have 1 in its list
        let mut nbs_7 = Vec::new();
        for i in 1..=new_conf.mat[7][0] {
            nbs_7.push(new_conf.mat[7][i]);
        }
        assert!(nbs_7.contains(&1));
    }

    #[test]
    fn test_generate_internal_flips_existing_edge() {
        let mut conf = setup_valid_flip_config();
        // Edge (1, 7) would be the new edge for flip (4, 5).
        // Let's add (1, 7) to the original configuration by modifying 1 and 7.
        // We just append to make it look like the edge already exists.
        let deg_1 = conf.mat[1][0];
        conf.mat[1][0] = deg_1 + 1;
        conf.mat[1][deg_1 + 1] = 7;

        let deg_7 = conf.mat[7][0];
        conf.mat[7][0] = deg_7 + 1;
        conf.mat[7][deg_7 + 1] = 1;

        let flips = generate_internal_flips(&conf);
        // By adding an edge (1, 7), we might create new valid internal flips like (4, 7).
        // However, we want to check that flip (4, 5) with new edge (1, 7) is NOT in the list.
        let contains_4_5_flip = flips
            .iter()
            .any(|f| f.flipped_edge == (4, 5) || f.flipped_edge == (5, 4));
        assert!(
            !contains_4_5_flip,
            "Expected flip (4, 5) to be rejected since new edge (1, 7) already exists"
        );
    }

    #[test]
    fn test_generate_internal_flips_low_degree() {
        let mut conf = setup_valid_flip_config();
        // By removing an edge, we reduce degree to <= 5
        // We know (4, 5) is the flipped edge (u=4, v=5).
        // Let's reduce degree of vertex 5 to 5 (from 6).
        // It currently has neighbors: [1, 4, 7, 6, 8, 2] -> length 6
        // Let's just set the length to 5 (drop 2).
        conf.mat[5][0] = 5;

        let flips = generate_internal_flips(&conf);
        assert!(
            flips.is_empty(),
            "Expected no flips since degree drops below 5"
        );
    }

    #[test]
    fn test_generate_internal_flips_chord() {
        let mut conf = setup_valid_flip_config();
        // The flipped edge is (u=4, v=5). The candidate new edge is (w=1, z=7).
        // Here, w=1 (ring) and z=7 (internal).
        // To test chord rejection, both w and z must be <= ring.
        // Let's mock a flip where both are <= ring.
        // We can just temporarily increase the ring size to 7.
        conf.mat[0][1] = 7; // Ring size is now 7.
                            // w=1 <= 7, z=7 <= 7. This should be rejected as a chord!

        let flips = generate_internal_flips(&conf);
        assert!(
            flips.is_empty(),
            "Expected no flips since it forms a chord between ring vertices"
        );
    }

    #[test]
    fn test_generate_internal_flips_invalid_angle() {
        let mut conf = setup_valid_flip_config();
        // The logic checks if the triangles align correctly:
        // `if !((w_cand == w && z_cand == z) || (w_cand == z && z_cand == w))`
        // To trigger this, we need to mess up the cyclic order in `v` (5).
        // `w` and `z` are found from `u` (4). u=4, v=5.
        // Neighbors of 4: [1, 5, 7, 3, 8]. So w=1, z=7.
        // We need to ensure that in 5's cyclic list, 1 and 7 are not adjacent to 4.
        // Original neighbors of 5: [1, 4, 7, 6, 8, 2]
        // Here, around 4 in 5's list we have 1 and 7.
        // Let's swap 7 and 6 in 5's list, so neighbors of 5 are: [1, 4, 6, 7, 8, 2]
        conf.set_vertex(5, &[1, 4, 6, 7, 8, 2]);

        let flips = generate_internal_flips(&conf);
        // We should not see flip (4, 5) in the list anymore
        let contains_4_5_flip = flips
            .iter()
            .any(|f| f.flipped_edge == (4, 5) || f.flipped_edge == (5, 4));
        assert!(
            !contains_4_5_flip,
            "Expected flip (4, 5) to be rejected due to invalid planar triangulation face"
        );
    }
}
