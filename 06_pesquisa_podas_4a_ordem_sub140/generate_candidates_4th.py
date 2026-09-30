#!/usr/bin/env python3
"""
Gerador de Superconfigurações de 4ª Ordem e Podas Profundas para a Fronteira Sub-140.
Explora o espaço combinatório n -> n-4 a partir do catálogo canônico e do recorde de 149.
"""

import sys
import os

def parse_conf_full(block):
    lines = [l.strip() for l in block.split("\n") if l.strip()]
    if len(lines) < 3:
        return None
    name = lines[0]
    h = [int(x) for x in lines[1].split()]
    verts, ring = h[0], h[1]
    adj = {}
    for v in range(1, verts + 1):
        if 2 + v >= len(lines):
            return None
        parts = [int(x) for x in lines[2 + v].split()]
        if len(parts) < 2:
            return None
        deg = parts[1]
        adj[v] = [int(x) for x in parts[2:2+deg]]
    return name, ring, verts, adj

def check_rsst_modern(n, r, adj):
    if r < 2 or n <= r: return 1
    for i in range(1, r + 1):
        if len(adj[i]) < 3 or len(adj[i]) >= n: return 2
    for i in range(r + 1, n + 1):
        if len(adj[i]) < 5 or len(adj[i]) >= n: return 2
    for i in range(1, n + 1):
        for nb in adj[i]:
            if nb < 1 or nb > n: return 3
    for i in range(1, r + 1):
        next_r = 1 if i == r else i + 1
        prev_r = r if i == 1 else i - 1
        if adj[i][0] != next_r: return 4
        if adj[i][-1] != prev_r: return 4
        for j in range(1, len(adj[i]) - 1):
            if adj[i][j] <= r or adj[i][j] > n: return 4
    total_deg = sum(len(adj[i]) for i in range(1, n + 1))
    if total_deg != 6 * (n - 1) - 2 * r: return 5
    for i in range(1, n + 1):
        d = len(adj[i])
        for j in range(d):
            if j == d - 1:
                if i <= r: continue
                a = adj[i][0]
            else:
                a = adj[i][j + 1]
            k = adj[i][j]
            p = -1
            for idx in range(len(adj[k])):
                next_in_k = adj[k][(idx + 1) % len(adj[k])]
                if adj[k][idx] == a and next_in_k == i:
                    p = idx
                    break
            if p == -1: return 7
    return 0

def check_radius(n, r, adj):
    for u in range(r + 1, n + 1):
        reached = {u}
        for nb in adj[u]:
            reached.add(nb)
            if nb > r:
                for nb2 in adj[nb]:
                    reached.add(nb2)
        if all(v in reached for v in range(r + 1, n + 1)):
            return True
    return False

def prune_ear(ring, verts, adj, ear):
    nbs = adj[ear]
    if len(nbs) != 3: return None
    w = nbs[1]
    old_ring_list = list(range(1, ear)) + [w] + list(range(ear + 1, ring + 1))
    old_interior = [v for v in range(ring + 1, verts + 1) if v != w]
    new_ordering = old_ring_list + old_interior
    old_to_new = {old: new + 1 for new, old in enumerate(new_ordering)}
    new_adj = {}
    for old_u in new_ordering:
        new_u = old_to_new[old_u]
        new_nbs = []
        for nb in adj[old_u]:
            if nb != ear:
                new_nbs.append(old_to_new[nb])
        new_adj[new_u] = new_nbs
    return ring, verts - 1, new_adj

def graph_signature(ring, verts, adj):
    int_degs = tuple(sorted(len(adj[v]) for v in range(ring + 1, verts + 1)))
    ring_degs = tuple(len(adj[v]) for v in range(1, ring + 1))
    shifts = [ring_degs[i:] + ring_degs[:i] for i in range(ring)]
    rev = ring_degs[::-1]
    shifts += [rev[i:] + rev[:i] for i in range(ring)]
    can_ring = min(shifts)
    return (ring, verts, int_degs, can_ring)

def format_conf_block(name, ring, verts, adj):
    lines = [name, f"{verts} {ring} 0 0", " 0 "]
    for v in range(1, verts + 1):
        nbs = adj[v]
        deg = len(nbs)
        nbs_str = "\t" + " ".join(f"{x:2}" for x in nbs)
        lines.append(f"  {v:2} {deg:2}{nbs_str}")
    coords = ["1000"] * verts
    c_idx = 0
    while c_idx < verts:
        chunk = min(8, verts - c_idx)
        lines.append(" " + " ".join(coords[c_idx:c_idx+chunk]))
        c_idx += chunk
    return "\n".join(lines)

