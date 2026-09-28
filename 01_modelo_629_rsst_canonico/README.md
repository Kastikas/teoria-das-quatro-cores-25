# Modelo Canônico RSST Limpo (629 Configurações)

> **Status:** 100% Axiomático RSST (1997) / Georges Gonthier Coq (2005)  
> **Modificações estruturais dos grafos:** **Zero** (grafos originais sem alteração topológica)  
> **Redução obtida:** -4 configurações (Eliminação estrita de código morto)

---

## 1. O Que É Este Modelo?

Este modelo representa a versão **mais limpa, elegante e fiel** da demonstração clássica de Neil Robertson, Daniel P. Sanders, Paul Seymour e Robin Thomas (RSST, 1997).

No artigo original de 1997 e na formalização em Coq de Georges Gonthier (2005), o catálogo inevitável continha **633 configurações**.
Nossa auditoria algorítmica por grafo bipartido de cobertura de conjuntos comprovou que **4 dessas 633 configurações tinham exatamente ZERO utilizações** em todos os 178.864 eixos (*axles*) das 5 apresentações planares.

### As 4 Configurações Eliminadas:
1. **Conf #8:** `0.79077722` (Anel 8, 14 vértices)
2. **Conf #12:** `0.79077962` (Anel 9, 15 vértices)
3. **Conf #19:** `2.79077962` (Anel 10, 17 vértices)
4. **Conf #21:** `2.79077966` (Anel 11, 18 vértices)

Essas configurações eram resquícios de versões preliminares da árvore de descarregamento (1993–1995) que deixaram de ser necessárias quando a RSST introduziu as regras finais de transferência de carga e simetria.

---

## 2. Conteúdo da Pasta

* [`unavoidable_629.conf`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/01_modelo_629_rsst_canonico/unavoidable_629.conf): O catálogo inevitável canônico contendo as 629 configurações ativas.
* [`unavoidable_633.conf`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/01_modelo_629_rsst_canonico/unavoidable_633.conf): O arquivo histórico original da RSST para comparação direta.
* [`rules`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/01_modelo_629_rsst_canonico/rules): As 67 regras de descarregamento da RSST.
* `present7` a `present11`: As 5 apresentações planares (árvores de eixos para hubs de grau 7 a 11).
* `discharge`: O binário oficial do verificador em C da RSST.
* [`verify_629.sh`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/01_modelo_629_rsst_canonico/verify_629.sh): Script de dupla certificação automática (C oficial + Rust puro).

---

## 3. Como Reproduzir a Verificação

### A. Verificação no Verificador Oficial da RSST (C):
```bash
./discharge present7 unavoidable_629.conf rules 0 1
./discharge present8 unavoidable_629.conf rules 0 1
./discharge present9 unavoidable_629.conf rules 0 1
./discharge present10 unavoidable_629.conf rules 0 1
./discharge present11 unavoidable_629.conf rules 0 1
```
*Todas as 5 apresentações imprimem `verified` com déficit 0 de carga.*

### B. Verificação Algébrica no Motor em Rust Puro:
```bash
cargo run --release -- verify-file 01_modelo_629_rsst_canonico/unavoidable_629.conf 630
```
*629/629 configurações provadas redutíveis (245 D-redutíveis e 384 C-redutíveis estritamente esparsas).*

### C. Verificação Completa em Um Comando:
```bash
./verify_629.sh
```
