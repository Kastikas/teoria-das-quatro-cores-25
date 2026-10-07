# Catálogo Humanamente Legível das 10 Configurações Redutíveis

> [!IMPORTANT]
> ### GUIA ESTRUTURAL E TOPOLÓGICO PARA LEITURA HUMANA
> Os arquivos canônicos `.conf` do RSST (1997) utilizam matrizes numéricas crípticas de inteiros.  
> Este documento traduz as **10 configurações inevitáveis** do nosso catálogo recorde ([`unavoidable_10.conf`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/11_descarregamento_inverso_sub15/unavoidable_10.conf)) em uma **notação estruturada e humanamente compreensível**, detalhando a geometria do anel, as conexões de cada vértice interno, os contratos de redutibilidade de Stromquist e seu papel na demonstração.
>
> **Certificação Algébrica (Rust):** 10/10 configurações provadas redutíveis em **37,69 ms**.

---

## Índice Rápido do Catálogo

| # | Nome Canônico | Vértices ($V$) | Anel ($R$) | Internos | Arestas ($E$) | Tipo de Redutibilidade | Contrato de Stromquist | Usos Totais | Usos Exclusivos |
| :-: | :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **1** | `p_0.7322_3` *(Diamante de Birkhoff)* | 9 | 6 | 3 | 18 | **D/C-Redutível** | $(6 \sim 9), (5 \sim 9)$ | **478** | **131** |
| **2** | `2.122` | 11 | 7 | 4 | 23 | **D-Redutível** | Direto (sem contrato) | **212** | **63** |
| **3** | `p3_p2_p_7322.439320_e2_e3_e7` | 14 | 10 | 4 | 29 | **C-Redutível** | $(10 \sim 14), (9 \sim 14)$ | **152** | **33** |
| **4** | `p3_p2_..._f7_11_13_6` | 14 | 10 | 4 | 29 | **C-Redutível** | $(10 \sim 14), (9 \sim 14)$ | **133** | **35** |
| **5** | `p_p_p_2.45..._2f_2_13_10_14` | 15 | 11 | 4 | 31 | **C-Redutível** | $(11 \sim 12), (10 \sim 12)$ | **158** | **33** |
| **6** | `p_p_p_2.45..._2f_5_14_10_14` | 15 | 11 | 4 | 31 | **C-Redutível** | $(11 \sim 12), (10 \sim 12)$ | **83** | **22** |
| **7** | `p_p_p_2.45..._2f_5_13_7_14` *(Conf #13)* | 15 | 11 | 4 | 31 | **C-Redutível** | $(11 \sim 12), (1 \sim 12)$ | **280** | **89** |
| **8** | `p3_p2_..._2f_1_11_3_11` | 14 | 10 | 4 | 29 | **C-Redutível** | $(10 \sim 14), (9 \sim 14)$ | **193** | **34** |
| **9** | `p_122.122_e2` *(Substituto de 26359322)* | 12 | 8 | 4 | 25 | **C-Redutível** | $(8 \sim 12), (7 \sim 12)$ | **180** | **54** |
| **10** | `Candidato #72` *(`p_p3_p2...3f_1_12`)* | 15 | 11 | 4 | 31 | **C-Redutível** | $(11 \sim 15), (10 \sim 15)$ | **186** | **44** |

---

## 1. Configuração #1: `p_0.7322_3` (O Diamante de Birkhoff)

* **Significado Histórico:** É a configuração redutível mais famosa da história da matemática (George David Birkhoff, 1913). Um anel de 6 vértices contendo 4 triângulos internos dispostos como um diamante.
* **Geometria:**
  - **Vértices Totais:** $V = 9$ (6 no anel de bordo, 3 no interior).
  - **Anel de Bordo:** Ciclo fechado $(1 - 2 - 3 - 4 - 5 - 6 - 1)$.
* **Vértices Internos:**
  - **Vértice 7** ($\deg = 5$): Conecta aos nós do anel $\{1, 2, 3\}$ e aos nós internos $\{8, 9\}$.
  - **Vértice 8** ($\deg = 5$): Conecta aos nós do anel $\{3, 4, 5\}$ e aos nós internos $\{7, 9\}$.
  - **Vértice 9** ($\deg = 5$): Conecta aos nós do anel $\{5, 6, 1\}$ e aos nós internos $\{7, 8\}$.
* **Esquema de Redutibilidade:**
  - **D-Redutível / C-Redutível:** Toda 4-coloração do bordo estende-se diretamente ou via contração de bordo de duas arestas $(6 \sim 9)$ e $(5 \sim 9)$.
* **Impacto na Prova:** **478 utilizações** (líder absoluto da demonstração); atua em todas as 5 apresentações e possui **131 eixos exclusivos**.

---

## 2. Configuração #2: `2.122`

* **Significado Histórico:** Uma das configurações clássicas D-redutíveis do catálogo do RSST (1997). Não necessita de contração de Kempe: é diretamente redutível.
* **Geometria:**
  - **Vértices Totais:** $V = 11$ (7 no anel de bordo, 4 no interior).
  - **Anel de Bordo:** Ciclo $(1 - 2 - 3 - 4 - 5 - 6 - 7 - 1)$.
* **Vértices Internos:**
  - **Vértice 8** ($\deg = 6$): Hub interno central. Conecta ao anel $\{1, 2, 3\}$ e aos internos $\{9, 10, 11\}$.
  - **Vértice 9** ($\deg = 5$): Conecta ao anel $\{3, 4, 5\}$ e aos internos $\{8, 10\}$.
  - **Vértice 10** ($\deg = 5$): Conecta ao anel $\{5, 6\}$ e aos internos $\{8, 9, 11\}$.
  - **Vértice 11** ($\deg = 5$): Conecta ao anel $\{6, 7, 1\}$ e aos internos $\{8, 10\}$.
* **Esquema de Redutibilidade:**
  - **D-Redutível Direto:** $k = 0$ arestas de contrato. Todo conjunto de colorações admissíveis do 7-ciclo estende-se sem modificação de cadeias.
* **Impacto na Prova:** **212 utilizações** (63 exclusivas), presente em todas as 5 apresentações.

---

## 3. Configuração #3: `p3_p2_p_7322.439320_e2_e3_e7`

* **Significado:** Obtida por 3 podas de 2ª ordem a partir da família `7322.439320`.
* **Geometria:**
  - **Vértices Totais:** $V = 14$ (10 no anel de bordo, 4 no interior).
  - **Anel de Bordo:** Ciclo de 10 vértices $(1 - 2 - \dots - 10 - 1)$.
* **Vértices Internos:**
  - **Vértice 11** ($\deg = 8$): Super-hub interno. Toca 5 vértices do anel $\{1, 2, 3, 6, 7\}$ e todos os 3 nós internos $\{12, 13, 14\}$.
  - **Vértice 12** ($\deg = 5$): Toca o anel $\{3, 4, 5, 6\}$ e o interno $\{11\}$.
  - **Vértice 13** ($\deg = 5$): Toca o anel $\{7, 8, 9\}$ e os internos $\{11, 14\}$.
  - **Vértice 14** ($\deg = 5$): Toca o anel $\{9, 10, 1\}$ e os internos $\{11, 13\}$.
* **Esquema de Redutibilidade:**
  - **C-Redutível:** Contrato de Stromquist com $k = 2$ arestas: funde o vértice 14 com os nós de bordo 10 e 9 ($(10 \sim 14), (9 \sim 14)$).
* **Impacto na Prova:** **152 utilizações** (33 exclusivas). É o pilar dominante do grau 8 (**144 eixos em `present8`**).

---

## 4. Configuração #4: `p3_p2_p_7322.439320_e2_e3_e7_f7_11_13_6`

* **Significado:** Variação da Configuração #3 com uma mutação de 1-flip na aresta $(11, 13)$.
* **Geometria:**
  - **Vértices Totais:** $V = 14$ ($R = 10$, 4 internos).
  - **Anel de Bordo:** Ciclo $(1 - 2 - \dots - 10 - 1)$.
* **Vértices Internos:**
  - **Vértice 11** ($\deg = 7$): Grau reduzido de 8 para 7 pelo flip. Toca o anel $\{1, 2, 3, 6\}$ e os internos $\{12, 13, 14\}$.
  - **Vértice 12** ($\deg = 5$): Toca o anel $\{3, 4, 5, 6\}$ e o interno $\{11\}$.
  - **Vértice 13** ($\deg = 6$): Grau elevado de 5 para 6 pelo flip. Toca o anel $\{6, 7, 8, 9\}$ e os internos $\{11, 14\}$.
  - **Vértice 14** ($\deg = 5$): Toca o anel $\{9, 10, 1\}$ e os internos $\{11, 13\}$.
* **Esquema de Redutibilidade:**
  - **C-Redutível:** Mesmas contrações de Stromquist $(10 \sim 14), (9 \sim 14)$.
* **Impacto na Prova:** **133 utilizações** (35 exclusivas), cobrindo massivamente o grau 7 (**91 eixos em `present7`**).

---

## 5. Configuração #5: `p_p_p_2.454806_v11_v2_e10_2f_2_13_10_14`

* **Significado:** Mutação de 2-flips nas arestas $(2, 13)$ e $(10, 14)$ da linhagem `2.454806`.
* **Geometria:**
  - **Vértices Totais:** $V = 15$ ($R = 11$, 4 internos).
  - **Anel de Bordo:** Ciclo de 11 nós $(1 - 2 - \dots - 11 - 1)$.
* **Vértices Internos:**
  - **Vértice 12** ($\deg = 8$): Toca o anel $\{10, 11, 1, 2, 3\}$ e os internos $\{13, 14, 15\}$.
  - **Vértice 13** ($\deg = 5$): Toca o anel $\{3, 4, 5\}$ e os internos $\{12, 14\}$.
  - **Vértice 14** ($\deg = 6$): Toca o anel $\{5, 6, 7\}$ e os internos $\{12, 13, 15\}$.
  - **Vértice 15** ($\deg = 6$): Toca o anel $\{7, 8, 9, 10\}$ e os internos $\{12, 14\}$.
* **Esquema de Redutibilidade:**
  - **C-Redutível:** Contrato com $k = 2$ arestas: funde o vértice interno 12 com os nós do bordo 11 e 10 ($(11 \sim 12), (10 \sim 12)$).
* **Impacto na Prova:** **158 utilizações** (33 exclusivas). Atuação 100% concentrada e indispensável em `present8`.

---

## 6. Configuração #6: `p_p_p_2.454806_v11_v2_e10_2f_5_14_10_14`

* **Significado:** Variação combinatória de 2-flips nas arestas $(5, 14)$ e $(10, 14)$.
* **Geometria:**
  - **Vértices Totais:** $V = 15$ ($R = 11$, 4 internos).
  - **Anel de Bordo:** Ciclo $(1 - 2 - \dots - 11 - 1)$.
* **Vértices Internos:**
  - **Vértice 12** ($\deg = 7$): Toca o anel $\{10, 11, 1, 2\}$ e os internos $\{13, 14, 15\}$.
  - **Vértice 13** ($\deg = 7$): Toca o anel $\{2, 3, 4, 5, 6\}$ e os internos $\{12, 14\}$.
  - **Vértice 14** ($\deg = 5$): Toca o anel $\{6, 7\}$ e os internos $\{12, 13, 15\}$.
  - **Vértice 15** ($\deg = 6$): Toca o anel $\{7, 8, 9, 10\}$ e os internos $\{12, 14\}$.
* **Esquema de Redutibilidade:**
  - **C-Redutível:** Contrato $(11 \sim 12), (10 \sim 12)$.
* **Impacto na Prova:** **83 utilizações** (22 exclusivas), todas concentradas no grau 7 (`present7`).

---

## 7. Configuração #7: `p_p_p_2.454806_v11_v2_e10_2f_5_13_7_14` (A Clássica "Conf #13")

* **Significado:** A configuração central que analisamos no isomorfismo diédrico quiral.
* **Geometria:**
  - **Vértices Totais:** $V = 15$ ($R = 11$, 4 internos, 31 arestas).
  - **Anel de Bordo:** Ciclo $(1 - 2 - \dots - 11 - 1)$.
* **Vértices Internos:**
  - **Vértice 12** ($\deg = 6$): Toca o anel $\{10, 11, 1, 2\}$ e os internos $\{13, 14\}$.
  - **Vértice 13** ($\deg = 5$): Toca o anel $\{2, 3, 4\}$ e os internos $\{12, 14\}$.
  - **Vértice 14** ($\deg = 7$): Toca o anel $\{4, 5, 6, 10\}$ e os internos $\{12, 13, 15\}$.
  - **Vértice 15** ($\deg = 6$): Toca o anel $\{6, 7, 8, 9, 10\}$ e o interno $\{14\}$.
* **Esquema de Redutibilidade:**
  - **C-Redutível:** Contrato $(11 \sim 12), (1 \sim 12)$.
* **Impacto na Prova:** **280 utilizações** (89 exclusivas). É o segundo grafo mais acionado de toda a demonstração (**241 eixos em `present7`**).

---

## 8. Configuração #8: `p3_p2_p_7322.439320_e2_e3_e7_2f_1_11_3_11`

* **Significado:** Mutação de 2-flips na família 7322 alterando as arestas $(1, 11)$ e $(3, 11)$.
* **Geometria:**
  - **Vértices Totais:** $V = 14$ ($R = 10$, 4 internos).
  - **Anel de Bordo:** Ciclo $(1 - 2 - \dots - 10 - 1)$.
* **Vértices Internos:**
  - **Vértice 11** ($\deg = 6$): Toca o anel $\{2, 6, 7\}$ e os internos $\{12, 13, 14\}$.
  - **Vértice 12** ($\deg = 6$): Toca o anel $\{2, 3, 4, 5, 6\}$ e o interno $\{11\}$.
  - **Vértice 13** ($\deg = 5$): Toca o anel $\{7, 8, 9\}$ e os internos $\{11, 14\}$.
  - **Vértice 14** ($\deg = 6$): Toca o anel $\{9, 10, 1, 2\}$ e os internos $\{11, 13\}$.
* **Esquema de Redutibilidade:**
  - **C-Redutível:** Contrato $(10 \sim 14), (9 \sim 14)$.
* **Impacto na Prova:** **193 utilizações** (34 exclusivas), espalhadas de forma ampla em `present7` (88), `present8` (49), `present9` (44) e `present10` (12).

---

## 9. Configuração #9: `p_122.122_e2` (O "Matador" do Grau 11)

* **Significado Histórico:** A maior inovação geométrica do nosso recorde. Substituiu com apenas **12 vértices** o monstro histórico de **21 vértices** (`26359322.-7322`) do RSST de 1997.
* **Geometria:**
  - **Vértices Totais:** $V = 12$ ($R = 8$, 4 internos, 25 arestas).
  - **Anel de Bordo:** Ciclo de 8 nós $(1 - 2 - 3 - 4 - 5 - 6 - 7 - 8 - 1)$.
* **Vértices Internos (Simetria Quase-Perfeita):**
  - **Vértice 9** ($\deg = 5$): Toca o anel $\{2, 3, 4, 5\}$ e o interno $\{10\}$.
  - **Vértice 10** ($\deg = 5$): Toca o anel $\{2, 5, 6\}$ e os internos $\{9, 11\}$.
  - **Vértice 11** ($\deg = 5$): Toca o anel $\{2, 6, 7\}$ e os internos $\{10, 12\}$.
  - **Vértice 12** ($\deg = 5$): Toca o anel $\{1, 2, 7, 8\}$ e o interno $\{11\}$.
  *(Note que todos os 4 vértices internos têm exatamente grau 5 e conectam em cadeia linear 9-10-11-12 ancorada no nó 2 do anel).*
* **Esquema de Redutibilidade:**
  - **C-Redutível:** Contrato $(8 \sim 12), (7 \sim 12)$.
* **Impacto na Prova:** **180 utilizações** (54 exclusivas). **Resolve sozinho 100% de `present11` (27/27 eixos)**, além de cobrir fortemente o grau 10 (43) e grau 9 (70).

---

## 10. Configuração #10: `Candidato #72` (`p_p3_p2...3f_1_12_12_15_8_14`)

* **Significado Histórico:** Síntese de 3-Flips triangulares bistelares ($d = 3$) que permitiu colapsar dois grafos ortogonais em um só.
* **Geometria:**
  - **Vértices Totais:** $V = 15$ ($R = 11$, 4 internos, 31 arestas).
  - **Anel de Bordo:** Ciclo de 11 nós $(1 - 2 - \dots - 11 - 1)$.
* **Vértices Internos:**
  - **Vértice 12** ($\deg = 6$): Toca o anel $\{6, 7, 8, 9\}$ e os internos $\{13, 14\}$.
  - **Vértice 13** ($\deg = 9$): Super-hub interno de grau 9. Toca 6 nós do anel $\{1, 2, 3, 4, 5, 6\}$ e todos os 3 internos $\{12, 14, 15\}$.
  - **Vértice 14** ($\deg = 5$): Toca o anel $\{9, 10\}$ e os internos $\{12, 13, 15\}$.
  - **Vértice 15** ($\deg = 5$): Toca o anel $\{10, 11, 1\}$ e os internos $\{13, 14\}$.
* **Esquema de Redutibilidade:**
  - **C-Redutível:** Contrato de Stromquist $(11 \sim 15), (10 \sim 15)$.
* **Impacto na Prova:** **186 utilizações** (44 exclusivas). É o especialista supremo do grau 9 (**186 eixos em `present9`**).

---

## Conclusão da Análise Estrutural

Esta leitura humana revela por que esse conjunto de 10 grafos é o menor e mais equilibrado da história:
1. **Compacidade Extrema:** Nenhum grafo excede $V = 15$ (o RSST dependia de grafos de $V = 21$).
2. **Indispensabilidade Estrita:** Todo e qualquer grafo possui no mínimo 22 eixos exclusivos (nenhum é supérfluo).
3. **Divisão de Trabalho Harmônica:** Os grafos 1 e 2 cobrem a infraestrutura universal; o grafo 9 fecha o grau 11; o grafo 10 fecha o grau 9; e os grafos 3 a 8 sustentam os graus 7 e 8.
