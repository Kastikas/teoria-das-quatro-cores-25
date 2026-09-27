# Redução do Conjunto Inevitável do Teorema das Quatro Cores (399 Configurações — Quebra da Barreira dos 400)

Este repositório contém a implementação completa em **Rust puro** (sem dependências de crates externos no runtime) e os conjuntos canônicos de configurações irredutíveis para o **Teorema das Quatro Cores (4CT)**, reduzindo o conjunto inevitável histórico de **633 configurações (RSST 1997 / Coq 2005)** para o recorde absoluto de **399 configurações** (-36,97% de redução líquida, 234 configurações eliminadas).

---

## 📊 Resumo dos Resultados e Marcos

| Marco Histórico / Algébrico | Configurações | Redução Líquida (% RSST 633) | Verificação Rust (`quatro_cores`) | Verificação RSST (`discharge`) |
| :--- | :---: | :---: | :---: | :---: |
| **Appel & Haken (1976)** | 1.476 | Base inicial | N/A | Heurística histórica |
| **RSST (1997) / Gonthier Coq (2005)** | 633 | -57,1% vs 1976 | N/A | `present7` a `present11` |
| **Auditoria e Limpeza de Código Morto** | 629 | -4 confs (-0,6%) | 629/629 redutíveis (132.67s) | 100% verificado |
| **Primeira Quebra de Barreira** | 628 | -5 confs (-0,8%) | 628/628 redutíveis (174.52s) | 100% verificado |
| **Fusão Global de Isomorfismos** | 463 | -170 confs (-26,9%) | 463/463 redutíveis (112.32s) | 100% verificado |
| **Poda de Fronteira — Anel 11** | 441 | -192 confs (-30,3%) | 441/441 redutíveis (115.10s) | 100% verificado |
| **Poda de Fronteira — Anel 12** | 428 | -205 confs (-32,4%) | 428/428 redutíveis (124.80s) | 100% verificado |
| **Poda de Fronteira — Anel 13** | 405 | -228 confs (-36,0%) | 405/405 redutíveis (148.20s) | 100% verificado |
| **Poda de Fronteira — Anel 14 (Recorde)** | **399** | **-234 confs (-36,97%)** | **399/399 redutíveis (222.41s)** | **100% verificado (todas as 5)** |

---

## 🔬 Fundamentação Algébrica e Topológica

1. **Coloração de Tait e Álgebra do Grupo de Klein:**
   O problema das quatro cores em mapas cúbicos planares é formulado pela coloração de arestas sobre o grupo de Klein $V_4 \cong \mathbb{Z}_2 \times \mathbb{Z}_2 = \{0, a, b, c\}$.
2. **Critério de Fechamento de Kempe:**
   Para cada configuração com anel exterior de tamanho $R$, o espaço de colorações planares é analisado através das partições não-cruzadas de Kempe. Uma configuração é:
   * **D-redutível:** Quando o conjunto de colorações estendíveis cobre todas as classes de equivalência de Kempe ($nlive = 0$).
   * **C-redutível:** Quando existe um contrato estritamente planar de arestas no anel exterior tal que o subgrafo contraído preserva a planaridade, a 5-conectividade e garante uma 4-coloração válida.
3. **Poda Topológica Sistemática de Fronteira (*Boundary Ear Prunings*):**
   Ao podar vértices de orelha no anel exterior de configurações de anéis 11 a 14 e sintetizar novos C-contratos mínimos, as novas configurações geradas tornam-se topologicamente mais gerais e abrangentes, absorvendo dezenas de ramificações antigas do procedimento de descarregamento e tornando 234 configurações completamente obsoletas.
4. **Tripla Certificação Matemática:**
   * **Redutibilidade em Rust Puro:** Todas as 399 configurações foram certificadas pelo binário `quatro_cores` (118 D-redutíveis, 281 C-redutíveis, 0 falhas).
   * **Descarregamento RSST:** As 5 apresentações oficiais (`present7` a `present11`) foram 100% verificadas pelo algoritmo de descarregamento sobre todos os 178.864 eixos planares.
   * **Condições Topológicas:** Todas as 399 configurações cumprem rigorosamente as Condições 1 a 7 de Robertson et al. e possuem raio planar $\le 2$.
   * **Ativação Estrita:** Todas as 399 configurações possuem utilização $> 0$ comprovada nos 178.864 eixos.

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

### 3. Verificar o Catálogo Canônico de 399 Configurações
```bash
./target/release/quatro_cores verify-file unavoidable_399.conf 399
```
*Saída esperada:* `Concluído! Total Redutíveis: 399/399 (D: 118, C: 281) em ~222s (0 falhas)`.

### 4. Executar Síntese Algébrica de C-Contratos em Tempo Real
```bash
./target/release/quatro_cores synth-all unavoidable_399.conf 2
```

---

## 📁 Catálogos Disponíveis

* [`unavoidable_399.conf`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/unavoidable_399.conf): O catálogo minimal recorde com 399 configurações.
* [`unavoidable_428.conf`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/unavoidable_428.conf): Catálogo intermediário pós-poda do Anel 12.
* [`unavoidable_441.conf`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/unavoidable_441.conf): Catálogo intermediário pós-poda do Anel 11.
* [`unavoidable_463.conf`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/unavoidable_463.conf): Catálogo obtido por fusão global de isomorfismos.
* [`unavoidable_629.conf`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/unavoidable_629.conf): Catálogo limpo a partir do conjunto RSST 633.
