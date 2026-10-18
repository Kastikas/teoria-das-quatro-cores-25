#!/usr/bin/env python3
"""
scripts/presentation_editor.py

Motor de manipulação e edição de árvores de apresentação (present7–present11)
do modelo RSST, com suporte a inserção/substituição de blocos (splice) e
reindexação automática de números de linha em instruções de simetria 'S'.

Regra Crítica RSST:
Em CheckSymmetry (discharge.c linha 438), a instrução:
    L<lev> S <k> <eps> <level> <target_line>
busca na tabela sym[] uma condição 'C' registrada com T->number == target_line.
Portanto, ao substituir a linha target_lineno (1-indexada) por um bloco de
N linhas (delta = N - 1), todas as instruções 'S' no arquivo cujo
target_line > target_lineno devem ter target_line atualizado para target_line + delta.
"""

import argparse
import re
import sys
from pathlib import Path
from typing import Dict, List, Optional, Tuple

SYM_PATTERN = re.compile(r"^(\s*L\d+\s+S\s+\d+\s+\d+\s+\d+\s+)(\d+)(\s*)$")
COND_PATTERN = re.compile(r"^\s*L(\d+)\s+C\s+(-?\d+)\s+(-?\d+)")
LINE_PATTERN = re.compile(r"^\s*L(\d+)\s+([CRHS])\b(.*)$")
DEGREE_PATTERN = re.compile(r"^\s*Degree\s*(\d+)")


def splice_presentation_lines(
    lines: List[str],
    target_lineno: int,
    new_block: List[str],
) -> List[str]:
    """
    Substitui a linha 1-indexada `target_lineno` por `new_block` e atualiza
    automaticamente todas as referências de número de linha nas instruções 'S'.

    Args:
        lines: Lista de linhas da apresentação (1-indexadas externamente).
        target_lineno: Número da linha (1-indexado) a ser substituída.
        new_block: Lista de novas linhas a inserir no lugar da linha alvo.

    Returns:
        Nova lista de linhas com as simetrias devidamente reindexadas.
    """
    if target_lineno < 1 or target_lineno > len(lines):
        raise ValueError(
            f"target_lineno={target_lineno} fora dos limites (1..{len(lines)})"
        )

    # Assegurar terminação com '\n' nas novas linhas
    normalized_block = [
        line if line.endswith("\n") else line + "\n"
        for line in new_block
    ]

    delta = len(normalized_block) - 1
    updated_lines: List[str] = []

    for idx, line in enumerate(lines, start=1):
        if idx == target_lineno:
            updated_lines.extend(normalized_block)
            continue

        stripped = line.rstrip("\r\n")
        m = SYM_PATTERN.match(stripped)
        if m:
            ref_line = int(m.group(2))
            if ref_line > target_lineno:
                line = f"{m.group(1)}{ref_line + delta}{m.group(3)}\n"

        if not line.endswith("\n"):
            line = line + "\n"
        updated_lines.append(line)

    return updated_lines


def parse_line_info(line: str) -> Optional[Dict]:
    """Extrai informações estruturadas de uma linha de apresentação."""
    stripped = line.strip()
    if not stripped:
        return None

    m_deg = DEGREE_PATTERN.match(stripped)
    if m_deg:
        return {
            "type": "Degree",
            "degree": int(m_deg.group(1)),
            "level": -1,
            "raw": line,
        }

    m = LINE_PATTERN.match(line)
    if not m:
        if stripped.startswith("Q.E.D."):
            return {"type": "QED", "level": -1, "raw": line}
        return {"type": "UNKNOWN", "level": -1, "raw": line}

    level = int(m.group(1))
    itype = m.group(2)
    rest = m.group(3).strip()

    info = {
        "type": itype,
        "level": level,
        "rest": rest,
        "raw": line,
    }

    if itype == "C":
        parts = rest.split()
        if len(parts) >= 2:
            info["vertex"] = int(parts[0])
            info["bound"] = int(parts[1])
    elif itype == "S":
        m_sym = SYM_PATTERN.match(line.rstrip("\r\n"))
        if m_sym:
            parts = rest.split()
            info["k"] = int(parts[0])
            info["eps"] = int(parts[1])
            info["s_level"] = int(parts[2])
            info["ref_line"] = int(parts[3])

    return info


