#!/usr/bin/env python3
"""
scripts/map_rline_usage.py

Mapeador de dependências e uso de configurações nas árvores de apresentação (present7–present11)
do modelo RSST, utilizando a versão instrumentada do verificador C (discharge_mapper.c).

Para cada sub-eixo processado em Reduce:
- Identifica o arquivo e número da linha (lineno);
- Identifica a origem da chamada (instrução explícita 'R' ou 'H' via CheckBound);
- Identifica a configuração escolhida para descarregar o sub-eixo;
- Identifica TODAS as configurações do conjunto (canônicas + candidatas) que também casam com o sub-eixo.

Gera relatório estruturado em formato JSON e exibe sumário estatístico de raridade.
"""

import argparse
import json
import os
import subprocess
import sys
import tempfile
from collections import defaultdict
from pathlib import Path

ROOT_DIR = Path(__file__).resolve().parent.parent
MAPPER_C = ROOT_DIR / "scripts" / "discharge_mapper.c"
DEFAULT_CONF = ROOT_DIR / "01_modelo_629_rsst_canonico" / "unavoidable_629.conf"
DEFAULT_RULES = ROOT_DIR / "01_modelo_629_rsst_canonico" / "rules"
DEFAULT_PRES_DIR = ROOT_DIR / "01_modelo_629_rsst_canonico"
DEFAULT_OUTPUT = ROOT_DIR / "data" / "rline_usage_map.json"


def ensure_binary(bin_path: Path, recompile: bool = False) -> Path:
    """Compila o verificador C instrumentado se necessário."""
    bin_path.parent.mkdir(parents=True, exist_ok=True)
    if not bin_path.exists() or recompile or (MAPPER_C.stat().st_mtime > bin_path.stat().st_mtime):
        print(f"[*] Compilando {MAPPER_C} -> {bin_path}...")
        cmd = ["gcc", "-O3", str(MAPPER_C), "-o", str(bin_path)]
        subprocess.run(cmd, check=True)
    return bin_path


def count_configurations(conf_file: Path) -> int:
    """Conta quantas configurações existem no arquivo .conf."""
    count = 0
    with open(conf_file, "r") as f:
        lines = f.readlines()
    idx = 0
    while idx < len(lines):
        while idx < len(lines) and not lines[idx].strip():
            idx += 1
        if idx >= len(lines):
            break
        idx += 1  # name
        if idx >= len(lines):
            break
        parts = lines[idx].split()
        if not parts:
            break
        n = int(parts[0])
        idx += 1  # verts line
        idx += 1  # contract line
        idx += n  # n adj lines
        coord_count = 0
        while coord_count < n:
            chunk = min(n - coord_count, 8)
            coord_count += chunk
            idx += 1
        count += 1
    return count


