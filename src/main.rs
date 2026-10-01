mod flipper;
mod fusion;
mod graph;
mod reducibility;
mod set_cover;

use graph::{find_angles, read_configurations, Configuration};
use reducibility::ReducibilityEngine;
use std::env;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

fn format_configuration(conf: &Configuration) -> String {
    let verts = conf.verts();
    let ring = conf.ring();
    let extent = conf.extent_claim();
    let max_cons = conf.max_cons_subset();
    let mut out = String::new();
    out.push_str(&format!("{}\n", conf.name));
    out.push_str(&format!("{} {} {} {}\n", verts, ring, extent, max_cons));
    if conf.contract_edges.is_empty() {
        out.push_str(" 0 \n");
    } else {
        out.push_str(&format!(" {}", conf.contract_edges.len()));
        for (u, v) in &conf.contract_edges {
            out.push_str(&format!(" {} {}", u, v));
        }
        out.push('\n');
    }
    for v in 1..=verts {
        let deg = conf.mat[v][0];
        out.push_str(&format!("  {:2} {:2}\t", v, deg));
        for i in 1..=deg {
            out.push_str(&format!(" {:2}", conf.mat[v][i]));
        }
        out.push('\n');
    }
    let mut coord_count = 0;
    while coord_count < verts {
        let chunk = (verts - coord_count).min(8);
        for _ in 0..chunk {
            out.push_str(" 1000");
        }
        out.push('\n');
        coord_count += chunk;
    }
    out.push('\n');
    out
}

fn canonical_edge_signature(conf: &Configuration) -> (usize, usize, Vec<(usize, usize)>) {
    let mut edges = Vec::new();
    for u in 1..=conf.verts() {
        for h in 1..=conf.mat[u][0] {
            let v = conf.mat[u][h];
            if u < v {
                edges.push((u, v));
            }
        }
    }
    edges.sort_unstable();
    (conf.ring(), conf.verts(), edges)
}

fn make_birkhoff_diamond() -> Configuration {
    let mut conf = Configuration::new(1, 10, 6, 16);
    conf.set_vertex(1, &[2, 9, 6]);
    conf.set_vertex(2, &[3, 8, 9, 1]);
    conf.set_vertex(3, &[4, 7, 8, 2]);
    conf.set_vertex(4, &[5, 7, 3]);
    conf.set_vertex(5, &[6, 10, 7, 4]);
    conf.set_vertex(6, &[1, 9, 10, 5]);
    conf.set_vertex(7, &[10, 8, 3, 4, 5]);
    conf.set_vertex(8, &[7, 10, 9, 2, 3]);
    conf.set_vertex(9, &[2, 8, 10, 6, 1]);
    conf.set_vertex(10, &[9, 8, 7, 5, 6]);
    conf
}

fn make_single_deg5() -> Configuration {
    // 1 vertex in the interior (vertex 6) connected to all 5 ring vertices (1, 2, 3, 4, 5)
    let mut conf = Configuration::new(0, 6, 5, 0);
    conf.set_vertex(1, &[2, 6, 5]);
    conf.set_vertex(2, &[3, 6, 1]);
    conf.set_vertex(3, &[4, 6, 2]);
    conf.set_vertex(4, &[5, 6, 3]);
    conf.set_vertex(5, &[1, 6, 4]);
    conf.set_vertex(6, &[1, 2, 3, 4, 5]);
    conf
}