def validate_presentation_syntax(lines: List[str]) -> Tuple[bool, str]:
    """
    Valida a consistência sintática e o encadeamento de níveis de uma apresentação.
    Verifica que:
    1. A linha 1 possui Degree<d> (d entre 5 e 11).
    2. Cada linha L<lev> respeita o nível da árvore (C incrementa lev, folhas R/H/S decrementam).
    3. lev < 17 (MAXLEV em discharge.c).
    4. Toda instrução 'S' referencia uma simetria ativa na pilha sym[] (com tamanho < MAXSYM=50).
    5. Termina com 'Q.E.D.'.
    """
    if not lines:
        return False, "Arquivo vazio"

    first = lines[0].strip()
    m_deg = DEGREE_PATTERN.match(first)
    if not m_deg:
        return False, f"Linha 1 inválida (esperava Degree<d>): {first}"

    deg = int(m_deg.group(1))
    if deg < 5 or deg > 12:
        return False, f"Grau inválido na linha 1: {deg}"

    lev = 0
    max_lev = 17
    max_sym = 50
    sym_stack: List[Dict] = []  # list of {"number": lineno, "nolines": lev + 1}
    cond_history: List[Dict] = []  # list of {"n": vertex, "m": bound}

    for lineno, line in enumerate(lines[1:], start=2):
        stripped = line.strip()
        if not stripped:
            continue
        if stripped.startswith("Q.E.D."):
            if lev != -1:
                return False, f"Linha {lineno}: Q.E.D. encontrado com lev={lev} (esperado -1)"
            break

        info = parse_line_info(line)
        if not info or info["type"] == "UNKNOWN":
            return False, f"Linha {lineno}: Instrução desconhecida: {stripped}"

        if info["level"] != lev:
            return False, f"Linha {lineno}: Esperava nível L{lev}, obteve L{info['level']} ({stripped})"

        if lev >= max_lev:
            return False, f"Linha {lineno}: Nível L{lev} atinge ou excede MAXLEV={max_lev}"

        itype = info["type"]
        if itype == "C":
            if len(sym_stack) >= max_sym:
                return False, f"Linha {lineno}: Limite MAXSYM={max_sym} excedido"
            sym_stack.append({"number": lineno, "nolines": lev + 1})

            lev += 1
            if lev >= max_lev:
                return False, f"Linha {lineno}: Próximo nível L{lev} atinge ou excede MAXLEV={max_lev}"
            continue

        elif itype in ("R", "H", "S"):
            if itype == "S":
                ref = info.get("ref_line")
                active_numbers = [s["number"] for s in sym_stack]
                if ref not in active_numbers:
                    return False, f"Linha {lineno}: Simetria {ref} não está ativa na pilha sym[] (ativas: {active_numbers})"
                t_sym = next(s for s in sym_stack if s["number"] == ref)
                s_level = info.get("s_level")
                if t_sym["nolines"] != s_level + 1:
                    return False, f"Linha {lineno}: Level mismatch na simetria (esperado {t_sym['nolines'] - 1}, obteve {s_level})"

            # Deletar simetrias com o nível lev da folha ANTES de decrementar lev:
            # (conforme discharge.c linhas 393-396)
            while sym_stack and sym_stack[-1]["nolines"] - 1 >= lev:
                sym_stack.pop()

            lev -= 1

    return True, "Apresentação 100% válida"


