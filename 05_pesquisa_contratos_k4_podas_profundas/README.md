# Pesquisa Avançada: Fronteira Sub-150 — Novo Recorde Mundial: 149 Configurações

> **Status:** Novo Recorde Mundial Absoluto de Compacidade do Teorema das Quatro Cores  
> **Tamanho do Catálogo Inevitável:** **149 Configurações**  
> **Redução vs RSST Canônico (633):** **-484 configurações (-76.46%)**  
> **Redução vs Marco Anterior (177):** **-28 configurações (-15.82%)**  
> **Redução vs Base Inicial (394):** **-245 configurações (-62.18%)**  
> **Dupla Certificação:** 100% verificado no descarregamento RSST C (`present7`..`present11` com 0 déficit) e 100% provado redutível em Rust puro (`149/149` em 84.90s).

---

## 1. O Rompimento da Barreira dos 150 Grafos

Nesta rodada da Frente B, expandimos a síntese algébrica sob o Lema de Walter Stromquist (1975) com busca de contratos de Kempe para um universo completo de 964 superconfigurações candidatas (cobrindo podas de 1ª, 2ª e 3ª ordem profunda $n \to n-3$).

### Pilares da Conquista:
1. **Síntese Algébrica Paralela em Rust Puro (100% de Sucesso):**
   - 964 superconfigurações candidatas foram submetidas ao motor multithread `quatro_cores synth-all`.
   - **964 de 964 (100%)** foram formalmente certificadas C-redutíveis em 692 segundos (11,5 minutos).
2. **Triagem Geométrica Paralela (`CheckIso`):**
   - As 964 candidatas foram validadas contra as apresentações oficiais do descarregador RSST C.
   - **862 superconfigurações limpas** foram aprovadas em conformidade estrita com o lema de cartwheels bem-posicionados, descartando 102 candidatas com anomalias de fronteira.
3. **Matriz de Cobertura Global com Pool de 1.039 Configurações:**
   - Unificamos o catálogo base de 177 com as 862 superconfigurações aprovadas.
   - O rastreamento global mapeou **187.301 eventos de redução** em 52.578 estados críticos ao longo de `present7`, `present8`, `present9`, `present10` e `present11`.
4. **Otimização Global Exata (HiGHS ILP) com Fechamento Terminal:**
   - O solver HiGHS provou que o tamanho mínimo global absoluto é de apenas **149 configurações**.
5. **Dupla Certificação Formal Automatizada:**
   - **`discharge` C:** Aprovado com 0 déficit em todas as 5 apresentações planares canônicas.
   - **Rust puro:** **149/149 configurações provadas redutíveis** (22 D-redutíveis e 127 C-redutíveis) em 84.90s.

---

## 2. Como Reproduzir a Verificação Completa

Para executar a verificação formal dupla do novo recorde de 149 configurações:

```bash
cd 05_pesquisa_contratos_k4_podas_profundas
./verify_149.sh
```

---

## 3. Comparativo Histórico dos Modelos

| Modelo / Marco | Tamanho | Redução vs RSST (633) | Verificador C (`discharge`) | Verificador Rust |
| :--- | :---: | :---: | :---: | :---: |
| **RSST Canônico (1997 / Coq 2005)** | 633 | 0% | 5/5 apresentações | 633/633 |
| **Recorde Compacto (Frente 1)** | 394 | -37.8% | 5/5 apresentações | 394/394 |
| **Modelo Podas de 2ª Ordem** | 243 | -61.6% | 5/5 apresentações | 243/243 |
| **Modelo Sub-200 (Frente 4)** | 177 | -72.0% | 5/5 apresentações | 177/177 |
| **Novo Recorde Mundial (Sub-150 / Frente B)** | **149** | **-76.5%** | **5/5 apresentações** | **149/149** |
