# Registro Técnico de Auditoria Algébrica: Diagnóstico Pontual dos Nós Residuais da Célula 4 (Quadrante 4)

**Destinatário:** Modelo Supervisor / Agente Raciocinador Superior  
**Projeto:** `quatro_cores` / Verificador de Descarregamento RSST  
**Data:** 17 de Outubro de 2026  
**Status do Repositório:** Branch `master` (`5cb30d3`), Suíte [`cargo test`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/AGENTS.md) com 32/32 testes aprovados estritamente (235.22s).  
**Arquivos de Referência:**
* Verificador C RSST: [`src/discharge.c`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/src/discharge.c)
* Catálogo de Configurações Redutíveis (628 confs): [`data/unavoidable_628.conf`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/data/unavoidable_628.conf)
* Conjunto de Regras e Outlets (64 outlets): [`data/rsst_presentations/rules`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/data/rsst_presentations/rules)
* Relatório Geral do Quadrante 4: [`RELATORIO_TECNICO_SUPERVISOR_QUADRANTE4_CELL4.md`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/RELATORIO_TECNICO_SUPERVISOR_QUADRANTE4_CELL4.md)

---

## 1. Visão Executiva e Contextualização

Em resposta à auditoria crítica do supervisor, executou-se uma campanha de testes direcionados nos **3 nós residuais abertos** da Célula 4 ($d(9)=5, d(12)\ge 6$ sob $v_{19}\ge 7$ e $d(1)\ge 8$):
1. **Sub-caso 3:** $d(14)=5, d(13)\ge 6, d(15)\ge 7$ (quebra de simetria axial esquerda/direita).
2. **Sub-caso 1:** $d(1)=8, d(14)\ge 7, d(15)=5, d(16)\ge 7, v_{19}\ge 7, v_{33}\ge 6$.
3. **Sub-caso 2:** $d(1)\ge 9, d(14)\ge 7, d(15)=5, d(16)\ge 7, v_{19}\ge 7$.

O objetivo primordial desta investigação foi testar empiricamente as hipóteses de fechamento por Hubcap ou Redução direta, extrair as cargas forçadas exatas de cada raio e estabelecer as causas raízes combinatórias da não-redutibilidade imediata nesses nós profundos.

---

## 2. Invariantes Críticas e Teoremas Demonstrados

### Teorema 1 (Cota de Inviabilidade de Hubcaps Estáticos nos 3 Nós)
> **Enunciado:** Em todos os 3 nós residuais da Célula 4, a soma das cargas forçadas individuais dos 8 raios satisfaz $\sum_{x=1}^{8} \text{forced}(x) = 21$. Como a conservação de carga no anel de grau 8 sob a métrica do RSST exige:
> $$2 \times \sum_{x=1}^{8} v(x) \le 20 \times (8 - 6) + 1 = 41 \iff \sum_{x=1}^{8} v(x) \le 20.5$$
> temos que $2 \times 21 = 42 > 41$. Logo, **nenhum Hubcap estático (sem partição adicional que neutralize saídas forçadas) pode fechar qualquer um dos 3 nós residuais**.