def simulate_axle_at_line(lines: List[str], target_lineno: int) -> Dict:
    """
    Simula o estado do eixo (low[] e upp[]) imediatamente antes da execução
    da linha 1-indexada `target_lineno`.
    Retorna também a lista exata de branchable_vertices conforme discharge.c.
    """
    if target_lineno < 1 or target_lineno > len(lines):
        raise ValueError(f"target_lineno={target_lineno} fora dos limites")

    deg_match = DEGREE_PATTERN.match(lines[0].strip())
    if not deg_match:
        raise ValueError("Linha 1 não contém Degree<d>")
    deg = int(deg_match.group(1))

    # Inicializar pilha de eixos
    max_val = 12
    cartvert = 5 * max_val + 2
    max_lev = 20

    low_stack = [[5] * cartvert for _ in range(max_lev)]
    upp_stack = [[12] * cartvert for _ in range(max_lev)]

    # low[0] = upp[0] = deg
    for l in range(max_lev):
        low_stack[l][0] = deg
        upp_stack[l][0] = deg

    lev = 0

    for idx, line in enumerate(lines[1:], start=2):
        if idx == target_lineno:
            # Calcular branchable_vertices espelhando GetBranchableVertices de discharge.c
            d = deg
            branchable = []
            cur_low = low_stack[lev]
            cur_upp = upp_stack[lev]

            # 1. Spokes (1 .. d) with open interval
            for i in range(1, d + 1):
                if cur_low[i] < cur_upp[i]:
                    branchable.append(i)

            # 2. Hats (d+1 .. 2*d) and unlocked Fan vertices
            for s in range(1, d + 1):
                hat = d + s
                if cur_low[hat] < cur_upp[hat]:
                    branchable.append(hat)
                if cur_low[s] == cur_upp[s]:
                    deg_s = cur_low[s]
                    if 6 <= deg_s <= 8:
                        f1 = 2 * d + s
                        if cur_low[f1] < cur_upp[f1]:
                            branchable.append(f1)
                    if 7 <= deg_s <= 8:
                        f2 = 3 * d + s
                        if cur_low[f2] < cur_upp[f2]:
                            branchable.append(f2)
                    if deg_s == 8:
                        f3 = 4 * d + s
                        if cur_low[f3] < cur_upp[f3]:
                            branchable.append(f3)

            return {
                "lineno": target_lineno,
                "level": lev,
                "degree": deg,
                "low": list(cur_low),
                "upp": list(cur_upp),
                "branchable_vertices": branchable,
                "raw": line.strip(),
            }

        stripped = line.strip()
        if not stripped or stripped.startswith("Q.E.D."):
            continue

        info = parse_line_info(line)
        if not info:
            continue

        itype = info["type"]
        if itype == "C":
            n = info["vertex"]
            m = info["bound"]
            for v in range(cartvert):
                low_stack[lev + 1][v] = low_stack[lev][v]
                upp_stack[lev + 1][v] = upp_stack[lev][v]

            if m > 0:
                upp_stack[lev][n] = m - 1
                low_stack[lev + 1][n] = m
            else:
                low_stack[lev][n] = 1 - m
                upp_stack[lev + 1][n] = -m

            lev += 1
        elif itype in ("R", "H", "S"):
            lev -= 1

    raise ValueError(f"Linha {target_lineno} não alcançada")