fn explain_reduction(conf: &Configuration, report: &reducibility::ReducibilityReport) {
    println!("================================================================================");
    println!("           RELATÓRIO ALGÉBRICO DE REDUTIBILIDADE (D/C-REDUCIBILITY)             ");
    println!("================================================================================");
    println!("Configuração: {} (ID numérico: {})", conf.name, conf.id);
    let verts = conf.verts();
    let ring = conf.ring();
    println!(
        "Vértices totais: {} (Anel: {} vértices, Interior: {} vértices)",
        verts,
        ring,
        verts - ring
    );
    println!(
        "Graus dos vértices internos: {:?}",
        (ring + 1..=verts)
            .map(|v| conf.mat[v][0])
            .collect::<Vec<_>>()
    );
    if !conf.contract_edges.is_empty() {
        println!("Contrato de arestas proposto: {:?}", conf.contract_edges);
    }
    println!();
    println!("1. ESPAÇO DE COLORAÇÕES DE TAIT DO ANEL ({}-ring):", ring);
    println!(
        "   - Total de códigos de colorações canônicas do anel: {}",
        report.total_colorings
    );
    println!(
        "   - Colorações que SE ESTENDEM para o interior da configuração: {}",
        report.extending_colorings
    );
    println!(
        "   - Colorações em FALHA (que não se estendem diretamente): {}",
        report.initial_failed_colorings
    );
    println!(
        "   - Total de emparelhamentos balanceados com sinal (Kempe matchings): {}",
        report.total_signed_matchings
    );
    println!();
    println!("2. CICLO DE REDUÇÃO POR PONTO FIXO DE KEMPE / BIRKHOFF:");
    println!("   A cada rodada, as colorações em falha que NÃO possuem suporte planar não-cruzado");
    println!("   para TODAS as 3 partições de cores do grupo Klein 4-group são eliminadas.\n");

    println!(
        "   {:<8} | {:<24} | {:<28}",
        "Rodada", "Colorações Restantes", "Emparelhamentos Reais"
    );
    println!("   ---------+--------------------------+-----------------------------");
    println!(
        "   {:<8} | {:<24} | {:<28}",
        "Inicial", report.initial_failed_colorings, report.total_signed_matchings
    );

    for s in &report.steps {
        println!(
            "   {:<8} | {:<24} | {:<28}",
            format!("R{}", s.round),
            s.remaining_colorings,
            s.remaining_signed_matchings
        );
    }

    println!();
    match report.reduction_type {
        reducibility::ReductionType::DReducible => {
            println!("===> RESULTADO: *** D-REDUTÍVEL PROVADO COM SUCESSO! ***");
            println!();
            println!("EXPLICAÇÃO MATEMÁTICA DA REDUÇÃO:");
            println!("* Como o conjunto de colorações em falha residual convergiu para VAZIO (0),");
            println!("  o conjunto maximal consistente do complemento de extensão é vazio.");
            println!("* Isso demonstra formalmente que NENHUM grafo planar exterior pode impor");
            println!(
                "  uma coloração de fronteira que não possa ser convertida por trocas válidas"
            );
            println!("  de cadeias de Kempe em uma coloração compatível com o interior!");
            println!(
                "* Conclusão: Esta configuração NUNCA pode estar presente em um contraexemplo"
            );
            println!("  minimal do Teorema das Quatro Cores.");
        }
        reducibility::ReductionType::CReducible => {
            println!("===> RESULTADO: *** C-REDUTÍVEL PROVADO COM SUCESSO VIA CONTRATO! ***");
            println!();
            println!("EXPLICAÇÃO MATEMÁTICA DA REDUÇÃO:");
            println!(
                "* O ciclo de Kempe estabilizou em um conjunto maximal consistente não-vazio com"
            );
            println!(
                "  {} colorações (portanto, NÃO é D-redutível).",
                report
                    .steps
                    .last()
                    .map(|s| s.remaining_colorings)
                    .unwrap_or(report.initial_failed_colorings)
            );
            println!(
                "* No entanto, o contrato de arestas {:?} produz um subgrafo menor S'.",
                conf.contract_edges
            );
            println!("* O algoritmo verificou que NENHUMA das colorações de Tait do subgrafo contraído S'");
            println!("  possui restrição de anel pertencente ao conjunto consistente residual.");
            println!("* Portanto, qualquer coloração válida de um grafo contendo S' se estende");
            println!(
                "  indiretamente por descontracção para uma coloração válida do grafo original!"
            );
            println!("* Conclusão: Configuração formalmente C-redutível provada.");
        }
        reducibility::ReductionType::NotReducible => {
            println!("===> RESULTADO: *** NÃO É REDUTÍVEL (NEM D NEM C) ***");
            println!();
            println!("EXPLICAÇÃO MATEMÁTICA:");
            println!(
                "* O processo estabilizou em um ponto fixo não-vazio com {} colorações.",
                report
                    .steps
                    .last()
                    .map(|s| s.remaining_colorings)
                    .unwrap_or(report.initial_failed_colorings)
            );
            if !conf.contract_edges.is_empty() {
                println!(
                    "* O contrato de arestas proposto {:?} falhou na verificação.",
                    conf.contract_edges
                );
            } else {
                println!(
                    "* Nenhum contrato de contração foi proposto para tentar C-redutibilidade."
                );
            }
        }
    }
    println!("================================================================================\n");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let engine = ReducibilityEngine::new();

    if args.len() > 1 && args[1] == "test-c-red" {
        println!("Testando Configuração 2.126 de RSST (Exemplo Clássico C-Redutível)...");
        let mut conf = Configuration::new(2126, 12, 8, 81);
        conf.name = "2.126".to_string();
        conf.set_max_cons_subset(154);
        conf.contract_edges = vec![(1, 9), (3, 9), (5, 11), (7, 11)];
        conf.set_vertex(1, &[2, 9, 12, 8]);
        conf.set_vertex(2, &[3, 9, 1]);
        conf.set_vertex(3, &[4, 10, 9, 2]);
        conf.set_vertex(4, &[5, 10, 3]);
        conf.set_vertex(5, &[6, 11, 10, 4]);
        conf.set_vertex(6, &[7, 11, 5]);
        conf.set_vertex(7, &[8, 12, 11, 6]);
        conf.set_vertex(8, &[1, 12, 7]);
        conf.set_vertex(9, &[2, 3, 10, 11, 12, 1]);
        conf.set_vertex(10, &[3, 4, 5, 11, 9]);
        conf.set_vertex(11, &[10, 5, 6, 7, 12, 9]);
        conf.set_vertex(12, &[11, 7, 8, 1, 9]);

        let angles = find_angles(&conf);
        let report = engine.test_configuration(&conf, &angles);
        explain_reduction(&conf, &report);
        return;
    }

    if args.len() > 1 && args[1] == "test-deg5" {
        println!("Testando Vértice de Grau 5 Isolado (Controle Negativo de Kempe)...");
        let conf = make_single_deg5();
        let angles = find_angles(&conf);
        let report = engine.test_configuration(&conf, &angles);
        explain_reduction(&conf, &report);
        return;
    }

    if args.len() > 1 && args[1] == "inspect" {
        let path = if args.len() > 2 {
            &args[2]
        } else {
            "nova_configuracao_8ring.conf"
        };
        match read_configurations(path) {
            Ok(configs) => {
                if let Some(conf) = configs.first() {
                    let angles = find_angles(conf);
                    let report = engine.test_configuration(conf, &angles);
                    explain_reduction(conf, &report);
                } else {
                    eprintln!("Nenhuma configuração encontrada em {}", path);
                }
            }
            Err(e) => {
                eprintln!("Erro ao abrir arquivo {}: {}", path, e);
            }
        }
        return;
    }

    if args.len() > 1 && args[1] == "verify-file" {
        let path = if args.len() > 2 {
            &args[2]
        } else {
            "U_2822.conf"
        };
        let limit = if args.len() > 3 {
            args[3].parse::<usize>().unwrap_or(10)
        } else {
            10
        };
        println!("Lendo até {} configurações de {}...", limit, path);
        let start = Instant::now();
        match read_configurations(path) {
            Ok(configs) => {
                let to_check = limit.min(configs.len());
                let num_threads = std::thread::available_parallelism()
                    .map(|n| n.get())
                    .unwrap_or(4);
                let chunk_size = to_check.div_ceil(num_threads);
                let slice = &configs[..to_check];

                println!(
                    "Carregadas {} configurações. Verificando {} em paralelo ({} threads std)...",
                    configs.len(),
                    to_check,
                    num_threads
                );

                let (d_red_count, c_red_count): (usize, usize) = std::thread::scope(|s| {
                    let mut handles = Vec::new();
                    for chunk in slice.chunks(chunk_size) {
                        handles.push(s.spawn(move || {
                            let eng = ReducibilityEngine::new();
                            let mut local_d = 0;
                            let mut local_c = 0;
                            for conf in chunk {
                                let angles = find_angles(conf);
                                let report = eng.test_configuration(conf, &angles);
                                match report.reduction_type {
                                    reducibility::ReductionType::DReducible => local_d += 1,
                                    reducibility::ReductionType::CReducible => local_c += 1,
                                    reducibility::ReductionType::NotReducible => {}
                                }
                            }
                            (local_d, local_c)
                        }));
                    }
                    let mut total_d = 0;
                    let mut total_c = 0;
                    for h in handles {
                        let (d, c) = h.join().unwrap();
                        total_d += d;
                        total_c += c;
                    }
                    (total_d, total_c)
                });

                let dur = start.elapsed();
                let total_red = d_red_count + c_red_count;
                println!(
                    "Concluído! Total Redutíveis: {}/{} (D: {}, C: {}) em {:.2?} ({:.1} confs/seg)",
                    total_red,
                    to_check,
                    d_red_count,
                    c_red_count,
                    dur,
                    to_check as f64 / dur.as_secs_f64()
                );
            }
            Err(e) => {
                eprintln!("Erro ao abrir arquivo {}: {}", path, e);
            }
        }
        return;
    }

    if args.len() > 1 && args[1] == "fuse-scan" {
        let path = if args.len() > 2 {
            &args[2]
        } else {
            "unavoidable_629.conf"
        };
        println!(
            "Carregando configurações de {} para análise de fusão...",
            path
        );
        match read_configurations(path) {
            Ok(configs) => {
                println!("Carregadas {} configurações.", configs.len());
                let pairs = fusion::find_fusion_candidate_pairs(&configs);
                println!("Encontrados {} pares candidatos com alta similaridade topológica e mesmo anel!", pairs.len());

                let mut same_prefix = 0;
                let mut identical_degs = 0;
                let mut diff1_degs = 0;

                for p in &pairs {
                    if p.shared_prefix.is_some() {
                        same_prefix += 1;
                    }
                    if p.degree_difference == 0 {
                        identical_degs += 1;
                    } else if p.degree_difference == 1 {
                        diff1_degs += 1;
                    }
                }

                println!("  - Pares na mesma família de prefixo: {}", same_prefix);
                println!(
                    "  - Pares com graus internos exatamente idênticos: {}",
                    identical_degs
                );
                println!(
                    "  - Pares diferindo por apenas 1 grau interno: {}",
                    diff1_degs
                );
                println!();
                println!("Top 15 pares de candidatos a fusão:");
                for (i, p) in pairs.iter().take(15).enumerate() {
                    let pref = p.shared_prefix.as_deref().unwrap_or("nenhum");
                    println!(
                        "  #{:2}: Conf {} (idx {}) <-> Conf {} (idx {}) | Anel: {}, V: {}, DiffGraus: {}, Família: {}",
                        i + 1, p.name1, p.idx1, p.name2, p.idx2, p.ring, p.verts, p.degree_difference, pref
                    );
                }
            }
            Err(e) => {
                eprintln!("Erro ao abrir arquivo {}: {}", path, e);
            }
        }
        return;
    }

    if args.len() > 1 && args[1] == "synthesize-c-red" {
        let path = if args.len() > 2 {
            &args[2]
        } else {
            "unavoidable_629.conf"
        };
        let target = if args.len() > 3 { &args[3] } else { "2.126" };
        println!(
            "Buscando configuração '{}' em {} para sintetizar contrato C-redutível...",
            target, path
        );
        match read_configurations(path) {
            Ok(configs) => {
                let target_conf = configs
                    .iter()
                    .find(|c| c.name == *target || c.id.to_string() == *target);
                if let Some(conf) = target_conf {
                    println!(
                        "Configuração encontrada: {} (Anel: {}, Vértices: {})",
                        conf.name,
                        conf.ring(),
                        conf.verts()
                    );
                    println!("Contrato original no arquivo: {:?}", conf.contract_edges);
                    println!("Iniciando síntese exaustiva de contratos de 1 a 4 arestas...");
                    let start = Instant::now();
                    match fusion::synthesize_contract(conf, &engine, 4) {
                        Some(res) => {
                            let dur = start.elapsed();
                            println!("\n===> [SUCESSO!] Contrato sintetizado em {:.2?}!", dur);
                            println!("Arestas contraídas: {:?}", res.edges);
                            println!("Número de arestas: {}", res.num_edges);
                            println!(
                                "Conjunto maximal consistente residual: {} colorações",
                                res.maximal_consistent_subset
                            );
                            println!("Formalmente C-REDUTÍVEL provado por busca exaustiva!");
                        }
                        None => {
                            println!("Nenhum contrato de até 4 arestas foi capaz de reduzir esta configuração.");
                        }
                    }
                } else {
                    eprintln!("Configuração '{}' não encontrada em {}", target, path);
                }
            }
            Err(e) => {
                eprintln!("Erro ao ler {}: {}", path, e);
            }
        }
        return;
    }

    if args.len() > 1 && args[1] == "test-synth-suite" {
        let path = if args.len() > 2 {
            &args[2]
        } else {
            "unavoidable_629.conf"
        };
        let limit = if args.len() > 3 {
            args[3].parse::<usize>().unwrap_or(20)
        } else {
            20
        };
        println!(
            "Testando síntese autônoma de C-redutibilidade nas primeiras {} configurações de {}...",
            limit, path
        );
        match read_configurations(path) {
            Ok(configs) => {
                let mut c_candidates = Vec::new();
                for conf in &configs {
                    if !conf.contract_edges.is_empty() {
                        c_candidates.push(conf);
                        if c_candidates.len() >= limit {
                            break;
                        }
                    }
                }
                println!(
                    "Encontradas {} configurações C-redutíveis históricas para validação.",
                    c_candidates.len()
                );
                let t_start = Instant::now();
                let mut synthesized_count = 0;
                let mut simpler_contract_count = 0;

                for (idx, conf) in c_candidates.iter().enumerate() {
                    let old_len = conf.contract_edges.len();
                    let t0 = Instant::now();
                    match fusion::synthesize_contract(conf, &engine, 4) {
                        Some(res) => {
                            let dur = t0.elapsed();
                            synthesized_count += 1;
                            let is_simpler = res.num_edges < old_len;
                            if is_simpler {
                                simpler_contract_count += 1;
                            }
                            println!(
                                "  [{:2}/{:2}] Conf {}: Sintetizado em {:.2?} | Arestas: {} (Original RSST: {}){}",
                                idx + 1, c_candidates.len(), conf.name, dur, res.num_edges, old_len,
                                if is_simpler { " *** MAIS SIMPLES QUE RSST! ***" } else { "" }
                            );
                        }
                        None => {
                            println!(
                                "  [{:2}/{:2}] Conf {}: Não encontrado até 4 arestas",
                                idx + 1,
                                c_candidates.len(),
                                conf.name
                            );
                        }
                    }
                }

                let total_dur = t_start.elapsed();
                println!("\n================================================================================");
                println!("            RELATÓRIO DE SÍNTESE AUTÔNOMA DE C-REDUTIBILIDADE                   ");
                println!("================================================================================");
                println!("Configurações avaliadas: {}", c_candidates.len());
                println!(
                    "Contratos C-redutíveis sintetizados com sucesso: {}/{}",
                    synthesized_count,
                    c_candidates.len()
                );
                println!(
                    "Contratos que encontraram redutores MAIS SIMPLES que RSST: {}",
                    simpler_contract_count
                );
                println!(
                    "Tempo total: {:.2?} (Média: {:.2?} por configuração)",
                    total_dur,
                    total_dur / (c_candidates.len() as u32)
                );
                println!("================================================================================\n");
            }
            Err(e) => {
                eprintln!("Erro ao ler {}: {}", path, e);
            }
        }
        return;
    }

    if args.len() > 1 && args[1] == "optimize-all-contracts" {
        let path = if args.len() > 2 {
            &args[2]
        } else {
            "unavoidable_629.conf"
        };
        let limit = if args.len() > 3 {
            Some(args[3].parse::<usize>().unwrap_or(50))
        } else {
            None
        };
        println!("Iniciando otimização global de contratos em {}...", path);
        match read_configurations(path) {
            Ok(configs) => {
                let t0 = Instant::now();
                let (opt_configs, stats) = fusion::optimize_all_contracts(&configs, &engine, limit);
                let dur = t0.elapsed();

                println!("\n================================================================================");
                println!("            RELATÓRIO DE OTIMIZAÇÃO GLOBAL DE CONTRATOS (C-REDUCIBILITY)        ");
                println!("================================================================================");
                println!(
                    "Total de configurações C-redutíveis avaliadas: {}",
                    stats.total_c_confs
                );
                println!(
                    "  - Reduzidas para exatamente 1 aresta: {}",
                    stats.reduced_to_1
                );
                println!(
                    "  - Reduzidas para exatamente 2 arestas: {}",
                    stats.reduced_to_2
                );
                println!(
                    "  - Reduzidas para exatamente 3 arestas: {}",
                    stats.reduced_to_3
                );
                println!("  - Inalteradas (já eram mínimas): {}", stats.unchanged);
                println!(
                    "Total de arestas de restrição ELIMINADAS: {}",
                    stats.total_edges_saved
                );
                println!(
                    "Tempo total de processamento: {:.2?} ({:.1} confs/seg)",
                    dur,
                    stats.total_c_confs as f64 / dur.as_secs_f64()
                );
                println!("================================================================================\n");

                let out_path = if args.len() > 4 {
                    &args[4]
                } else {
                    "unavoidable_629_optimized.conf"
                };
                if let Err(e) = fusion::save_optimized_conf(path, out_path, &opt_configs) {
                    eprintln!("Erro ao salvar arquivo otimizado: {}", e);
                } else {
                    println!(
                        "===> Arquivo com contratos mínimos gerado e salvo com sucesso em '{}'!",
                        out_path
                    );
                }
            }
            Err(e) => {
                eprintln!("Erro ao ler {}: {}", path, e);
            }
        }
        return;
    }

    if args.len() > 1 && args[1] == "scan-flips" {
        let path = if args.len() > 2 {
            &args[2]
        } else {
            "unavoidable_629.conf"
        };
        println!(
            "Mapeando bifurcações por diagonal flip de quadrilátero em {}...",
            path
        );
        match read_configurations(path) {
            Ok(configs) => {
                let flips = fusion::find_exact_flip_pairs(&configs);
                println!(
                    "\nEncontrados {} pares de configurações com exatamente 1 DIAGONAL FLIP!",
                    flips.len()
                );
                println!("Esses pares são irmãos diretos surgidos da bifurcação de faces quadrangulares no descarregamento.\n");
                for (i, fp) in flips.iter().enumerate() {
                    println!(
                        "  #{:2}: Conf {} (idx {}) <-> Conf {} (idx {}) | Anel: {}, V: {} | Diagonal 1: {:?} vs Diagonal 2: {:?}",
                        i + 1, fp.name1, fp.idx1, fp.name2, fp.idx2, fp.ring, fp.verts, fp.edge1, fp.edge2
                    );
                }
            }
            Err(e) => {
                eprintln!("Erro ao ler {}: {}", path, e);
            }
        }
        return;
    }

    if args.len() > 1 && args[1] == "synth-all" {
        let path = if args.len() > 2 {
            &args[2]
        } else {
            "test_small_pruned.conf"
        };
        let max_edges = if args.len() > 3 {
            args[3].parse::<usize>().unwrap_or(2)
        } else {
            2
        };
        let out_path = if args.len() > 4 { Some(&args[4]) } else { None };
        println!(
            "Testando síntese de C-redutibilidade em {} (até {} arestas)...",
            path, max_edges
        );
        match read_configurations(path) {
            Ok(configs) => {
                let num_threads = std::thread::available_parallelism()
                    .map(|n| n.get())
                    .unwrap_or(4);
                let chunk_size = configs.len().div_ceil(num_threads);
                println!(
                    "Carregadas {} configurações. Processando em paralelo ({} threads std)...",
                    configs.len(),
                    num_threads
                );
                let t_start = Instant::now();

                let certified: Vec<Configuration> = std::thread::scope(|s| {
                    let mut handles = Vec::new();
                    for chunk in configs.chunks(chunk_size) {
                        handles.push(s.spawn(move || {
                            let eng = ReducibilityEngine::new();
                            let mut local_res = Vec::new();
                            for conf in chunk {
                                let mut clean_conf = conf.clone();
                                clean_conf.contract_edges.clear();
                                let angles = find_angles(&clean_conf);
                                let (live, nlive) =
                                    eng.compute_consistent_live(&clean_conf, &angles);
                                if nlive == 0 {
                                    local_res.push(clean_conf);
                                    continue;
                                }
                                let eff_max_edges = if clean_conf.ring() >= 13 {
                                    max_edges.min(2)
                                } else {
                                    max_edges
                                };
                                if let Some(res) = fusion::synthesize_contract_with_live(
                                    &clean_conf,
                                    &eng,
                                    &angles,
                                    &live,
                                    nlive,
                                    eff_max_edges,
                                ) {
                                    clean_conf.contract_edges = res.edges;
                                    local_res.push(clean_conf);
                                }
                            }
                            local_res
                        }));
                    }
                    let mut all = Vec::new();
                    for h in handles {
                        all.extend(h.join().unwrap());
                    }
                    all
                });

                let dur = t_start.elapsed();
                let d_count = certified
                    .iter()
                    .filter(|c| c.contract_edges.is_empty())
                    .count();
                let c_count = certified.len() - d_count;
                println!(
                    "\nTotal Redutíveis Certificados: {}/{} (D: {}, C: {}) em {:.2?} ({:.1} confs/seg)\n",
                    certified.len(),
                    configs.len(),
                    d_count,
                    c_count,
                    dur,
                    configs.len() as f64 / dur.as_secs_f64().max(0.001)
                );

                if let Some(out_p) = out_path {
                    let mut file = File::create(out_p).expect("Erro ao criar arquivo de saída");
                    for c in &certified {
                        let text = format_configuration(c);
                        file.write_all(text.as_bytes()).expect("Erro ao escrever");
                    }
                    println!(
                        "Salvas {} configurações certificadas em {}",
                        certified.len(),
                        out_p
                    );
                }
            }
            Err(e) => eprintln!("Erro: {}", e),
        }
        return;
    }

    if args.len() > 1 && args[1] == "fuse-pairs" {
        let path = if args.len() > 2 {
            &args[2]
        } else {
            "unavoidable_629_optimized.conf"
        };
        let limit = if args.len() > 3 {
            args[3].parse::<usize>().unwrap_or(20)
        } else {
            20
        };
        println!(
            "Iniciando Rodada de Fusão Sistemática de Casos em {} (avaliando até {} pares)...",
            path, limit
        );
        match read_configurations(path) {
            Ok(configs) => {
                let flips = fusion::find_exact_flip_pairs(&configs);
                println!(
                    "Mapeados {} pares com bifurcação de quadrilátero planar.\n",
                    flips.len()
                );

                let to_eval = limit.min(flips.len());
                let t_start = Instant::now();

                println!("================================================================================");
                println!("           RELATÓRIO DA RODADA DE FUSÃO DE CASOS (TOPOLOGICAL FUSION)           ");
                println!("================================================================================");

                let mut fusible_count = 0;
                let mut contract_equiv_count = 0;

                for (idx, fp) in flips.iter().take(to_eval).enumerate() {
                    let c1 = &configs[fp.idx1];
                    let c2 = &configs[fp.idx2];

                    let res1 = fusion::synthesize_contract(c1, &engine, 4);
                    let res2 = fusion::synthesize_contract(c2, &engine, 4);

                    let c1_is_d = c1.contract_edges.is_empty();
                    let c2_is_d = c2.contract_edges.is_empty();

                    let contracts1 = res1
                        .as_ref()
                        .map(|r| r.edges.len())
                        .unwrap_or(c1.contract_edges.len());
                    let contracts2 = res2
                        .as_ref()
                        .map(|r| r.edges.len())
                        .unwrap_or(c2.contract_edges.len());

                    let status =
                        if (c1_is_d && c2_is_d) || (contracts1 == contracts2 && contracts1 > 0) {
                            contract_equiv_count += 1;
                            "DUAL SIMÉTRICO (Bifurcação Pura de Face 4)"
                        } else {
                            fusible_count += 1;
                            "ASSIMÉTRICO (Candidato a Absorção por Contrato)"
                        };

                    println!(
                        "  [{:2}/{:2}] Par: Conf {:<14} (idx {:3}) <-> Conf {:<14} (idx {:3})",
                        idx + 1,
                        to_eval,
                        fp.name1,
                        fp.idx1,
                        fp.name2,
                        fp.idx2
                    );
                    println!(
                        "         Anel: {}, Vértices: {} | Quadrilátero: {:?} <-> {:?}",
                        fp.ring, fp.verts, fp.edge1, fp.edge2
                    );
                    println!(
                        "         Classificação: {} | Redutores: K1={} arestas, K2={} arestas",
                        status, contracts1, contracts2
                    );
                    println!();
                }

                let total_dur = t_start.elapsed();
                println!("--------------------------------------------------------------------------------");
                println!("Pares avaliados na rodada: {}", to_eval);
                println!(
                    "Pares com bifurcação de face quadrangular simétrica: {}",
                    contract_equiv_count
                );
                println!(
                    "Pares com assimetria de redutores (candidatos a absorção): {}",
                    fusible_count
                );
                println!("Tempo total da rodada de fusão: {:.2?}", total_dur);
                println!("================================================================================\n");
            }
            Err(e) => {
                eprintln!("Erro ao ler {}: {}", path, e);
            }
        }
        return;
    }

    if args.len() > 1 && args[1] == "optimize-cover" {
        let path = if args.len() > 2 {
            &args[2]
        } else {
            "/home/ivanlrk/.gemini/antigravity-cli/brain/be9d8bee-e277-4595-936c-a66abb98f5d7/scratch/anc/coverage_bipartite.txt"
        };
        match set_cover::SetCoverProblem::from_bipartite_file(path) {
            Ok(problem) => {
                let (opt_confs, greedy_confs) = problem.solve_minimum_set_cover();
                let opt_set: std::collections::HashSet<usize> = opt_confs.iter().copied().collect();
                let greedy_set: std::collections::HashSet<usize> =
                    greedy_confs.iter().copied().collect();
                let mut eliminated: Vec<usize> = greedy_set.difference(&opt_set).copied().collect();
                eliminated.sort();
                let mut added_in_exchange: Vec<usize> =
                    opt_set.difference(&greedy_set).copied().collect();
                added_in_exchange.sort();

                println!("================================================================================");
                println!("             RESULTADO DA OTIMIZAÇÃO GLOBAL SET COVER (RUST)                    ");
                println!("================================================================================");
                println!("Total de eixos cobertos: {}", problem.num_axles);
                println!(
                    "Configurações escolhidas pelo algoritmo guloso histórico: {}",
                    greedy_confs.len()
                );
                println!("Configurações no ÓTIMO GLOBAL provado: {}", opt_confs.len());
                println!(
                    "Redução líquida obtida: {} configuração(ões) eliminada(s)!",
                    greedy_confs.len().saturating_sub(opt_confs.len())
                );
                println!("Configurações eliminadas (redundantes): {:?}", eliminated);
                if !added_in_exchange.is_empty() {
                    println!(
                        "Configurações substitutas adicionadas: {:?}",
                        added_in_exchange
                    );
                }
                println!("Configurações mínimas necessárias: {:?}", opt_confs);
                println!("================================================================================\n");
            }
            Err(e) => {
                eprintln!("Erro ao ler arquivo de cobertura {}: {}", path, e);
            }
        }
        return;
    }

    if args.len() > 1 && args[1] == "explore-flips" {
        let path = if args.len() > 2 {
            &args[2]
        } else {
            "/home/ivanlrk/.gemini/antigravity-cli/brain/be9d8bee-e277-4595-936c-a66abb98f5d7/scratch/anc/conf4.conf"
        };
        println!("Carregando configuração base de {}...", path);
        if let Ok(configs) = read_configurations(path) {
            if let Some(base_conf) = configs.first() {
                println!("Explorando mutações topológicas (diagonal flips) preservando triangulação planar...");
                let flips = flipper::generate_internal_flips(base_conf);
                println!("Encontradas {} mutações candidatas válidas!", flips.len());
                for (idx, flip) in flips.iter().enumerate() {
                    let angles = find_angles(&flip.conf);
                    let report = engine.test_configuration(&flip.conf, &angles);
                    if report.is_d_reducible {
                        println!(
                            "\n  ===> [NOVA REDUÇÃO ENCONTRADA!] Mutação #{}: aresta ({}, {}) -> ({}, {}) | D-REDUTÍVEL! (Estendeu {} colorações)\n",
                            idx + 1, flip.flipped_edge.0, flip.flipped_edge.1, flip.new_edge.0, flip.new_edge.1, report.extending_colorings
                        );
                        explain_reduction(&flip.conf, &report);
                    } else {
                        println!(
                            "  Mutação #{}: aresta ({}, {}) -> ({}, {}) | Não D-redutível (parou em {} colorações)",
                            idx + 1, flip.flipped_edge.0, flip.flipped_edge.1, flip.new_edge.0, flip.new_edge.1,
                            report.steps.last().map(|s| s.remaining_colorings).unwrap_or(report.initial_failed_colorings)
                        );
                    }
                }
            }
        }
        return;
    }

    if args.len() > 1 && args[1] == "generate-flips" {
        let in_path = if args.len() > 2 {
            &args[2]
        } else {
            "unavoidable.conf"
        };
        let out_path = if args.len() > 3 {
            &args[3]
        } else {
            "candidates_flips.conf"
        };
        let filter_path = if args.len() > 4 { Some(&args[4]) } else { None };

        let mut seen = std::collections::HashSet::new();

        if let Some(fp) = filter_path {
            println!(
                "Carregando configurações existentes de {} para filtragem de duplicatas...",
                fp
            );
            if let Ok(pool_confs) = read_configurations(fp) {
                for c in &pool_confs {
                    seen.insert(canonical_edge_signature(c));
                }
                println!("Carregadas {} assinaturas do pool existente.", seen.len());
            }
        }

        println!(
            "Carregando configurações base de {} para mutações planares...",
            in_path
        );
        match read_configurations(in_path) {
            Ok(configs) => {
                let mut out_file = File::create(out_path).expect("Erro ao criar arquivo de saída");
                let mut generated_count = 0;
                let mut unique_count = 0;

                for conf in &configs {
                    seen.insert(canonical_edge_signature(conf));

                    let flips = flipper::generate_internal_flips(conf);
                    generated_count += flips.len();

                    for f in flips {
                        let sig = canonical_edge_signature(&f.conf);
                        if seen.insert(sig) {
                            let mut flipped_conf = f.conf;
                            flipped_conf.name = format!(
                                "{}_f{}_{}_{}_{}",
                                conf.name,
                                f.flipped_edge.0,
                                f.flipped_edge.1,
                                f.new_edge.0,
                                f.new_edge.1
                            );
                            flipped_conf.contract_edges.clear();
                            let text = format_configuration(&flipped_conf);
                            out_file.write_all(text.as_bytes()).unwrap();
                            unique_count += 1;
                        }
                    }
                }

                println!("Concluído! Total de mutações geradas: {}", generated_count);
                println!(
                    "Mutações inéditas e únicas adicionadas: {} salvas em {}",
                    unique_count, out_path
                );
            }
            Err(e) => eprintln!("Erro ao ler {}: {}", in_path, e),
        }
        return;
    }

    if args.len() > 1 && args[1] == "generate-2flips" {
        let in_path = if args.len() > 2 {
            &args[2]
        } else {
            "unavoidable.conf"
        };
        let out_path = if args.len() > 3 {
            &args[3]
        } else {
            "candidates_2flips.conf"
        };
        let filter_path = if args.len() > 4 { Some(&args[4]) } else { None };

        let mut seen = std::collections::HashSet::new();

        if let Some(fp) = filter_path {
            println!(
                "Carregando configurações existentes de {} para filtragem de duplicatas...",
                fp
            );
            if let Ok(pool_confs) = read_configurations(fp) {
                for c in &pool_confs {
                    seen.insert(canonical_edge_signature(c));
                }
                println!("Carregadas {} assinaturas do pool existente.", seen.len());
            }
        }

        println!(
            "Carregando configurações base de {} para mutações de 2-flips (d=2)...",
            in_path
        );
        match read_configurations(in_path) {
            Ok(configs) => {
                let mut out_file = File::create(out_path).expect("Erro ao criar arquivo de saída");
                let mut generated_count = 0;
                let mut unique_count = 0;

                for conf in &configs {
                    seen.insert(canonical_edge_signature(conf));

                    let flips1 = flipper::generate_internal_flips(conf);
                    for f1 in &flips1 {
                        seen.insert(canonical_edge_signature(&f1.conf));
                        let flips2 = flipper::generate_internal_flips(&f1.conf);
                        generated_count += flips2.len();

                        for f2 in flips2 {
                            let sig = canonical_edge_signature(&f2.conf);
                            if seen.insert(sig) {
                                let mut flipped_conf = f2.conf;
                                flipped_conf.name = format!(
                                    "{}_2f_{}_{}_{}_{}",
                                    conf.name,
                                    f1.flipped_edge.0,
                                    f1.flipped_edge.1,
                                    f2.flipped_edge.0,
                                    f2.flipped_edge.1
                                );
                                flipped_conf.contract_edges.clear();
                                let text = format_configuration(&flipped_conf);
                                out_file.write_all(text.as_bytes()).unwrap();
                                unique_count += 1;
                            }
                        }
                    }
                }

                println!(
                    "Concluído! Total de mutações de 2-flips geradas: {}",
                    generated_count
                );
                println!(
                    "Mutações inéditas e únicas adicionadas: {} salvas em {}",
                    unique_count, out_path
                );
            }
            Err(e) => eprintln!("Erro ao ler {}: {}", in_path, e),
        }
        return;
    }

    if args.len() > 1 && args[1] == "mine-parallel" {
        let path = if args.len() > 2 {
            &args[2]
        } else {
            "/home/ivanlrk/.gemini/antigravity-cli/brain/be9d8bee-e277-4595-936c-a66abb98f5d7/scratch/anc/U_2822.conf"
        };
        let count = if args.len() > 3 {
            args[3].parse::<usize>().unwrap_or(20)
        } else {
            20
        };
        println!("Carregando {} configurações base de {}...", count, path);
        if let Ok(configs) = read_configurations(path) {
            let base_slice = &configs[..count.min(configs.len())];
            println!("Gerando todas as mutações planares possíveis das configurações base...");
            let mut all_flips = Vec::new();
            for conf in base_slice {
                let flips = flipper::generate_internal_flips(conf);
                all_flips.extend(flips);
            }

            let num_threads = std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4);
            let total_mutations = all_flips.len();
            let chunk_size = (total_mutations + num_threads - 1).max(1) / num_threads.max(1);

            println!(
                "Total de {} mutações geradas. Testando D-redutibilidade em PARALELO ({} threads)...",
                total_mutations, num_threads
            );

            let t0 = Instant::now();
            let mut new_reducible = Vec::new();

            std::thread::scope(|s| {
                let mut handles = Vec::new();
                for chunk in all_flips.chunks(chunk_size.max(1)) {
                    handles.push(s.spawn(move || {
                        let eng = ReducibilityEngine::new();
                        let mut local_found = Vec::new();
                        for flip in chunk {
                            let angles = find_angles(&flip.conf);
                            let report = eng.test_configuration(&flip.conf, &angles);
                            if report.is_d_reducible {
                                local_found.push((
                                    flip.flipped_edge,
                                    flip.new_edge,
                                    flip.conf.ring(),
                                    report.extending_colorings,
                                ));
                            }
                        }
                        local_found
                    }));
                }
                for h in handles {
                    new_reducible.extend(h.join().unwrap());
                }
            });

            let elapsed = t0.elapsed();
            println!("\n================================================================================");
            println!(
                "            RELATÓRIO DE MINERAÇÃO PARALELA (MULTI-THREAD)                      "
            );
            println!(
                "================================================================================"
            );
            println!("Mutações testadas: {}", total_mutations);
            println!(
                "Tempo total: {:.2?} ({:.1} testes/seg)",
                elapsed,
                total_mutations as f64 / elapsed.as_secs_f64()
            );
            println!(
                "Novas configurações D-redutíveis encontradas: {}",
                new_reducible.len()
            );
            for (idx, (f_edge, n_edge, ring, ext)) in new_reducible.iter().enumerate() {
                println!(
                    "  [NOVA #{:2}] Anel {}: flip ({}, {}) -> ({}, {}) | Extensões: {}",
                    idx + 1,
                    ring,
                    f_edge.0,
                    f_edge.1,
                    n_edge.0,
                    n_edge.1,
                    ext
                );
            }
            println!("================================================================================\n");
        }
        return;
    }

    // Default: Run Birkhoff Diamond and explain
    println!("Executando análise algébrica do Diamante de Birkhoff (1913)...");
    let conf = make_birkhoff_diamond();
    let angles = find_angles(&conf);
    let t0 = Instant::now();
    let report = engine.test_configuration(&conf, &angles);
    let elapsed = t0.elapsed();

    explain_reduction(&conf, &report);
    println!("Tempo de cálculo no processador: {:.3?}", elapsed);
}
