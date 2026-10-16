use crate::graph::Configuration;
use std::collections::{BTreeMap, HashSet};

/// Representa a junção entre dois hubs de grau alto (ex: aresta 8-8 e seus vizinhos comuns).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HubDiamond {
    pub h1: usize,
    pub deg_h1: usize,
    pub h2: usize,
    pub deg_h2: usize,
    pub common_neighbors: Vec<(usize, usize, bool)>, // (v, grau, is_ring)
}

/// Análise estrutural e decomposição de motifs planares de uma configuração.
#[derive(Clone, Debug)]
pub struct MotifAnalysis {
    pub id: usize,
    pub name: String,
    pub v: usize,
    pub r: usize,
    pub i: usize,
    pub ring_degrees: Vec<usize>,
    pub internal_degrees: Vec<usize>,
    pub internal_degree_counts: BTreeMap<usize, usize>,
    pub ring_degree_counts: BTreeMap<usize, usize>,
    pub internal_hubs: Vec<(usize, usize)>, // (v, grau) para grau >= 7
    pub ring_hubs: Vec<(usize, usize)>,
    pub hub_edges: Vec<(usize, usize)>,
    pub diamonds: Vec<HubDiamond>,
    pub euler_ring_sum: i32,
    pub euler_internal_sum: i32,
    pub contracts_count: usize,
    pub is_pure_d_candidate: bool,
}

impl MotifAnalysis {
    pub fn from_configuration(conf: &Configuration) -> Self {
        let v = conf.verts();
        let r = conf.ring();
        let i = if v >= r { v - r } else { 0 };

        let mut ring_degrees = Vec::with_capacity(r);
        let mut ring_degree_counts = BTreeMap::new();
        let mut euler_ring_sum = 0i32;

        for u in 1..=r {
            let deg = conf.mat[u][0];
            ring_degrees.push(deg);
            *ring_degree_counts.entry(deg).or_insert(0) += 1;
            euler_ring_sum += 4 - deg as i32;
        }

        let mut internal_degrees = Vec::with_capacity(i);
        let mut internal_degree_counts = BTreeMap::new();
        let mut euler_internal_sum = 0i32;

        for u in (r + 1)..=v {
            let deg = conf.mat[u][0];
            internal_degrees.push(deg);
            *internal_degree_counts.entry(deg).or_insert(0) += 1;
            euler_internal_sum += 6 - deg as i32;
        }

        // Hubs (grau >= 7)
        let mut internal_hubs = Vec::new();
        let mut ring_hubs = Vec::new();

        for u in 1..=v {
            let deg = conf.mat[u][0];
            if deg >= 7 {
                if u <= r {
                    ring_hubs.push((u, deg));
                } else {
                    internal_hubs.push((u, deg));
                }
            }
        }

        // Arestas entre hubs internos
        let mut hub_edges = Vec::new();
        let mut diamonds = Vec::new();

        for idx1 in 0..internal_hubs.len() {
            let (h1, deg1) = internal_hubs[idx1];
            let nbrs1: HashSet<usize> = (1..=deg1).map(|k| conf.mat[h1][k]).collect();

            for idx2 in (idx1 + 1)..internal_hubs.len() {
                let (h2, deg2) = internal_hubs[idx2];
                if nbrs1.contains(&h2) {
                    hub_edges.push((h1, h2));

                    // Vizinhos comuns entre h1 e h2
                    let nbrs2: HashSet<usize> = (1..=deg2).map(|k| conf.mat[h2][k]).collect();
                    let common: Vec<usize> = nbrs1.intersection(&nbrs2).cloned().collect();

                    let mut common_details = Vec::new();
                    for &c in &common {
                        common_details.push((c, conf.mat[c][0], c <= r));
                    }
                    common_details.sort_by_key(|x| x.0);

                    diamonds.push(HubDiamond {
                        h1,
                        deg_h1: deg1,
                        h2,
                        deg_h2: deg2,
                        common_neighbors: common_details,
                    });
                }
            }
        }

        let contracts_count = conf.contract_edges.len();
        let is_pure_d_candidate = contracts_count == 0;

        Self {
            id: conf.id,
            name: conf.name.clone(),
            v,
            r,
            i,
            ring_degrees,
            internal_degrees,
            internal_degree_counts,
            ring_degree_counts,
            internal_hubs,
            ring_hubs,
            hub_edges,
            diamonds,
            euler_ring_sum,
            euler_internal_sum,
            contracts_count,
            is_pure_d_candidate,
        }
    }