def self_test():
    """Testes unitários internos de splice e reindexação de simetrias."""
    print("[*] Executando testes unitários de presentation_editor.py...")

    sample_presentation = [
        "Degree7\n",
        "L0 C 1 6\n",        # linha 2
        "  L1 R\n",           # linha 3
        "L0 C 2 -6\n",        # linha 4
        "  L1 S 1 0 1 2\n",   # linha 5 (ref_line = 2)
        "L0 R\n",             # linha 6
        "Q.E.D.\n",
    ]

    # Teste 1: Substituição 1-para-1 (delta = 0)
    res1 = splice_presentation_lines(sample_presentation, 3, ["  L1 R\n"])
    assert res1 == sample_presentation, "Falha no teste delta=0"

    # Teste 2: Inserção de 2 linhas no lugar da linha 3 (delta = +1)
    # Linha 5 (S com ref_line=2): 2 < 3, logo ref_line permanece 2.
    new_block = [
        "  L1 C 3 6\n",
        "    L2 R\n",
    ]
    res2 = splice_presentation_lines(sample_presentation, 3, new_block)
    assert len(res2) == len(sample_presentation) + 1
    assert "ref_line = 2 não deve mudar"
    m_sym = SYM_PATTERN.match(res2[5].rstrip("\r\n"))
    assert m_sym and int(m_sym.group(2)) == 2

    # Teste 3: Substituição na linha 1 (delta = +2) antes da linha 2 (ref_line=2 > 1)
    res3 = splice_presentation_lines(sample_presentation, 1, ["Degree7\n", "# extra1\n", "# extra2\n"])
    # Agora a linha que continha ref_line=2 deve ter sido incrementada por delta=+2 => ref_line=4
    sym_line = [l for l in res3 if " S " in l][0]
    m_sym3 = SYM_PATTERN.match(sym_line.rstrip("\r\n"))
    assert m_sym3 and int(m_sym3.group(2)) == 4, f"Esperava ref_line=4, obteve {m_sym3.group(2)}"

    print("[+] Todos os testes unitários internos passaram com sucesso!")


def main():
    parser = argparse.ArgumentParser(description="Motor de manipulação de apresentações RSST.")
    subparsers = parser.add_subparsers(dest="command")

    # Comando: test
    subparsers.add_parser("test", help="Executa testes unitários internos")

    # Comando: validate
    v_parser = subparsers.add_parser("validate", help="Valida sintaxe e simetrias de uma apresentação")
    v_parser.add_argument("file", type=Path, help="Caminho do arquivo present*")

    # Comando: inspect
    i_parser = subparsers.add_parser("inspect", help="Inspeciona o estado do eixo em uma linha específica")
    i_parser.add_argument("file", type=Path, help="Caminho do arquivo present*")
    i_parser.add_argument("line", type=int, help="Número da linha 1-indexada")

    # Comando: splice
    s_parser = subparsers.add_parser("splice", help="Substitui uma linha por um novo bloco")
    s_parser.add_argument("file", type=Path, help="Caminho do arquivo present*")
    s_parser.add_argument("line", type=int, help="Número da linha 1-indexada a substituir")
    s_parser.add_argument("replacement_file", type=Path, help="Arquivo contendo as novas linhas")
    s_parser.add_argument("--output", "-o", type=Path, default=None, help="Arquivo de saída (padrão: stdout)")

    args = parser.parse_args()

    if args.command == "test":
        self_test()
    elif args.command == "validate":
        with open(args.file, "r") as f:
            lines = f.readlines()
        ok, msg = validate_presentation_syntax(lines)
        if ok:
            print(f"[OK] {args.file}: {msg}")
        else:
            print(f"[ERRO] {args.file}: {msg}", file=sys.stderr)
            sys.exit(1)
    elif args.command == "inspect":
        with open(args.file, "r") as f:
            lines = f.readlines()
        state = simulate_axle_at_line(lines, args.line)
        print(f"Linha {args.line} ({state['raw']}):")
        print(f"  Nível: L{state['level']}")
        print(f"  Grau central: {state['degree']}")
        print("  Vértices com grau em aberto (low < upp):")
        for v in range(1, 5 * state["degree"] + 1):
            if state["low"][v] < state["upp"][v]:
                print(f"    v{v}: [{state['low'][v]}, {state['upp'][v]}]")
    elif args.command == "splice":
        with open(args.file, "r") as f:
            lines = f.readlines()
        with open(args.replacement_file, "r") as f:
            new_block = f.readlines()
        updated = splice_presentation_lines(lines, args.line, new_block)
        if args.output:
            with open(args.output, "w") as f:
                f.writelines(updated)
            print(f"[+] Apresentação atualizada salva em {args.output}")
        else:
            sys.stdout.writelines(updated)
    else:
        parser.print_help()


if __name__ == "__main__":
    main()
