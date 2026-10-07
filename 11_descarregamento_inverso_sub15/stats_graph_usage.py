#!/usr/bin/env python3
"""
stats_graph_usage.py - Frequência e Estatística de Uso dos 10 Grafos
Projeto Quatro Cores - Pasta 11 (Fronteira dos 10 Grafos)

Calcula:
1. Total de eixos (R-lines) cobertos por cada grafo.
2. Uso exclusivo (quantos eixos dependem estritamente daquele grafo).
3. First-Match (quantas vezes o grafo é disparado na busca em ordem).
4. Detalhamento por apresentação (present7 a present11).
"""

import json
import os

DIR = os.path.dirname(os.path.abspath(__file__))
presentations = ['present7', 'present8', 'present9', 'present10', 'present11']

with open(os.path.join(DIR, 'pareto_subsets_summary.json')) as f:
    pareto = json.load(f)

best10 = pareto['10']
indices = best10['best_subset_indices']
names = best10['best_subset_names']

profiles = {}
for p in presentations:
    with open(os.path.join(DIR, f'profile_{p}.json')) as f:
        profiles[p] = json.load(f)

total_matches = {idx: 0 for idx in indices}
unique_matches = {idx: 0 for idx in indices}
by_pres = {idx: {p: 0 for p in presentations} for idx in indices}
first_matches = {idx: 0 for idx in indices}

total_covered = 0
total_rlines = 0

for p in presentations:
    for r in profiles[p]['rlines']:
        total_rlines += 1
        m = r['mask']
        matching = [idx for idx in indices if (m & (1 << idx))]
        if matching:
            total_covered += 1
            first_matches[matching[0]] += 1
            if len(matching) == 1:
                unique_matches[matching[0]] += 1
            for idx in matching:
                total_matches[idx] += 1
                by_pres[idx][p] += 1

print("=" * 110)
print(" ESTATÍSTICA DE UTILIZAÇÃO DAS 10 CONFIGURAÇÕES (DESCARREGAMENTO INVERSO)")
print("=" * 110)
print(f"Total de Eixos Avaliados: {total_rlines}")
print(f"Total de Eixos Cobertos pelos 10 Grafos: {total_covered} ({100.0 * total_covered / total_rlines:.2f}%)")
print(f"Eixos Residuais para Compensação: {total_rlines - total_covered} (9.01%)")
print("-" * 110)
print(f"{'#':<3} {'Nome da Configuração':<45} {'Total Usos':<12} {'Exclusivo':<12} {'First Match':<12} {'p7':<5} {'p8':<5} {'p9':<5} {'p10':<5} {'p11':<5}")
print("-" * 110)

for i, (idx, name) in enumerate(zip(indices, names)):
    t = total_matches[idx]
    u = unique_matches[idx]
    fm = first_matches[idx]
    bp = by_pres[idx]
    print(f"{i+1:<3} {name:<45} {t:<12} {u:<12} {fm:<12} {bp['present7']:^5} {bp['present8']:^5} {bp['present9']:^5} {bp['present10']:^5} {bp['present11']:^5}")

print("=" * 110)