def run_mapper(
    bin_path: Path,
    pres_files: list,
    conf_path: Path,
    rules_path: Path,
    canonical_count: int,
) -> dict:
    """Executa o discharge_mapper sobre todas as apresentações e agrega estatísticas."""
    # Estruturas de dados para agregação
    # config_name -> {
    #     "canonical_index": int,
    #     "r_lines": [{"file": str, "line": int}],
    #     "h_lines": [{"file": str, "line": int}],
    #     "subaxles_chosen": int,
    #     "subaxles_matched": int,
    #     "alternative_matches_on_chosen": {other_conf: count}
    # }
    conf_stats = defaultdict(lambda: {
        "r_lines": set(),
        "h_lines": set(),
        "subaxles_chosen": 0,
        "subaxles_matched": 0,
        "co_matched": defaultdict(int),
    })

    subaxles_total = 0

    for pres in pres_files:
        p_path = Path(pres)
        p_name = p_path.name
        print(f"[*] Processando apresentação {p_name}...")

        cmd = [
            str(bin_path),
            str(p_path),
            str(conf_path),
            str(rules_path),
            "0",
            "0",
            str(canonical_count),
        ]
        proc = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, check=True)

        for line in proc.stdout.splitlines():
            if not line.startswith("MAP_RLINE|"):
                continue

            parts = line.split("|")
            # Format: MAP_RLINE|<file>|<lineno>|<R|H>|<chosen_conf>|<num_matches>|<c1>|<c2>|...
            lineno = int(parts[2])
            ltype = parts[3]
            chosen = parts[4]
            num_matches = int(parts[5])
            matches = parts[6: 6 + num_matches]

            subaxles_total += 1

            loc = (p_name, lineno)
            if ltype == "R":
                conf_stats[chosen]["r_lines"].add(loc)
            else:
                conf_stats[chosen]["h_lines"].add(loc)

            conf_stats[chosen]["subaxles_chosen"] += 1

            for m in matches:
                conf_stats[m]["subaxles_matched"] += 1
                if m != chosen:
                    conf_stats[chosen]["co_matched"][m] += 1

    # Formatar para saída serializável
    formatted_confs = {}
    for cname, data in conf_stats.items():
        r_list = sorted([{"file": f, "line": l} for f, l in data["r_lines"]], key=lambda x: (x["file"], x["line"]))
        h_list = sorted([{"file": f, "line": l} for f, l in data["h_lines"]], key=lambda x: (x["file"], x["line"]))
        total_lines = len(r_list) + len(h_list)

        formatted_confs[cname] = {
            "name": cname,
            "total_lines": total_lines,
            "total_r_lines": len(r_list),
            "total_h_lines": len(h_list),
            "r_lines": r_list,
            "h_lines": h_list,
            "subaxles_chosen": data["subaxles_chosen"],
            "subaxles_matched": data["subaxles_matched"],
            "co_matched": dict(sorted(data["co_matched"].items(), key=lambda kv: -kv[1])),
        }

    # Distribuição de frequência
    freq_counts = defaultdict(int)
    single_r = []
    single_h = []
    for cname, d in formatted_confs.items():
        tot = d["total_lines"]
        freq_counts[tot] += 1
        if tot == 1:
            if d["total_r_lines"] == 1:
                single_r.append(d)
            else:
                single_h.append(d)

    single_r.sort(key=lambda x: (x["r_lines"][0]["file"], x["r_lines"][0]["line"]))
    single_h.sort(key=lambda x: (x["h_lines"][0]["file"], x["h_lines"][0]["line"]))

    summary = {
        "total_configurations_used": len(formatted_confs),
        "total_subaxles_evaluated": subaxles_total,
        "single_line_total": len(single_r) + len(single_h),
        "single_r_only_count": len(single_r),
        "single_h_only_count": len(single_h),
        "single_r_only": single_r,
        "single_h_only": single_h,
        "rarity_distribution": {
            "1_line": freq_counts[1],
            "2_lines": freq_counts[2],
            "3_lines": freq_counts[3],
            "4_lines": freq_counts[4],
            "5_lines": freq_counts[5],
            "le_3_lines": sum(freq_counts[k] for k in range(1, 4)),
            "le_5_lines": sum(freq_counts[k] for k in range(1, 6)),
        },
        "configurations": formatted_confs,
    }

    return summary


