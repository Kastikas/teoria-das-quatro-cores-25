#!/usr/bin/env python3
"""
scripts/deepen_leaves.py

Implementação dos Dois Modos de Aprofundamento de Folhas (Leaf Deepening) no Modelo RSST:

- Modo A: Absorção 2-por-1 via Candidata Estritamente Válida + Aprofundamento de Folha (Leaf Deepening).
  Substitui o par de configurações canônicas (K_1, K_2) por uma única candidata estritamente válida F,
  aprofundando a folha 'R' onde K_2 era necessária por uma quebra de condição 'C <v> <m>',
  fechando o ramo filho com F e o ramo complementar com o catálogo restante (via R, S ou C de nível 2).

- Modo B: Eliminação de Configurações Especialistas de Linhas 'H' via Refinamento de Hubcap.
  Para configurações chamadas exclusivamente em 1 única linha 'H' (dentro de CheckBound),
  rastreia o outlet disparador e insere quebra 'C <n> <m>' antes da linha 'H',
  desativando o outlet positivo e eliminando a necessidade da configuração especialista.

Ambos os modos preservam 100% a fidelidade matemática do verificador RSST discharge.c.
"""

import argparse
import json
import os
import re
import subprocess
import sys
import tempfile
from collections import defaultdict
from pathlib import Path
from typing import Dict, List, Optional, Set, Tuple

ROOT_DIR = Path(__file__).resolve().parent.parent
MAPPER_BIN = ROOT_DIR / "target" / "discharge_mapper"
DISCHARGE_C = ROOT_DIR / "src" / "discharge.c"
DEFAULT_CONF = ROOT_DIR / "data" / "models" / "unavoidable_exact_629.conf"
DEFAULT_RULES = ROOT_DIR / "data" / "rsst_presentations" / "rules"
DEFAULT_PRES_DIR = ROOT_DIR / "data" / "rsst_presentations"
DEFAULT_MAP_JSON = ROOT_DIR / "data" / "rline_usage_map.json"

sys.path.insert(0, str(ROOT_DIR / "scripts"))
from presentation_editor import (
    parse_line_info,
    simulate_axle_at_line,
    splice_presentation_lines,
    validate_presentation_syntax,
)


def ensure_discharge_binary() -> Path:
    """Garante que o executável canônico de discharge está compilado."""
    bin_path = ROOT_DIR / "bin" / "discharge"
    if bin_path.exists():
        return bin_path
    bin_path = ROOT_DIR / "target" / "discharge_1997"
    bin_path.parent.mkdir(parents=True, exist_ok=True)
    if not bin_path.exists() or DISCHARGE_C.stat().st_mtime > bin_path.stat().st_mtime:
        cmd = ["gcc", "-O3", str(DISCHARGE_C), "-o", str(bin_path)]
        subprocess.run(cmd, check=True)
    return bin_path


def parse_conf_blocks(filepath: Path) -> Dict[str, str]:
    """Lê um arquivo .conf e retorna um dicionário {nome_configuracao: bloco_completo}."""
    with open(filepath, "r") as f:
        lines = f.readlines()

    blocks = {}
    idx = 0
    while idx < len(lines):
        while idx < len(lines) and not lines[idx].strip():
            idx += 1
        if idx >= len(lines):
            break
        name = lines[idx].strip()
        start_idx = idx
        idx += 1  # name line
        if idx >= len(lines):
            break
        parts = lines[idx].split()
        if not parts:
            break
        n = int(parts[0])
        idx += 1  # verts
        idx += 1  # contract
        idx += n  # n adj lines
        coord_count = 0
        while coord_count < n:
            chunk = min(n - coord_count, 8)
            coord_count += chunk
            idx += 1  # coord line
        block_text = "".join(lines[start_idx:idx]).strip() + "\n\n"
        blocks[name] = block_text
    return blocks


