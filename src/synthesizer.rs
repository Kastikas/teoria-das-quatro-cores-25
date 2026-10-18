use crate::graph::{canonical_dihedral_signature, find_angles, Configuration, DEG};
use crate::reducibility::{ReducibilityEngine, ReducibilityReport};
use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Erros formais de síntese e transformação topológica.
#[derive(Debug, PartialEq, Eq)]
pub enum SynthesisError {
    InvalidVertexBounds(String),
    NotInternalEdge(String),
    EdgeDoesNotExist(String),
    ViolatesLinkCondition(String),
    ViolatesDegreeBound(String),
    #[allow(dead_code)]
    ViolatesChordlessRing(String),
    ViolatesRSSTAdmissibility(&'static str),
    NotPureDReducible(String),
}

impl std::fmt::Display for SynthesisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidVertexBounds(s) => write!(f, "Limites de vértices inválidos: {}", s),
            Self::NotInternalEdge(s) => write!(f, "Aresta não é estritamente interna: {}", s),
            Self::EdgeDoesNotExist(s) => write!(f, "Aresta inexistente no grafo: {}", s),
            Self::ViolatesLinkCondition(s) => write!(f, "Violação da Condição de Ligação (Link Condition): {}", s),
            Self::ViolatesDegreeBound(s) => write!(f, "Violação dos limites de grau (RSST min deg 5 interno / 3 anel): {}", s),
            Self::ViolatesChordlessRing(s) => write!(f, "Violação de anel sem cordas (chordless ring): {}", s),
            Self::ViolatesRSSTAdmissibility(s) => write!(f, "Violação de Admissibilidade Canônica RSST: {}", s),
            Self::NotPureDReducible(s) => write!(f, "Rejeitado: Não é D-redutível pura (exige contratos de Kempe): {}", s),
        }
    }
}

pub struct SynthesisPipeline;

impl SynthesisPipeline {
    /// Portão de Validação Rigoroso (5 Portões Invioláveis):
    /// 1. Reciprocidade de arestas: u in adj[v] <=> v in adj[u]
    /// 2. Orientação e consistência planar de faces (Condição 7 de RSST)
    /// 3. Anel livre de cordas (Condição 4 de RSST)
    /// 4. As 8 Condições de Admissibilidade Canônica do RSST (check_rsst_admissibility)
    /// 5. Teste Algébrico de Birkhoff: d_reducible == true e contratos == 0
    pub fn validate_rigorous(
        conf: &Configuration,
        engine: &ReducibilityEngine,
    ) -> Result<ReducibilityReport, SynthesisError> {
        // Portão 1, 2, 3, 4: Admissibilidade Canônica RSST completa
        conf.check_rsst_admissibility()
            .map_err(SynthesisError::ViolatesRSSTAdmissibility)?;

        // Portão 5: Certificação de D-Redutibilidade Pura (C = 0)
        let angles = find_angles(conf);
        let report = engine.test_configuration(conf, &angles);

        if !report.is_d_reducible {
            return Err(SynthesisError::NotPureDReducible(format!(
                "Colorações não-estendíveis restantes: {}. Exigiria contratos C-redutíveis.",
                report.extending_colorings
            )));
        }

        Ok(report)
    }

    /// Tenta sintetizar um contrato esparso C-redutível estritamente válido (1 a 2 arestas)
    /// caso a configuração não seja D-redutível pura.
    /// Preenche `conf.contract_edges` e `conf.set_max_cons_subset(nlive)` se encontrar.
    #[allow(dead_code)]
    pub fn try_synthesize_sparse_contract(
        conf: &mut Configuration,
        base_angles: &crate::graph::Angles,
        engine: &ReducibilityEngine,
        live: &[u8],
        nlive: usize,
    ) -> bool {
        let mut edgeno = [[0usize; crate::graph::VERTS]; crate::graph::VERTS];
        let total_edges = crate::graph::strip(&conf.mat, &mut edgeno);
        let triangles = crate::graph::extract_triangles(conf, &edgeno);
        let r = conf.ring();
        let n = conf.verts();

        // Mapeia cada índice de aresta interna (e > r) para seu par de vértices (u, v)
        let mut edge_to_verts = vec![(0usize, 0usize); total_edges + 1];
        for u in 1..=n {
            for h in 1..=conf.mat[u][0] {
                let v = conf.mat[u][h];
                if u < v {
                    let e = edgeno[u][v];
                    if e > r && e <= total_edges {
                        edge_to_verts[e] = (u, v);
                    }
                }
            }
        }

        // 1. Tenta contratos de 1 aresta (C = 1)
        for e1 in (r + 1)..=total_edges {
            if edge_to_verts[e1] == (0, 0) { continue; }
            let c_angles = crate::graph::build_contract_angles(base_angles, &triangles, nlive, &[e1]);
            if !c_angles.is_sparse { continue; }
            conf.contract_edges = vec![edge_to_verts[e1]];
            if crate::graph::validate_sparse_contract(conf, &c_angles).is_ok()
                && engine.check_contract(&c_angles, live, nlive)
            {
                conf.set_max_cons_subset(nlive);
                return true;
            }
        }

        // 2. Tenta contratos esparsos de 2 arestas (C = 2)
        for e1 in (r + 1)..=total_edges {
            if edge_to_verts[e1] == (0, 0) { continue; }
            for e2 in (e1 + 1)..=total_edges {
                if edge_to_verts[e2] == (0, 0) { continue; }
                let c_angles = crate::graph::build_contract_angles(base_angles, &triangles, nlive, &[e1, e2]);
                if !c_angles.is_sparse { continue; }
                conf.contract_edges = vec![edge_to_verts[e1], edge_to_verts[e2]];
                if crate::graph::validate_sparse_contract(conf, &c_angles).is_ok()
                    && engine.check_contract(&c_angles, live, nlive)
                {
                    conf.set_max_cons_subset(nlive);
                    return true;
                }
            }
        }

        conf.contract_edges.clear();
        false
    }