    /// Imprime um relatório técnico detalhado e formatado da geometria do grafo.
    pub fn display_summary(&self) {
        println!("================================================================================");
        println!("ANÁLISE ESTRUTURAL DE MOTIF: {} (ID: {})", self.name, self.id);
        println!("================================================================================");
        println!(
            "Dimensões: V = {}, Anel R = {}, Vértices Internos I = {}",
            self.v, self.r, self.i
        );
        println!(
            "Status Redutibilidade: {} (Contratos de Kempe: {})",
            if self.is_pure_d_candidate { "D-Redutível Pura (C=0)" } else { "C-Redutível (C>0)" },
            self.contracts_count
        );
        println!(
            "Conservação Euler: Anel ∑(4-d) = {}, Interior ∑(6-d) = {}, Total = {}",
            self.euler_ring_sum,
            self.euler_internal_sum,
            self.euler_ring_sum + self.euler_internal_sum
        );

        println!("\n--- Núcleo de Hubs (Grau >= 7) ---");
        if self.internal_hubs.is_empty() {
            println!("  Nenhum hub interno de grau >= 7.");
        } else {
            for (h, d) in &self.internal_hubs {
                println!("  Hub Interno v{}: Grau {}", h, d);
            }
        }
        if !self.ring_hubs.is_empty() {
            for (h, d) in &self.ring_hubs {
                println!("  Hub no Anel v{}: Grau {}", h, d);
            }
        }

        if !self.diamonds.is_empty() {
            println!("\n--- Diamantes e Arestas entre Hubs ---");
            for d in &self.diamonds {
                print!("  Aresta Central (v{} [d={}], v{} [d={}]): ", d.h1, d.deg_h1, d.h2, d.deg_h2);
                let c_strs: Vec<String> = d
                    .common_neighbors
                    .iter()
                    .map(|(v, deg, is_ring)| {
                        format!("v{} [d={}, {}]", v, deg, if *is_ring { "anel" } else { "interno" })
                    })
                    .collect();
                println!("Vizinhos Comuns = [{}]", c_strs.join(", "));
            }
        }

        println!("\n--- Perfil de Graus ---");
        print!("  Interior: ");
        for (deg, count) in &self.internal_degree_counts {
            print!("d{}: {}x | ", deg, count);
        }
        println!();
        print!("  Anel:     ");
        for (deg, count) in &self.ring_degree_counts {
            print!("d{}: {}x | ", deg, count);
        }
        println!();
        println!("================================================================================\n");
    }
}

/// Diagnóstico estrutural de compatibilidade entre um cartwheel de folha e uma configuração.
#[derive(Clone, Debug)]
pub struct LeafDiagnosis {
    pub leaf_line: usize,
    pub center_degree: usize,
    pub fixed_spokes: Vec<(usize, usize)>,
    pub open_spokes: Vec<(usize, usize)>,
    pub fixed_vertices_total: usize,
    pub max_internal_capacity: usize,
    pub conf_internal_vertices: usize,
    pub capacity_deficit: i32,
    pub has_compatible_hub: bool,
    pub can_fit_in_level_0: bool,
    pub bottleneck_explanation: String,
}

impl LeafDiagnosis {
    /// Analisa um eixo axial no formato RSST (ex: "1:7+ 2:5 3:6 4:5 5:7+ 6:5 7:5 8:5")
    /// contra uma configuração proposta.
    pub fn diagnose(center_deg: usize, axle_str: &str, leaf_line: usize, conf: &Configuration) -> Self {
        let mut fixed_spokes = Vec::new();
        let mut open_spokes = Vec::new();

        for token in axle_str.split_whitespace() {
            if let Some((s_str, d_str)) = token.split_once(':') {
                if let Ok(spoke) = s_str.parse::<usize>() {
                    if d_str.ends_with('+') {
                        let base_deg = d_str.trim_end_matches('+').parse::<usize>().unwrap_or(7);
                        open_spokes.push((spoke, base_deg));
                    } else if let Ok(deg) = d_str.parse::<usize>() {
                        fixed_spokes.push((spoke, deg));
                    }
                }
            }
        }

        // Vértices finitos fixos = centro (1) + raios fixos
        let fixed_vertices_total = 1 + fixed_spokes.len();
        let max_internal_capacity = fixed_vertices_total;

        let conf_i = conf.verts() - conf.ring();
        let capacity_deficit = conf_i as i32 - max_internal_capacity as i32;

        // Verifica se a configuração possui um hub interno com grau igual ao centro
        let has_compatible_hub = (conf.ring() + 1..=conf.verts()).any(|v| conf.mat[v][0] == center_deg);

        let can_fit_in_level_0 = capacity_deficit <= 0 && has_compatible_hub;

        let mut explanations = Vec::new();
        if capacity_deficit > 0 {
            explanations.push(format!(
                "DÉFICIT DE CAPACIDADE: O grafo possui I = {} vértices internos, mas o cartwheel no Nível 0 possui apenas {} vértices finitos fixos (Déficit de {} vértices que teriam de mapear para ∞).",
                conf_i, max_internal_capacity, capacity_deficit
            ));
        }
        if !has_compatible_hub {
            explanations.push(format!(
                "INCOMPATIBILIDADE DE HUB: O centro do cartwheel possui grau {}, mas o grafo não possui nenhum hub interno com grau {}.",
                center_deg, center_deg
            ));
        }
        if can_fit_in_level_0 {
            explanations.push(
                "COMPATIBILIDADE ESTRUTURAL CONFIRMADA: A capacidade finita é suficiente e o grau do hub é compatível no Nível 0.".to_string(),
            );
        }

        Self {
            leaf_line,
            center_degree: center_deg,
            fixed_spokes,
            open_spokes,
            fixed_vertices_total,
            max_internal_capacity,
            conf_internal_vertices: conf_i,
            capacity_deficit,
            has_compatible_hub,
            can_fit_in_level_0,
            bottleneck_explanation: explanations.join(" "),
        }
    }

