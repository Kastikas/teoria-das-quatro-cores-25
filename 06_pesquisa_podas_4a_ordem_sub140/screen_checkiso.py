#!/usr/bin/env python3
"""
Triagem das 136 novas superconfigurações contra o verificador oficial RSST CheckIso.
Filtra apenas candidatas que não causam nenhuma incompatibilidade de anel.
"""

import os
import subprocess
from concurrent.futures import ThreadPoolExecutor

def main():
    print("="*80)
    print(" TRIAGEM GEOMÉTRICA CONTRA CHECKISO (DISCHARGE OFICIAL RSST)")
    print("="*80)

    with open("candidates_new_certified.conf") as f:
        text_cands = f.read()
    blocks = [b.strip() for b in text_cands.split("\n\n") if b.strip()]

    with open("unavoidable_149.conf") as f:
        base = f.read()

    test_cases = [
        ("present7", "14"),
        ("present8", "35"),
        ("present9", "12"),
        ("present10", "8"),
        ("present11", "3")
    ]

    def test_cand(idx_block):
        idx, b = idx_block
        fname = f"check_{idx}.conf"
        with open(fname, "w") as f:
            f.write(b + "\n\n" + base + "\n\n")

        clean = True
        fails = []
        for pres, line in test_cases:
            res = subprocess.run(["./discharge", pres, fname, "rules", line, "1"],
                                 capture_output=True, text=True)
            if res.returncode != 0:
                clean = False
                fails.append((pres, line, res.stderr.strip() or res.stdout.strip()))
                break

        try:
            os.remove(fname)
        except:
            pass
        return idx, clean, fails

    print(f"Triando {len(blocks)} candidatas em paralelo (4 workers)...")
    with ThreadPoolExecutor(max_workers=4) as executor:
        results = list(executor.map(test_cand, enumerate(blocks)))

    clean = [r for r in results if r[1]]
    dirty = [r for r in results if not r[1]]

    print(f"Total avaliado: {len(blocks)}")
    print(f"Candidatas limpas e 100% compatíveis: {len(clean)}")
    print(f"Candidatas descartadas por restrições de anel: {len(dirty)}")

    clean_blocks = [blocks[r[0]] for r in clean]
    with open("candidates_new_clean.conf", "w") as f:
        f.write("\n\n".join(clean_blocks) + "\n\n")

    print(f"Salvo 'candidates_new_clean.conf' com {len(clean_blocks)} superconfigurações!")

if __name__ == "__main__":
    main()
