# Redução do Conjunto Inevitável do Teorema das Quatro Cores (463 Configurações)

Este repositório contém a implementação completa em **Rust puro** (sem dependências de crates externos no runtime) e os conjuntos canônicos de configurações irredutíveis para o **Teorema das Quatro Cores (4CT)**, reduzindo o conjunto inevitável histórico de **633 configurações (RSST 1997 / Coq 2005)** para o recorde de **463 configurações** (-26,9% de redução).

---

## 📊 Resumo dos Resultados e Marcos

| Marco Histórico / Algébrico | Configurações | Redução Líquida | Verificação Rust (`quatro_cores`) | Verificação RSST (`discharge`) |
| :--- | :---: | :---: | :---: | :---: |
| **Appel & Haken (1976)** | 1.476 | Base inicial | N/A | Heurística histórica |
| **RSST (1997) / Gonthier Coq (2005)** | 633 | -57,1% | N/A | `present7` a `present11` |
| **Auditoria e Limpeza de Código Morto** | 629 | -4 confs | 629/629 redutíveis (132.67s) | 100% verificado |
| **Primeira Quebra de Barreira** | 628 | -5 confs | 628/628 redutíveis (174.52s) | 100% verificado |
| **Poda de Fronteira e Colapso em Cascata** | 469 | -164 confs | 469/469 redutíveis (109.88s) | 100% verificado |
| **Varredura Cruzada Multiespectral** | **463** | **-170 confs (-26,9%)** | **463/463 redutíveis (112.32s)** | **100% verificado** |

---

## 🔬 Fundamentação Algébrica e Topológica

1. **Coloração de Tait e Grupo de Klein:**
   O problema das quatro cores em mapas cúbicos planares é formulado pela coloração de arestas sobre o grupo de Klein $V_4 \cong \mathbb{Z}_2 \times \mathbb{Z}_2 = \{0, a, b, c\}$.
2. **Critério de Fechamento de Kempe:**
   Para cada configuração com anel exterior de tamanho $R$, o espaço de colorações planares é analisado através das partições de Kempe. Uma configuração é:
   * **D-redutível:** Quando o conjunto de colorações estendíveis cobre todas as classes de equivalência de Kempe ($nlive = 0$).
   * **C-redutível:** Quando existe um conjunto estritamente planar de arestas de contração no anel exterior tal que o contraexemplo induzido preserva a 5-conectividade e garante uma 4-coloração válida.
3. **Poda Topológica de Fronteira:**
   A configuração `p_0.7322_3` ($N=9, R=6$) foi sintetizada podando o vértice de orelha de borda $v=3$ da configuração mãe `0.7322`. O motor em Rust sintetizou em **147 µs** um contrato C-redutível mínimo de 2 arestas `[(6, 9), (5, 9)]`, absorvendo mais de 11.000 eixos planares e desobrigando o catálogo de centenas de bifurcações secundárias de quadriláteros.
4. **Verificação de Redundância Universal:**
   Uma varredura exaustiva nos 178.864 eixos das 5 apresentações oficiais de descarregamento (`present7` a `present11`) permitiu a eliminação segura de configurações adicionais cujos cartwheels já estavam completamente cobertos por membros mais gerais do catálogo.

---

## 🛠️ Como Executar e Reproduzir

### 1. Pré-requisitos
* Compilador **Rust** (`cargo`, `rustc` $\ge 1.70$)
* Compilador **C** (`gcc` ou `clang`) e **Python 3**
* `pdflatex` (opcional, para compilação dos relatórios técnicos)

### 2. Compilar o Verificador Rust
```bash
cargo build --release
```

### 3. Verificar o Catálogo Canônico de 463 Configurações
```bash
./target/release/quatro_cores verify-file unavoidable_463.conf 463
```
*Saída esperada:* `Concluído! Total Redutíveis: 463/463 (D: 160, C: 303) em ~112s (0 falhas)`.

### 4. Executar Síntese Algébrica de C-Contratos em Tempo Real
```bash
./target/release/quatro_cores synth-all unavoidable_463.conf 2
```

---

## 📄 Relatórios Técnicos em PDF

O repositório inclui dois relatórios acadêmicos completos gerados em LaTeX:

1. **`relatorio_469_reducoes.pdf`** (11 páginas):
   Histórico formal, fundamentação matemática (Tait, Kempe, matrizes de transição) e catálogo exaustivo com as métricas de ativação e contratos de cada configuração.
2. **`relatorio_explicacao_remocoes.pdf`** (9 páginas):
   Dicionário analítico detalhando a justificativa e o mecanismo de eliminação individual de cada uma das configurações removidas do catálogo clássico de Robertson et al.