    pub fn display(&self) {
        println!("--------------------------------------------------------------------------------");
        println!(
            "DIAGNÓSTICO DE COMPATIBILIDADE FOLHA-GRAFO (Linha {} | Centro Grau {})",
            self.leaf_line, self.center_degree
        );
        println!("--------------------------------------------------------------------------------");
        print!("Raios Fixos ({}): ", self.fixed_spokes.len());
        for (s, d) in &self.fixed_spokes {
            print!("{}:{} ", s, d);
        }
        println!();
        print!("Raios Abertos ({}): ", self.open_spokes.len());
        for (s, d) in &self.open_spokes {
            print!("{}:{}+ ", s, d);
        }
        println!();
        println!(
            "Capacidade Máxima Suportável no Nível 0: I_max = {} vértices internos",
            self.max_internal_capacity
        );
        println!(
            "Vértices Internos do Grafo Testado:    I_conf = {} vértices",
            self.conf_internal_vertices
        );
        println!(
            "Déficit Combinatório:                  {} (Status: {})",
            self.capacity_deficit,
            if self.can_fit_in_level_0 { "VIÁVEL NO NÍVEL 0" } else { "BLOQUEADO NO NÍVEL 0" }
        );
        println!("Diagnóstico Causal:\n  {}", self.bottleneck_explanation);
        println!("--------------------------------------------------------------------------------\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motif_analysis_birkhoff_diamond() {
        let mut conf = Configuration::new(1, 10, 6, 16);
        conf.set_vertex(1, &[2, 9, 6]);
        conf.set_vertex(2, &[3, 8, 9, 1]);
        conf.set_vertex(3, &[4, 7, 8, 2]);
        conf.set_vertex(4, &[5, 10, 7, 3]);
        conf.set_vertex(5, &[6, 10, 4]);
        conf.set_vertex(6, &[1, 9, 10, 5]);
        conf.set_vertex(7, &[3, 8, 10, 4]);
        conf.set_vertex(8, &[2, 9, 10, 7, 3]);
        conf.set_vertex(9, &[1, 6, 10, 8, 2]);
        conf.set_vertex(10, &[4, 5, 6, 9, 8, 7]);

        let analysis = MotifAnalysis::from_configuration(&conf);
        assert_eq!(analysis.v, 10);
        assert_eq!(analysis.r, 6);
        assert_eq!(analysis.i, 4);
        assert_eq!(analysis.euler_ring_sum + analysis.euler_internal_sum, 6);
        assert!(analysis.is_pure_d_candidate);
    }

    #[test]
    fn test_leaf_diagnosis_line_204_capacity_deficit() {
        // Simula Config 326 (I = 7) contra linha 204
        let conf_326 = Configuration::new(326, 18, 11, 0);
        let diag_326 = LeafDiagnosis::diagnose(
            8,
            "1:7+ 2:5 3:6 4:5 5:7+ 6:5 7:5 8:5",
            204,
            &conf_326,
        );
        assert_eq!(diag_326.max_internal_capacity, 7);
        assert_eq!(diag_326.conf_internal_vertices, 7);
        assert_eq!(diag_326.capacity_deficit, 0);

        // Simula Candidato 3234 (I = 9) contra linha 204
        let conf_3234 = Configuration::new(3234, 22, 13, 0);
        let diag_3234 = LeafDiagnosis::diagnose(
            8,
            "1:7+ 2:5 3:6 4:5 5:7+ 6:5 7:5 8:5",
            204,
            &conf_3234,
        );
        assert_eq!(diag_3234.conf_internal_vertices, 9);
        assert_eq!(diag_3234.capacity_deficit, 2);
        assert!(!diag_3234.can_fit_in_level_0);
    }
}
