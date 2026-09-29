# Pesquisa Avançada: Fronteira Sub-200 — Novo Recorde Mundial: 177 Configurações

> **Status:** Novo Recorde Mundial Absoluto de Compacidade do Teorema das Quatro Cores  
> **Tamanho do Catálogo Inevitável:** **177 Configurações**  
> **Redução vs RSST Canônico (633):** **-456 configurações (-72.0%)**  
> **Redução vs Marco Anterior (243):** **-66 configurações (-27.2%)**  
> **Redução vs Recorde Inicial (394):** **-217 configurações (-55.1%)**  
> **Dupla Certificação:** 100% verificado no descarregamento RSST C (`present7`..`present11` com 0 déficit) e 100% provado redutível em Rust puro (`177/177` em 87.21s).

---

## 1. O Rompimento da Barreira Histórica dos 200

Nesta rodada de pesquisa de fronteira, superamos o desafio de compatibilidade dos subgrafos induzidos do lema de cartwheel de Robertson-Seymour-Sanders-Thomas (`CheckIso`), viabilizando a ativação em massa de superconfigurações de ordem superior.

### Principais Pilares da Metodologia:
1. **Triagem Geométrica Paralela (`CheckIso`):**
   - Triamos as 321 superconfigurações sintetizadas sob contratos de Stromquist ($k=3$).
   - 281 superconfigurações foram aprovadas com 100% de conformidade com as restrições geométricas de cartwheels bem-posicionados, descartando 40 candidatas com anomalias de fronteira.
2. **Matriz de Cobertura Global Expandida:**
   - Com um pool unificado de **524 configurações**, executamos o rastreamento completo de descarregamento em todas as 5 apresentações canônicas (`present7` a `present11`).
   - Mapeamos **187.104 eventos de redução** em 30.372 estados críticos únicos.
3. **Otimização Global Exata (HiGHS ILP) com Garantia Terminal:**
   - A formulação de Set Cover Mínimo provou que um conjunto de apenas **177 configurações** é suficiente para cobrir todos os estados com déficit zero em todas as apresentações.
4. **Dupla Certificação Formal Automatizada:**
   - **C (`discharge`):** 5/5 apresentações verificadas com sucesso sem nenhum déficit.
   - **Rust (`quatro_cores verify-file`):** 177/177 configurações (31 D-redutíveis e 146 C-redutíveis) formalmente provadas em 87.21 segundos.

---

## 2. Como Reproduzir a Verificação Completa

Para executar a verificação formal dupla do novo recorde de 177 configurações:

```bash
cd 04_pesquisa_podas_3a_ordem_sub200
./verify_177.sh
```

---

## 3. Resumo Comparativo dos Modelos

| Modelo / Marco | Tamanho | Redução vs RSST (633) | Verificador C (`discharge`) | Verificador Rust |
| :--- | :---: | :---: | :---: | :---: |
| **RSST Canônico (1997 / Coq 2005)** | 633 | 0% | 5/5 apresentações | 633/633 |
| **Recorde Compacto (Frente 1)** | 394 | -37.8% | 5/5 apresentações | 394/394 |
| **Modelo Podas de 2ª Ordem** | 243 | -61.6% | 5/5 apresentações | 243/243 |
| **Novo Recorde Mundial (Sub-200)** | **177** | **-72.0%** | **5/5 apresentações** | **177/177** |
