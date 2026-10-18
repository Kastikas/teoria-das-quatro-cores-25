#!/usr/bin/env python3
"""
scripts/analyze_superpool_coverage.py

Analisa a cobertura das 33 novas configurações topológicas sintetizadas (CIDs 630..662)
contra as 162.699 folhas das apresentações present7..present11 do descarregamento RSST.
"""

import sys
from pathlib import Path
from collections import defaultdict
import numpy as np
from scipy.optimize import milp, LinearConstraint
from scipy.sparse import csr_matrix

ROOT_DIR = Path(__file__).resolve().parent.parent
POOL_CONF = ROOT_DIR / "data" / "pool" / "pool_629_plus_33.conf"
SUPERPOOL_33_CONF = ROOT_DIR / "data" / "pool" / "synthesized_superpool_33.conf"
CANONICAL_CONF = ROOT_DIR / "01_modelo_629_rsst_canonico" / "unavoidable_629.conf"

def get_default_log_files():
    pool_dir = ROOT_DIR / "data" / "pool"
    logs = {}
    for d in [7, 8, 9, 10, 11]:
        p_cand = pool_dir / f"matrix_deg{d}.log"
        if not p_cand.exists():
            p_cand = pool_dir / f"matrix3451_deg{d}.log"
        if not p_cand.exists():
            p_cand = Path(f"/tmp/matrix_deg{d}.log")
        logs[d] = p_cand
    return logs

LOG_FILES = get_default_log_files()

def parse_conf_metadata(conf_path: Path):
    configs = []
    with open(conf_path, "r", encoding="utf-8") as f:
        lines = f.readlines()
    
    idx = 0
    while idx < len(lines):
        while idx < len(lines) and not lines[idx].strip():
            idx += 1
        if idx >= len(lines):
            break
        name = lines[idx].strip()
        idx += 1
        while idx < len(lines) and not lines[idx].strip():
            idx += 1
        if idx >= len(lines):
            break
        parts = lines[idx].split()
        verts = int(parts[0])
        ring = int(parts[1])
        extent = int(parts[2]) if len(parts) > 2 else 0
        max_cons = int(parts[3]) if len(parts) > 3 else 0
        idx += 1
        # Contract line
        while idx < len(lines) and not lines[idx].strip():
            idx += 1
        c_parts = lines[idx].split()
        num_contracts = int(c_parts[0]) if c_parts else 0
        idx += 1
        # Adjacency
        degrees = {}
        for _ in range(verts):
            while idx < len(lines) and not lines[idx].strip():
                idx += 1
            adj_parts = lines[idx].split()
            v = int(adj_parts[0])
            d = int(adj_parts[1])
            degrees[v] = d
            idx += 1
        # Coordinates
        coord_count = 0
        while coord_count < verts and idx < len(lines):
            while idx < len(lines) and not lines[idx].strip():
                idx += 1
            if idx >= len(lines):
                break
            coords_in_line = len(lines[idx].split())
            coord_count += coords_in_line
            idx += 1
        
        internal_degs = [degrees[v] for v in range(ring + 1, verts + 1)]
        hubs_8 = sum(1 for d in internal_degs if d == 8)
        hubs_7 = sum(1 for d in internal_degs if d == 7)
        hubs_ge7 = sum(1 for d in internal_degs if d >= 7)

        configs.append({
            "name": name,
            "verts": verts,
            "ring": ring,
            "extent": extent,
            "max_cons": max_cons,
            "num_contracts": num_contracts,
            "internal_degs": sorted(internal_degs),
            "hubs_8": hubs_8,
            "hubs_7": hubs_7,
            "hubs_ge7": hubs_ge7,
        })
    return configs

