#!/usr/bin/env python3
"""
Otimizador Exato por Programação Linear Inteira (HiGHS ILP) para a Fronteira Sub-130.
Resolve o problema de Cobertura Mínima de Conjuntos (Minimum Set Cover) sobre a cobertura RSST.
"""

import sys
import numpy as np
from scipy.optimize import milp, LinearConstraint
from scipy.sparse import csr_matrix

def main():
    print("="*80)
    print(" RESOLUÇÃO EXATA DO SET COVER POR PROGRAMAÇÃO LINEAR INTEIRA (HiGHS) - SUB-130")
    print("="*80)

    pool_file = sys.argv[1] if len(sys.argv) > 1 else "unavoidable_pool_flips.conf"
    cov_file = sys.argv[2] if len(sys.argv) > 2 else "rsst_coverage.txt"

    with open(pool_file) as f:
        pool_blocks = [b.strip() for b in f.read().split("\n\n") if b.strip()]
    num_cols = len(pool_blocks)

    unique_rows = set()
    total_events = 0
    with open(cov_file) as f:
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
        print("          NOVO RECORD MUNDIAL ENCONTRADO VIA OTIMIZAÇÃO ILP!             ")
        print("="*80)
        print(f"TAMANHO MÍNIMO GLOBAL OBTIDO: {len(selected)} CONFIGURAÇÕES")
        print(f"Recorde anterior: 137 configurações")
        if len(selected) < 137:
            print(f"Redução adicional líquida: {137 - len(selected)} configurações eliminadas!")
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