def main():
    print("="*80)
    print(" GERADOR DE CANDIDATAS DE 4ª ORDEM E PODAS PROFUNDAS PARA SUB-140")
    print("="*80)

    seen_sigs = set()

    # 1. Carrega todas as configurações do pool existente de 1.039
    print("1. Indexando configurações já presentes no pool de 1.039...")
    with open("unavoidable_pool_1039.conf") as f:
        blocks_pool = [b.strip() for b in f.read().split("\n\n") if b.strip()]
    for b in blocks_pool:
        parsed = parse_conf_full(b)
        if parsed:
            name, r, v, adj = parsed
            seen_sigs.add(graph_signature(r, v, adj))
    print(f"  -> {len(seen_sigs)} assinaturas canônicas únicas já registradas no pool.")

    # 2. Carrega catálogo 629 e base 149
    print("2. Carregando catálogos base (RSST 629 e Recorde 149)...")
    with open("../01_modelo_629_rsst_canonico/unavoidable_629.conf") as f:
        blocks_629 = [b.strip() for b in f.read().split("\n\n") if b.strip()]
    with open("unavoidable_149.conf") as f:
        blocks_149 = [b.strip() for b in f.read().split("\n\n") if b.strip()]

    all_bases = []
    base_names = set()
    for b in blocks_149 + blocks_629:
        parsed = parse_conf_full(b)
        if parsed:
            name, r, v, adj = parsed
            if name not in base_names:
                base_names.add(name)
                all_bases.append(parsed)

    print(f"  -> Total de bases distintas para expansão: {len(all_bases)}")

    # 3. Geração em Cascata: 1ª, 2ª, 3ª e 4ª Ordens
    new_candidates = []

    print("3. Gerando podas de 1ª ordem (n -> n-1)...")
    order1 = []
    for name, r, v, adj in all_bases:
        for ear1 in range(1, r + 1):
            if len(adj[ear1]) == 3:
                pr1 = prune_ear(r, v, adj, ear1)
                if not pr1: continue
                nr1, nv1, nadj1 = pr1
                if nv1 - nr1 < 3: continue
                if check_rsst_modern(nv1, nr1, nadj1) == 0 and check_radius(nv1, nr1, nadj1):
                    sig = graph_signature(nr1, nv1, nadj1)
                    if sig not in seen_sigs:
                        seen_sigs.add(sig)
                        item = (f"p_{name}_e{ear1}", nr1, nv1, nadj1)
                        order1.append(item)
                        new_candidates.append(item)
    print(f"  -> Novas de 1ª ordem: {len(order1)}")

    print("4. Gerando podas de 2ª ordem (n -> n-2)...")
    order2 = []
    for p1_name, nr1, nv1, nadj1 in order1:
        for ear2 in range(1, nr1 + 1):
            if len(nadj1[ear2]) == 3:
                pr2 = prune_ear(nr1, nv1, nadj1, ear2)
                if not pr2: continue
                nr2, nv2, nadj2 = pr2
                if nv2 - nr2 < 3: continue
                if check_rsst_modern(nv2, nr2, nadj2) == 0 and check_radius(nv2, nr2, nadj2):
                    sig = graph_signature(nr2, nv2, nadj2)
                    if sig not in seen_sigs:
                        seen_sigs.add(sig)
                        item = (f"p2_{p1_name}_e{ear2}", nr2, nv2, nadj2)
                        order2.append(item)
                        new_candidates.append(item)
    print(f"  -> Novas de 2ª ordem: {len(order2)}")

    print("5. Gerando podas de 3ª ordem (n -> n-3)...")
    order3 = []
    for p2_name, nr2, nv2, nadj2 in order2:
        for ear3 in range(1, nr2 + 1):
            if len(nadj2[ear3]) == 3:
                pr3 = prune_ear(nr2, nv2, nadj2, ear3)
                if not pr3: continue
                nr3, nv3, nadj3 = pr3
                if nv3 - nr3 < 3: continue
                if check_rsst_modern(nv3, nr3, nadj3) == 0 and check_radius(nv3, nr3, nadj3):
                    sig = graph_signature(nr3, nv3, nadj3)
                    if sig not in seen_sigs:
                        seen_sigs.add(sig)
                        item = (f"p3_{p2_name}_e{ear3}", nr3, nv3, nadj3)
                        order3.append(item)
                        new_candidates.append(item)
    print(f"  -> Novas de 3ª ordem: {len(order3)}")

    print("6. Gerando podas de 4ª ordem profunda (n -> n-4)...")
    order4 = []
    for p3_name, nr3, nv3, nadj3 in order3:
        for ear4 in range(1, nr3 + 1):
            if len(nadj3[ear4]) == 3:
                pr4 = prune_ear(nr3, nv3, nadj3, ear4)
                if not pr4: continue
                nr4, nv4, nadj4 = pr4
                if nv4 - nr4 < 3: continue
                if check_rsst_modern(nv4, nr4, nadj4) == 0 and check_radius(nv4, nr4, nadj4):
                    sig = graph_signature(nr4, nv4, nadj4)
                    if sig not in seen_sigs:
                        seen_sigs.add(sig)
                        item = (f"p4_{p3_name}_e{ear4}", nr4, nv4, nadj4)
                        order4.append(item)
                        new_candidates.append(item)
    print(f"  -> Novas de 4ª ordem profunda: {len(order4)}")

    print("="*80)
    print(f"TOTAL DE NOVAS SUPERCONFIGURAÇÕES INÉDITAS GERADAS: {len(new_candidates)}")
    print("="*80)

    out_file = "candidates_raw_new.conf"
    with open(out_file, "w") as f:
        for c_name, r, v, adj in new_candidates:
            f.write(format_conf_block(c_name, r, v, adj) + "\n\n")

    print(f"Arquivo salvo com sucesso em '{out_file}'!")

if __name__ == "__main__":
    main()
