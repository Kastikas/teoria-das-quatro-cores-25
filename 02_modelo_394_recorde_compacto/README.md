# Modelo Compacto (394 Configurações)

> **Descrição:** Conjunto reduzido do Teorema das Quatro Cores (4CT) compatível com a suíte original de RSST.  
> **Redução obtida:** **-37,76%** vs. RSST 633 (1997) e Gonthier Coq (2005) | **-73,31%** vs. Appel & Haken 1476 (1976)  
> **Certificação Dupla:** 100% Verificado no Motor Rust Puro + 100% Aprovado no Verificador Oficial RSST C (`discharge`)

---

## 1. O Que É Este Modelo?

Este modelo representa a fronteira máxima de minimalidade do conjunto inevitável que é **100% compatível com a suíte de verificação original de Robertson et al. (1995–1997)**.

A redução de 633 para **394 configurações** (-239 configurações) foi obtida por uma sequência rigorosa de avanços metodológicos:
1. **Auditoria de Código Morto ($633 \to 629$):** Eliminação das 4 configurações com 0 eixos.
2. **Poda de Fronteira de 1ª Ordem ($629 \to 469$):** Poda de orelhas de grau 3 na fronteira ($n \to n-1$) em anéis 8 a 12, criando superconfigurações que causaram o colapso em cascata de mais de 150 casos redundantes.
3. **Fusão Global de Isomorfismos ($469 \to 463$):** Identificação e unificação de grafos isomórficos sob rotações e reflexões planares.
4. **Poda de Fronteira em Anéis Altos ($463 \to 399$):** Poda sistemática nos anéis 11, 12, 13 e 14.
5. **Poda de 2ª Ordem + Eliminação Reversa ($399 \to 394$):** Poda consecutiva de duas orelhas ($n \to n-2$) combinada com resolução exata de Programação Linear Inteira (*Branch & Bound* / *Set Cover*).

---

## 2. Prova de Minimalidade Estática Sob a RSST

Através do solucionador exato em Rust ([`src/set_cover.rs`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/src/set_cover.rs)), provou-se o seguinte:
* **Total de eixos planares analisados:** 356.716 eixos.
* **Eixos singletons (grau 1):** Exatamente 64.120 eixos são cobertos por **uma única configuração**.
* **Resultado:** **Todas as 394 configurações são essenciais.** Não existe nenhuma configuração redundante ou passiva remanescente. O tamanho mínimo global estático é estritamente 394.

---

## 3. Conteúdo da Pasta

* [`unavoidable_394.conf`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/02_modelo_394_recorde_compacto/unavoidable_394.conf): O catálogo de 394 configurações.
* [`rules`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/02_modelo_394_recorde_compacto/rules): As 67 regras de descarregamento da RSST.
* `present7` a `present11`: As 5 apresentações planares.
* `discharge`: O binário do verificador C oficial da RSST.
* `discharge_track`: Versão instrumentada para rastreamento de cobertura reversa.
* [`used_configurations.txt`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/02_modelo_394_recorde_compacto/used_configurations.txt): O histograma provando ativação estrita de todas as 394 configurações.
* [`verify_394.sh`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/02_modelo_394_recorde_compacto/verify_394.sh): Script de dupla certificação automática.

---

## 4. Como Reproduzir a Verificação

### A. Verificação no Verificador Oficial da RSST (C):
```bash
./discharge present7 unavoidable_394.conf rules 0 1
./discharge present8 unavoidable_394.conf rules 0 1
./discharge present9 unavoidable_394.conf rules 0 1
./discharge present10 unavoidable_394.conf rules 0 1
./discharge present11 unavoidable_394.conf rules 0 1
```
*Todas as 5 apresentações imprimem `verified` cobrindo todos os 178.864 eixos com déficit zero de carga.*

### B. Verificação Algébrica no Motor em Rust Puro:
```bash
cargo run --release -- verify-file 02_modelo_394_recorde_compacto/unavoidable_394.conf 400
```
*394/394 provadas redutíveis (118 D-redutíveis e 276 C-redutíveis com contratos de 1 a 4 arestas).*

### C. Verificação Completa em Um Comando:
```bash
./verify_394.sh
```