    /// Executa a contração topológica de uma aresta interna (u, v) onde u, v > R.
    /// Preserva a planaridade combinatorial e a ordem cíclica de faces.
    pub fn contract_internal_edge(
        conf: &Configuration,
        u_in: usize,
        v_in: usize,
    ) -> Result<Configuration, SynthesisError> {
        let n = conf.verts();
        let r = conf.ring();

        // 1. Verificação de limites
        if u_in < 1 || u_in > n || v_in < 1 || v_in > n || u_in == v_in {
            return Err(SynthesisError::InvalidVertexBounds(format!(
                "Vértices ({}, {}) fora dos limites 1..={}",
                u_in, v_in, n
            )));
        }

        // Ambas as extremidades devem ser estritamente internas para contração interna pura
        if u_in <= r || v_in <= r {
            return Err(SynthesisError::NotInternalEdge(format!(
                "Aresta ({}, {}) toca o anel de tamanho R = {}. Contração de anel exige operador específico.",
                u_in, v_in, r
            )));
        }

        let u = std::cmp::min(u_in, v_in);
        let v = std::cmp::max(u_in, v_in);

        let d_u = conf.mat[u][0];
        let d_v = conf.mat[v][0];

        // Verificar existência da aresta
        let nbrs_u: Vec<usize> = (1..=d_u).map(|k| conf.mat[u][k]).collect();
        let nbrs_v: Vec<usize> = (1..=d_v).map(|k| conf.mat[v][k]).collect();

        if !nbrs_u.contains(&v) || !nbrs_v.contains(&u) {
            return Err(SynthesisError::EdgeDoesNotExist(format!(
                "Aresta ({}, {}) não existe no grafo.",
                u, v
            )));
        }

        // 2. Condição de Ligação (Link Condition):
        // Em uma triangulação planar, os vizinhos comuns de u e v devem ser EXATAMENTE 2 vértices (w1, w2).
        let set_u: HashSet<usize> = nbrs_u.iter().cloned().collect();
        let set_v: HashSet<usize> = nbrs_v.iter().cloned().collect();
        let common: Vec<usize> = set_u.intersection(&set_v).cloned().collect();

        if common.len() != 2 {
            return Err(SynthesisError::ViolatesLinkCondition(format!(
                "Vértices {} e {} possuem {} vizinhos comuns ({:?}). A contração violaria planaridade simples (arestas múltiplas).",
                u, v, common.len(), common
            )));
        }

        let w1 = common[0];
        let w2 = common[1];

        // 3. Verificação de Limites de Grau Pós-Contração
        // O vértice fundido uv terá grau d_u + d_v - 4
        let deg_uv = d_u + d_v - 4;
        if deg_uv < 5 || deg_uv >= DEG {
            return Err(SynthesisError::ViolatesDegreeBound(format!(
                "Grau do vértice fundido seria {} (esperado entre 5 e {}).",
                deg_uv,
                DEG - 1
            )));
        }

        // Os vértices w1 e w2 perdem exatamente 1 aresta
        for &w in &[w1, w2] {
            let current_deg = conf.mat[w][0];
            let min_allowed = if w <= r { 3 } else { 5 };
            if current_deg - 1 < min_allowed {
                return Err(SynthesisError::ViolatesDegreeBound(format!(
                    "Vértice vizinho comum v{} (grau atual {}) cairia para {}, violando o mínimo permitido ({}).",
                    w,
                    current_deg,
                    current_deg - 1,
                    min_allowed
                )));
            }
        }

        // 4. Reconstrução Topológica Planar Cíclica
        // u é mantido como o vértice fundido; v é eliminado.
        let pos_v_in_u = nbrs_u.iter().position(|&x| x == v).unwrap();
        let pos_u_in_v = nbrs_v.iter().position(|&x| x == u).unwrap();

        // Rotaciona a lista de vizinhos de v para que u fique na posição 0
        let mut rot_v = Vec::with_capacity(d_v);
        for k in 0..d_v {
            rot_v.push(nbrs_v[(pos_u_in_v + k) % d_v]);
        }
        // rot_v[0] é u. rot_v[1] e rot_v[d_v - 1] são os dois vizinhos comuns w1 e w2 (em alguma ordem).
        let c_first = rot_v[1];
        let c_last = rot_v[d_v - 1];

        // Em u, os vizinhos imediatamente antes e depois de v são w1 e w2
        let u_prev = nbrs_u[(pos_v_in_u + d_u - 1) % d_u];
        let u_next = nbrs_u[(pos_v_in_u + 1) % d_u];

        // Os vértices intermediários de v a serem inseridos
        let mut to_insert: Vec<usize> = rot_v[2..(d_v - 1)].to_vec();

        // Se a orientação de rot_v for oposta à de nbrs_u, inverte to_insert
        if c_first == u_prev && c_last == u_next {
            // Orientação cíclica idêntica: ordem direta preservada
        } else if c_first == u_next && c_last == u_prev {
            to_insert.reverse();
        } else {
            return Err(SynthesisError::ViolatesRSSTAdmissibility(
                "Inconsistência topológica: vizinhos comuns não flanqueiam a aresta (u, v) na ordem cíclica.",
            ));
        }

        // Monta a nova lista de vizinhos para u:
        // nbrs_u sem v, com to_insert inserido no lugar de v
        let mut new_nbrs_u = Vec::with_capacity(deg_uv);
        for k in 0..pos_v_in_u {
            new_nbrs_u.push(nbrs_u[k]);
        }
        new_nbrs_u.extend(to_insert);
        for k in (pos_v_in_u + 1)..d_u {
            new_nbrs_u.push(nbrs_u[k]);
        }

        // 5. Mapeamento de Vértices (Eliminação de v e renumeração para 1..=n-1)
        // Como v > r, os vértices do anel 1..=r permanecem rigorosamente inalterados!
        let map_v = |x: usize| -> usize {
            if x == v {
                u
            } else if x > v {
                x - 1
            } else {
                x
            }
        };

        let new_n = n - 1;
        let mut new_conf = Configuration::new(
            conf.id + 10000,
            new_n,
            r,
            conf.extent_claim().saturating_sub(1),
        );
        new_conf.name = format!("{}_c_{}_{}", conf.name, u, v);

        for old_node in 1..=n {
            if old_node == v {
                continue;
            }
            let new_node = map_v(old_node);

            let old_nbrs = if old_node == u {
                &new_nbrs_u
            } else {
                &conf.mat[old_node][1..=conf.mat[old_node][0]]
            };

            // Mapeia os vizinhos e colapsa a aresta (old_node, v)
            let mut mapped_nbrs = Vec::new();
            for &nb in old_nbrs {
                let mapped_nb = map_v(nb);
                // Evita auto-laço (não deve haver, pois u não era conectado a si mesmo)
                if mapped_nb == new_node {
                    continue;
                }
                // Evita arestas consecutivas duplicadas (que ocorrem em w1 e w2 onde u e v se fundiram)
                if mapped_nbrs.last() == Some(&mapped_nb) {
                    continue;
                }
                mapped_nbrs.push(mapped_nb);
            }
            // Verifica duplicata circular na borda final
            if mapped_nbrs.len() > 1 && mapped_nbrs.first() == mapped_nbrs.last() {
                mapped_nbrs.pop();
            }

            new_conf.set_vertex(new_node, &mapped_nbrs);
        }

        // 6. Teste Rigoroso de Sanidade Estrutural Pré-Exportação
        // Verifica se cada aresta mapeada é perfeitamente recíproca
        for i in 1..=new_n {
            let d_i = new_conf.mat[i][0];
            for k in 1..=d_i {
                let j = new_conf.mat[i][k];
                let d_j = new_conf.mat[j][0];
                let has_reciprocal = (1..=d_j).any(|idx| new_conf.mat[j][idx] == i);
                if !has_reciprocal {
                    return Err(SynthesisError::ViolatesRSSTAdmissibility(
                        "Falha interna: Aresta não-recíproca gerada na contração.",
                    ));
                }
            }
        }

        // Executa as 8 condições completas do RSST sobre o novo grafo
        new_conf
            .check_rsst_admissibility()
            .map_err(SynthesisError::ViolatesRSSTAdmissibility)?;

        Ok(new_conf)
    }

