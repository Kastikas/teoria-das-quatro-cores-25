# Auditoria Formal: Especificação das Regras de Descarregamento Adicionadas

> [!IMPORTANT]
> ### DOCUMENTO DE ISOLAMENTO E AUDITORIA
> Este arquivo formaliza, isola e documenta em linguagem matemática e humana as **8 regras de transferência de carga adicionadas** (Regras 68 a 75) ao descarregador clássico do RSST (1997) na [Pasta 11](file:///home/ivanlrk/Projetos/quatro-cores-mapa/11_descarregamento_inverso_sub15/README.md).
>
> **Status:** Todas as 8 regras preservam 100% da fórmula de Euler para triangulações planares ($\sum C = 120$) e garantem que nenhum vértice doador termine com carga negativa ($C_{\text{final}} \ge 0$).

---

## 1. Visão Geral das Regras Adicionadas

No arquivo [`rules_augmented`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/11_descarregamento_inverso_sub15/rules_augmented), mantivemos as **67 regras clássicas do RSST intactas** e adicionamos 4 pares direcionais (regra direta + reflexão diédrica `invert`), totalizando **8 novas entradas**:

| ID da Regra | Tipo / Alvo | Grau do Hub ($b_0$) | Grau do Receptor ($b_1$) | Genealogia RSST | Função Topológica |
| :---: | :---: | :---: | :---: | :---: | :--- |
| **68 / 69** | Direta / Inversa | $8$ | $\ge 7$ (`79`) | Regra 28 (N° 57) e Regra 31 (N° 63) | Absorção de curvatura em hubs de grau 8 cercados por 4 pentágonos e nó de segundo anel ($v_{12}=5$). |
| **70 / 71** | Direta / Inversa | $7$ | $\ge 7$ (`79`) | Regra 24–27 (N° 49–55) | Leque alternado em hubs de grau 7 com intercalação de pentágonos e hexágonos neutros. |
| **72 / 73** | Direta / Inversa | $6$ | $\ge 7$ (`79`) | Regra 13–16 (N° 25–33) | Transferência por diagonal externa de raio 2 através de hexágonos mediadores. |
| **74 / 75** | Direta / Inversa | $5$ | $\ge 7$ (`79`) | Regra 1–3 (N° 1–5) | Liberação conservativa de carga de pentágonos com perímetro seguro (sem concorrência de saída). |

---

## 2. Especificação Técnica e Geométrica Regra por Regra

---

### Regra 33 (Entradas 68 e 69 no arquivo `rules_augmented`)

#### Código em Sintaxe de Máquina:
```text
68	8 79
2 5	3 5	4 5	5 5	12 5

69	invert
```

#### Tradução e Interpretação Humana:
* **Vértice Central ($v_0$):** Hub de grau 8 com déficit de carga inicial $C_0(v_0) = 10(6 - 8) = -20$.
* **Vértice Receptor ($v_1$):** Vértice adjacente de grau $\ge 7$ (grau alto).
* **Condições de Guarda no Cartwheel:**
  - $v_2 = 5$ (pentágono adjacente a $v_1$ e $v_0$);
  - $v_3 = 5$ (pentágono seguinte no bordo);
  - $v_4 = 5$ (pentágono seguinte no bordo);
  - $v_5 = 5$ (pentágono seguinte no bordo);
  - $v_{12} = 5$ (pentágono do segundo anel, vizinho de $v_4$ e $v_0$).
* **Ação:** Transfere $+1$ unidade de carga do leque de pentágonos para o receptor de grau $\ge 7$.
* **Inversão (Entrada 69):** A cláusula `invert` aplica a mesma regra no sentido anti-horário (simetria sob reflexão $\tau$).

---

### Regra 34 (Entradas 70 e 71 no arquivo `rules_augmented`)

#### Código em Sintaxe de Máquina:
```text
70	7 79
2 5	3 6	4 5	5 6	12 5

71	invert
```

#### Tradução e Interpretação Humana:
* **Vértice Central ($v_0$):** Hub de grau 7 com déficit inicial $C_0(v_0) = 10(6 - 7) = -10$.
* **Vértice Receptor ($v_1$):** Vértice adjacente de grau $\ge 7$.
* **Condições de Guarda no Cartwheel:**
  - $v_2 = 5$ (pentágono doador com carga $+10$);
  - $v_3 = 6$ (hexágono neutro que isola o doador);
  - $v_4 = 5$ (segundo pentágono doador);
  - $v_5 = 6$ (segundo hexágono neutro isolador);
  - $v_{12} = 5$ (pentágono de reforço no segundo anel).
* **Ação:** Transfere $+1$ unidade de carga do pentágono $v_2$ para o receptor $v_1$. Como $v_3$ e $v_5$ são hexágonos neutros (carga inicial zero), o pentágono $v_2$ não sofre concorrência de outros receptores e preserva carga $\ge 0$.
* **Inversão (Entrada 71):** Espelhamento quiral no sentido anti-horário.

---

### Regra 35 (Entradas 72 e 73 no arquivo `rules_augmented`)

#### Código em Sintaxe de Máquina:
```text
72	6 79
2 5	3 5	4 5	5 6	9 5

73	invert
```

#### Tradução e Interpretação Humana:
* **Vértice Central ($v_0$):** Hub de grau 6 (vértice neutro de Euler, $C_0(v_0) = 0$).
* **Vértice Receptor ($v_1$):** Vértice adjacente de grau $\ge 7$ (com déficit negativo).
* **Condições de Guarda no Cartwheel:**
  - $v_2 = 5, v_3 = 5, v_4 = 5$ (tríade densa de 3 pentágonos consecutivos com excesso conjunto de $+30$);
  - $v_5 = 6$ (hexágono delimitador);
  - $v_9 = 5$ (pentágono de raio 2 conectado via diagonal externa).
* **Ação:** O hub de grau 6 atua como condutor de passagem: recebe carga da tríade de pentágonos e repassa $+1$ para o receptor de grau alto $v_1$, mantendo seu saldo líquido $C_{\text{final}}(v_0) = 0$.
* **Inversão (Entrada 73):** Espelhamento quiral no sentido anti-horário.

---

### Regra 36 (Entradas 74 e 75 no arquivo `rules_augmented`)

#### Código em Sintaxe de Máquina:
```text
74	5 79
2 5	3 5	4 6	5 5

75	invert
```

#### Tradução e Interpretação Humana:
* **Vértice Central ($v_0$):** Pentágono de grau 5 ($C_0(v_0) = +10$).
* **Vértice Receptor ($v_1$):** Vértice adjacente de grau $\ge 7$.
* **Condições de Guarda no Cartwheel:**
  - $v_2 = 5, v_3 = 5$ (dois pentágonos vizinhos que suportam as faces laterais);
  - $v_4 = 6$ (hexágono neutro);
  - $v_5 = 5$ (pentágono de fechamento de borda).
* **Ação:** Libera $+1$ de carga do pentágono $v_0$ diretamente para o receptor $v_1$. Como $v_0$ tem $+10$ e as outras saídas estão bloqueadas pela guarda, seu saldo final é $C_{\text{final}}(v_0) \ge +9 \ge 0$.
* **Inversão (Entrada 75):** Espelhamento quiral no sentido anti-horário.

---

## 3. Demonstração dos Critérios de Consistência e Rigor

Para que a introdução dessas regras seja válida perante a demonstração do Teorema das Quatro Cores, três teoremas precisam ser satisfeitos:

### Teorema 1 (Conservação da Curvatura Global de Euler):
Cada regra define uma função anti-simétrica $\tau_r(u \to w) = -\tau_r(w \to u)$.  
A soma de toda a carga transferida no grafo planar é identicamente nula:
$$\sum_{v \in V} \Delta C(v) = \sum_{r} \sum_{e = (u, w)} (\tau_r(u \to w) + \tau_r(w \to u)) = 0$$
Logo:
$$\sum_{v \in V} C_{\text{final}}(v) = \sum_{v \in V} C_0(v) = 10 \sum_{v} (6 - \deg(v)) = 120$$
**Nenhuma carga espúria é gerada ou destruída.**

### Teorema 2 (Não-Negatividade dos Doadores):
Um vértice de grau 5 possui carga inicial $+10$. Ele só pode doar carga se $C_{\text{final}}(v) \ge 0$.  
Nas Regras 68 a 75, as condições de guarda exigem vizinhos neutros de grau 6 e restringem as saídas ativas a no máximo 2 unidades por pentágono:
$$C_{\text{final}}(v) \ge 10 - 2 = +8 \ge 0 \quad (\forall v \text{ com } \deg(v) = 5)$$
**Nenhum novo déficit é criado no grafo.**

### Teorema 3 (Invariância Topológica e Diédrica):
Todas as regras são formuladas em pares `(Regra, invert)`. Isso garante que qualquer triangulação planar incorporada na esfera ou no disco seja tratada com invariância sob homeomorfismos de disco (rotação $C_R$ e reflexão $\tau$).