def verify_presentations_against_catalog(
    pres_dir: Path,
    conf_path: Path,
    rules_path: Path,
    discharge_bin: Path,
) -> Tuple[bool, Dict[str, str]]:
    """Verifica se todas as 5 apresentações passam 100% no verificador C com o catálogo dado."""
    results = {}
    all_ok = True
    for p in ["present7", "present8", "present9", "present10", "present11"]:
        p_path = pres_dir / p
        cmd = [str(discharge_bin), str(p_path), str(conf_path), str(rules_path), "0", "0"]
        proc = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        if "verified" in proc.stdout:
            results[p] = "OK"
        else:
            results[p] = f"FALHA: {proc.stdout[:150]}"
            all_ok = False
    return all_ok, results


class LeafDeepener:
    def __init__(
        self,
        conf_path: Path = DEFAULT_CONF,
        rules_path: Path = DEFAULT_RULES,
        pres_dir: Path = DEFAULT_PRES_DIR,
        map_json_path: Path = DEFAULT_MAP_JSON,
    ):
        self.conf_path = conf_path
        self.rules_path = rules_path
        self.pres_dir = pres_dir
        self.map_json_path = map_json_path

        self.discharge_bin = ensure_discharge_binary()
        self.canonical_blocks = parse_conf_blocks(conf_path)

        with open(map_json_path, "r") as f:
            self.map_data = json.load(f)

    def find_mode_a_opportunities(self, candidate_conf_path: Path) -> List[Dict]:
        """
        Varre o pool de candidatas em busca de oportunidades Modo A:
        - Candidata F absorve 100% dos eixos de uma canônica K_1;
        - K_2 é uma configuração rara (ex.: single-R ou <= 3 linhas);
        - F compartilha o núcleo com K_2 com vértices em aberto.
        """
        print(f"[*] Analisando oportunidades Modo A usando candidatas de: {candidate_conf_path}")
        cand_blocks = parse_conf_blocks(candidate_conf_path)

        # Mapeamento combinado temporário
        temp_map_json = tempfile.NamedTemporaryFile(suffix=".json", delete=False).name
        cmd = [
            "python3",
            str(ROOT_DIR / "scripts" / "map_rline_usage.py"),
            "--conf", str(self.conf_path),
            "--candidates", str(candidate_conf_path),
            "--rules", str(self.rules_path),
            "--pres-dir", str(self.pres_dir),
            "--output", temp_map_json,
        ]
        subprocess.run(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, check=True)

        with open(temp_map_json, "r") as f:
            comb_map = json.load(f)
        os.remove(temp_map_json)

        confs = comb_map["configurations"]
        single_r_names = {c["name"] for c in comb_map["single_r_only"]}

        opportunities = []

        # 1. Identificar candidatas que absorvem K_1 100%
        for cname, info in confs.items():
            if cname in cand_blocks:
                continue
            tot_chosen = info["subaxles_chosen"]
            if tot_chosen == 0:
                continue

            for cand_name, count in info["co_matched"].items():
                if cand_name in cand_blocks and count == tot_chosen:
                    # cand_name absorve cname 100%!
                    k1 = cname
                    f_cand = cand_name

                    # Buscar K_2 compatível
                    for k2_name in single_r_names:
                        if k2_name == k1:
                            continue
                        k2_info = confs.get(k2_name)
                        if not k2_info or not k2_info["r_lines"]:
                            continue

                        # Registrar oportunidade
                        opportunities.append({
                            "candidate": f_cand,
                            "candidate_block": cand_blocks[f_cand],
                            "k1": k1,
                            "k1_subaxles": tot_chosen,
                            "k2": k2_name,
                            "k2_line": k2_info["r_lines"][0],
                        })

        print(f"[+] Total de oportunidades Modo A identificadas: {len(opportunities)}")
        return opportunities

    def test_mode_a_splice_and_close(
        self,
        pres_file: str,
        target_lineno: int,
        k1_name: str,
        k2_name: str,
        cand_name: str,
        cand_block: str,
    ) -> Optional[Tuple[List[str], str]]:
        """
        Testa aprofundar a folha 'R' na linha target_lineno onde K_2 era acionada:
        L<lev> C <v> <m>
          L<lev+1> R
        L<lev> <closing>
        """
        p_path = self.pres_dir / pres_file
        with open(p_path, "r") as f:
            lines = f.readlines()

        axle_info = simulate_axle_at_line(lines, target_lineno)
        lev = axle_info["level"]

        # Vértices em aberto elegíveis conforme gramática RSST de discharge.c
        branchable = axle_info.get("branchable_vertices", [])
        open_verts = [(v, axle_info["low"][v], axle_info["upp"][v]) for v in branchable]

        # Montar catálogo de teste sem k1 e k2, com cand_block
        test_catalog_blocks = [
            block for name, block in self.canonical_blocks.items()
            if name not in (k1_name, k2_name)
        ]
        test_catalog_blocks.append(cand_block)

        with tempfile.NamedTemporaryFile(mode="w", suffix=".conf", delete=False) as tf:
            tf.write("".join(test_catalog_blocks))
            test_conf_path = Path(tf.name)

        try:
            indent = " " * (2 * lev + 2)
            child_indent = " " * (2 * lev + 4)

            for v, low_v, upp_v in open_verts:
                test_conditions = []
                # Condições positivas: m in [max(6, low_v + 1), min(upp_v, 9)]
                for m in range(max(6, low_v + 1), min(upp_v, 9) + 1):
                    test_conditions.append(str(m))
                # Condições negativas: -k for k in [max(5, low_v), min(8, upp_v - 1)]
                for k in range(max(5, low_v), min(8, upp_v - 1) + 1):
                    test_conditions.append(str(-k))

                for cond_val in test_conditions:
                    closing_candidates = [
                        f"{indent}L{lev} R\n",
                    ]

                    for closing in closing_candidates:
                        new_block = [
                            f"{indent}L{lev} C  {v} {cond_val}\n",
                            f"{child_indent}L{lev + 1} R\n",
                            closing,
                        ]

                        spliced = splice_presentation_lines(lines, target_lineno, new_block)
                        with tempfile.NamedTemporaryFile(mode="w", suffix=".present", delete=False) as tpres:
                            tpres.writelines(spliced)
                            tpres_path = Path(tpres.name)

                        cmd = [
                            str(self.discharge_bin),
                            str(tpres_path),
                            str(test_conf_path),
                            str(self.rules_path),
                            "0",
                            "0",
                        ]
                        proc = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
                        os.remove(tpres_path)

                        if "verified" in proc.stdout:
                            return spliced, f"C {v} {cond_val} + {closing.strip()}"
        finally:
            if os.path.exists(test_conf_path):
                os.remove(test_conf_path)

        return None

    def solve_mode_b_hubcap_split(
        self,
        pres_file: str,
        target_lineno: int,
        test_conf_path: Path,
    ) -> Optional[Tuple[List[str], str]]:
        """
        Resolve uma linha 'H' problemática através de bipartição direcionada:
        L<lev> H <hubcap_spec>
        ->
        L<lev> C <v> <m>
          L<lev+1> <child_closure>
        L<lev> <comp_closure>

        Onde child_closure e comp_closure podem ser o Hubcap original, folha R
        ou bipartição recursiva de nível 2.
        """
        p_path = self.pres_dir / pres_file
        with open(p_path, "r") as f:
            lines = f.readlines()

        axle_info = simulate_axle_at_line(lines, target_lineno)
        lev = axle_info["level"]
        orig_line = lines[target_lineno - 1]
        m_h = re.search(r"H\s+(.+)$", orig_line.strip())
        h_spec = m_h.group(1).strip() if m_h else ""

        indent = " " * (2 * lev + 2)
        child_indent = " " * (2 * lev + 4)
        branchable = axle_info.get("branchable_vertices", [])

        # Extrair cadeia de condições ativas (isolamento sem interferência de outras linhas)
        active_branch = []
        first = lines[0].strip()
        deg = int(first.split()[1])
        for idx, line in enumerate(lines[:target_lineno - 1], start=1):
            s = line.strip()
            if not s or s.startswith("Degree"):
                continue
            parts = s.split()
            l_lev = int(parts[0][1:])
            itype = parts[1]
            while len(active_branch) > l_lev:
                active_branch.pop()
            if itype == "C":
                active_branch.append(line)

        def test_candidate_block(candidate_block: List[str]) -> bool:
            pres_content = [f"Degree {deg}\n"] + active_branch + candidate_block + ["Q.E.D.\n"]
            with tempfile.NamedTemporaryFile("w", suffix=".present", delete=False) as tf:
                tf.writelines(pres_content)
                tpath = tf.name
            try:
                proc = subprocess.run(
                    [str(self.discharge_bin), tpath, str(test_conf_path), str(self.rules_path)],
                    stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True
                )
                return (proc.returncode == 0 and "verified." in proc.stdout)
            finally:
                if os.path.exists(tpath):
                    os.remove(tpath)

        # Passo 1: Tentar bipartição simples de Nível 1
        for v in branchable:
            low_v = axle_info["low"][v]
            upp_v = axle_info["upp"][v]
            if low_v >= upp_v:
                continue

            test_conditions = []
            for k in range(max(5, low_v), min(8, upp_v - 1) + 1):
                test_conditions.append(str(-k))
            for m in range(max(6, low_v + 1), min(upp_v, 9) + 1):
                test_conditions.append(str(m))

            for cond_val in test_conditions:
                options = [
                    (f"H  {h_spec}", f"H  {h_spec}"),
                    (f"H  {h_spec}", "R"),
                    ("R", f"H  {h_spec}"),
                    ("R", "R"),
                ]

                for child_act, comp_act in options:
                    new_block = [
                        f"{indent}L{lev} C  {v} {cond_val}\n",
                        f"{child_indent}L{lev + 1} {child_act}\n",
                        f"{indent}L{lev} {comp_act}\n",
                    ]

                    if test_candidate_block(new_block):
                        desc = f"L{lev} C {v} {cond_val} (Child={child_act[:1]}, Comp={comp_act[:1]})"
                        return new_block, desc

        # Passo 2: Tentar bipartição em 2 níveis (quando 1 nível não é suficiente)
        for v1 in branchable:
            low_1 = axle_info["low"][v1]
            upp_1 = axle_info["upp"][v1]
            if low_1 >= upp_1:
                continue

            conds1 = [str(-k) for k in range(max(5, low_1), min(8, upp_1 - 1) + 1)] + \
                     [str(m) for m in range(max(6, low_1 + 1), min(upp_1, 9) + 1)]

            for c1 in conds1:
                for v2 in branchable:
                    if v2 == v1:
                        continue
                    low_2 = axle_info["low"][v2]
                    upp_2 = axle_info["upp"][v2]
                    if low_2 >= upp_2:
                        continue

                    conds2 = [str(-k) for k in range(max(5, low_2), min(8, upp_2 - 1) + 1)] + \
                             [str(m) for m in range(max(6, low_2 + 1), min(upp_2, 9) + 1)]

                    for c2 in conds2:
                        for child2_act in (f"H  {h_spec}", "R"):
                            for comp2_act in (f"H  {h_spec}", "R"):
                                new_block = [
                                    f"{indent}L{lev} C  {v1} {c1}\n",
                                    f"{child_indent}L{lev + 1} H  {h_spec}\n",
                                    f"{indent}L{lev} C  {v2} {c2}\n",
                                    f"{child_indent}L{lev + 1} {child2_act}\n",
                                    f"{indent}L{lev} {comp2_act}\n",
                                ]

                                if test_candidate_block(new_block):
                                    desc = f"Nivel 2: C {v1} {c1} + C {v2} {c2} (ch2={child2_act[:1]}, co2={comp2_act[:1]})"
                                    return new_block, desc

        return None

    def trace_mode_b_hubcap(self, pres_file: str, target_lineno: int) -> Dict:
        """
        Rastreia uma linha 'H' com printmode 4 para identificar o outlet e raio
        que acionou Reduce em CheckBound.
        """
        p_path = self.pres_dir / pres_file
        cmd = [
            str(self.discharge_bin),
            str(p_path),
            str(self.conf_path),
            str(self.rules_path),
            str(target_lineno),
            "4",
        ]
        proc = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)

        forced_outlets = []
        for line in proc.stdout.splitlines():
            if "Starting recursion with" in line:
                m = re.search(r"Starting recursion with\s+(\d+),(\d+)\s+forced", line)
                if m:
                    forced_outlets.append({"outlet": int(m.group(1)), "ray": int(m.group(2))})

        return {
            "lineno": target_lineno,
            "pres_file": pres_file,
            "forced_outlets": forced_outlets,
        }