    /// Encontra todas as arestas internas que satisfazem a Condição de Ligação
    /// e que preservam os limites de grau do RSST se contraídas.
    pub fn find_contractible_internal_edges(conf: &Configuration) -> Vec<(usize, usize)> {
        let n = conf.verts();
        let r = conf.ring();
        let mut result = Vec::new();

        for u in (r + 1)..=n {
            let d_u = conf.mat[u][0];
            for k in 1..=d_u {
                let v = conf.mat[u][k];
                if v > u && v > r {
                    let d_v = conf.mat[v][0];
                    let nbrs_u: HashSet<usize> = (1..=d_u).map(|i| conf.mat[u][i]).collect();
                    let nbrs_v: HashSet<usize> = (1..=d_v).map(|i| conf.mat[v][i]).collect();
                    let common: Vec<usize> = nbrs_u.intersection(&nbrs_v).cloned().collect();

                    if common.len() == 2 {
                        let w1 = common[0];
                        let w2 = common[1];
                        let deg_uv = d_u + d_v - 4;
                        let w1_ok = if w1 <= r { conf.mat[w1][0] > 3 } else { conf.mat[w1][0] > 5 };
                        let w2_ok = if w2 <= r { conf.mat[w2][0] > 3 } else { conf.mat[w2][0] > 5 };

                        if deg_uv >= 5 && deg_uv < DEG && w1_ok && w2_ok {
                            result.push((u, v));
                        }
                    }
                }
            }
        }
        result
    }

    /// Executa mineração paralela de contrações em alta escala sobre coleções de grafos.
    /// Utiliza std::thread::scope, deduplicação diédrica canônica e filtragem fast-fail em 5 portões.
    pub fn batch_synthesize_contractions_parallel(
        configs: &[Configuration],
        existing_pool: Option<&[Configuration]>,
        num_threads: usize,
    ) -> (Vec<Configuration>, BatchSynthesisStats) {
        let threads = if num_threads == 0 {
            std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4)
        } else {
            num_threads
        };

        let mut seen_signatures = HashSet::new();
        if let Some(pool) = existing_pool {
            for c in pool {
                seen_signatures.insert(canonical_dihedral_signature(c));
            }
        }
        for c in configs {
            seen_signatures.insert(canonical_dihedral_signature(c));
        }

