use std::collections::{BTreeSet, HashMap, HashSet};
use std::time::Instant;

pub struct SetCoverProblem {
    pub num_axles: usize,
    pub num_confs: usize,
    pub conf_ids: Vec<usize>,
    pub axle_to_confs: Vec<Vec<usize>>, // 0-indexed axles, values are indices into conf_ids
    pub conf_to_axles: Vec<Vec<usize>>, // 0-indexed confs, values are 0-indexed axles
}

impl SetCoverProblem {
    pub fn from_bipartite_file(path: &str) -> std::io::Result<Self> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut raw_axles: Vec<Vec<usize>> = Vec::new();
        let mut all_confs = BTreeSet::new();

        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();
            if !trimmed.starts_with("AXLE") {
                continue;
            }
            if let Some((_header, rest)) = trimmed.split_once(':') {
                let confs: Vec<usize> = rest
                    .split_whitespace()
                    .filter_map(|t| t.parse::<usize>().ok())
                    .collect();
                if !confs.is_empty() {
                    for &c in &confs {
                        all_confs.insert(c);
                    }
                    raw_axles.push(confs);
                }
            }
        }

        let conf_ids: Vec<usize> = all_confs.into_iter().collect();
        let conf_map: HashMap<usize, usize> = conf_ids
            .iter()
            .enumerate()
            .map(|(idx, &c)| (c, idx))
            .collect();

        let num_axles = raw_axles.len();
        let num_confs = conf_ids.len();

        let mut axle_to_confs = vec![Vec::new(); num_axles];
        let mut conf_to_axles = vec![Vec::new(); num_confs];

        for (a_idx, c_list) in raw_axles.into_iter().enumerate() {
            for c in c_list {
                let c_idx = conf_map[&c];
                axle_to_confs[a_idx].push(c_idx);
                conf_to_axles[c_idx].push(a_idx);
            }
        }

        Ok(Self {
            num_axles,
            num_confs,
            conf_ids,
            axle_to_confs,
            conf_to_axles,
        })
    }

    pub fn greedy_first_match_confs(&self) -> Vec<usize> {
        let mut chosen = HashSet::new();
        for c_list in &self.axle_to_confs {
            if let Some(&first) = c_list.first() {
                chosen.insert(self.conf_ids[first]);
            }
        }
        let mut res: Vec<usize> = chosen.into_iter().collect();
        res.sort();
        res
    }

    pub fn solve_minimum_set_cover(&self) -> (Vec<usize>, Vec<usize>) {
        let t0 = Instant::now();
        println!("Resolvendo Cobertura Mínima de Conjuntos em Rust (HiGHS-equivalent B&B)...");
        println!(
            "  -> {} eixos críticos a cobrir, {} configurações candidatas.",
            self.num_axles, self.num_confs
        );

        let mut mandatory_confs = HashSet::new();
        let mut active_axles: HashSet<usize> = (0..self.num_axles).collect();
        let mut active_confs: HashSet<usize> = (0..self.num_confs).collect();

        // 1. Reduções exatas
        loop {
            let mut changed = false;

            // Regra 1: Eixos essenciais (cobertos por apenas 1 configuração)
            let mut essential_confs = HashSet::new();
            for &a in &active_axles {
                let available: Vec<usize> = self.axle_to_confs[a]
                    .iter()
                    .filter(|&&c| active_confs.contains(&c))
                    .copied()
                    .collect();
                if available.len() == 1 {
                    essential_confs.insert(available[0]);
                }
            }

            for c in essential_confs {
                mandatory_confs.insert(c);
                active_confs.remove(&c);
                for &covered_a in &self.conf_to_axles[c] {
                    if active_axles.remove(&covered_a) {
                        changed = true;
                    }
                }
            }

            // Regra 2: Eixos dominados
            // Se as opções para a1 são subconjunto das opções para a2, a2 pode ser removido
            let axle_vec: Vec<usize> = active_axles.iter().copied().collect();
            let precomputed_sets: Vec<HashSet<usize>> = axle_vec
                .iter()
                .map(|&a| {
                    self.axle_to_confs[a]
                        .iter()
                        .filter(|&&c| active_confs.contains(&c))
                        .copied()
                        .collect()
                })
                .collect();

            let mut dominated_axles = HashSet::new();
            for i in 0..axle_vec.len() {
                let a1 = axle_vec[i];
                let set1 = &precomputed_sets[i];
                for j in (i + 1)..axle_vec.len() {
                    let a2 = axle_vec[j];
                    if dominated_axles.contains(&a2) {
                        continue;
                    }
                    let set2 = &precomputed_sets[j];
                    if set1.is_subset(set2) {
                        dominated_axles.insert(a2);
                        changed = true;
                    } else if set2.is_subset(set1) {
                        dominated_axles.insert(a1);
                        changed = true;
                        break;
                    }
                }
            }
            for a in dominated_axles {
                active_axles.remove(&a);
            }

            if !changed {
                break;
            }
        }

        println!(
            "  -> Pré-processamento reduziu para: {} eixos ativos, {} configurações candidatas ativas.",
            active_axles.len(),
            active_confs.len()
        );
        println!(
            "  -> Configurações essenciais obrigatórias já fixadas: {}",
            mandatory_confs.len()
        );

        // 2. Branch and Bound para o subproblema residual
        let mut best_solution = self.greedy_heuristic(&active_axles, &active_confs);
        let mut current_solution = Vec::new();

        self.branch_and_bound(
            &active_axles,
            &active_confs,
            &mut current_solution,
            &mut best_solution,
        );

        let mut final_confs = mandatory_confs;
        for c in best_solution {
            final_confs.insert(c);
        }

        let mut result_ids: Vec<usize> = final_confs
            .into_iter()
            .map(|idx| self.conf_ids[idx])
            .collect();
        result_ids.sort();

        let elapsed = t0.elapsed();
        println!(
            "  -> Otimização concluída em {:.2?} com mínimo global provado de {} configurações!",
            elapsed,
            result_ids.len()
        );

        (result_ids, self.greedy_first_match_confs())
    }

    fn greedy_heuristic(
        &self,
        active_axles: &HashSet<usize>,
        active_confs: &HashSet<usize>,
    ) -> Vec<usize> {
        let mut uncovered = active_axles.clone();
        let mut chosen = Vec::new();

        while !uncovered.is_empty() {
            let mut best_c = None;
            let mut best_count = 0;

            for &c in active_confs {
                let count = self.conf_to_axles[c]
                    .iter()
                    .filter(|&&a| uncovered.contains(&a))
                    .count();
                if count > best_count {
                    best_count = count;
                    best_c = Some(c);
                }
            }

            if let Some(c) = best_c {
                chosen.push(c);
                for &a in &self.conf_to_axles[c] {
                    uncovered.remove(&a);
                }
            } else {
                break;
            }
        }

        chosen
    }

    fn branch_and_bound(
        &self,
        uncovered: &HashSet<usize>,
        available_confs: &HashSet<usize>,
        current: &mut Vec<usize>,
        best: &mut Vec<usize>,
    ) {
        if uncovered.is_empty() {
            if current.len() < best.len() {
                *best = current.clone();
            }
            return;
        }

        // Lower bound check: current.len() + 1 >= best.len()
        if current.len() + 1 >= best.len() {
            return;
        }

        // Pick most constrained axle (the one covered by fewest available confs)
        let mut target_axle = None;
        let mut min_options = usize::MAX;

        for &a in uncovered {
            let options = self.axle_to_confs[a]
                .iter()
                .filter(|&&c| available_confs.contains(&c))
                .count();
            if options < min_options {
                min_options = options;
                target_axle = Some(a);
                if options <= 1 {
                    break;
                }
            }
        }

        let target_axle = match target_axle {
            Some(a) => a,
            None => return,
        };

        let candidate_confs: Vec<usize> = self.axle_to_confs[target_axle]
            .iter()
            .filter(|&&c| available_confs.contains(&c))
            .copied()
            .collect();

        for c in candidate_confs {
            current.push(c);

            let mut next_uncovered = uncovered.clone();
            for &covered_a in &self.conf_to_axles[c] {
                next_uncovered.remove(&covered_a);
            }

            let mut next_available = available_confs.clone();
            next_available.remove(&c);

            self.branch_and_bound(&next_uncovered, &next_available, current, best);

            current.pop();
        }
    }
}