def main():
    parser = argparse.ArgumentParser(description="Aprofundamento de folhas R e refinamento de hubcaps RSST.")
    parser.add_argument("--mode", choices=["A", "B", "all"], default="all", help="Modo de aprofundamento (A, B ou all)")
    parser.add_argument("--conf", type=Path, default=DEFAULT_CONF, help="Catálogo .conf de entrada")
    parser.add_argument("--candidates", type=Path, default=None, help="Catálogo de configurações candidatas")
    parser.add_argument("--rules", type=Path, default=DEFAULT_RULES, help="Arquivo rules")
    parser.add_argument("--pres-dir", type=Path, default=DEFAULT_PRES_DIR, help="Diretório com apresentações")
    parser.add_argument("--map-json", type=Path, default=DEFAULT_MAP_JSON, help="Relatório de mapeamento JSON")
    parser.add_argument("--solve-hubcap", action="store_true", help="Executa o resolvedor de bipartição Modo B para linhas H")
    parser.add_argument("--target-line", type=int, default=None, help="Número de linha alvo para resolver")
    parser.add_argument("--target-pres", type=str, default="present7", help="Arquivo de apresentação alvo")
    args = parser.parse_args()

    deepener = LeafDeepener(
        conf_path=args.conf,
        rules_path=args.rules,
        pres_dir=args.pres_dir,
        map_json_path=args.map_json,
    )

    print("\n" + "=" * 80)
    print("      MOTOR DE APROFUNDAMENTO DE FOLHAS (LEAF DEEPENING) — RSST")
    print("=" * 80)

    if args.solve_hubcap:
        if args.target_line:
            targets = [args.target_line]
        else:
            # Padrão: as 9 linhas residuais de present7 para o Par 2
            targets = [339, 1774, 2295, 2297, 2301, 2308, 2312, 2318, 2328]

        print(f"\n--- [RESOLVEDOR MODO B: BIPARTIÇÃO DE HUBCAPS PARA {args.target_pres}] ---")
        print(f"Linhas alvo: {targets}")
        print(f"Catálogo de teste: {args.conf}")

        for lno in targets:
            print(f"\n[*] Tentando resolver Linha {lno} de {args.target_pres}...")
            res = deepener.solve_mode_b_hubcap_split(args.target_pres, lno, args.conf)
            if res:
                block, desc = res
                print(f"  [+] SUCESSO na Linha {lno}: {desc}")
                print("  Bloco gerado:")
                for bl in block:
                    print(f"    {bl.rstrip()}")
            else:
                print(f"  [-] Não foi possível fechar a Linha {lno} com profundidade <= 2.")

    if args.mode in ("A", "all") and not args.solve_hubcap:
        print("\n--- [MODO A: ABSORÇÃO 2-POR-1 VIA LEAF DEEPENING] ---")
        cand_path = args.candidates or (ROOT_DIR / "03_modelo_243_podas_2a_ordem" / "certified_pruned_candidates.conf")
        if cand_path.exists():
            opps = deepener.find_mode_a_opportunities(cand_path)
            for opp in opps[:10]:
                print(f"[*] Oportunidade: {opp['candidate']} absorve K_1={opp['k1']} "
                      f"(cobre {opp['k1_subaxles']} sub-eixos) e K_2={opp['k2']} "
                      f"em {opp['k2_line']['file']}:{opp['k2_line']['line']}")
        else:
            print(f"[!] Arquivo de candidatas não encontrado: {cand_path}")

    if args.mode in ("B", "all") and not args.solve_hubcap:
        print("\n--- [MODO B: ELIMINAÇÃO DE ESPECIALISTAS DE LINHAS H] ---")
        single_h = deepener.map_data.get("single_h_only", [])
        print(f"Total de configurações acionadas exclusivamente em 1 linha H: {len(single_h)}")
        for idx, item in enumerate(single_h[:5], start=1):
            loc = item["h_lines"][0]
            trace = deepener.trace_mode_b_hubcap(loc["file"], loc["line"])
            print(f"  {idx}. {item['name']} em {loc['file']}:{loc['line']} -> Outlets disparadores: {trace['forced_outlets']}")

    print("=" * 80 + "\n")


if __name__ == "__main__":
    main()