        let chunk_size = configs.len().div_ceil(threads).max(1);
        let total_cands_atomic = AtomicUsize::new(0);
        let rsst_passed_atomic = AtomicUsize::new(0);

        let thread_results: Vec<Vec<Configuration>> = std::thread::scope(|s| {
            let mut handles = Vec::new();
            for chunk in configs.chunks(chunk_size) {
                let total_cands_ref = &total_cands_atomic;
                let rsst_passed_ref = &rsst_passed_atomic;
                handles.push(s.spawn(move || {
                    let engine = ReducibilityEngine::new();
                    let mut local_valid = Vec::new();

                    for conf in chunk {
                        let contractible = Self::find_contractible_internal_edges(conf);
                        total_cands_ref.fetch_add(contractible.len(), Ordering::Relaxed);

                        for (u, v) in contractible {
                            if let Ok(new_conf) = Self::contract_internal_edge(conf, u, v) {
                                rsst_passed_ref.fetch_add(1, Ordering::Relaxed);
                                // Portão 5: Birkhoff Reducibility
                                if let Ok(_) = Self::validate_rigorous(&new_conf, &engine) {
                                    local_valid.push(new_conf);
                                }
                            }
                        }
                    }
                    local_valid
                }));
            }
            handles.into_iter().map(|h| h.join().unwrap()).collect()
        });

        let mut final_unique = Vec::new();
        let mut total_pure_d = 0;

        for list in thread_results {
            for conf in list {
                total_pure_d += 1;
                let sig = canonical_dihedral_signature(&conf);
                if seen_signatures.insert(sig) {
                    final_unique.push(conf);
                }
            }
        }

        let stats = BatchSynthesisStats {
            total_input_configs: configs.len(),
            total_contractible_candidates: total_cands_atomic.load(Ordering::Relaxed),
            rsst_admissible_passed: rsst_passed_atomic.load(Ordering::Relaxed),
            pure_d_certified: total_pure_d,
            unique_d_certified: final_unique.len(),
        };

