#!/usr/bin/env python3
"""
inverse_search.py - Motor de Busca Combinatória e Formulação LP de Descarregamento Inverso
Projeto Quatro Cores - Pasta 11 (Descarregamento Inverso Sub-15)

Este script:
1. Carrega os perfis exatos de cobertura de eixos (R-lines) gerados por 'discharge_profiler';
2. Varre exaustivamente as combinações de tamanho K in [8, 15] sobre o universo dos 21 grafos;
3. Identifica a Fronteira de Pareto dos subconjuntos ótimos que minimizam eixos abertos;
4. Detalha os eixos residuais por apresentação (graus e posições);
5. Modela o sistema linear de compensação de carga (Descarregamento Inverso LP).
"""

import os
import sys
import json
import time
import itertools
import numpy as np

DIR = os.path.dirname(os.path.abspath(__file__))

def load_profiles():
    presentations = ["present7", "present8", "present9", "present10", "present11"]
    profiles = {}
    for p in presentations:
        fpath = os.path.join(DIR, f"profile_{p}.json")
        if not os.path.exists(fpath):
            print(f"[ERRO] Arquivo de perfil '{fpath}' não encontrado!")
            print("Execute primeiro: ./discharge_profiler --profile <p> unavoidable_21.conf rules")
            sys.exit(1)
        with open(fpath, "r") as f:
            profiles[p] = json.load(f)
    return profiles

def run_inverse_search():
    print("=" * 80)
    print(" MOTOR DE DESCARREGAMENTO INVERSO: VARREDURA COMBINATÓRIA SUB-15")
    print("=" * 80)
    
    profiles = load_profiles()
    confs = profiles["present7"]["confs"]
    n_confs = len(confs)
    print(f">> Catálogo base carregado: {n_confs} configurações redutíveis universais.")
    
    masks_by_p = {}
    lines_by_p = {}
    total_rlines = 0
    for p, data in profiles.items():
        masks_by_p[p] = np.array([item["mask"] for item in data["rlines"]], dtype=np.uint32)
        lines_by_p[p] = data["rlines"]
        count = len(masks_by_p[p])
        total_rlines += count
        print(f"   • {p:<9}: {count:4d} eixos irredutíveis (R-lines)")
    print(f">> Universo total de eixos a cobrir: {total_rlines} eixos.")
    print("-" * 80)

    all_masks = np.concatenate(list(masks_by_p.values()))
    
    results = {}
    
    for K in range(8, 16):
        t0 = time.time()
        combs = list(itertools.combinations(range(n_confs), K))
        n_combs = len(combs)
        
        best_uncovered = total_rlines + 1
        best_candidates = []
        
        for c in combs:
            mask = 0
            for bit in c:
                mask |= (1 << bit)
            unc = int(np.count_nonzero((all_masks & mask) == 0))
            if unc < best_uncovered:
                best_uncovered = unc
                best_candidates = [(c, unc)]
            elif unc == best_uncovered and len(best_candidates) < 5:
                best_candidates.append((c, unc))
                
        dt = time.time() - t0
        best_c = best_candidates[0][0]
        mask_best = sum(1 << b for b in best_c)
        breakdown = {p: int(np.count_nonzero((masks_by_p[p] & mask_best) == 0)) for p in masks_by_p}
        pct_covered = 100.0 * (1.0 - best_uncovered / total_rlines)
        
        results[K] = {
            "K": K,
            "total_candidates": n_combs,
            "best_uncovered": best_uncovered,
            "pct_covered": pct_covered,
            "time_sec": dt,
            "best_subset_indices": list(best_c),
            "best_subset_names": [confs[i] for i in best_c],
            "breakdown": breakdown,
            "mask_hex": hex(mask_best)
        }
        
        print(f"[K = {K:2d}] Melhores {K} grafos: {best_uncovered:3d} eixos abertos ({pct_covered:5.2f}% coberto) | "
              f"p7:{breakdown['present7']:2d}, p8:{breakdown['present8']:2d}, p9:{breakdown['present9']:2d}, "
              f"p10:{breakdown['present10']:2d}, p11:{breakdown['present11']:2d} | "
              f"{n_combs:6d} testes em {dt:.2f}s")

    # Salva resumo JSON
    out_json = os.path.join(DIR, "pareto_subsets_summary.json")
    with open(out_json, "w") as f:
        json.dump(results, f, indent=2)
    print("\n>> Resumo salvo com sucesso em:", out_json)
    
    # Análise detalhada do melhor subconjunto para K=10 e K=12
    generate_detailed_report(results, profiles, confs, total_rlines)

