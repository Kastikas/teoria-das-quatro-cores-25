# Redução do Conjunto Inevitável do Teorema das Quatro Cores
### 🏆 Novo Recorde Mundial: 149 Configurações (Quebra da Barreira dos 150)

Este repositório contém a implementação completa em **Rust puro** (com zero alocações em caminhos críticos e sem dependências externas de runtime) e os conjuntos canônicos certificados para o **Teorema das Quatro Cores (4CT)**, reduzindo o conjunto inevitável histórico de **633 configurações (RSST 1997 / Gonthier Coq 2005)** para o recorde absoluto de **149 configurações** (**-76,46%** de redução líquida, 484 configurações eliminadas).

---

## 📊 Linha do Tempo e Marcos Históricos de Redução

| Marco Histórico / Metodologia | Configurações | Redução vs RSST 633 | Verificação Algébrica (Rust) | Verificador RSST (`discharge`) |
| :--- | :---: | :---: | :---: | :---: |
| **Appel & Haken (1976)** | 1.476 | Base inicial (+133%) | N/A | Heurística histórica |
| **Robertson, Sanders, Seymour & Thomas (1997) / Coq (2005)** | 633 | Referência canônica | N/A | `present7` a `present11` |
| **Auditoria e Limpeza Canônica** | 629 | -4 confs (-0,6%) | 629/629 redutíveis | 100% verificado |
| **Fusão Global de Isomorfismos** | 463 | -170 confs (-26,9%) | 463/463 redutíveis | 100% verificado |
| **Poda de Fronteira e Reconhecimento Universal** | 394 | -239 confs (-37,8%) | 394/394 redutíveis | 100% verificado |
| **Podas Topológicas de 1ª Ordem** | 305 | -328 confs (-51,8%) | 305/305 redutíveis | 100% verificado |
| **Podas Topológicas de 2ª Ordem** | 243 | -390 confs (-61,6%) | 243/243 redutíveis | 100% verificado |
| **Otimização Global e Poda Exaustiva** | 177 | -456 confs (-72,0%) | 177/177 redutíveis | 100% verificado |
| 🚀 **Fronteira Sub-150: Síntese $k=4$ sob Lema de Stromquist** | **149** | **-484 confs (-76,46%)** | **149/149 redutíveis** (22 D, 127 C) | **100% verificado (todas as 5)** |

---

## 🔬 Fundamentação Algébrica e Topológica

1. **Coloração de Tait e Álgebra do Grupo de Klein:**
   O problema das quatro cores em mapas cúbicos planares é formulado como uma 3-coloração de arestas sobre o grupo de Klein $V_4 \cong \mathbb{Z}_2 \times \mathbb{Z}_2 = \{0, a, b, c\}$.
2. **Critério de Fechamento de Kempe:**
   Para cada anel exterior de tamanho $R$, o espaço de colorações planares é analisado através das partições não-cruzadas de Kempe:
   * **D-redutibilidade:** O conjunto de colorações estendíveis ao interior da configuração cobre todas as classes de equivalência de Kempe ($nlive = 0$).
   * **C-redutibilidade:** Síntese de contratos esparsos de arestas no interior ($k \le 4$) sob o **Lema de Stromquist (1975)** e as condições de admissibilidade de Robertson et al. (1997), garantindo que todo grafo planar que contenha a configuração possa ser reduzido a um grafo estritamente menor 4-colorível.
3. **Dupla Certificação Matemática Rigorosa:**
   * **Verificador RSST Oficial em C (`discharge`):** As 5 apresentações canônicas (`present7`, `present8`, `present9`, `present10` e `present11`) são 100% verificadas sobre as árvores completas de descarregamento.
   * **Verificador Algébrico em Rust Puro (`quatro_cores`):** Todas as 149 configurações possuem redutibilidade provada de forma exata e determinística (zero falhas).

---

## 🛠️ Como Executar e Reproduzir a Verificação

### 1. Pré-requisitos
* Compilador **Rust** (`cargo`, `rustc` $\ge 1.70$)
* Compilador **C** (`gcc` ou `clang`)

### 2. Certificação Completa do Recorde Mundial (149 Configurações)
Para rodar a dupla certificação formal (RSST `discharge` em C + Verificador Algébrico em Rust) com um único comando:

```bash
cd 05_pesquisa_contratos_k4_podas_profundas
./verify_149.sh
```

*Saída esperada:*
```text
================================================================================
    VERIFICAÇÃO FORMAL DO NOVO RECORDE MUNDIAL: 149 CONFIGURAÇÕES               
    Fronteira Sub-150: Síntese de Contratos k=4 sob o Lema de Stromquist (1975) 
================================================================================
1. Executando Verificador Oficial RSST (discharge) com Reconhecedor Universal...
  -> Verificando present7... OK (present7 verified.)
  -> Verificando present8... OK (present8 verified.)
  -> Verificando present9... OK (present9 verified.)
  -> Verificando present10... OK (present10 verified.)
  -> Verificando present11... OK (present11 verified.)

2. Executando Verificador Algébrico em Rust Puro...
Carregadas 149 configurações. Verificando 149 em paralelo...
Concluído! Total Redutíveis: 149/149 (D: 22, C: 127) em ~137s

================================================================================
 CERTIFICAÇÃO CONCLUÍDA: 149/149 CONFIGURAÇÕES 100% VÁLIDAS E INEVITÁVEIS!      
 Redução vs RSST 633: -484 (-76.5%) | Redução vs Recorde 177: -28 (-15.8%)      
================================================================================
```

### 3. Rodar Testes Unitários e Benchmarks
```bash
cargo test
cargo bench
```

---

## 📁 Estrutura do Repositório

* `01_modelo_629_rsst_canonico/`: Modelo canônico limpo baseado em Robertson et al. (1997).
* `02_modelo_394_recorde_compacto/`: Primeiro conjunto compacto pós-podas de anéis 11-14.
* `03_modelo_243_podas_2a_ordem/`: Modelo com podas de 2ª ordem e reconfiguração de anéis.
* `04_modelo_177_sub200_otimizacao_global/`: Modelo recorde sub-200 obtido por Set Cover exato.
* `05_pesquisa_contratos_k4_podas_profundas/`: **Recorde Mundial de 149 configurações** com script de verificação formal dual `verify_149.sh`.
* `src/`: Motor algébrico de alto desempenho em Rust puro (redutibilidade, fusão de grafos, set cover e mutações).
* `benches/`: Benchmarks estáveis com medição de microssegundos.
* `data/`: Matrizes bipartidas de incidência de descarregamento e arquivos de cobertura.
* `docs/`: Relatórios técnicos formais e documentos LaTeX.
* `scripts/`: Scripts auxiliares de geração de relatórios e compilação de PDFs.