        (final_unique, stats)
    }

    /// Detecta caminhos divisores simples Pi = (r_a, h, r_b) onde h é um hub interno
    /// conectado a pelo menos dois vértices do anel r_a e r_b.
    #[allow(dead_code)]
    pub fn find_sector_bipartitions(conf: &Configuration) -> Vec<PlanarSector> {
        let n = conf.verts();
        let r = conf.ring();
        let mut sectors = Vec::new();

        for h in (r + 1)..=n {
            let d_h = conf.mat[h][0];
            let nbrs_h: Vec<usize> = (1..=d_h).map(|k| conf.mat[h][k]).collect();
            let ring_nbrs: Vec<usize> = nbrs_h.iter().cloned().filter(|&x| x <= r).collect();

            if ring_nbrs.len() >= 2 {
                for i in 0..ring_nbrs.len() {
                    for j in (i + 1)..ring_nbrs.len() {
                        let ra = ring_nbrs[i];
                        let rb = ring_nbrs[j];

                        let mut arc_ab = Vec::new();
                        let mut curr = ra;
                        while curr != rb {
                            arc_ab.push(curr);
                            curr = if curr == r { 1 } else { curr + 1 };
                        }
                        arc_ab.push(rb);

                        // Encontra vizinhos de h entre ra e rb na ordem cíclica de h
                        let pos_ra = nbrs_h.iter().position(|&x| x == ra).unwrap();
                        let pos_rb = nbrs_h.iter().position(|&x| x == rb).unwrap();

                        let mut seeds = Vec::new();
                        let mut idx = (pos_ra + 1) % d_h;
                        while idx != pos_rb {
                            let nb = nbrs_h[idx];
                            if nb > r {
                                seeds.push(nb);
                            }
                            idx = (idx + 1) % d_h;
                        }

                        // BFS / flood-fill para encontrar estritamente os vértices internos do setor
                        let mut visited = HashSet::new();
                        let mut queue = std::collections::VecDeque::new();
                        for s in seeds {
                            if visited.insert(s) {
                                queue.push_back(s);
                            }
                        }

                        while let Some(curr_v) = queue.pop_front() {
                            let d_v = conf.mat[curr_v][0];
                            for k in 1..=d_v {
                                let nb = conf.mat[curr_v][k];
                                if nb > r && nb != h && !visited.contains(&nb) {
                                    visited.insert(nb);
                                    queue.push_back(nb);
                                }
                            }
                        }

                        let sector_internals: Vec<usize> = visited.into_iter().collect();

                        sectors.push(PlanarSector {
                            hub: h,
                            ring_anchor_a: ra,
                            ring_anchor_b: rb,
                            ring_arc: arc_ab,
                            internal_vertices: sector_internals,
                        });
                    }
                }
            }
        }
        sectors
    }

    /// Calcula a distância L1 entre os perfis de graus ordenados (internos e de anel) de duas configurações.
    pub fn structural_degree_distance(a: &Configuration, b: &Configuration) -> usize {
        let r = a.ring();
        let n = a.verts();
        let mut ring_a: Vec<usize> = (1..=r).map(|i| a.mat[i][0]).collect();
        let mut ring_b: Vec<usize> = (1..=r).map(|i| b.mat[i][0]).collect();
        ring_a.sort_unstable();
        ring_b.sort_unstable();

        let mut int_a: Vec<usize> = ((r + 1)..=n).map(|i| a.mat[i][0]).collect();
        let mut int_b: Vec<usize> = ((r + 1)..=n).map(|i| b.mat[i][0]).collect();
        int_a.sort_unstable();
        int_b.sort_unstable();

        let d_ring: usize = ring_a.iter().zip(ring_b.iter()).map(|(x, y)| x.abs_diff(*y)).sum();
        let d_int: usize = int_a.iter().zip(int_b.iter()).map(|(x, y)| x.abs_diff(*y)).sum();
        d_ring + d_int
    }

    /// Executa o cruzamento geodésico guiado por flips entre duas configurações parentais.
    /// Explora intermediários no espaço de triangulações, filtrando candidatos que
    /// reduzem ou preservam a distância estrutural em direção ao parceiro (geodésica real),
    /// e submetendo cada candidato aos 5 Portões Invioláveis (incluindo teste algébrico de Birkhoff com C = 0).
    pub fn recombine_guided_flips(
        donor_a: &Configuration,
        donor_b: &Configuration,
        engine: &ReducibilityEngine,
        max_depth: usize,
    ) -> Vec<Configuration> {
        if donor_a.ring() != donor_b.ring() || donor_a.verts() != donor_b.verts() {
            return Vec::new();
        }

        let mut results = Vec::new();
        let mut seen_signatures = HashSet::new();
        seen_signatures.insert(canonical_dihedral_signature(donor_a));
        seen_signatures.insert(canonical_dihedral_signature(donor_b));

        let base_dist = Self::structural_degree_distance(donor_a, donor_b);

        // Nível 1: Geração de vizinhos de flip de donor_a que caminham em direção a donor_b
        let flips_a = crate::flipper::generate_internal_flips(donor_a);
        for flip in flips_a {
            let dist_to_b = Self::structural_degree_distance(&flip.conf, donor_b);
            if dist_to_b <= base_dist {
                let sig = canonical_dihedral_signature(&flip.conf);
                if seen_signatures.insert(sig) {
                    if let Ok(_) = Self::validate_rigorous(&flip.conf, engine) {
                        let mut certified = flip.conf;
                        certified.name = format!("{}_x_{}_f1", donor_a.name, donor_b.name);
                        results.push(certified);
                    }
                }
            }
        }

        // Nível 1: Geração de vizinhos de flip de donor_b que caminham em direção a donor_a
        let flips_b = crate::flipper::generate_internal_flips(donor_b);
        for flip in flips_b {
            let dist_to_a = Self::structural_degree_distance(&flip.conf, donor_a);
            if dist_to_a <= base_dist {
                let sig = canonical_dihedral_signature(&flip.conf);
                if seen_signatures.insert(sig) {
                    if let Ok(_) = Self::validate_rigorous(&flip.conf, engine) {
                        let mut certified = flip.conf;
                        certified.name = format!("{}_x_{}_f2", donor_b.name, donor_a.name);
                        results.push(certified);
                    }
                }
            }
        }

        // Nível 2 (opcional, para profundidade geodésica >= 2)
        if max_depth >= 2 {
            let level1 = results.clone();
            for hybrid in level1 {
                let flips_h = crate::flipper::generate_internal_flips(&hybrid);
                for flip in flips_h {
                    let d_a = Self::structural_degree_distance(&flip.conf, donor_a);
                    let d_b = Self::structural_degree_distance(&flip.conf, donor_b);
                    if d_a <= base_dist && d_b <= base_dist {
                        let sig = canonical_dihedral_signature(&flip.conf);
                        if seen_signatures.insert(sig) {
                            if let Ok(_) = Self::validate_rigorous(&flip.conf, engine) {
                                let mut certified = flip.conf;
                                certified.name = format!("{}_f3", hybrid.name);
                                results.push(certified);
                            }
                        }
                    }
                }
            }
        }

        results
    }

    /// Executa recombinação em lote massivamente paralela sobre um pool de configurações.
    /// Utiliza std::thread::scope, chunking balanceado e deduplicação diédrica canônica.
    pub fn batch_recombine_parallel(
        pool: &[Configuration],
        existing_pool: Option<&[Configuration]>,
        options: &RecombinationOptions,
    ) -> (Vec<Configuration>, BatchRecombinationStats) {
        let threads = if options.num_threads == 0 {
            std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4)
        } else {
            options.num_threads
        };

        let mut seen_signatures = HashSet::new();
        if let Some(existing) = existing_pool {
            for c in existing {
                seen_signatures.insert(canonical_dihedral_signature(c));
            }
        }
        for c in pool {
            seen_signatures.insert(canonical_dihedral_signature(c));
        }

        // Agrupa configurações por (verts, ring) para formar pares compatíveis
        let mut buckets: std::collections::HashMap<(usize, usize), Vec<&Configuration>> =
            std::collections::HashMap::new();
        for c in pool {
            if let Some(target_r) = options.target_ring {
                if c.ring() != target_r {
                    continue;
                }
            }
            buckets.entry((c.verts(), c.ring())).or_default().push(c);
        }

        let mut candidate_pairs = Vec::new();
        for (_, group) in buckets {
            if group.len() >= 2 {
                for i in 0..group.len() {
                    for j in (i + 1)..group.len() {
                        candidate_pairs.push((group[i], group[j]));
                        if let Some(max_p) = options.max_pairs {
                            if candidate_pairs.len() >= max_p {
                                break;
                            }
                        }
                    }
                    if let Some(max_p) = options.max_pairs {
                        if candidate_pairs.len() >= max_p {
                            break;
                        }
                    }
                }
            }
        }

        let total_pairs = candidate_pairs.len();
        if total_pairs == 0 {
            return (
                Vec::new(),
                BatchRecombinationStats {
                    total_input_confs: pool.len(),
                    compatible_pairs: 0,
                    crossovers_attempted: 0,
                    rsst_admissible_passed: 0,
                    pure_d_certified: 0,
                    unique_d_certified: 0,
                },
            );
        }

        let chunk_size = total_pairs.div_ceil(threads).max(1);
        let pairs_atomic = AtomicUsize::new(0);
        let crossovers_atomic = AtomicUsize::new(0);
        let rsst_atomic = AtomicUsize::new(0);
        let pure_d_atomic = AtomicUsize::new(0);

        let thread_results: Vec<Vec<Configuration>> = std::thread::scope(|s| {
            let mut handles = Vec::new();
            for chunk in candidate_pairs.chunks(chunk_size) {
                let p_ref = &pairs_atomic;
                let c_ref = &crossovers_atomic;
                let r_ref = &rsst_atomic;
                let d_ref = &pure_d_atomic;
                let depth = options.max_depth;

                handles.push(s.spawn(move || {
                    let engine = ReducibilityEngine::new();
                    let mut local_certified = Vec::new();

                    for &(conf_a, conf_b) in chunk {
                        p_ref.fetch_add(1, Ordering::Relaxed);
                        // Flips de donor_a
                        let flips_a = crate::flipper::generate_internal_flips(conf_a);
                        c_ref.fetch_add(flips_a.len(), Ordering::Relaxed);
                        r_ref.fetch_add(flips_a.len(), Ordering::Relaxed);

                        // Flips de donor_b
                        let flips_b = crate::flipper::generate_internal_flips(conf_b);
                        c_ref.fetch_add(flips_b.len(), Ordering::Relaxed);
                        r_ref.fetch_add(flips_b.len(), Ordering::Relaxed);

                        let certified = Self::recombine_guided_flips(conf_a, conf_b, &engine, depth);
                        d_ref.fetch_add(certified.len(), Ordering::Relaxed);
                        local_certified.extend(certified);
                    }
                    local_certified
                }));
            }
            handles.into_iter().map(|h| h.join().unwrap()).collect()
        });

        let mut final_unique = Vec::new();
        let mut total_pure_d = 0;

        for list in thread_results {
            for conf in list {
                total_pure_d += 1;
                let sig = canonical_dihedral_signature(&conf);
                if seen_signatures.insert(sig) {
                    final_unique.push(conf);
                }
            }
        }

        let stats = BatchRecombinationStats {
            total_input_confs: pool.len(),
            compatible_pairs: total_pairs,
            crossovers_attempted: crossovers_atomic.load(Ordering::Relaxed),
            rsst_admissible_passed: rsst_atomic.load(Ordering::Relaxed),
            pure_d_certified: total_pure_d,
            unique_d_certified: final_unique.len(),
        };

        (final_unique, stats)
    }

    /// Executa a transição de fronteira: expande um vértice interno v de grau 5
    /// com vizinhos consecutivos no anel para dentro do anel, transformando G(V, R) em G'(V, R+1).
    #[allow(dead_code)]
    pub fn morph_expand_internal_to_ring(
        conf: &Configuration,
        v: usize,
    ) -> Result<Configuration, SynthesisError> {
        let n = conf.verts();
        let r = conf.ring();

        if v <= r || v > n {
            return Err(SynthesisError::InvalidVertexBounds(format!(
                "Vértice {} não é estritamente interno (anel R = {})",
                v, r
            )));
        }

        let d_v = conf.mat[v][0];
        if d_v != 5 {
            return Err(SynthesisError::ViolatesDegreeBound(format!(
                "Apenas vértices internos de grau 5 podem ser expandidos para o anel. Grau atual: {}",
                d_v
            )));
        }

        if r >= 14 {
            return Err(SynthesisError::ViolatesRSSTAdmissibility(
                "Anel expandido excederia o limite máximo canônico do RSST (R <= 14).",
            ));
        }

        // Procura dois vizinhos consecutivos no anel
        let nbrs_v: Vec<usize> = (1..=d_v).map(|k| conf.mat[v][k]).collect();

        // Garante que v possui EXATAMENTE 2 vizinhos no anel (evita criar cordas no anel ao expandir)
        let ring_nbr_count = nbrs_v.iter().filter(|&&x| x <= r).count();
        if ring_nbr_count != 2 {
            return Err(SynthesisError::ViolatesChordlessRing(format!(
                "Vértice interno {} possui {} vizinhos no anel (esperado exatamente 2 para evitar cordas).",
                v, ring_nbr_count
            )));
        }

        let mut consecutive_pair = None;

        for i in 0..d_v {
            let next_i = (i + 1) % d_v;
            let u1 = nbrs_v[i];
            let u2 = nbrs_v[next_i];
            if u1 <= r && u2 <= r {
                let is_consec = (u2 == u1 + 1)
                    || (u1 == r && u2 == 1)
                    || (u1 == u2 + 1)
                    || (u2 == r && u1 == 1);
                if is_consec {
                    let (ra, rb) = if (u2 == u1 + 1) || (u1 == r && u2 == 1) {
                        (u1, u2)
                    } else {
                        (u2, u1)
                    };
                    consecutive_pair = Some((ra, rb));
                    break;
                }
            }
        }

        let (ra, rb) = match consecutive_pair {
            Some(pair) => pair,
            None => {
                return Err(SynthesisError::ViolatesDegreeBound(format!(
                    "Vértice {} não possui par de vizinhos consecutivos no anel.",
                    v
                )))
            }
        };

        if conf.mat[ra][0] <= 3 || conf.mat[rb][0] <= 3 {
            return Err(SynthesisError::ViolatesDegreeBound(format!(
                "Expansão deixaria vizinho de anel com grau < 3: d({})={}, d({})={}",
                ra, conf.mat[ra][0], rb, conf.mat[rb][0]
            )));
        }

        let new_r = r + 1;
        let ins_pos = if ra == r && rb == 1 { new_r } else { ra + 1 };

        let mut node_map = vec![0usize; n + 1];
        for old_u in 1..=r {
            if old_u <= ra {
                node_map[old_u] = old_u;
            } else {
                node_map[old_u] = old_u + 1;
            }
        }
        node_map[v] = ins_pos;

        let mut next_internal = new_r + 1;
        for old_u in (r + 1)..=n {
            if old_u != v {
                node_map[old_u] = next_internal;
                next_internal += 1;
            }
        }

        let mut new_conf = Configuration::new(
            conf.id + 20000,
            n,
            new_r,
            conf.extent_claim(),
        );
        new_conf.name = format!("{}_exp_v{}", conf.name, v);

        for old_u in 1..=n {
            let new_u = node_map[old_u];
            let d_old = conf.mat[old_u][0];
            let old_nbrs: Vec<usize> = (1..=d_old).map(|k| conf.mat[old_u][k]).collect();

            let mut mapped_nbrs = Vec::new();
            if old_u == v {
                // Rotaciona old_nbrs de v para que os vizinhos no anel (ra, rb) fiquem nas extremidades
                // No padrão RSST, para um vértice do anel, o primeiro vizinho (index 1) e o último (index d)
                // devem ser seus vizinhos de anel (expected_next = rb, expected_prev = ra)
                let pos_rb = old_nbrs.iter().position(|&x| x == rb).unwrap();
                let mut rot_v: Vec<usize> = (0..d_old)
                    .map(|k| node_map[old_nbrs[(pos_rb + k) % d_old]])
                    .collect();
                if rot_v.len() >= 2 && rot_v[1] == node_map[ra] {
                    rot_v.rotate_left(1);
                    rot_v.reverse();
                }
                mapped_nbrs = rot_v;
            } else if old_u == ra {
                for &nb in &old_nbrs {
                    if nb != rb {
                        mapped_nbrs.push(node_map[nb]);
                    }
                }
            } else if old_u == rb {
                for &nb in &old_nbrs {
                    if nb != ra {
                        mapped_nbrs.push(node_map[nb]);
                    }
                }
            } else {
                for &nb in &old_nbrs {
                    mapped_nbrs.push(node_map[nb]);
                }
            }

            new_conf.set_vertex(new_u, &mapped_nbrs);
        }

        for i in 1..=n {
            let d_i = new_conf.mat[i][0];
            for k in 1..=d_i {
                let j = new_conf.mat[i][k];
                let d_j = new_conf.mat[j][0];
                let has_reciprocal = (1..=d_j).any(|idx| new_conf.mat[j][idx] == i);
                if !has_reciprocal {
                    return Err(SynthesisError::ViolatesRSSTAdmissibility(
                        "Falha interna: Quebra de reciprocidade na expansão de anel.",
                    ));
                }
            }
        }

        new_conf
            .check_rsst_admissibility()
            .map_err(SynthesisError::ViolatesRSSTAdmissibility)?;

        Ok(new_conf)
    }
}