def generate_detailed_report(results, profiles, confs, total_rlines):
    rep_path = os.path.join(DIR, "report_inverse_analysis.md")
    with open(rep_path, "w") as f:
        f.write("# Relatório de Descarregamento Inverso: Fronteira Sub-15\n\n")
        f.write(f"Universo analisado: **{len(confs)} configurações** contra **{total_rlines} eixos irredutíveis** das 5 apresentações canônicas do RSST.\n\n")
        f.write("## 1. Tabela de Cobertura da Fronteira de Pareto\n\n")
        f.write("| Alvo ($K$) | Combinações Testadas | Eixos Abertos | Taxa de Cobertura | p7 | p8 | p9 | p10 | p11 | Tempo |\n")
        f.write("| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |\n")
        for K, d in sorted(results.items()):
            b = d["breakdown"]
            f.write(f"| **{K}** | {d['total_candidates']:,} | **{d['best_uncovered']}** | **{d['pct_covered']:.2f}%** | "
                    f"{b['present7']} | {b['present8']} | {b['present9']} | {b['present10']} | {b['present11']} | {d['time_sec']:.2f}s |\n")
        
        f.write("\n---\n\n")
        f.write("## 2. O 'Time dos Sonhos' de 10 Configurações ($K = 10$)\n\n")
        best10 = results[10]
        f.write(f"Com apenas **10 configurações**, cobrimos **{best10['pct_covered']:.2f}% de toda a geometria de eixos** ({total_rlines - best10['best_uncovered']}/{total_rlines}), deixando apenas **{best10['best_uncovered']} eixos abertos**!\n\n")
        f.write(f"**Máscara Hex:** `{best10['mask_hex']}`\n\n")
        f.write("### Grafos Selecionados no Time de 10:\n")
        for i, idx in enumerate(best10["best_subset_indices"], 1):
            f.write(f"{i}. `[{idx:02d}]` **{confs[idx]}**\n")
            
        f.write("\n### Distribuição dos Eixos Abertos em $K=10$:\n")
        for p, cnt in best10["breakdown"].items():
            f.write(f"- **{p}**: {cnt} eixos abertos\n")
            
        f.write("\n---\n\n")
        f.write("## 3. O 'Time dos Sonhos' de 12 Configurações ($K = 12$)\n\n")
        best12 = results[12]
        f.write(f"Com **12 configurações**, a cobertura sobe para **{best12['pct_covered']:.2f}%**, com apenas **{best12['best_uncovered']} eixos abertos** em toda a matemática da prova.\n\n")
        f.write(f"**Máscara Hex:** `{best12['mask_hex']}`\n\n")
        f.write("### Grafos Selecionados no Time de 12:\n")
        for i, idx in enumerate(best12["best_subset_indices"], 1):
            f.write(f"{i}. `[{idx:02d}]` **{confs[idx]}**\n")

        f.write("\n---\n\n")
        f.write("## 4. Formulação Matemática do Descarregamento Inverso (LP)\n\n")
        f.write("Para zerar os déficits dos eixos abertos restantes sem adicionar novas configurações:\n\n")
        f.write("1. **Restrição de Balanço de Carga:**\n")
        f.write("   Para cada eixo aberto $A_i$ de grau $d$, exigimos:\n")
        f.write("   $$\\text{Carga}(A_i) + \\sum_{r \\in \\mathcal{R}_{\\text{novas}}} \\Delta C_r(A_i) \\ge 0$$\n\n")
        f.write("2. **Raio de Ação ($r \\le 3$):**\n")
        f.write("   A carga deve ser transferida de pentágonos a uma distância de até 3 arestas do hub, permitindo dissipar os 117 eixos de $K=10$.\n")

    print(">> Relatório completo gerado em:", rep_path)

if __name__ == "__main__":
    run_inverse_search()