def print_summary(summary: dict):
    """Exibe no terminal um sumário formatado dos resultados."""
    print("\n" + "=" * 80)
    print("        RELATÓRIO DE MAPEAMENTO DE LINHAS R E H — MODELO RSST")
    print("=" * 80)
    print(f"Total de configurações utilizadas: {summary['total_configurations_used']}")
    print(f"Total de sub-eixos avaliados:       {summary['total_subaxles_evaluated']}")
    print(f"Configurações em apenas 1 linha:    {summary['single_line_total']} "
          f"(R explícito: {summary['single_r_only_count']}, H hubcap: {summary['single_h_only_count']})")
    print(f"Configurações em apenas 2 linhas:   {summary['rarity_distribution']['2_lines']}")
    print(f"Configurações em apenas 3 linhas:   {summary['rarity_distribution']['3_lines']}")
    print(f"Total ocorrendo em <= 3 linhas:     {summary['rarity_distribution']['le_3_lines']}")
    print(f"Total ocorrendo em <= 5 linhas:     {summary['rarity_distribution']['le_5_lines']}")
    print("-" * 80)

    print("\n[+] LISTA DAS CONFIGURAÇÕES ACIONADAS EM APENAS 1 LINHA 'R' EXPLÍCITA:")
    print(f"{'#':<4} {'Nome da Configuração':<24} {'Apresentação':<14} {'Linha':<8} {'Instrução'}")
    print("-" * 65)
    for idx, c in enumerate(summary["single_r_only"], start=1):
        loc = c["r_lines"][0]
        print(f"{idx:<4} {c['name']:<24} {loc['file']:<14} {loc['line']:<8} L* R")

    print("\n[+] CONFIGURAÇÕES ACIONADAS EM APENAS 1 LINHA 'H' (HUBCAP - AMOSTRA DOS 10 PRIMEIROS):")
    print(f"{'#':<4} {'Nome da Configuração':<24} {'Apresentação':<14} {'Linha':<8} {'Sub-eixos'}")
    print("-" * 65)
    for idx, c in enumerate(summary["single_h_only"][:10], start=1):
        loc = c["h_lines"][0]
        print(f"{idx:<4} {c['name']:<24} {loc['file']:<14} {loc['line']:<8} {c['subaxles_chosen']}")
    print("=" * 80 + "\n")


def main():
    parser = argparse.ArgumentParser(description="Mapeador de uso de linhas R e H em apresentações RSST.")
    parser.add_argument("--conf", type=Path, default=DEFAULT_CONF, help="Caminho do arquivo .conf (padrão: unavoidable_629.conf)")
    parser.add_argument("--candidates", type=Path, default=None, help="Caminho opcional para arquivo de configurações candidatas a unir")
    parser.add_argument("--rules", type=Path, default=DEFAULT_RULES, help="Caminho do arquivo rules")
    parser.add_argument("--pres-dir", type=Path, default=DEFAULT_PRES_DIR, help="Diretório com as apresentações present7..present11")
    parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT, help="Caminho para salvar o relatório JSON")
    parser.add_argument("--recompile", action="store_true", help="Forçar recompilação do discharge_mapper")
    args = parser.parse_args()

    bin_path = ROOT_DIR / "target" / "discharge_mapper"
    ensure_binary(bin_path, recompile=args.recompile)

    pres_files = [args.pres_dir / f"present{d}" for d in [7, 8, 9, 10, 11]]
    for pf in pres_files:
        if not pf.exists():
            print(f"Erro: Arquivo de apresentação não encontrado: {pf}", file=sys.stderr)
            sys.exit(1)

    temp_merged = None
    canonical_count = count_configurations(args.conf)

    if args.candidates is not None:
        if not args.candidates.exists():
            print(f"Erro: Arquivo de candidatas não encontrado: {args.candidates}", file=sys.stderr)
            sys.exit(1)
        temp_merged = tempfile.NamedTemporaryFile(mode="w", suffix=".conf", delete=False)
        with open(args.conf, "r") as f_canon:
            temp_merged.write(f_canon.read())
        temp_merged.write("\n")
        with open(args.candidates, "r") as f_cand:
            temp_merged.write(f_cand.read())
        temp_merged.flush()
        temp_merged.close()
        conf_to_run = Path(temp_merged.name)
        print(f"[*] Arquivo combinado temporário gerado: {conf_to_run} ({canonical_count} canônicas + candidatas)")
    else:
        conf_to_run = args.conf

    try:
        summary = run_mapper(
            bin_path=bin_path,
            pres_files=pres_files,
            conf_path=conf_to_run,
            rules_path=args.rules,
            canonical_count=canonical_count,
        )

        print_summary(summary)

        if args.output:
            args.output.parent.mkdir(parents=True, exist_ok=True)
            with open(args.output, "w") as f:
                json.dump(summary, f, indent=2)
            print(f"[+] Relatório estruturado salvo com sucesso em: {args.output}")

    finally:
        if temp_merged is not None and os.path.exists(temp_merged.name):
            os.remove(temp_merged.name)


if __name__ == "__main__":
    main()
