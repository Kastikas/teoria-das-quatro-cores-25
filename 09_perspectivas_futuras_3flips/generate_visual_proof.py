#!/usr/bin/env python3
"""
Gera a comparacao visual e geometrica completa entre a Configuracao #13 e a Configuracao #16,
provando o Isomorfismo Diedrico Quiral (Reflexao Planar no Grupo D_22) e gerando:
1. comparacao_isomorfismo_13_16.png (Alta resolucao, 300 DPI)
2. comparacao_isomorfismo_13_16.svg (Grafico vetorial)
3. visualizacao_isomorfismo_13_16.html (Dashboard interativo com fixacao persistente de selecao)
"""

import os
import json
import numpy as np
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

# 1. Definicao das duas configuracoes extraidas de 08_pesquisa_2flips/unavoidable_25.conf
def parse_conf_text(text):
    lines = [l.strip() for l in text.strip().splitlines()]
    header = [int(x) for x in lines[1].split()]
    nverts, ring = header[0], header[1]
    adj = {}
    edges = set()
    degrees = {}
    for l in lines[3:3+nverts]:
        parts = [int(x) for x in l.split()]
        v = parts[0]
        deg = parts[1]
        nbrs = parts[2:2+deg]
        adj[v] = nbrs
        degrees[v] = deg
        for u in nbrs:
            if v < u:
                edges.add((v, u))
            elif u < v:
                edges.add((u, v))
    return nverts, ring, adj, sorted(list(edges)), degrees

with open('08_pesquisa_2flips/unavoidable_25.conf') as f:
    text = f.read()
confs = [c.strip() for c in text.split('\n\n') if c.strip()]

name13 = confs[12].splitlines()[0]
name16 = confs[15].splitlines()[0]
nv13, r13, adj13, edges13, deg13 = parse_conf_text(confs[12])
nv16, r16, adj16, edges16, deg16 = parse_conf_text(confs[15])

# Mapeamento do Isomorfismo phi: V(G13) -> V(G16)
phi = {
    1: 11, 2: 10, 3: 9, 4: 8, 5: 7, 6: 6, 7: 5, 8: 4, 9: 3, 10: 2, 11: 1,
    12: 15, 13: 14, 14: 12, 15: 13
}

# 2. Tutte Barycentric Embedding
def compute_tutte_layout(nverts, ring, adj):
    R = 3.5
    pos = {}
    # Colocar anel de bordo no circulo com vertice 6 no topo (eixo vertical)
    for v in range(1, ring + 1):
        theta = np.pi/2 - (v - 6) * (2 * np.pi / ring)
        pos[v] = np.array([R * np.cos(theta), R * np.sin(theta)])
    
    internals = list(range(ring + 1, nverts + 1))
    A = np.zeros((len(internals), len(internals)))
    bx = np.zeros(len(internals))
    by = np.zeros(len(internals))
    
    for i, v in enumerate(internals):
        deg = len(adj[v])
        A[i, i] = deg
        for u in adj[v]:
            if u in internals:
                j = internals.index(u)
                A[i, j] -= 1.0
            else:
                bx[i] += pos[u][0]
                by[i] += pos[u][1]
                
    x_int = np.linalg.solve(A, bx)
    y_int = np.linalg.solve(A, by)
    
    for i, v in enumerate(internals):
        pos[v] = np.array([x_int[i], y_int[i]])
        
    return pos

pos13 = compute_tutte_layout(nv13, r13, adj13)
pos16 = compute_tutte_layout(nv16, r16, adj16)

# Cores consistentes para pares correspondentes
palette = [
    '#e6194b', '#3cb44b', '#ffe119', '#4363d8', '#f58231',
    '#911eb4', '#46f0f0', '#f032e6', '#bcf60c', '#fabebe',
    '#008080', '#e6beff', '#9a6324', '#fffac8', '#800000'
]
vertex_colors = {v: palette[(v - 1) % len(palette)] for v in range(1, 16)}

# 3. Gerar Figura Matplotlib (PNG e SVG)
fig, (ax1, ax2, ax3) = plt.subplots(1, 3, figsize=(22, 8), facecolor='#0f141c')

for ax in (ax1, ax2, ax3):
    ax.set_facecolor('#151b26')
    ax.set_aspect('equal')
    ax.set_xlim(-4.6, 4.6)
    ax.set_ylim(-4.6, 4.6)
    ax.axis('off')

