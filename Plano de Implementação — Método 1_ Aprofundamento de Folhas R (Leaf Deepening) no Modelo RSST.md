# **Plano de Implementação — Método 1: Aprofundamento de Folhas (Leaf Deepening) no Modelo RSST**

Este documento especifica o plano passo a passo para implementar o **Método 1 (Aprofundamento de Folhas `R` e `H` nas apresentações `present7` a `present11`)** no repositório [Kastikas/teoria-das-quatro-cores-25](https://github.com/Kastikas/teoria-das-quatro-cores-25), mantendo os verificadores em C ([01\_modelo\_629\_rsst\_canonico/discharge.c](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/01_modelo_629_rsst_canonico/discharge.c)) e Rust ([src/reducibility.rs](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/src/reducibility.rs)) 100% blindados sob as regras estritas de Robertson, Sanders, Seymour e Thomas (1997).

# **1\. Fundamentos Técnicos: Como `discharge.c` Executa `present7`–`present11`**

Para modificar as árvores de apresentação sem quebrar o verificador C canônico, o modelo codificador deve respeitar rigorosamente a semântica das 4 instruções lidas pelo laço principal de main() em [01\_modelo\_629\_rsst\_canonico/discharge.c](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/01_modelo_629_rsst_canonico/discharge.c) (linhas 240–295):

| Instrução no Arquivo present\* | Função Chamada em discharge.c | Efeito Exato na Pilha de Eixos (axles\[lev\]) e no Nível (lev) |
| :---- | :---- | :---- |
| `L<lev> C <n> <m>` | `CheckCondition` (linhas 355–420) | Copia `axles[lev]` para `axles[lev + 1]`. Se `m > 0`, define `axles[lev].upp[n] = m - 1` (ramo visitado **depois**) e `axles[lev + 1].low[n] = m` (ramo filho visitado **imediatamente na próxima linha**). Se `m < 0`, define `axles[lev].low[n] = 1 - m` (visitado **depois**) e `axles[lev + 1].upp[n] = -m` (visitado **na próxima linha**). Incrementa `lev++` |
| `L<lev> R` | `Reduce` (linhas 831–920) | Folha de redutibilidade explícita. Empilha `axles[lev]` em `Astack` e exige que exista uma configuração no arquivo `.conf` que seja subconfiguração induzida bem-posicionada (`SubConf` \+ `CheckIso`). Para cada vértice interno `v` da configuração com `low[v] < upp[v]`, empilha um sub-eixo com `upp[v] - 1`. Ao concluir, decrementa `lev--` |
| `L<lev> H (x1,y1,v1) ...` | `CheckHubcap` \+ `CheckBound` (linhas 480–730) | Folha de descarregamento (*hubcap*). Verifica que a cobertura dupla dos raios satisfaz a soma total `<= 20 * (deg - 6) + 1`. Para cada tupla `(x, y, v)`, ramifica recursivamente nos *outlets* positivos (`PO->T->value > 0`); sempre que a carga forçada ultrapassa `v` (`forcedch > maxch`), **chama `Reduce(AA, lineno, ...)` internamente** para provar que aquele subcaso é redutível. Ao concluir, decrementa `lev--` |
| `L<lev> S <k> <eps> <level> <line>` | `CheckSymmetry` (linhas 422–455) | Folha de simetria. Verifica se o eixo atual `axles[lev]` é coberto por rotação `k` e reflexão `eps` (0 ou 1\) pelo estado salvo na **linha `line` (1-indexada)** do próprio arquivo `present*`. Ao concluir, decrementa `lev--` |

# **2\. Levantamento Experimental de Uso das 629 Configurações Canônicas**

Executando o verificador estrito sobre [01\_modelo\_629\_rsst\_canonico/unavoidable\_629.conf](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/01_modelo_629_rsst_canonico/unavoidable_629.conf) e mapeando cada chamada de `CheckIso` às linhas de [present7](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/10_recorde_21_configuracoes/present7) a [present11](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/10_recorde_21_configuracoes/present11), obtemos a distribuição exata de raridade das 629 configurações:

* **33 configurações** são utilizadas em **apenas 1 única linha** em todas as 5 apresentações (sendo **7** delas chamadas exclusivamente em 1 linha `R` explícita e **26** chamadas exclusivamente dentro de `CheckBound` em 1 linha `H`).  
* **27 configurações** são utilizadas em **apenas 2 linhas**.  
* **23 configurações** são utilizadas em **apenas 3 linhas**.  
* No total, **83 configurações** ocorrem em `<= 3` linhas e **134 configurações** ocorrem em `<= 5` linhas.

## **Lista das 7 Configurações Acionadas em Apenas 1 Linha `R` Explícita (e 0 Linhas `H`)**

| Índice (0..628) | Nome da Configuração | Apresentação | Linha Exata (1-indexada) | Instrução Atual |
| :---- | :---- | :---- | :---- | :---- |
| **\#67** | `186.1307280` | `present7` | Linha `2518` | `L4 R` |
| **\#95** | `122.79077966` | `present7` | Linha `767` | `L3 R` |
| **\#309** | `184.3701176` | `present7` | Linha `908` | `L4 R` |
| **\#323** | `7326.122` | `present8` | Linha `407` | `L5 R` |
| **\#327** | `7326.442802` | `present8` | Linha `413` | `L7 R` |
| **\#563** | `439320.439322` | `present9` | Linha `139` | `L7 R` |
| **\#566** | `439326.122` | `present9` | Linha `324` | `L6 R` |

## **Exemplos de Configurações Acionadas em Apenas 1 Linha `H` (via `CheckBound`)**

* **\#40 (`182.1307280`)**: `present7`, linha `411` (3 sub-eixos)  
* **\#41 (`182.1310580`)**: `present7`, linha `364` (1 sub-eixo, quando o *outlet* `7,7` fixa `14:6` no membro `(7,7,1)`)  
* **\#66 (`184.1306806`)**: `present7`, linha `608` (1 sub-eixo)  
* **\#84 (`184.1324806`)**: `present7`, linha `644` (1 sub-eixo)  
* **\#89 (`184.1310466`)**: `present7`, linha `607` (1 sub-eixo)  
* **\#129 (`182.3683280`)**: `present7`, linha `431` (1 sub-eixo)  
* **\#166 (`182.3701280`)**: `present7`, linha `896` (1 sub-eixo)  
* **\#234 (`483.3927606`)**: `present7`, linha `4442` (1 sub-eixo)

# **3\. Passo a Passo de Implementação para o Modelo Codificador**

## **Tarefa 1: Corrigir o Buffer `fname[20]` em `discharge.c` e Criar o Mapeador de Linhas (`scripts/map_rline_usage.py`)**

1. **Correção de Buffer Overflow em `discharge.c`**:  
   * Em [01\_modelo\_629\_rsst\_canonico/discharge.c](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/01_modelo_629_rsst_canonico/discharge.c) (linha 163), a variável `char fname[20];` sofre *buffer overflow* se receber um caminho com diretório (como `"01_modelo_629_rsst_canonico/present7"`, que tem 36 caracteres).  
   * Altere `char fname[20];` para `char fname[256];` em todos os arquivos `discharge*.c`.  
2. **Script de Mapeamento de Dependências (`scripts/map_rline_usage.py`)**:  
   * Crie `scripts/map_rline_usage.py` que compila uma versão instrumentada de `discharge.c` onde `Reduce` imprime, para cada sub-eixo `B` processado:  
     * O arquivo `present*` e o número da linha `lineno`;  
     * Se a chamada veio de uma linha `R` explícita ou de `CheckBound` (linha `H`);  
     * A lista de **todas** as configurações válidas (tanto as 629 canônicas quanto as candidatas de *flips*/podas que passam nas travas estritas de `check_rsst_admissibility`, `validate_sparse_contract` e `GetQuestion`) que casam com aquele sub-eixo `B`.

## **Tarefa 2: Implementar o Motor de Inserção com Reindexação Automática de Simetrias (`scripts/presentation_editor.py`)**

**Regra Crítica**: Em `CheckSymmetry` ([01\_modelo\_629\_rsst\_canonico/discharge.c](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/01_modelo_629_rsst_canonico/discharge.c) linha 438), toda linha `L<lev> S <k> <eps> <level> <target_line>` busca na tabela `sym[]` uma condição `C` cujo número de linha original no arquivo era exatamente `target_line` (`T->number == line`).  
Portanto, se você substituir a linha `target_lineno` (1-indexada) por um bloco de `len(new_block)` linhas (adicionando `delta = len(new_block) - 1` linhas ao arquivo), **todas as instruções `S` no arquivo cujo `target_line > target_lineno` devem ter `target_line` incrementado em `+delta`**.

Implemente em `scripts/presentation_editor.py` a função abaixo:import re

def splice\_presentation\_lines(lines: list\[str\], target\_lineno: int, new\_block: list\[str\]) \-\> list\[str\]:

    """

    Substitui a linha 1-indexada \`target\_lineno\` por \`new\_block\` e atualiza

    automaticamente todas as referências de número de linha nas instruções 'S'.

    """

    delta \= len(new\_block) \- 1

    updated\_lines \= \[\]

    sym\_pattern \= re.compile(r"^(\\s\*L\\d+\\s+S\\s+\\d+\\s+\\d+\\s+\\d+\\s+)(\\d+)(\\s\*)\$")

    for idx, line in enumerate(lines, start=1):

        if idx \== target\_lineno:

            updated\_lines.extend(new\_block)

            continue

        m \= sym\_pattern.match(line.rstrip("\\r\\n"))

        if m:

            ref\_line \= int(m.group(2))

            if ref\_line \> target\_lineno:

                line \= f"{m.group(1)}{ref\_line \+ delta}\\n"

        updated\_lines.append(line)

    return updated\_lines

## **Tarefa 3: Implementar os Dois Modos de Aprofundamento de Folhas (`scripts/deepen_leaves.py`)**

Como as 629 configurações canônicas são minimalmente necessárias quando usadas isoladamente sobre present7–present11 estáticas, o script scripts/deepen\_leaves.py deve operar combinando o aprofundamento de folhas com o pool de candidatas **estritamente válidas** (as 281 configurações de flips e as 14 podas prune\_ear que passam 100% nas travas do `discharge.c` original de 1997 e no `reducibility.rs` corrigido):

### **Modo A — Absorção 2-por-1 via Candidata Estritamente Válida \+ Aprofundamento de Folha (`Leaf Deepening`)**

1. No mapeamento gerado na Tarefa 1, identifique uma configuração candidata estritamente válida `F` (por exemplo, uma das 84 configurações de *flip* D-redutíveis com `k = 0`, ou uma das 201 C-redutíveis com contrato esparso sem auto-laço) que:  
   * Já absorve 100% dos eixos de uma configuração canônica `K_1`; e  
   * Possui sobreposição parcial com uma segunda configuração canônica rara `K_2` (ou seja, `F` compartilha o mesmo núcleo de vértices internos de `K_2`, mas na folha `lineno` onde `K_2` é usada em `present*`, 1 ou 2 vértices `v` que pertencem a `F` ainda estão com grau aberto `[low[v], upp[v]]` no *cartwheel*).  
2. Na linha `lineno` (de nível `L<lev>`) onde `K_2` era acionada, substitua a folha `L<lev> R` por uma quebra de condição `C` sobre o vértice `v` necessário para `F`:

L\<lev\> C \<v\> \<m\>

  L\<lev+1\> R

L\<lev\> \<fechamento\_do\_ramo\_complementar\>

onde:

* O ramo filho `L<lev+1> R` fixa o grau de `v` no valor exigido por `F` (portanto fecha imediatamente usando `F`, sem precisar de `K_2`\!).  
* O ramo complementar `L<lev>` (onde `v` tem grau maior ou menor) é fechado testando automaticamente, nesta ordem:  
  1. `L<lev> R` (caso outra configuração já presente no catálogo cubra o ramo complementar);  
  2. `L<lev> S <k> <eps> <level> <line>` (testando se o ramo complementar é simétrico a algum ancestral na tabela `sym[]`);  
  3. `L<lev> H ...` (gerando uma linha de *hubcap* válida onde o aumento de grau de `v` reduziu a carga forçada `forcedch <= maxch`);  
  4. Uma segunda quebra `L<lev> C <v2> <m2>` (até profundidade máxima de 2 níveis extras).  
3. Se todos os sub-ramos fecharem sem usar `K_1` nem `K_2`, substitua o par `(K_1, K_2)` por `F` no catálogo `.conf` (obtendo uma redução líquida real de `-1` configuração no tamanho total do catálogo\!) e valide imediatamente as 5 apresentações no `discharge.c` canônico.

### **Modo B — Eliminação de Configurações Especialistas de Linhas `H` via Refinamento de Hubcap**

Para as 26 configurações canônicas que são acionadas em **apenas 1 única linha `H`** (dentro de `CheckBound`, quando `forcedch > maxch` para uma tupla (x, x, v)):

1. Identifique qual raio `x` e qual *outlet* `PO->T` disparou a chamada `Reduce(AA, lineno, ...)` que exigiu a configuração rara `K_r` (usando o modo de rastreio `prtline = lineno, printmode = 4` do `discharge.c`).  
2. Antes da linha `L<lev> H ...`, insira uma quebra `L<lev> C <n> <m>` sobre um vértice `n` do *outlet* `PO->T` (ou ajuste as tuplas `(x, y, v)` da cobertura dupla na linha `H` respeitando o limite `total <= 20 * (deg - 6) + 1`):  
   * No ramo onde o *outlet* positivo não se aplica mais, a linha `H` original passa sem chamar `Reduce` para `K_r`.  
   * No ramo onde os vértices de grau 5 do *outlet* estão fixados, teste se um vértice vizinho de grau 5 forma uma configuração pequena já existente no catálogo (como `0.7322`, `2.122` ou `2.126`).

## **Tarefa 4: Validação de Aceite Ponta a Ponta (`verify_strict.sh`)**

Crie um script `verify_strict.sh` que execute obrigatoriamente as 3 verificações abaixo a cada redução aceita pelo `scripts/deepen_leaves.py`:

1. **Validação Geométrica e Algébrica em Rust**:

cargo test \--release

cargo run \--release \-- verify-file \<novo\_catalogo.conf\> 1000

Exigindo que 100% das configurações do novo catálogo passem em check\_rsst\_admissibility (com `k <= 2` na Condição 6), `validate_sparse_contract` (sem duas arestas contraídas no mesmo triângulo, inclusive triângulos com aresta no anel c \<= ring) e test\_configuration (DReducible ou CReducible).

2. **Validação de Descarregamento no Verificador C Canônico de 1997**:

gcc \-O3 01\_modelo\_629\_rsst\_canonico/discharge.c \-o /tmp/discharge\_1997

for p in present7 present8 present9 present10 present11; do

    /tmp/discharge\_1997 "\$p" \<novo\_catalogo.conf\> rules 0 0

done

Exigindo código de saída `0` (`<present> verified.`) em todas as 5 apresentações, sem alterar nenhuma regra de `CheckIso`, `GetQuestion` ou ReadConf do programa de 1997\.