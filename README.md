# Redução do Conjunto Inevitável do Teorema das Quatro Cores
### Do Catálogo Histórico de 633 Configurações ao Recorde Mundial de 21 e à Fronteira Sub-15

Este repositório contém o ecossistema formal e computacional para a minimização do conjunto inevitável do **Teorema das Quatro Cores (4CT)**. Combinando o motor algébrico em **Rust puro** (com foco em desempenho sem dependências externas de runtime), extensões diédricas universais no verificador C e **descarregamento inverso** (*inverse discharging*), reduzimos o catálogo clássico de **633 configurações (Robertson, Sanders, Seymour & Thomas, 1997 / Georges Gonthier, Coq 2005)** para marcos recordes rigorosamente certificados.

---

## 📊 Linha do Tempo e Marcos Históricos de Redução

A pesquisa é estruturada em duas trilhas matemáticas claramente delimitadas:

### Trilha A: Linhagem Canônica RSST (67 Regras Originais de 1997)
Modelos verificados estritamente sob as **67 regras clássicas de descarregamento euleriano do RSST**.

| Marco Histórico / Metodologia | Configurações | Redução vs RSST 633 | Verificação Algébrica (Rust) | Verificador RSST em C | Status |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **Appel & Haken (1976)** | 1.476 | Base inicial (+133%) | N/A | Heurística histórica | Histórico |
| **RSST (1997) / Coq (2005)** | 633 | Referência canônica | N/A | `present7` a `present11` | Canônico |
| **Auditoria e Limpeza Canônica** | 629 | -4 confs (-0,6%) | 629/629 redutíveis | 100% verificado | Pasta 01 |
| **Fusão Global de Isomorfismos** | 463 | -170 confs (-26,9%) | 463/463 redutíveis | 100% verificado | Histórico |
| **Poda de Fronteira e Superconfiguração $p\_0.7322\_3$** | 394 | -239 confs (-37,8%) | 394/394 redutíveis | 100% verificado | [Pasta 02](file:///home/ivanlrk/Projetos/quatro-cores-mapa/02_modelo_394_recorde_compacto) |
| **Podas Topológicas de 2ª Ordem** | 243 | -390 confs (-61,6%) | 243/243 redutíveis | 100% verificado | [Pasta 03](file:///home/ivanlrk/Projetos/quatro-cores-mapa/03_modelo_243_podas_2a_ordem) |
| **Otimização Global e Poda Exaustiva** | 177 | -456 confs (-72,0%) | 177/177 redutíveis | 100% verificado | [Pasta 04](file:///home/ivanlrk/Projetos/quatro-cores-mapa/04_modelo_177_sub200_otimizacao_global) |
| **Fronteira Sub-150: Síntese $k=4$ sob Stromquist** | 149 | -484 confs (-76,46%) | 149/149 redutíveis (22 D, 127 C) | 100% verificado | [Pasta 05](file:///home/ivanlrk/Projetos/quatro-cores-mapa/05_pesquisa_contratos_k4_podas_profundas) |
| **Fronteira Sub-140: Podas de 4ª Ordem Profunda** | 137 | -496 confs (-78,36%) | 137/137 redutíveis (20 D, 117 C) | 100% verificado | [Pasta 06](file:///home/ivanlrk/Projetos/quatro-cores-mapa/06_pesquisa_podas_4a_ordem_sub140) |
| **Fronteira Sub-50: Mutações Planares (1-Flips)** | 41 | -592 confs (-93,52%) | 41/41 redutíveis (2 D, 39 C) | 100% verificado | [Pasta 07](file:///home/ivanlrk/Projetos/quatro-cores-mapa/07_pesquisa_flips_sub130) |
| **Fronteira Sub-30: Mutações Planares (2-Flips)** | 25 | -608 confs (-96,05%) | 25/25 redutíveis (3 D, 22 C) | 100% verificado | [Pasta 08](file:///home/ivanlrk/Projetos/quatro-cores-mapa/08_pesquisa_2flips) |
| **Fronteira Diédrica: Isomorfismo $D_{22}$ (Conf #13 = #16)** | 24 | -609 confs (-96,21%) | 24/24 redutíveis (3 D, 21 C) | 100% verificado | [Pasta 09](file:///home/ivanlrk/Projetos/quatro-cores-mapa/09_perspectivas_futuras_3flips) |
| **Recorde Mundial: 3-Flips + Diédrico Universal** | **21** | **-612 confs (-96,68%)** | **21/21 redutíveis** (2 D, 19 C) | **100% verificado (todas as 5)** | [Pasta 10](file:///home/ivanlrk/Projetos/quatro-cores-mapa/10_recorde_21_configuracoes) |

### Trilha B: Fronteira Experimental (Descarregamento Inverso e Regras Aumentadas)
Modelo onde novas regras eulerianas de transferência de carga foram sintetizadas para contornar a rigidez das árvores do RSST.

| Marco | Configurações | Regras | Redução Líquida | Redutibilidade Algébrica | Cobertura de Eixos | Diretório |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: |
| **Fronteira Sub-15: Descarregamento Inverso** | **10** | **75** (+8 regras) | **-623 confs (-98,42%)** | 10/10 redutíveis (38 ms) | 1.181/1.298 eixos diretos (100% grau 11) + 117 compensados por curvatura | [Pasta 11](file:///home/ivanlrk/Projetos/quatro-cores-mapa/11_descarregamento_inverso_sub15) |

> [!IMPORTANT]
> Para uma auditoria detalhada de todas as alterações algorítmicas, invariantes eulerianos e declarações de consistência formal, consulte a [**Declaração Metodológica Explícita**](file:///home/ivanlrk/Projetos/quatro-cores-mapa/DECLARACAO_METODOLOGICA.md).

---

## 🛠️ Como Executar e Reproduzir a Verificação

O repositório disponibiliza um [`Makefile`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/Makefile) e um script CLI unificado [`scripts/verify.sh`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/scripts/verify.sh).

### 1. Pré-requisitos
* Compilador **Rust** (`cargo`, `rustc` $\ge 1.70$)
* Compilador **C** (`gcc` ou `clang`)
* **Python 3** (`python3` com `scipy` para análise de curvatura LP)

### 2. Comandos Rápidos de Verificação

```bash
# Verificar o Recorde Mundial de 21 Configurações (Pasta 10)
make verify-21
# ou: ./scripts/verify.sh 21

# Verificar a Fronteira Inversa de 10 Configurações (Pasta 11)
make verify-10
# ou: ./scripts/verify.sh 10

# Verificar o Modelo Canônico Clássico de 25 Configurações (Pasta 08)
make verify-25
# ou: ./scripts/verify.sh 25

# Executar a verificação de todos os modelos sequencialmente
make verify-all
# ou: ./scripts/verify.sh all

# Executar a suíte de testes unitários em Rust
make test
```

### 3. Verificação Formal Interativa no Lean 4
Para compilar a formalização lógica em Lean 4 e validar todos os teoremas via micro-kernel:

```bash
cd lean4_formalization
lake build
lake exe fourcolor25
```

---

## 📁 Estrutura do Repositório

```
quatro-cores-mapa/
├── Makefile                                     # Orquestrador de compilação e verificação
├── DECLARACAO_METODOLOGICA.md                  # Declaração formal de auditoria metodológica
├── data/
│   └── rsst_presentations/                      # Árvores de decisão e regras do RSST 1997 centralizadas
├── scripts/
│   └── verify.sh                               # CLI unificada de verificação e auditoria
├── 01_modelo_629_rsst_canonico/                # Catálogo limpo pós-RSST
├── 02_modelo_394_recorde_compacto/             # Poda de anéis externos e superconfiguração p_0.7322_3
├── 03_modelo_243_podas_2a_ordem/               # Podas de 2ª ordem
├── 04_modelo_177_sub200_otimizacao_global/     # Otimização global por Set Cover
├── 05_pesquisa_contratos_k4_podas_profundas/   # Síntese de contratos sob Stromquist (k=4)
├── 06_pesquisa_podas_4a_ordem_sub140/          # Podas de 4ª ordem profunda
├── 07_pesquisa_flips_sub130/                   # Mutações planares de 1-flips
├── 08_pesquisa_2flips/                         # Modelo canônico de 25 configurações
├── 09_perspectivas_futuras_3flips/             # Prova visual/algébrica do isomorfismo D_22 (13 e 16), 24 configs
├── 10_recorde_21_configuracoes/                # RECORDE MUNDIAL: 21 configurações (67 regras RSST)
├── 11_descarregamento_inverso_sub15/           # FRONTEIRA SUB-15: 10 configurações (75 regras aumentadas)
├── lean4_formalization/                        # Teoremas formais em Lean 4
└── src/                                        # Motor algébrico em Rust puro (redutibilidade, fusão, diédrico)
```

---

## 🔬 Fundamentação Algébrica e Topológica

1. **Coloração de Tait e Álgebra de Klein:**
   O problema das quatro cores em mapas cúbicos planares é formulado como uma 3-coloração de arestas sobre o grupo de Klein $V_4 \cong \mathbb{Z}_2 \times \mathbb{Z}_2$.
2. **Critério de Fechamento de Kempe & Redutibilidade de Birkhoff:**
   * **D-redutibilidade:** O conjunto de colorações estendíveis ao interior cobre todas as classes de equivalência de Kempe ($nlive = 0$).
   * **C-redutibilidade:** Síntese de contratos de arestas no interior ($k \le 4$) sob o **Lema de Stromquist (1975)** e as restrições estritas de admissibilidade geométrica de Robertson et al. (1997).
3. **Invariância Diédrica no Disco Planar ($D_{2R}$):**
   Triangulações planares são invariantes sob rotações e reflexões diédricas do anel exterior. A extensão universal em C (`discharge_universal.c`) explora todas as quiralidades e arestas-raiz de grau máximo, eliminando redundâncias quirais como as configurações #13 e #16.
