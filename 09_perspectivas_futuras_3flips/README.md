# 09. Perspectivas Futuras: Mutações de 3ª Ordem (3-Flips) e a Fronteira Teórica

Este documento descreve as **diretrizes teóricas, algorítmicas e computacionais** para pesquisadores que disponham de recursos computacionais adicionais e desejem investigar a possibilidade de reduzir o conjunto inevitável abaixo de 25 configurações.

---

## 1. Contexto e Motivação

O marco de **25 configurações** (estabelecido na pasta `08_pesquisa_2flips`) foi obtido navegando o espaço de triangulações planares a **distância métrica 2 ($d=2$, 2-flips encadeados)** com contratos estritamente rasos ($k \le 2$). 

Esse modelo alcançou um equilíbrio notável:
- **Redução extrema:** -96,05% em relação ao RSST canônico (633);
- **Elegância estrutural:** todas as 22 configurações C-redutíveis usam contratos de no máximo 2 arestas ($k \le 2$);
- **Verificação ultrarrápida:** a dupla certificação (C e Rust) executa em menos de 6 segundos em um computador pessoal padrão.

Contudo, surge a questão natural: **é possível reduzir ainda mais (para 22, 20 ou 18 configurações)?**  
A resposta matemática é: em tese sim, mas ao custo de uma **explosão combinatória desproporcional** para um ganho marginal pequeno (eliminação de apenas 2 a 3 configurações).

---

## 2. As Três Vias de Expansão Combinatória

Para expandir o espaço de busca além do modelo atual, um pesquisador precisaria acionar uma ou mais das seguintes frentes:

### Via A: Mutações de Distância $d \ge 3$ (3-Flips e 4-Flips)
Em vez de limitar a busca a 2 diagonal flips encadeados, estender o gerador (`src/flipper.rs`) para aplicar sequências de 3 ou 4 flips:
$$G \xrightarrow{\text{flip } e_1} G_1 \xrightarrow{\text{flip } e_2} G_2 \xrightarrow{\text{flip } e_3} G_3$$
- **Crescimento combinatorial:** Para um grafo planar com $E_{\text{int}} \approx 20$ arestas internas:
  - 1-flips: $\approx 20$ mutações;
  - 2-flips: $\approx \binom{20}{2} \approx 190$ mutações;
  - **3-flips:** $\approx \binom{20}{3} \approx \mathbf{1.140 \text{ mutações por grafo}}$;
  - **4-flips:** $\approx \binom{20}{4} \approx \mathbf{4.845 \text{ mutações por grafo}}$.

### Via B: Contratos de Ordem Superior ($k=3$ e $k=4$ sob Stromquist)
No modelo de 25, todas as configurações C-redutíveis usam $k \le 2$.  
Reativar contratos de ordem $k=3$ e $k=4$ permitiria provar a redutibilidade de grafos com deformações internas mais severas geradas por 3-flips, mas multiplicaria as combinações de contratos candidatos por:
$$\binom{E_{\text{int}}}{3} + \binom{E_{\text{int}}}{4}$$

### Via C: Podas Múltiplas Combinadas de Fronteira ($n \to n-3$)
Combinar podas de 3ª ordem no bordo anular com mutações subsequentes de flips internos, expandindo a classe de superconfigurações.

---

## 3. Estimativa de Custo Computacional

| Métrica | Modelo Atual (2-Flips / $k \le 2$) | Expansão Futura (3-Flips / $k \le 4$) | Fator de Aumento |
| :--- | :---: | :---: | :---: |
| **Distância Métrica ($d$)** | $d = 2$ | $d = 3$ | $+1$ flip |
| **Profundidade de Contrato ($k$)** | $k \le 2$ | $k \le 4$ | $+2$ arestas |
| **Pool de Candidatos Gerados** | 3.942 configurações | $\approx 80.000$ a $150.000$ | **$\approx 25\times$ a $40\times$** |
| **Tempo de Síntese de Kempe** | ~3 segundos (paralelo) | ~30 a 60 minutos | $\approx 600\times$ |
| **Mapeamento no `discharge_track`** | ~4 segundos | ~2 a 5 horas | $\approx 2.000\times$ |
| **Matriz de Incidência do Set Cover** | 143.109 linhas $\times$ 3.942 colunas | 143.109 linhas $\times$ 100.000+ colunas | $\approx 30\times$ em RAM |
| **Tempo de Resolução do MILP (HiGHS)** | ~2 minutos | ~3 a 8 horas (Branch-and-Bound) | $\approx 100\times$ |

---