def main():
    print("=" * 80)
    print("   ANÁLISE DE COBERTURA: SUPERPOOL 33 vs. 162.699 FOLHAS DO DESCARREGAMENTO")
    print("=" * 80)

    # 1. Metadados
    all_confs = parse_conf_metadata(POOL_CONF)
    print(f"Total de configurações no pool combinado: {len(all_confs)}")
    assert len(all_confs) == 662, f"Esperado 662, obtido {len(all_confs)}"

    # 2. Leitura dos logs de cobertura
    # leaf_id -> (deg, leaf_index_in_deg, set of cids)
    leaves = []
    conf_covers = defaultdict(set) # cid -> set of global leaf ids
    deg_leaf_count = {}

    global_leaf_id = 0
    for deg in [7, 8, 9, 10, 11]:
        log_file = LOG_FILES[deg]
        deg_leaves = 0
        with open(log_file, "r", encoding="utf-8") as f:
            for line in f:
                if not line.startswith("LEAF_MATRIX"):
                    continue
                parts = line.strip().split(":")
                lineno = int(parts[0].split()[1])
                cids = set(int(x) for x in parts[1].split()) if len(parts) > 1 and parts[1].strip() else set()
                
                leaves.append((deg, lineno, cids))
                for cid in cids:
                    conf_covers[cid].add(global_leaf_id)
                global_leaf_id += 1
                deg_leaves += 1
        deg_leaf_count[deg] = deg_leaves
        print(f"  Grau {deg:2d}: {deg_leaves:6d} folhas processadas.")
    
    total_leaves = global_leaf_id
    print(f"\nTotal global de folhas: {total_leaves:d}")

    # 3. Análise individual das 33 novas configurações (CIDs 630..662)
    print("\n" + "=" * 80)
    print("  DESEMPENHO INDIVIDUAL DAS 33 NOVAS CONFIGURAÇÕES (CIDs 630..662)")
    print("=" * 80)
    print(f"{'CID':<5} {'Nome':<45} {'V':<3} {'R':<3} {'Hubs':<5} {'Deg 7':<7} {'Deg 8':<7} {'Deg 9':<7} {'Deg 10':<7} {'Deg 11':<7} {'TOTAL':<7}")
    print("-" * 105)

    active_33 = []
    for cid in range(630, 663):
        idx = cid - 1
        meta = all_confs[idx]
        covered_leaves = conf_covers[cid]
        deg_breakdown = defaultdict(int)
        for lid in covered_leaves:
            d = leaves[lid][0]
            deg_breakdown[d] += 1
        
        tot = len(covered_leaves)
        if tot > 0:
            active_33.append((cid, tot, deg_breakdown, meta))
        
        hub_str = f"{meta['hubs_8']}h8+{meta['hubs_7']}h7"
        name_short = meta['name'][:43]
        print(f"{cid:<5} {name_short:<45} {meta['verts']:<3} {meta['ring']:<3} {hub_str:<5} "
              f"{deg_breakdown[7]:<7} {deg_breakdown[8]:<7} {deg_breakdown[9]:<7} {deg_breakdown[10]:<7} {deg_breakdown[11]:<7} {tot:<7}")

    print("-" * 105)
    print(f"Total de configurações ativas das 33 (cobertura > 0): {len(active_33)} / 33 ({len(active_33)/33*100:.1f}%)")

    # Ordenar por cobertura decrescente
    active_33.sort(key=lambda x: x[1], reverse=True)
    print("\n>>> Top 10 Novas Configurações com Maior Cobertura Global:")
    for rank, (cid, tot, deg_b, meta) in enumerate(active_33[:10], 1):
        print(f"  {rank:2d}. CID {cid} ({meta['name']}): {tot} folhas (V={meta['verts']}, R={meta['ring']}, Hubs={meta['hubs_8']}h8+{meta['hubs_7']}h7)")

    # 4. Avaliação dos 4 Quase-Pares Canônicos
    print("\n" + "=" * 80)
    print("  AVALIAÇÃO DOS QUASE-PARES CANÔNICOS HISTÓRICOS")
    print("=" * 80)
    
    pairs = [
        ("Par 1", 326, 558),
        ("Par 2", 227, 235),
        ("Par 3", 163, 273),
        ("Par 4", 484, 499),
    ]

    for pname, c1, c2 in pairs:
        s1 = conf_covers[c1]
        s2 = conf_covers[c2]
        union_pair = s1 | s2
        inter_pair = s1 & s2
        
        # Singletons do par em relação aos 629
        # Folhas cobertas por c1 que nenhum outro de 1..629 cobre
        sing1 = set()
        for lid in s1:
            other_canonical = [c for c in leaves[lid][2] if c <= 629 and c != c1]
            if not other_canonical:
                sing1.add(lid)
        
        sing2 = set()
        for lid in s2:
            other_canonical = [c for c in leaves[lid][2] if c <= 629 and c != c2]
            if not other_canonical:
                sing2.add(lid)
        
        print(f"\n--- {pname}: CIDs ({c1}, {c2}) ---")
        print(f"  Cobertura CID {c1}: {len(s1)} folhas (Singletons canônicos: {len(sing1)})")
        print(f"  Cobertura CID {c2}: {len(s2)} folhas (Singletons canônicos: {len(sing2)})")
        print(f"  União do Par:      {len(union_pair)} folhas (Interseção: {len(inter_pair)})")
        
        # Verificar cobertura pelas 33 novas configurações
        print(f"  Impacto das 33 novas configurações sobre {pname}:")
        best_cov = 0
        best_cand = None
        for cid in range(630, 663):
            cov_union = len(conf_covers[cid] & union_pair)
            cov_s2 = len(conf_covers[cid] & s2)
            cov_sing2 = len(conf_covers[cid] & sing2)
            if cov_union > best_cov:
                best_cov = cov_union
                best_cand = (cid, cov_union, cov_s2, cov_sing2)
            if cov_union > 0:
                print(f"    CID {cid} cobre {cov_union}/{len(union_pair)} do par ({cov_s2}/{len(s2)} de {c2}, {cov_sing2}/{len(sing2)} singletons de {c2})")
        if best_cov == 0:
            print(f"    Nenhuma das 33 novas configurações cobre folhas deste par.")

    # 5. Análise de Substituição de Configurações Canônicas (Subsumption)
    print("\n" + "=" * 80)
    print("  ANÁLISE DE SUBSUNÇÃO DIRETA: CANÔNICA (1..629) COBERTA POR NOVA (630..662)")
    print("=" * 80)
    
    subsumed = []
    for c_can in range(1, 630):
        s_can = conf_covers[c_can]
        if not s_can:
            continue
        for c_new in range(630, 663):
            s_new = conf_covers[c_new]
            if s_can.issubset(s_new):
                subsumed.append((c_can, c_new, len(s_can), len(s_new)))

    if subsumed:
        print(f"ENCONTRADAS {len(subsumed)} SUBSUNÇÕES DIRETAS!")
        for c_can, c_new, n_can, n_new in subsumed:
            print(f"  -> CID Canônico {c_can} ({n_can} folhas) é 100% SUBSUMIDO pelo novo CID {c_new} ({n_new} folhas)!")
    else:
        print("Nenhuma configuração canônica 1..629 é 100% subsumida individualmente por uma única configuração nova no Nível 0.")

    # 6. Otimização Exata via MILP (HiGHS) sobre os 662 grafos
    print("\n" + "=" * 80)
    print("  RESOLUÇÃO EXATA DE SET COVER VIA MILP (HiGHS) SOBRE O POOL DE 662 CONFIGS")
    print("=" * 80)

    # Identificar restrições únicas a cobrir (para reduzir tamanho da matriz)
    unique_leaf_constraints = set()
    for deg, lineno, cids in leaves:
        if cids:
            frozen = tuple(sorted(cids))
            unique_leaf_constraints.add(frozen)

    print(f"Total de folhas originais: {total_leaves}")
    print(f"Restrições únicas comprimidas (linhas do MILP): {len(unique_leaf_constraints)}")

    # Matriz CSR: linhas = restrições únicas, colunas = 662 configs (índice 0..661)
    rows = []
    cols = []
    vals = []
    for r_idx, cids in enumerate(unique_leaf_constraints):
        for cid in cids:
            rows.append(r_idx)
            cols.append(cid - 1) # 0-indexed
            vals.append(1.0)
    
    A_sparse = csr_matrix((vals, (rows, cols)), shape=(len(unique_leaf_constraints), 662))
    
    # 6.1. Solver irrestrito (achar menor subconjunto)
    c_cost = np.ones(662)
    b_l = np.ones(len(unique_leaf_constraints))
    b_u = np.full(len(unique_leaf_constraints), np.inf)
    constraints = LinearConstraint(A_sparse, b_l, b_u)
    integrality = np.ones(662)

    print("\nExecutando solver HiGHS MILP sem restrição forçada...")
    res = milp(c=c_cost, integrality=integrality, constraints=constraints)
    if res.success:
        k_min = int(round(res.fun))
        selected_cids = [i + 1 for i, val in enumerate(res.x) if val > 0.5]
        new_selected = [cid for cid in selected_cids if cid >= 630]
        print(f"  Resultado do MILP Global: K_min = {k_min} configurações.")
        print(f"  Novas configurações selecionadas no ótimo global: {len(new_selected)} ({new_selected})")
    else:
        print(f"  MILP falhou: {res.status}")

    # 6.2. Testar se alguma das 33 novas configurações pode substituir uma canônica com K = 629
    print("\nTestando potencial de substituição (K = 629 com forçamento de x_new = 1)...")
    replacements_found = []
    for cid_new in range(630, 663):
        if len(conf_covers[cid_new]) == 0:
            continue
        col_new = cid_new - 1
        # Forçar x[col_new] == 1
        lb = np.zeros(662)
        ub = np.ones(662)
        lb[col_new] = 1.0
        
        # Limitar número de canônicos ou custo total
        res_forced = milp(c=c_cost, integrality=integrality, constraints=constraints, bounds=(lb, ub))
        if res_forced.success:
            cost = int(round(res_forced.fun))
            if cost == 629:
                sol_cids = set(i + 1 for i, val in enumerate(res_forced.x) if val > 0.5)
                missing_canonicals = set(range(1, 630)) - sol_cids
                included_news = sol_cids - set(range(1, 630))
                replacements_found.append((cid_new, cost, missing_canonicals, included_news))
                print(f"  -> CID {cid_new} pode entrar com K=629! Substitui canonical: {missing_canonicals}")
    
    if not replacements_found:
        print("  Nenhuma nova configuração isolada substitui 1 canônica mantendo K=629 sem Leaf Deepening.")
        print("  (Consistente com o teorema estrutural: o Nível 0 é inflexível e exige bipartição de folhas).")

if __name__ == "__main__":
    main()