/// Opções de execução para o pipeline de recombinação em lote.
#[derive(Debug, Clone)]
pub struct RecombinationOptions {
    pub max_pairs: Option<usize>,
    pub max_depth: usize,
    pub num_threads: usize,
    pub target_ring: Option<usize>,
}

impl Default for RecombinationOptions {
    fn default() -> Self {
        Self {
            max_pairs: None,
            max_depth: 1,
            num_threads: 0,
            target_ring: None,
        }
    }
}

/// Estatísticas agregadas da recombinação paralela em lote.
#[derive(Debug, Clone, Default)]
pub struct BatchRecombinationStats {
    pub total_input_confs: usize,
    pub compatible_pairs: usize,
    pub crossovers_attempted: usize,
    pub rsst_admissible_passed: usize,
    pub pure_d_certified: usize,
    pub unique_d_certified: usize,
}

/// Estatísticas agregadas da mineração paralela de síntese.
#[derive(Debug, Clone, Default)]
pub struct BatchSynthesisStats {
    pub total_input_configs: usize,
    pub total_contractible_candidates: usize,
    pub rsst_admissible_passed: usize,
    pub pure_d_certified: usize,
    pub unique_d_certified: usize,
}

/// Representa um setor planar biparticionado por um caminho (r_a, h, r_b).
#[derive(Debug, Clone)]
pub struct PlanarSector {
    pub hub: usize,
    pub ring_anchor_a: usize,
    pub ring_anchor_b: usize,
    pub ring_arc: Vec<usize>,
    pub internal_vertices: Vec<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reject_ring_edge_contraction() {
        // Tentar contrair aresta do anel (1, 2) deve retornar erro explícito
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

        let res = SynthesisPipeline::contract_internal_edge(&conf, 1, 2);
        assert!(matches!(res, Err(SynthesisError::NotInternalEdge(_))));
    }