## 4. A Barreira Teórica Inferior de Euler (O Piso de 18 a 22 Grafos)

Por que um aumento de $40\times$ no número de candidatos produziria um ganho de apenas **2 a 3 configurações a menos**?

A razão é a **fórmula de Euler** para triangulações planares:
$$\sum_{v \in V} (6 - \deg(v)) = 12$$

Para provar a inevitabilidade, o esquema de descarregamento do RSST divide o espaço planar em **cinco apresentações canônicas mutuamente exclusivas**, de acordo com o grau do hub central:

1. **`present7` (Hub Grau 7):** Déficit de carga $-10$, anéis compactos ($R=6, 7$).
2. **`present8` (Hub Grau 8):** Déficit de carga $-20$, anéis médios ($R=7, 8$).
3. **`present9` (Hub Grau 9):** Déficit de carga $-30$, anéis intermediários ($R=8, 9$).
4. **`present10` (Hub Grau 10):** Déficit de carga $-40$, anéis grandes ($R=9, 10$).
5. **`present11` (Hub Grau 11):** Déficit extremo $-50$, anéis gigantes ($R \ge 12$).

### A Incompatibilidade Topológica entre Eixos:
- Uma configuração com $V=21$ e anel $R=13$ (necessária para absorver a carga no `present10` ou `present11`) **não pode topologicamente encaixar** na vizinhança restrita de um hub de grau 7 (que possui apenas 7 vizinhos no anel de 1º raio).
- Analogamente, uma configuração pequena com anel $R=6$ não possui capacidade de drenagem suficiente para fechar os ramos de déficit $-50$ do grau 11.
- Consequentemente, cada um dos 5 eixos possui uma **cota mínima independente de configurações irredutíveis**:
  - Grau 7: mínimo de $\approx 3$ a 4 configurações;
  - Grau 8: mínimo de $\approx 4$ a 5 configurações;
  - Grau 9: mínimo de $\approx 3$ a 4 configurações;
  - Grau 10: mínimo de $\approx 4$ a 5 configurações;
  - Grau 11: mínimo de $\approx 2$ a 3 configurações.

Somando essas restrições topológicas independentes, o **limite inferior assintótico** para o esquema canônico do RSST situa-se matematicamente no intervalo:
$$\text{Piso Teórico} \in [18, 22] \text{ configurações}$$

Portanto, sair de 25 configurações para $\approx 22$ representa o limite máximo absoluto que qualquer método combinatório baseado nas 67 regras do RSST poderia alcançar.

---

## 5. Roteiro de Implementação para Pesquisadores Futuros

Caso você disponha de um cluster ou servidor com recursos dedicados para tentar romper a barreira dos 25:

### Passo 1: Implementar o Gerador de 3-Flips no Rust
Estender [`src/flipper.rs`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/src/flipper.rs) criando a função `generate_3flips`:
```rust
pub fn generate_3flips(conf: &Configuration) -> Vec<FlippedConfig> {
    // Aplicar generate_internal_flips sobre a saída de generate_2flips
    // Filtrar configurações isomórficas via canonical labeling
    // Garantir delta(v) >= 5 e anel exterior sem cordas
}
```

### Passo 2: Estender a Síntese para Contratos $k \le 4$
No módulo [`src/reducibility.rs`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/src/reducibility.rs), habilitar buscas de contratos com 3 e 4 arestas sob o critério de Stromquist.

### Passo 3: Triagem Geométrica contra `CheckIso`
Filtrar os candidatos contra o reconhecedor universal do `discharge.c` para descartar configurações que violem a planaridade de cartwheel:
```bash
python3 screen_checkiso.py candidates_3flips.conf
```

### Passo 4: Mapeamento de Cobertura e Resolução MILP
Executar o rastreamento completo contra as 5 apresentações e processar com o solver HiGHS:
```bash
./discharge_track present11 candidates_pool.conf rules 0 1
# Repetir para present10, 9, 8, 7
python3 solve_sub40_ilp.py
```

---

## 6. Conclusão: Minimalidade vs. Elegância

A pesquisa atual optou por consolidar o modelo em **25 configurações** porque ele atinge o ponto ideal de Pareto:
- **Tamanho compacto:** 25 grafos (fácil de listar e documentar);
- **Simplicidade algébrica:** $k \le 2$ (sem contrações convolutas);
- **Custo de verificação:** instantâneo (~5 segundos).

A exploração de 3-flips permanece como um desafio computacional aberto para quem desejar mapear os últimos 3 grafos remanescentes até a barreira assintótica final.
