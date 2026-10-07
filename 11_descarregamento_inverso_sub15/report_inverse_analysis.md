# Relatório de Descarregamento Inverso: Fronteira Sub-15

Universo analisado: **21 configurações** contra **1298 eixos irredutíveis** das 5 apresentações canônicas do RSST.

## 1. Tabela de Cobertura da Fronteira de Pareto

| Alvo ($K$) | Combinações Testadas | Eixos Abertos | Taxa de Cobertura | p7 | p8 | p9 | p10 | p11 | Tempo |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **8** | 203,490 | **172** | **86.75%** | 68 | 61 | 23 | 20 | 0 | 1.93s |
| **9** | 293,930 | **139** | **89.29%** | 68 | 28 | 23 | 20 | 0 | 2.74s |
| **10** | 352,716 | **117** | **90.99%** | 46 | 28 | 23 | 20 | 0 | 3.31s |
| **11** | 352,716 | **102** | **92.14%** | 31 | 28 | 23 | 20 | 0 | 3.33s |
| **12** | 293,930 | **87** | **93.30%** | 31 | 28 | 23 | 5 | 0 | 2.84s |
| **13** | 203,490 | **76** | **94.14%** | 31 | 17 | 23 | 5 | 0 | 2.00s |
| **14** | 116,280 | **68** | **94.76%** | 24 | 16 | 23 | 5 | 0 | 1.13s |
| **15** | 54,264 | **60** | **95.38%** | 24 | 16 | 15 | 5 | 0 | 0.52s |

---

## 2. O 'Time dos Sonhos' de 10 Configurações ($K = 10$)

Com apenas **10 configurações**, cobrimos **90.99% de toda a geometria de eixos** (1181/1298), deixando apenas **117 eixos abertos**!

**Máscara Hex:** `0x180f53`

### Grafos Selecionados no Time de 10:
1. `[00]` **p_0.7322_3**
2. `[01]` **2.122**
3. `[04]` **p3_p2_p_7322.439320_e2_e3_e7**
4. `[06]` **p3_p2_p_7322.439320_e2_e3_e7_f7_11_13_6**
5. `[08]` **p_p_p_2.454806_v11_v2_e10_2f_2_13_10_14**
6. `[09]` **p_p_p_2.454806_v11_v2_e10_2f_5_14_10_14**
7. `[10]` **p_p_p_2.454806_v11_v2_e10_2f_5_13_7_14**
8. `[11]` **p3_p2_p_7322.439320_e2_e3_e7_2f_1_11_3_11**
9. `[19]` **p_122.122_e2**
10. `[20]` **p_p3_p2_p_439320.439322_e2_e3_e7_e8_2f_3_12_2_12_3f_1_12_12_15_8_14**

### Distribuição dos Eixos Abertos em $K=10$:
- **present7**: 46 eixos abertos
- **present8**: 28 eixos abertos
- **present9**: 23 eixos abertos
- **present10**: 20 eixos abertos
- **present11**: 0 eixos abertos

---

## 3. O 'Time dos Sonhos' de 12 Configurações ($K = 12$)

Com **12 configurações**, a cobertura sobe para **93.30%**, com apenas **87 eixos abertos** em toda a matemática da prova.

**Máscara Hex:** `0x1c2f53`

### Grafos Selecionados no Time de 12:
1. `[00]` **p_0.7322_3**
2. `[01]` **2.122**
3. `[04]` **p3_p2_p_7322.439320_e2_e3_e7**
4. `[06]` **p3_p2_p_7322.439320_e2_e3_e7_f7_11_13_6**
5. `[08]` **p_p_p_2.454806_v11_v2_e10_2f_2_13_10_14**
6. `[09]` **p_p_p_2.454806_v11_v2_e10_2f_5_14_10_14**
7. `[10]` **p_p_p_2.454806_v11_v2_e10_2f_5_13_7_14**
8. `[11]` **p3_p2_p_7322.439320_e2_e3_e7_2f_1_11_3_11**
9. `[13]` **p_p3_p2_p_439320.439322_e2_e3_e7_e8_2f_3_12_2_12**
10. `[18]` **p_p_p_122.1368366_v2_v3_e13_f6_15_16_5_2f_8_16_14_16_3f_13_17_10_18_14_15**
11. `[19]` **p_122.122_e2**
12. `[20]` **p_p3_p2_p_439320.439322_e2_e3_e7_e8_2f_3_12_2_12_3f_1_12_12_15_8_14**

---

## 4. Formulação Matemática do Descarregamento Inverso (LP)

Para zerar os déficits dos eixos abertos restantes sem adicionar novas configurações:

1. **Restrição de Balanço de Carga:**
   Para cada eixo aberto $A_i$ de grau $d$, exigimos:
   $$\text{Carga}(A_i) + \sum_{r \in \mathcal{R}_{\text{novas}}} \Delta C_r(A_i) \ge 0$$

2. **Raio de Ação ($r \le 3$):**
   A carga deve ser transferida de pentágonos a uma distância de até 3 arestas do hub, permitindo dissipar os 117 eixos de $K=10$.
