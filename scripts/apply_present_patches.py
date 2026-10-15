#!/usr/bin/env python3
"""
scripts/apply_present_patches.py

Aplica patches de Leaf Deepening sobre os arquivos de apresentação present7..11.
Substitui nós folha por sub-árvores ramificadas, recalcula a numeração de linhas
e atualiza rigorosamente todas as referências de simetria (instruções 'S').

Garante que a apresentação resultante execute no ./bin/discharge puro
sem a necessidade de nenhuma flag especial (-d).
"""

import argparse
import json
import re
import sys
from pathlib import Path
from typing import Dict, List, Tuple


def parse_symmetry_target(line: str) -> Tuple[int, int, int]:
    """
    Identifica se a linha é uma instrução de simetria 'S'.
    Retorna (start_char_idx, end_char_idx, old_target_lineno) ou None.
    Formato esperado: ... L<k> S <rot> <refl> <level> <target_line>
    """
    # Regex para capturar os 4 inteiros após 'S'
    match = re.search(r'(L\d+\s+S\s+\d+\s+\d+\s+\d+\s+)(\d+)', line)
    if match:
        prefix = match.group(1)
        target_str = match.group(2)
        start_pos = match.start(2)
        end_pos = match.end(2)
        return start_pos, end_pos, int(target_str)
    return None


def apply_patches_to_presentation(
    src_file: Path,
    dst_file: Path,
    patches: Dict[int, List[str]],
) -> Dict[int, int]:
    """
    Lê src_file, substitui as linhas presentes em patches (1-based lineno),
    atualiza as referências de simetria e grava em dst_file.
    Retorna o dicionário old_to_new mapping.
    """
    with open(src_file, "r", encoding="utf-8") as f:
        src_lines = f.readlines()

    dst_lines = []
    old_to_new = {}

    # Passo 1: Construir a lista expandida e mapear old_lineno -> new_lineno
    curr_new_lineno = 1
    for old_lineno, line in enumerate(src_lines, 1):
        old_to_new[old_lineno] = curr_new_lineno

        if old_lineno in patches:
            replacement_lines = patches[old_lineno]
            for r_line in replacement_lines:
                # Adiciona quebra de linha se não houver
                if not r_line.endswith("\n"):
                    r_line += "\n"
                dst_lines.append((old_lineno, r_line))
                curr_new_lineno += 1
        else:
            dst_lines.append((old_lineno, line))
            curr_new_lineno += 1

    # Passo 2: Atualizar todas as instruções de simetria 'S'
    final_output_lines = []
    updated_symmetries = 0

    for new_idx, (orig_old_lineno, line) in enumerate(dst_lines, 1):
        sym_info = parse_symmetry_target(line)
        if sym_info:
            start_pos, end_pos, old_target = sym_info
            if old_target in old_to_new:
                new_target = old_to_new[old_target]
                if new_target != old_target:
                    line = line[:start_pos] + str(new_target) + line[end_pos:]
                    updated_symmetries += 1
            else:
                raise ValueError(
                    f"Erro de simetria na linha {new_idx} (orig {orig_old_lineno}): "
                    f"alvo original {old_target} não encontrado no mapeamento!"
                )
        final_output_lines.append(line)

    dst_file.parent.mkdir(parents=True, exist_ok=True)
    with open(dst_file, "w", encoding="utf-8") as f:
        f.writelines(final_output_lines)

    print(
        f"[{src_file.name} -> {dst_file.name}] Linhas: {len(src_lines)} -> {len(final_output_lines)} | "
        f"Patches aplicados: {len(patches)} | Simetrias remapeadas: {updated_symmetries}"
    )
    return old_to_new


def main():
    parser = argparse.ArgumentParser(description="Aplica patches estruturais em apresentações RSST.")
    parser.add_argument("--src", type=str, required=True, help="Arquivo de apresentação original")
    parser.add_argument("--dst", type=str, required=True, help="Arquivo de apresentação de destino")
    parser.add_argument("--patch-json", type=str, required=True, help="Arquivo JSON com os patches")
    args = parser.parse_args()

    src_path = Path(args.src)
    dst_path = Path(args.dst)
    patch_path = Path(args.patch_json)

    with open(patch_path, "r", encoding="utf-8") as f:
        raw_patches = json.load(f)

    # Converte chaves de lineno para int
    patches = {int(k): v for k, v in raw_patches.items()}

    apply_patches_to_presentation(src_path, dst_path, patches)


if __name__ == "__main__":
    main()