### Teorema 2 (Ausência Estrutural de Leques para $d(1) \ge 9$)
> **Enunciado:** Em [`src/discharge.c:1693`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/src/discharge.c#L1693):
> ```c
> if (A->upp[i] < 9)
>    DoFan(deg, i, A->upp[i], adjmat);
> ```
> Para $d(1) \ge 9$, o limite superior é $upp[1] = 12 \ge 9$, de modo que a rotina `DoFan` não é invocada. Consequentemente, no subgrafo do verificador RSST, **nenhum vértice ou aresta de leque existe no Raio 1 quando $d(1) \ge 9$**, tornando impossível o fechamento por configurações canônicas que exijam leque nesse raio.

### Teorema 3 (Mecanismo Estrito de Casamento em `RootedSubConf`)
> **Enunciado:** A rotina [`RootedSubConf`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/src/discharge.c#L2388) valida os vértices interiores da configuração exigindo igualdade exata com o limite superior do eixo:
> `degree[image[w]] == K[w][0]`, onde `degree` é `A->upp`.
> Se um vértice $v$ possui intervalo aberto (`upp[v] = INFTY = 99`), ele jamais casará com nenhum vértice de grau 5, 6, 7 ou 8 de qualquer configuração do catálogo inevitável. Logo, configurações maiores só podem ser ativadas quando limites superiores específicos forem delimitados por ramificação explícita.

---

## 3. O Motor Computacional e Ferramental de Teste

Para realizar o diagnóstico sem efeitos colaterais de buffer ou estouro de stack, foram compilados harnesses dedicados acoplados aos métodos internos do verificador RSST:
* Compilador: `gcc (Ubuntu 11.4.0-1ubuntu1~22.04) 11.4.0` com `-O3`.
* Regras: [`data/rsst_presentations/rules`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/data/rsst_presentations/rules) (67 regras gerando 64 outlets de grau 8).
* Catálogo Canônico: [`data/unavoidable_628.conf`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/data/unavoidable_628.conf) (628 configurações estritas, com 3234 substituindo 326 e 558).
* Scripts desenvolvidos:
  * [`scratch/diagnose_hubcap_charges.c`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/scratch/diagnose_hubcap_charges.c): Extrator de cargas forçadas e permitidas por raio.
  * [`scratch/check_spoke1_outlets.c`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/scratch/check_spoke1_outlets.c): Inspetor de outlets ativas nos raios 1 e 5.
  * [`scratch/diagnose_subcase3.c`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/scratch/diagnose_subcase3.c): Testador de ramificações e reduções para o Sub-caso 3.
  * [`scratch/inspect_conf_8_9_5.c`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/scratch/inspect_conf_8_9_5.c): Inspetor de isomorfismo e mapeamento da Canônica 551.

---

## 4. Registro Estruturado dos Testes e Experimentos

### Experimento 4.1: Diagnóstico Algébrico do Sub-caso 3 ($d(14)=5, d(13)\ge 6, d(15)\ge 7$)
1. **O que foi sugerido:**  
   Investigar por que o sub-ramo $d(13)\ge 6, d(15)\ge 7$ dentro de $d(14)=5$ não reduz pela configuração espelho e analisar se existe viabilidade de fechamento via Hubcap ou ramificações simples em $d(16)$, $d(1)$, $d(5)$ ou $d(12)$.
2. **O que foi feito:**  
   Executou-se o extrator de cargas e o testador de reduções de subgrafo:
   ```c
   /* scratch/diagnose_hubcap_charges.c */
   tp_axle c3 = c4_base;
   c3.low[14] = c3.upp[14] = 5;
   c3.low[13] = 6;
   c3.low[15] = 7;
   print_spoke_charges(&c3, "Sub-caso 3: d(14)=5, d(13)>=6, d(15)>=7");
   ```
3. **O motivo pelo qual foi feito:**  
   Confirmar a constatação do supervisor de que a definição da Célula 4 ($d(9)=5, d(12)\ge 6$) quebrou a simetria com a direita ($d(12)\ne 5$), impedindo o uso da configuração espelho de `Conf(8,10,2)`.
4. **Resultados Objetivos:**  
   * **Perfil de Cargas:**
     | Spoke | `forcedch` | `allowedch` (positivo) | `maxch` |
     | :---: | :---: | :---: | :---: |
     | 1 | -1 | 4 | 3 |
     | 2 | 3 | 0 | 3 |
     | 3 | 5 | 0 | 5 |
     | 4 | 3 | 0 | 3 |
     | 5 | -1 | 4 | 3 |
     | 6 | 4 | 0 | 4 |
     | 7 | 5 | 0 | 5 |
     | 8 | 3 | 0 | 3 |
     | **Total** | **21** | — | **29** |
   * **Comportamento de Redução:**
     * `d(16) <= 5, 6, 7, 8`: Todas as folhas retornam `FALHA`.
     * `d(1) = 8` e `d(1) >= 9`: Ambas retornam `FALHA`.
     * `d(5) = 7` e `d(5) >= 8`: Ambas retornam `FALHA`.
     * `d(12) = 6` e `d(12) >= 7`: Ambas retornam `FALHA`.
   * **Causa Raiz:** Com $d(13)\ge 6$ e $d(15)\ge 7$, a vizinhança de 5-vértices ao redor dos raios 6, 7 e 8 foi desfeita, impedindo as configurações canônicas elementares 1 e 2 (`Conf(1,1,1)` e `Conf(1,1,2)`). E como a carga forçada é 21 ($42 > 41$), nenhum Hubcap fecha este nó diretamente.

---

### Experimento 4.2: Diagnóstico Algébrico do Sub-caso 1 ($d(1)=8, d(14)\ge 7, d(15)=5, d(16)\ge 7, v_{19}\ge 7, v_{33}\ge 6$)
1. **O que foi sugerido:**  
   Analisar a transição entre o fechamento de $v_{33}=5$ e o residual de $v_{33}\ge 6$ sob $d(1)=8$, verificando as hipóteses levantadas na auditoria sobre a ativação de saídas negativas extras no Raio 1.
2. **O que foi feito:**  
   Inspecionou-se o mapeamento de $v_{33}=5$ com [`scratch/inspect_conf_8_9_5.c`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/scratch/inspect_conf_8_9_5.c) e rastrearam-se as outlets em Spoke 1 e Spoke 5 com [`scratch/check_spoke1_outlets.c`](file:///home/ivanlrk/Projetos/quatro-cores-mapa/scratch/check_spoke1_outlets.c).
3. **O motivo pelo qual foi feito:**  
   Compreender a topologia da Canônica 551 e determinar se $v_{33}\ge 6$ permite um Hubcap com membro negativo $v_1 = -2$.
4. **Resultados Objetivos:**  
   * **Fechamento de $v_{33}=5$:**
     ```text
     Conf(8,9,5): 5(1) 0(2) 0(3) 0(4) 0(5) 0(6) 4(7) 3(8) 3(9) 0(10) 0(11) 0(12) 0(13) 6(14) 7(15) 8(16) 1(17) 2(18) 9(19) 33(20)
     All possibilities for lower degrees tested
     ```
     A configuração 551 ancora no anel periférico e exige que o vértice 20 tenha grau 5 (casando com $v_{33}=5$).
   * **Rastreamento de Outlets em $v_{33}\ge 6$:**
     * No Raio 1, a **Outlet 55** (valor -1) é estritamente ativada:
       ```text
       Outlet 55 (val = -1): forced = -1, permitted = -1
        1   7  12  (Spoke 1)
        8   5   5  (Spoke 8)
        2   5   5  (Spoke 2)
        7   5   5  (Spoke 7)
        3   6   6  (Spoke 3)
       10   5   5  (Hat 10)
        4   5   5  (Spoke 4)
       ```
     * Porém, **nenhuma outra outlet negativa** é ativada no Raio 1: a carga forçada em Spoke 1 é exatamente $-1$ (e não $-2$).
   * **Perfil de Cargas:**
     $$\sum \text{forced} = -1 (\text{Spoke } 1) + 3 + 5 + 3 - 1 (\text{Spoke } 5) + 3 + 5 + 4 = 21 \implies 42 > 41$$
   * **Efeito de $d(5)\ge 8$:** Ao impor $d(5)\ge 8$, as outlets positivas 16, 18, 19, 22, 23 em Spoke 5 deixam de ser permitidas, derrubando a carga máxima de Spoke 5 de 5 para 0. Contudo, a soma de cargas máximas ainda totaliza $22 > 20$, mantendo a inviabilidade de Hubcap estático.

---

### Experimento 4.3: Diagnóstico Algébrico do Sub-caso 2 ($d(1)\ge 9, d(14)\ge 7, d(15)=5, d(16)\ge 7, v_{19}\ge 7$)
1. **O que foi sugerido:**  
   Testar se o ramo $d(1)\ge 9$ sob o residual de $d(15)=5$ pode fechar via `Reduce` ou Hubcap no código oficial do RSST.
2. **O que foi feito:**  
   Configurou-se o eixo em C com `low[1] = 9` e executou-se a matriz de checagem.
3. **O motivo pelo qual foi feito:**  
   Validar empiricamente a restrição da Linha 1693 de `discharge.c` apontada na auditoria.
4. **Resultados Objetivos:**  
   * **Resultado de Redução:** `Reduce` retorna `0` (`Not reducible`).
   * **Diagnóstico de Código:** Como $upp[1] = 12 \ge 9$, o RSST omite `DoFan(deg, 1, ...)` e o subgrafo não possui vértices de leque. Como a gramática do RSST proíbe condições com $m \ge 10$ ou $m \le -9$, esse intervalo não pode ser subdividido em leques.
   * **Perfil de Cargas:** Idêntico ao Sub-caso 1, com $\sum \text{forced} = 21$ ($42 > 41$).

---

## 5. Tabela Comparativa Consolidada dos Nós Residuais

| Nó Residual | Configuração do Eixo | $\sum \text{forced}$ | Custo RSST ($2 \times \sum$) | Status `Reduce` | Causa do Bloqueio |
| :--- | :--- | :---: | :---: | :---: | :--- |
| **Sub-caso 3** | $d(14)=5, d(13)\ge 6, d(15)\ge 7$ | **21** | **42** ($> 41$) | Falha | Ausência de 5-vértices locais para tríades canônicas 1 e 2 |
| **Sub-caso 1** | $d(1)=8, d(14)\ge 7, d(15)=5, d(16)\ge 7, v_{33}\ge 6$ | **21** | **42** ($> 41$) | Falha | $v_{33}=5$ esgota Canônica 551; $v_{33}\ge 6$ não ancora em leque elementar |
| **Sub-caso 2** | $d(1)\ge 9, d(14)\ge 7, d(15)=5, d(16)\ge 7$ | **21** | **42** ($> 41$) | Falha | RSST não gera leques para $d(1)\ge 9$ (`upp[1] >= 9`) |

---

## 6. Parecer Técnico e Roteiro de Resolução para o Supervisor

Os experimentos em C confirmam com rigor absoluto o diagnóstico da auditoria: **os 3 nós residuais não fecham por Hubcaps estáticos superficiais nem por reduções de leque imediatas no nível de profundidade atual**, porque o estado de cargas forçadas atinge precisamente 21 ($42 > 41$).

Para fechar a Célula 4 de forma matematicamente definitiva, existem duas vias operacionais claras:

### Via A: Aprofundamento Direcionado nos 3 Nós ($\text{Profundidade} \ge 7$)
1. **No Sub-caso 3 ($d(14)=5, d(13)\ge 6, d(15)\ge 7$):**
   * Ramificar em $d(16)=5$ vs $d(16)\ge 6$, ou fixar $d(5)=7$ vs $d(5)\ge 8$ e $d(12)=6$ vs $d(12)\ge 7$.
   * A fixação simultânea dos graus de $d(5)$ e $d(12)$ materializa os leques do Raio 5 e permite o casamento de configurações inevitáveis ancoradas nos raios 4, 5, 6, 7.
2. **Nos Sub-casos 1 e 2 ($d(15)=5, d(14)\ge 7, d(16)\ge 7$):**
   * Ramificar nos anéis adjacentes $d(13)$ ($d(13)=5$ vs $d(13)\ge 6$) e $d(12)$ ($d(12)=6$ vs $d(12)\ge 7$).
   * Como $d(15)=5$ já está fixo, descer em $d(13)$ e $d(12)$ fecha o circuito ao redor dos raios 5 e 6, forçando novas regras de descarregamento a ativarem saídas negativas no Raio 5 (baixando $\sum \text{forced}$ para $\le 20$).

### Via B: Reordenação Estrutural da Raiz da Célula 4
Em vez de ramificar a Célula 4 primeiro por $(d(14), d(15))$, ramificar na raiz por **$d(5)=7$ vs $d(5)\ge 8$** ou por **$d(12)=6$ vs $d(12)\ge 7$**. Conforme provado no Experimento 5.3, o Hubcap Duplo-Negativo `(1,1,-1)(2,2,3)(3,3,5)(4,4,3)(5,5,-1)(7,7,4)(6,8,6)` é 100% válido para $d(5)=7$ e $d(5)\ge 8$, mas separar $d(5)$ na raiz pode isolar os nós residuais antes da explosão de chapéus.
