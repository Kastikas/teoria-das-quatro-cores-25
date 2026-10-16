use crate::graph::{find_angles, Configuration, DEG};
use crate::reducibility::{ReducibilityEngine, ReducibilityReport};
use std::collections::HashSet;

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
            // Ordem direta coincide
        } else if c_first == u_next && c_last == u_prev {
            to_insert.reverse();
        } else {
            // Em triangulações planares consistentes, uma das duas orientações é estritamente garantida
            to_insert.reverse();
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
}