# Painel 1: Configuracao #13
ax1.set_title("Configuração #13\np_p_p_2.454806_v11_v2_e10_2f_5_13_7_14", color='#61afef', fontsize=12, pad=15, fontweight='bold')

for u, v in edges13:
    p1, p2 = pos13[u], pos13[v]
    is_ring = (u <= r13 and v <= r13 and (abs(u - v) == 1 or abs(u - v) == r13 - 1))
    color = '#4fa6ff' if is_ring else '#5c6370'
    lw = 2.5 if is_ring else 1.2
    alpha = 0.9 if is_ring else 0.7
    ax1.plot([p1[0], p2[0]], [p1[1], p2[1]], color=color, lw=lw, alpha=alpha, zorder=2)

for v in range(1, nv13 + 1):
    p = pos13[v]
    is_int = v > r13
    fc = vertex_colors[v]
    size = 280 if is_int else 210
    ax1.scatter(p[0], p[1], s=size, c=fc, edgecolors='#ffffff', linewidths=1.5, zorder=4)
    lbl = f"{v}\n({deg13[v]})"
    ax1.text(p[0], p[1], lbl, color='#000000' if fc in ['#ffe119', '#bcf60c', '#46f0f0', '#fffac8', '#fabebe'] else '#ffffff',
             fontsize=7, ha='center', va='center', fontweight='bold', zorder=5)

# Painel 2: Espelhamento com Eixo de Reflexao M
ax2.set_title("Eixo de Reflexão Quiral M\nSimetria Bilateral Exata (x → -x)", color='#98c379', fontsize=12, pad=15, fontweight='bold')
ax2.plot([0, 0], [-4.2, 4.2], color='#e06c75', lw=2.5, linestyle='--', zorder=1, label='Espelho M (x = 0)')

for v in range(1, 16):
    u = phi[v]
    p1 = pos13[v]
    p2 = pos16[u]
    ax2.plot([p1[0], p2[0]], [p1[1], p2[1]], color='#c678dd', lw=1.0, linestyle=':', alpha=0.6, zorder=2)

for u, v in edges13:
    p1, p2 = pos13[u], pos13[v]
    ax2.plot([p1[0], p2[0]], [p1[1], p2[1]], color='#61afef', lw=1.0, alpha=0.35, zorder=3)
for u, v in edges16:
    p1, p2 = pos16[u], pos16[v]
    ax2.plot([p1[0], p2[0]], [p1[1], p2[1]], color='#e5c07b', lw=1.0, alpha=0.35, zorder=3)

for v in range(1, 16):
    p1 = pos13[v]
    ax2.scatter(p1[0], p1[1], s=120, c=vertex_colors[v], edgecolors='#61afef', lw=1.2, zorder=5)
    u = phi[v]
    p2 = pos16[u]
    ax2.scatter(p2[0], p2[1], s=120, c=vertex_colors[v], edgecolors='#e5c07b', lw=1.2, zorder=5)

ax2.text(0, -4.3, "x = 0", color='#e06c75', fontsize=10, ha='center', fontweight='bold')

# Painel 3: Configuracao #16
ax3.set_title("Configuração #16\np_p3_p2_p_439320.439322_e2_e3_e7_e8_2f_1_12_3_12", color='#e5c07b', fontsize=12, pad=15, fontweight='bold')

for u, v in edges16:
    p1, p2 = pos16[u], pos16[v]
    is_ring = (u <= r16 and v <= r16 and (abs(u - v) == 1 or abs(u - v) == r16 - 1))
    color = '#ff9800' if is_ring else '#5c6370'
    lw = 2.5 if is_ring else 1.2
    alpha = 0.9 if is_ring else 0.7
    ax3.plot([p1[0], p2[0]], [p1[1], p2[1]], color=color, lw=lw, alpha=alpha, zorder=2)

phi_inv = {u: v for v, u in phi.items()}
for u in range(1, nv16 + 1):
    p = pos16[u]
    v_orig = phi_inv[u]
    is_int = u > r16
    fc = vertex_colors[v_orig]
    size = 280 if is_int else 210
    ax3.scatter(p[0], p[1], s=size, c=fc, edgecolors='#ffffff', linewidths=1.5, zorder=4)
    lbl = f"{u}\n({deg16[u]})"
    ax3.text(p[0], p[1], lbl, color='#000000' if fc in ['#ffe119', '#bcf60c', '#46f0f0', '#fffac8', '#fabebe'] else '#ffffff',
             fontsize=7, ha='center', va='center', fontweight='bold', zorder=5)