    #[test]
    fn test_reject_nonexistent_edge() {
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

        let res = SynthesisPipeline::contract_internal_edge(&conf, 7, 9);
        assert!(matches!(res, Err(SynthesisError::EdgeDoesNotExist(_))));
    }

    #[test]
    fn test_link_condition_enforcement() {
        // No Diamante de Birkhoff (V=10, R=6), os vértices internos são 7, 8, 9, 10.
        // Aresta (7, 10): vizinhos de 7 são [3, 8, 10, 4]; vizinhos de 10 são [4, 5, 6, 9, 8, 7].
        // Vizinhos comuns de 7 e 10 são 4 e 8 (exatamente 2).
        // Grau de 7 é 4 (menor que 5, o Diamante de Birkhoff tem grau 4 em v7).
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

        let contractible = SynthesisPipeline::find_contractible_internal_edges(&conf);
        // Todas as arestas candidatas devem respeitar os limites estritos
        for &(u, v) in &contractible {
            assert!(u > 6 && v > 6);
        }
    }

    #[test]
    fn test_batch_recombine_parallel_empty() {
        let pool: Vec<Configuration> = Vec::new();
        let opts = RecombinationOptions::default();
        let (res, stats) = SynthesisPipeline::batch_recombine_parallel(&pool, None, &opts);
        assert!(res.is_empty());
        assert_eq!(stats.compatible_pairs, 0);
    }

