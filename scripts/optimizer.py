"""
Optimizer for the Four Color Theorem Unavoidable Set
Formulates the Unavoidable Set Selection as a 0-1 Integer Linear Program (Minimum Set Cover)
and solves it using HiGHS via scipy.optimize.milp.
"""

import sys
import numpy as np
from scipy.optimize import milp, LinearConstraint

def optimize_coverage(coverage_file_path):
    print(f"Carregando grafo bipartido de cobertura de: {coverage_file_path}")
    axles = []
    all_confs = set()

    with open(coverage_file_path, "r") as f:
        for line in f:
            line = line.strip()
            if not line or not line.startswith("AXLE"):
                continue
            parts = line.split(":")
            if len(parts) == 2:
                conf_ids = [int(x) for x in parts[1].split() if x.isdigit()]
                if conf_ids:
                    axles.append(conf_ids)
                    all_confs.update(conf_ids)

    num_axles = len(axles)
    conf_list = sorted(list(all_confs))
    conf_to_idx = {c: i for i, c in enumerate(conf_list)}
    num_confs = len(conf_list)

    print(f"-> Total de Axles criticos a cobrir: {num_axles}")
    print(f"-> Total de configuracoes candidatas disponiveis: {num_confs}")

    # Heurística gulosa histórica (primeira correspondência encontrada)
    greedy_chosen = set(c_ids[0] for c_ids in axles)
    print(f"-> Configuracoes selecionadas pelo algoritmo guloso tradicional: {len(greedy_chosen)}")

    # Construção da Matriz de Restrições: A @ x >= 1
    A = np.zeros((num_axles, num_confs))
    for i, c_ids in enumerate(axles):
        for c in c_ids:
            A[i, conf_to_idx[c]] = 1.0

    # Função Objetivo: Minimizar a soma de x_i (cada x_i binário em {0, 1})
    c = np.ones(num_confs)
    constraints = LinearConstraint(A, 1, np.inf)
    integrality = np.ones(num_confs) # 1 = inteiro binário

    print("Executando solver de Programacao Inteira Mista (HiGHS MILP)...")
    res = milp(c=c, constraints=constraints, integrality=integrality)

    if res.success:
        opt_x = np.round(res.x).astype(int)
        opt_confs = set(conf_list[i] for i in range(num_confs) if opt_x[i] == 1)
        eliminated = greedy_chosen - opt_confs
        print("\n" + "="*80)
        print("          RESULTADO DA OTIMIZACAO POR PROGRAMACAO LINEAR (HiGHS)          ")
        print("="*80)
        print(f"Numero MINIMO global de configuracoes necessarias: {len(opt_confs)}")
        print(f"Reducao obtida em relacao ao algoritmo guloso: {len(greedy_chosen) - len(opt_confs)} configuracao(oes) eliminada(s)!")
        if eliminated:
            print(f"Configuracoes provadas redundantes: {sorted(list(eliminated))}")
        print("="*80 + "\n")
        return opt_confs
    else:
        print(f"Falha na otimizacao MILP. Status: {res.status}")
        return None

if __name__ == "__main__":
    path = sys.argv[1] if len(sys.argv) > 1 else "/home/ivanlrk/.gemini/antigravity-cli/brain/be9d8bee-e277-4595-936c-a66abb98f5d7/scratch/anc/coverage_bipartite.txt"
    optimize_coverage(path)