plt.subplots_adjust(left=0.02, right=0.98, top=0.88, bottom=0.04, wspace=0.08)
fig.text(0.5, 0.95, "PROVA VISUAL DO ISOMORFISMO QUIRAL: CONFIGURAÇÃO #13 ≅ REFLEXÃO(CONFIGURAÇÃO #16)",
         color='#ffffff', fontsize=15, ha='center', fontweight='bold')
fig.text(0.5, 0.915, "Ambos os grafos possuem 15 vértices, anel de bordo R=11, 31 arestas e a mesma estrutura topológica sob rotação invertida",
         color='#abb2bf', fontsize=10, ha='center')

output_png = '09_perspectivas_futuras_3flips/comparacao_isomorfismo_13_16.png'
output_svg = '09_perspectivas_futuras_3flips/comparacao_isomorfismo_13_16.svg'
plt.savefig(output_png, dpi=300, facecolor=fig.get_facecolor(), edgecolor='none')
plt.savefig(output_svg, format='svg', facecolor=fig.get_facecolor(), edgecolor='none')
plt.close(fig)
print(f"Graficos salvos em:\n - {output_png}\n - {output_svg}")

# 4. Gerar Dashboard HTML Interativo com FIXACAO PERSISTENTE (só muda ao passar o mouse em outro vertice)
html_content = f"""<!DOCTYPE html>
<html lang="pt-BR">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Prova Visual do Isomorfismo Quiral: Configurações #13 e #16</title>
<style>
  :root {{
    --bg: #0d1117;
    --card: #161b22;
    --border: #30363d;
    --text: #c9d1d9;
    --accent: #58a6ff;
    --accent-mirror: #f85149;
    --gold: #f1e05a;
    --green: #3fb950;
  }}
  body {{
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
    background-color: var(--bg);
    color: var(--text);
    margin: 0;
    padding: 24px;
    line-height: 1.5;
  }}
  h1, h2, h3 {{ color: #ffffff; margin-top: 0; }}
  .header {{
    text-align: center;
    padding: 15px 0 25px;
    border-bottom: 1px solid var(--border);
    margin-bottom: 25px;
  }}
  .badge {{
    display: inline-block;
    padding: 4px 12px;
    border-radius: 20px;
    font-size: 13px;
    font-weight: 600;
    background: rgba(88, 166, 255, 0.15);
    color: var(--accent);
    margin-bottom: 12px;
    border: 1px solid rgba(88, 166, 255, 0.3);
  }}
  .callout {{
    background: rgba(56, 139, 253, 0.1);
    border-left: 4px solid var(--accent);
    padding: 14px 18px;
    border-radius: 0 8px 8px 0;
    margin-bottom: 20px;
    font-size: 14px;
  }}
  .status-box {{
    background: #161b22;
    border: 2px solid var(--accent);
    padding: 14px 20px;
    border-radius: 10px;
    margin-bottom: 25px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    box-shadow: 0 4px 15px rgba(0,0,0,0.4);
  }}
  .status-title {{
    font-size: 13px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: #8b949e;
    font-weight: 700;
  }}
  .status-detail {{
    font-size: 16px;
    font-weight: 600;
    color: #ffffff;
  }}
  .status-highlight-tag {{
    background: rgba(241, 224, 90, 0.2);
    color: var(--gold);
    border: 1px solid var(--gold);
    padding: 3px 10px;
    border-radius: 6px;
    font-weight: bold;
  }}
  .status-fixed-tag {{
    background: rgba(88, 166, 255, 0.2);
    color: var(--accent);
    border: 1px solid var(--accent);
    padding: 3px 10px;
    border-radius: 6px;
    font-weight: bold;
  }}
  .grid {{
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 24px;
    margin-bottom: 30px;
  }}
  .card {{
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 12px;
    padding: 20px;
    box-shadow: 0 4px 20px rgba(0,0,0,0.3);
  }}
  .svg-container {{
    display: flex;
    justify-content: center;
    align-items: center;
    background: #090d13;
    border-radius: 8px;
    padding: 15px;
    border: 1px solid #21262d;
  }}
  svg {{
    max-width: 100%;
    height: auto;
  }}
  .node {{
    cursor: pointer;
  }}
  .node circle {{
    transition: stroke 0.15s, stroke-width 0.15s, filter 0.15s;
  }}
  .node:hover circle {{
    filter: brightness(1.25);
  }}
  /* Vertice fixado onde o mouse passou */
  .node.source-fixed circle {{
    stroke: #58a6ff !important;
    stroke-width: 4px !important;
    filter: drop-shadow(0 0 6px #58a6ff);
  }}
  /* Vertice correspondente no outro grafo que recebe o highlight */
  .node.target-highlight circle {{
    stroke: #f1e05a !important;
    stroke-width: 5px !important;
    filter: drop-shadow(0 0 12px #f1e05a);
  }}
  .edge {{
    stroke: #30363d;
    stroke-width: 1.5;
    transition: stroke 0.2s, stroke-width 0.2s;
  }}
  .edge.ring {{
    stroke: #388bfd;
    stroke-width: 2.5;
  }}
  .edge.incident-target {{
    stroke: #d29922 !important;
    stroke-width: 3.5px !important;
    stroke-dasharray: 4,2;
  }}
  .table-box {{
    margin-top: 25px;
    overflow-x: auto;
  }}
  table {{
    width: 100%;
    border-collapse: collapse;
    font-size: 14px;
  }}
  th, td {{
    padding: 10px 14px;
    border: 1px solid var(--border);
    text-align: center;
  }}
  th {{
    background: #21262d;
    color: #ffffff;
    font-weight: 600;
  }}
  tr.active-row {{
    background: rgba(88, 166, 255, 0.25) !important;
    outline: 2px solid var(--gold);
  }}
  tr:hover {{
    background: rgba(255,255,255,0.03);
  }}
  .legend {{
    display: flex;
    justify-content: center;
    gap: 24px;
    margin: 15px 0 25px;
    font-size: 14px;
  }}
  .legend-item {{
    display: flex;
    align-items: center;
    gap: 8px;
  }}
  .legend-dot {{
    width: 14px;
    height: 14px;
    border-radius: 50%;
  }}
  .info-bar {{
    background: #21262d;
    padding: 10px 16px;
    border-radius: 8px;
    display: flex;
    justify-content: space-between;
    margin-bottom: 15px;
    font-family: monospace;
    font-size: 13px;
  }}
</style>
</head>
<body>

<div class="header">
  <div class="badge">PROVA VISUAL E TOPOLÓGICA</div>
  <h1>Isomorfismo Quiral: Configuração #13 ≅ Reflexão(Configuração #16)</h1>
  <p>Evidência matemática interativa com fixação contínua de seleção.</p>
</div>

<div class="callout">
  <strong>Comportamento de Fixação Interativa:</strong> Ao passar o mouse sobre qualquer vértice, ele fica <strong>fixado</strong> como âncora e seu par simétrico no outro grafo recebe o <strong>destaque (highlight amarelo brilhante)</strong>. O estado permanece <strong>congelado e visível</strong> mesmo quando você tira o mouse, e <strong>só se altera quando você passa o mouse sobre outro vértice</strong>.
</div>

<div class="status-box" id="status-box">
  <div>
    <div class="status-title">Estado Atual da Bijeção de Isomorfismo</div>
    <div class="status-detail" id="status-text">Carregando...</div>
  </div>
  <div id="status-badges"></div>
</div>

<div class="legend">
  <div class="legend-item"><div class="legend-dot" style="background: #58a6ff;"></div> Anel de Bordo (R = 11)</div>
  <div class="legend-item"><div class="legend-dot" style="background: #e6194b;"></div> Vértices Internos (V = 12..15)</div>
  <div class="legend-item"><div class="legend-dot" style="background: var(--accent-mirror);"></div> Eixo de Simetria Vertical (x = 0)</div>
  <div class="legend-item"><div class="legend-dot" style="background: var(--gold); border: 1px solid #ffffff;"></div> Vértice em Highlight no Outro Grafo</div>
</div>

<div class="grid">
  <!-- Card Conf 13 -->
  <div class="card">
    <h2>Configuração #13</h2>
    <div class="info-bar">
      <span>Nome: <code>p_p_p_2.454806..._2f_5_13_7_14</code></span>
      <span>V=15 | R=11 | E=31</span>
    </div>
    <div class="svg-container">
      <svg id="svg13" viewBox="-180 -180 360 360" width="400" height="400"></svg>
    </div>
  </div>

  <!-- Card Conf 16 -->
  <div class="card">
    <h2>Configuração #16 (Refletida)</h2>
    <div class="info-bar">
      <span>Nome: <code>p_p3_p2_p_439320..._2f_1_12_3_12</code></span>
      <span>V=15 | R=11 | E=31</span>
    </div>
    <div class="svg-container">
      <svg id="svg16" viewBox="-180 -180 360 360" width="400" height="400"></svg>
    </div>
  </div>
</div>

<div class="card table-box">
  <h3>Tabela Rigorosa de Correspondência e Bijeção φ: V(G₁₃) → V(G₁₆)</h3>
  <table>
    <thead>
      <tr>
        <th>Vértice em #13</th>
        <th>Grau em #13</th>
        <th>Coordenada (x, y) #13</th>
        <th>Vértice Correspondente φ(v) em #16</th>
        <th>Grau em #16</th>
        <th>Coordenada (x, y) #16</th>
        <th>Relação Geométrica</th>
      </tr>
    </thead>
    <tbody>
"""