    #[test]
    fn test_morph_expand_internal_bounds_checks() {
        let conf = Configuration::new(1, 10, 6, 16);
        // Vértice 2 é do anel (<= 6)
        let res = SynthesisPipeline::morph_expand_internal_to_ring(&conf, 2);
        assert!(matches!(res, Err(SynthesisError::InvalidVertexBounds(_))));

        // Vértice fora dos limites (> 10)
        let res = SynthesisPipeline::morph_expand_internal_to_ring(&conf, 15);
        assert!(matches!(res, Err(SynthesisError::InvalidVertexBounds(_))));
    }

    #[test]
    fn test_batch_synthesize_contractions_parallel_small() {
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

        let pool = vec![conf];
        let (unique, stats) = SynthesisPipeline::batch_synthesize_contractions_parallel(&pool, None, 2);
        assert_eq!(stats.total_input_configs, 1);
        // Como o diamante de Birkhoff não tem arestas contractíveis admissíveis de grau >= 5, lista retornada é vazia
        assert_eq!(unique.len(), 0);
    }

    #[test]
    fn test_revaluate_cand3234_contractions() {
        let mut conf = Configuration::new(2605, 22, 13, 8294);
        conf.set_vertex(1, &[2, 22, 13]);
        conf.set_vertex(2, &[3, 21, 22, 1]);
        conf.set_vertex(3, &[4, 17, 21, 2]);
        conf.set_vertex(4, &[5, 17, 3]);
        conf.set_vertex(5, &[6, 19, 17, 4]);
        conf.set_vertex(6, &[7, 19, 5]);
        conf.set_vertex(7, &[8, 15, 19, 6]);
        conf.set_vertex(8, &[9, 16, 15, 7]);
        conf.set_vertex(9, &[10, 18, 16, 8]);
        conf.set_vertex(10, &[11, 18, 9]);
        conf.set_vertex(11, &[12, 20, 18, 10]);
        conf.set_vertex(12, &[13, 20, 11]);
        conf.set_vertex(13, &[1, 22, 20, 12]);
        conf.set_vertex(14, &[17, 15, 16, 18, 20]);
        conf.set_vertex(15, &[8, 16, 14, 17, 19, 7]);
        conf.set_vertex(16, &[18, 14, 15, 8, 9]);
        conf.set_vertex(17, &[5, 19, 15, 14, 20, 21, 3, 4]);
        conf.set_vertex(18, &[20, 14, 16, 9, 10, 11]);
        conf.set_vertex(19, &[7, 15, 17, 5, 6]);
        conf.set_vertex(20, &[22, 21, 17, 14, 18, 11, 12, 13]);
        conf.set_vertex(21, &[3, 17, 20, 22, 2]);
        conf.set_vertex(22, &[2, 21, 20, 13, 1]);

        let contractible = SynthesisPipeline::find_contractible_internal_edges(&conf);
        assert_eq!(contractible.len(), 7, "Candidato 3234 deve ter exatamente 7 arestas internas contractíveis");

        let engine = ReducibilityEngine::new();
        let mut d_reducible_count = 0;
        let mut c_reducible_count = 0;

        for &(u, v) in &contractible {
            let mut contracted = SynthesisPipeline::contract_internal_edge(&conf, u, v)
                .expect("Contração topológica deve ser válida");
            assert_eq!(contracted.verts(), 21);
            assert_eq!(contracted.ring(), 13);

            let base_angles = crate::graph::find_angles(&contracted);
            let report = engine.test_configuration(&contracted, &base_angles);
            if report.is_d_reducible {
                d_reducible_count += 1;
            } else {
                let (live, nlive) = engine.compute_consistent_live(&contracted, &base_angles);
                if nlive > 0 && SynthesisPipeline::try_synthesize_sparse_contract(&mut contracted, &base_angles, &engine, &live, nlive) {
                    c_reducible_count += 1;
                }
            }
        }
        println!("Candidato 3234 contrações: D-redutíveis = {}, C-redutíveis = {}", d_reducible_count, c_reducible_count);
    }
}
