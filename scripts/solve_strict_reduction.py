#!/usr/bin/env python3
"""
scripts/solve_strict_reduction.py

Executa a Otimização Global Exata por Programação Linear Inteira (HiGHS MILP)
sobre o pool de 891 configurações estritamente certificadas (629 canônicas + 262 flips/podas válidas).
Identifica o menor subconjunto estritamente admissível que cobre 100% dos sub-eixos
nas apresentações canônicas present7 a present11.
"""

import os
import subprocess
import sys
from pathlib import Path
import numpy as np
from scipy.optimize import milp, LinearConstraint
from scipy.sparse import csr_matrix

ROOT_DIR = Path(__file__).resolve().parent.parent
DISCHARGE_MAPPER = ROOT_DIR / "target" / "discharge_mapper"
RULES = ROOT_DIR / "01_modelo_629_rsst_canonico" / "rules"
POOL_CONF = ROOT_DIR / "data" / "combined_891_pool.conf"
PRES_DIR = ROOT_DIR / "01_modelo_629_rsst_canonico"
PRES_FILES = ["present7", "present8", "present9", "present10", "present11"]

def get_pool_names(conf_path: Path):
    with open(conf_path) as f:
        lines = f.readlines()
    names = []
    idx = 0
    while idx < len(lines):
        while idx < len(lines) and not lines[idx].strip():
            idx += 1
        if idx >= len(lines):
            break
        name = lines[idx].strip()
        names.append(name)
        idx += 1
        if idx >= len(lines):
            break
        parts = lines[idx].split()
        if not parts:
            break
        n = int(parts[0])
        idx += 2 + n
        coord_count = 0
        while coord_count < n:
            chunk = min(n - coord_count, 8)
            coord_count += chunk
            idx += 1
    return names

def main():
    print("=" * 80)
    print("  OTIMIZADOR GLOBAL POR PROGRAMAÇÃO LINEAR INTEIRA (HiGHS MILP) — RSST ESTRITO")
    print("=" * 80)

    pool_names = get_pool_names(POOL_CONF)
    name_to_col = {name: i for i, name in enumerate(pool_names)}
    num_cols = len(pool_names)
    print(f"[*] Total de configurações no pool certificado: {num_cols}")

    unique_restrictions = set()
    total_events = 0

    cov_file = ROOT_DIR / "data" / "strict_coverage_891.txt"
    if cov_file.exists() and cov_file.stat().st_size > 1000:
        print(f"[*] Carregando restrições do arquivo existente: {cov_file}")
        with open(cov_file, "r") as f:
            for line in f:
                parts = line.strip().split()
                if not parts:
                    continue
                indices = tuple(sorted(set(int(x) for x in parts)))
                unique_restrictions.add(indices)
                total_events += 1
    else:
        print("[*] Mapeando eventos de redução nas 5 apresentações...")
        cov_lines = []
        for p in PRES_FILES:
            p_path = PRES_DIR / p
            print(f"  -> Executando {p}...")
            cmd = [
                str(DISCHARGE_MAPPER),
                str(p_path),
                str(POOL_CONF),
                str(RULES),
                "0",
                "0",
                "629",
            ]
            proc = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, check=True)
            for line in proc.stdout.splitlines():
                if not line.startswith("MAP_RLINE|"):
                    continue
                parts = line.split("|")
                num_matches = int(parts[5])
                matches = parts[6: 6 + num_matches]
                col_indices = tuple(sorted(set(name_to_col[m] for m in matches if m in name_to_col)))
                if col_indices:
                    unique_restrictions.add(col_indices)
                    total_events += 1
                    cov_lines.append(" ".join(str(c) for c in col_indices) + "\n")

        with open(cov_file, "w") as f:
            f.writelines(cov_lines)
        print(f"[+] Restrições salvas em {cov_file}")

    num_rows = len(unique_restrictions)
    print(f"[+] Total de eventos de redução processados: {total_events}")
    print(f"[+] Restrições únicas a cobrir: {num_rows}")

    # Montar matriz esparsa CSR
    indptr = [0]
    indices = []
    data = []
    for r in unique_restrictions:
        for c in r:
            indices.append(c)
            data.append(1.0)
        indptr.append(len(indices))

    A = csr_matrix((data, indices, indptr), shape=(num_rows, num_cols))

    # Definir custos do MILP:
    # Custo 1.0 para todas as colunas
    c = np.ones(num_cols)
    constraints = LinearConstraint(A, 1.0, np.inf)
    integrality = np.ones(num_cols)

    print("\n[*] Executando solver HiGHS Branch-and-Bound...")
    res = milp(c=c, constraints=constraints, integrality=integrality)

    if res.success:
        selected_indices = [i for i in range(num_cols) if np.round(res.x[i]) == 1]
        print("\n" + "=" * 80)
        print(f"          RESULTADO DA OTIMIZAÇÃO GLOBAL: {len(selected_indices)} CONFIGURAÇÕES")
        print("=" * 80)
        selected_names = [pool_names[i] for i in selected_indices]
        print(f"Tamanho do catálogo original: 629")
        print(f"Tamanho do catálogo ótimo:    {len(selected_indices)}")
        if len(selected_indices) < 629:
            print(f">>> REDUÇÃO LÍQUIDA ALCANÇADA: -{629 - len(selected_indices)} configurações! <<<")
        else:
            print("O catálogo de 629 é uma cobertura mínima estática sobre as apresentações atuais.")

        # Identificar configurações substituídas
        canonical_629 = set(pool_names[:629])
        selected_set = set(selected_names)
        removed_canonical = canonical_629 - selected_set
        added_candidates = selected_set - canonical_629

        print(f"\nConfigurações canônicas removidas ({len(removed_canonical)}):")
        for name in sorted(removed_canonical):
            print(f"  - {name}")

        print(f"\nNovas candidatas incorporadas ({len(added_candidates)}):")
        for name in sorted(added_candidates):
            print(f"  + {name}")

        return selected_names
    else:
        print(f"[!] Falha no solver HiGHS: status {res.status}")
        return None

if __name__ == "__main__":
    main()