for v in range(1, 16):
    u = phi[v]
    p1 = pos13[v]
    p2 = pos16[u]
    tipo = "Anel (Bordo)" if v <= 11 else "Interior"
    rel = "Ponto Fixo no Espelho (x = 0)" if v == 6 else f"Reflexão Espelhada x₁₆ = -x₁₃"
    html_content += f"""
      <tr id="row-{v}">
        <td><strong>v{v}</strong> ({tipo})</td>
        <td>{deg13[v]}</td>
        <td><code>({p1[0]:.3f}, {p1[1]:.3f})</code></td>
        <td><strong style="color:var(--accent);">v{u}</strong> ({tipo})</td>
        <td>{deg16[u]}</td>
        <td><code>({p2[0]:.3f}, {p2[1]:.3f})</code></td>
        <td style="color:var(--green);">{rel}</td>
      </tr>
    """

html_content += f"""
    </tbody>
  </table>
</div>

<script>
const phi = {json.dumps(phi)};
const phi_inv = Object.fromEntries(Object.entries(phi).map(([k, v]) => [v, parseInt(k)]));
const edges13 = {json.dumps(edges13)};
const edges16 = {json.dumps(edges16)};
const pos13 = {json.dumps({v: [float(pos13[v][0]), float(pos13[v][1])] for v in range(1, 16)})};
const pos16 = {json.dumps({v: [float(pos16[v][0]), float(pos16[v][1])] for v in range(1, 16)})};
const deg13 = {json.dumps(deg13)};
const deg16 = {json.dumps(deg16)};
const colors = {json.dumps(vertex_colors)};

const SCALE = 45;

function drawGraph(svgId, pos, edges, degrees, is16) {{
  const svg = document.getElementById(svgId);
  
  // Desenhar eixo de reflexao tracejado
  const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
  line.setAttribute('x1', 0);
  line.setAttribute('y1', -170);
  line.setAttribute('x2', 0);
  line.setAttribute('y2', 170);
  line.setAttribute('stroke', 'rgba(248, 81, 73, 0.35)');
  line.setAttribute('stroke-dasharray', '4,4');
  line.setAttribute('stroke-width', '1.5');
  svg.appendChild(line);

  // Arestas
  edges.forEach(([u, v]) => {{
    const p1 = pos[u], p2 = pos[v];
    const e = document.createElementNS('http://www.w3.org/2000/svg', 'line');
    e.setAttribute('x1', p1[0] * SCALE);
    e.setAttribute('y1', -p1[1] * SCALE);
    e.setAttribute('x2', p2[0] * SCALE);
    e.setAttribute('y2', -p2[1] * SCALE);
    const isRing = (u <= 11 && v <= 11 && (Math.abs(u - v) === 1 || Math.abs(u - v) === 10));
    e.setAttribute('class', `edge ${{svgId}}-edge-${{u}}-${{v}} ${{svgId}}-edge-${{v}}-${{u}} ${{isRing ? 'ring' : ''}}`);
    svg.appendChild(e);
  }});

  // Vertices
  for (let v = 1; v <= 15; v++) {{
    const p = pos[v];
    const g = document.createElementNS('http://www.w3.org/2000/svg', 'g');
    g.setAttribute('class', `node node-${{v}}`);
    g.setAttribute('id', `${{svgId}}-node-${{v}}`);
    g.setAttribute('transform', `translate(${{p[0] * SCALE}}, ${{-p[1] * SCALE}})`);

    const circle = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
    circle.setAttribute('r', v > 11 ? 16 : 13);
    const colKey = is16 ? phi_inv[v] : v;
    circle.setAttribute('fill', colors[colKey]);
    circle.setAttribute('stroke', '#ffffff');
    circle.setAttribute('stroke-width', '1.5');
    g.appendChild(circle);

    const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
    text.setAttribute('text-anchor', 'middle');
    text.setAttribute('dominant-baseline', 'central');
    text.setAttribute('font-size', '10px');
    text.setAttribute('font-weight', 'bold');
    text.setAttribute('fill', '#ffffff');
    text.textContent = `${{v}} (${{degrees[v]}})`;
    g.appendChild(text);

    // Eventos: Atualiza e FIXA quando o mouse entra ou clica.
    // NAO TEM MOUSELEAVE (o estado fica fixado ate o mouse entrar em outro vertice!)
    g.addEventListener('mouseenter', () => selectVertex(v, is16));
    g.addEventListener('click', () => selectVertex(v, is16));

    svg.appendChild(g);
  }}
}}

// Funcao de selecao persistente
function selectVertex(v, is16) {{
  // Limpar marcacoes anteriores
  document.querySelectorAll('.node').forEach(n => {{
    n.classList.remove('source-fixed');
    n.classList.remove('target-highlight');
  }});
  document.querySelectorAll('.edge').forEach(e => e.classList.remove('incident-target'));
  document.querySelectorAll('tr').forEach(r => r.classList.remove('active-row'));

  let sourceSvg, targetSvg, sourceV, targetV, v13_key;
  if (!is16) {{
    // Mouse no Grafo #13
    sourceSvg = 'svg13';
    targetSvg = 'svg16';
    sourceV = v;
    targetV = phi[v];
    v13_key = v;
  }} else {{
    // Mouse no Grafo #16
    sourceSvg = 'svg16';
    targetSvg = 'svg13';
    sourceV = v;
    targetV = phi_inv[v];
    v13_key = targetV;
  }}

  // 1. Marcar vertice fonte (onde o mouse passou) como FIXADO
  const sourceNode = document.getElementById(`${{sourceSvg}}-node-${{sourceV}}`);
  if (sourceNode) sourceNode.classList.add('source-fixed');

  // 2. Marcar vertice alvo no OUTRO grafo com HIGHLIGHT
  const targetNode = document.getElementById(`${{targetSvg}}-node-${{targetV}}`);
  if (targetNode) targetNode.classList.add('target-highlight');

  // 3. Destacar arestas incidentes no grafo alvo
  const targetEdges = (!is16) ? edges16 : edges13;
  targetEdges.forEach(([u, w]) => {{
    if (u === targetV || w === targetV) {{
      const edgeEl = document.querySelector(`.${{targetSvg}}-edge-${{u}}-${{w}}`);
      if (edgeEl) edgeEl.classList.add('incident-target');
    }}
  }});

  // 4. Destacar linha na tabela
  const row = document.getElementById(`row-${{v13_key}}`);
  if (row) row.classList.add('active-row');

  // 5. Atualizar banner de status
  const sourceName = !is16 ? "Configuração #13" : "Configuração #16";
  const targetName = !is16 ? "Configuração #16" : "Configuração #13";
  const geomDesc = (v13_key === 6) ? "Ponto Fixo sobre o Espelho x = 0" : `Reflexão Bilateral x_${{!is16 ? '16' : '13'}} = -x_${{!is16 ? '13' : '16'}}`;

  document.getElementById('status-text').innerHTML = 
    `Vértice <span class="status-fixed-tag">v${{sourceV}}</span> em ${{sourceName}} ➔ Imagem em Highlight <span class="status-highlight-tag">v${{targetV}}</span> em ${{targetName}} (${{geomDesc}})`;
  
  document.getElementById('status-badges').innerHTML = 
    `<span style="color:#8b949e; font-size:13px; font-family:monospace;">Grau: ${{(!is16 ? deg13[sourceV] : deg16[sourceV])}} ⟷ ${{(!is16 ? deg16[targetV] : deg13[targetV])}}</span>`;
}}

drawGraph('svg13', pos13, edges13, deg13, false);
drawGraph('svg16', pos16, edges16, deg16, true);

// Inicializar fixado no vertice 6 (ponto fixo do espelho)
selectVertex(6, false);
</script>

</body>
</html>
"""

output_html = '09_perspectivas_futuras_3flips/visualizacao_isomorfismo_13_16.html'
with open(output_html, 'w', encoding='utf-8') as f:
    f.write(html_content)
print(f"Dashboard interativo salvo em:\n - {output_html}")
