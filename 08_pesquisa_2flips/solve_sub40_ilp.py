#!/usr/bin/env python3
"""
Otimizador Exato por Programação Linear Inteira (HiGHS ILP) para a Fronteira Sub-40.
Resolve o problema de Cobertura Mínima de Conjuntos (Minimum Set Cover) sobre a cobertura RSST.
"""

import sys
import os
import gzip
import numpy as np
from scipy.optimize import milp, LinearConstraint
from scipy.sparse import csr_matrix

def main():
    print("="*80)
    print(" RESOLUÇÃO EXATA DO SET COVER POR PROGRAMAÇÃO LINEAR INTEIRA (HiGHS) - SUB-40")
    print("="*80)

    pool_file = sys.argv[1] if len(sys.argv) > 1 else "unavoidable_pool_2flips.conf"
    cov_file = sys.argv[2] if len(sys.argv) > 2 else "rsst_coverage.txt"
    if not os.path.exists(cov_file) and os.path.exists(cov_file + ".gz"):
        cov_file = cov_file + ".gz"

    with open(pool_file) as f:
        pool_blocks = [b.strip() for b in f.read().split("\n\n") if b.strip()]
    num_cols = len(pool_blocks)

    unique_rows = set()
    total_events = 0
    open_fn = gzip.open if cov_file.endswith(".gz") else open
    mode = "rt" if cov_file.endswith(".gz") else "r"
    with open_fn(cov_file, mode) as f:
        for line in f:
            tokens = line.split()
            if not tokens:
                continue
            try:
                row = tuple(sorted(set(int(x) for x in tokens)))
                if row:
                    unique_rows.add(row)
                    total_events += 1
            except ValueError:
                continue

    rows = list(unique_rows)
    num_rows = len(rows)

    print(f"Total de eventos de redução processados: {total_events}")
    print(f"Linhas únicas de restrição a cobrir: {num_rows}")
    print(f"Colunas (superconfigurações candidatas disponíveis no pool): {num_cols}")

    indptr = [0]
    indices = []
    data = []
    for r in rows:
        for c in r:
            indices.append(c - 1)
            data.append(1.0)
        indptr.append(len(indices))

    A = csr_matrix((data, indices, indptr), shape=(num_rows, num_cols))
    c = np.ones(num_cols)
    constraints = LinearConstraint(A, 1.0, np.inf)
    integrality = np.ones(num_cols)

    print("\nExecutando solver de otimização combinatória exata (HiGHS Branch-and-Bound)...")
    res = milp(c=c, constraints=constraints, integrality=integrality)

    if res.success:
        selected = [i for i in range(num_cols) if np.round(res.x[i]) == 1]
        print("\n" + "="*80)
        print("          RESULTADO DA OTIMIZAÇÃO ILP GLOBAL (2-FLIPS)                  ")
        print("="*80)
        print(f"TAMANHO MÍNIMO GLOBAL OBTIDO: {len(selected)} CONFIGURAÇÕES")
        print(f"Recorde anterior: 41 configurações")
        if len(selected) < 41:
            print(f"NOVO RECORDE MUNDIAL: {41 - len(selected)} configurações eliminadas!")
        else:
            print("Limite de 41 atingido/mantido.")
        print("="*80)

        selected_blocks = [pool_blocks[i] for i in selected]
        out_name = f"unavoidable_raw_{len(selected)}.conf"
        with open(out_name, "w") as f:
            f.write("\n\n".join(selected_blocks) + "\n\n")
        print(f"Salvo arquivo intermediário '{out_name}' com {len(selected_blocks)} blocos.")
    else:
        print(f"Falha na resolução ILP: status {res.status}")
        sys.exit(1)

if __name__ == "__main__":
    main()
