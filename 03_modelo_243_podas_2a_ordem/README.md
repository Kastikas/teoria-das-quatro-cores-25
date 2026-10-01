# Pesquisa: Fronteira Sub-394 — Conjunto de 243 Configurações

> **Tamanho do Catálogo Inevitável:** **243 Configurações**  
> **Redução vs RSST (1997 / Coq 2005):** **-390 configurações (-61.6%)**  
> **Redução vs Etapa Anterior (305):** **-62 configurações (-20.3%)**  
> **Redução vs Base Inicial (394):** **-151 configurações (-38.3%)**  
> **Dupla Certificação:** 100% verificado no descarregamento RSST C (`present7`..`present11`) e 100% provado redutível em Rust puro (`243/243`).

---

## 1. O Salto para 243 Configurações

Nesta etapa de pesquisa (Frente 3), aplicamos podas de 2ª ordem ($n \to n-2$) combinadas com contratos de ordem $k=3$ sob o Lema de Stromquist (1975).

O fluxo metodológico:
1. **Síntese Algébrica Paralela em Rust Puro:**
   - Sintetizamos contratos de ordem superior ($k=3$) para um conjunto não-isomórfico de 321 ultra-superconfigurações.
   - **321 de 321 (100%)** foram formalmente certificadas C-redutíveis em 179 segundos.
2. **Reconhecedor Universal Estendido em C:**
   - Generalizamos o verificador oficial RSST (`discharge.c` e `discharge_track.c`) para aceitar contratos de ordem $k \le 4$, preservando a integridade geométrica de subgrafos induzidos (`CheckIso`).
3. **Absorção dos Pares de Flip:**
   - As ultra-superconfigurações geradas pela Frente 3 absorveram os pares de flip da Frente 2 (como os pares `7354` e `7326`, e os irmãos minoritários de `7322`).
4. **Otimização Global Exata (HiGHS MILP):**
   - Mapeamos **187.281 eventos de redução** em 26.535 estados críticos únicos nas 5 apresentações canônicas (`present7` a `present11`).
   - O solver HiGHS encontrou a solução com **243 configurações** dentro do pool avaliado.

---

## 2. Composição do Catálogo de 243 Configurações

* **Total:** 243 configurações
* **Configurações Originais Preservadas:** 140
* **Superconfigurações de Fronteira Ativadas:** 103 (74 de ordem 1 e 29 de ordem 2)
* **Perfil de Redutibilidade Algébrica (Rust Puro):**
  - **D-Redutíveis:** 47 configurações
  - **C-Redutíveis:** 196 configurações
  - **Falhas de Redução:** 0 (100% redutíveis)
* **Perfil de Descarregamento (C `discharge`):**
  - `present7`: Aprovado (0 déficit)
  - `present8`: Aprovado (0 déficit)
  - `present9`: Aprovado (0 déficit)
  - `present10`: Aprovado (0 déficit)
  - `present11`: Aprovado (0 déficit)

---

## 3. Como Reproduzir a Verificação Completa

Para executar a verificação formal dupla e automatizada:

```bash
cd 03_pesquisa_sub394_nova_fronteira
./verify_243.sh
```

O script executa:
1. O verificador oficial RSST `discharge` sobre `present7`, `present8`, `present9`, `present10` e `present11`.
2. O verificador algébrico independente em Rust puro (`quatro_cores verify-file unavoidable_243.conf 300`) em todas as 243 configurações em paralelo.
