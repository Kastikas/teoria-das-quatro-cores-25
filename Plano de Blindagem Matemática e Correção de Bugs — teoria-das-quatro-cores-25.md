# **Plano de Blindagem Matemática e Correção de Bugs — teoria-das-quatro-cores-25**

Este documento contém o diagnóstico exato de linhas de código (obtido a partir da auditoria do repositório [Kastikas/teoria-das-quatro-cores-25](https://github.com/Kastikas/teoria-das-quatro-cores-25)) e o passo a passo determinístico para um modelo codificador corrigir as brechas matemáticas do projeto.

# **Regras Estritas de Execução para o Modelo Codificador**

1. **Proibição de Atalhos**: Nunca afrouxe, remova ou comente uma verificação matemática para fazer um catálogo existente passar nos testes. Se um arquivo `.conf` falhar nas verificações abaixo, o comportamento correto é rejeitar o arquivo `.conf` com erro explícito.  
2. **Ordem de Execução**: Implemente as 4 tarefas na sequência exata descrita abaixo, rodando `cargo test` após cada tarefa antes de avançar para a seguinte.  
3. **Critério de Ouro (Teorema de Birkhoff / RSST)**:  
   * O catálogo canônico [01\_modelo\_629\_rsst\_canonico/unavoidable\_629.conf](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/01_modelo_629_rsst_canonico/unavoidable_629.conf) DEVE PASSAR em 100% das verificações.  
   * As configurações degeneradas `p_0.7322_3` (N \= 9, R \= 6, 3 vértices internos de grau 5\) e `p_122.122_e2` (N \= 12, R \= 8, 4 vértices internos de grau 5\) presentes em [08\_pesquisa\_2flips/unavoidable\_25.conf](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/08_pesquisa_2flips/unavoidable_25.conf) e [10\_recorde\_21\_configuracoes/unavoidable\_21.conf](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/10_recorde_21_configuracoes/unavoidable_21.conf) DEVEM SER REJEITADAS automaticamente pelo verificador.

# **Resumo dos 4 Bugs Matemáticos Encontrados no Código-Fonte**

| \# | Arquivo e Linhas Exatas | O que o Código Faz Hoje | Efeito Colateral Matemático |
| :---- | :---- | :---- | :---- |
| **1** | [src/graph.rs](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/src/graph.rs) (linhas 473–480 e 569–571), [src/fusion.rs](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/src/fusion.rs) (linhas 74–87) e [src/reducibility.rs](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/src/reducibility.rs) (linha 127\) | Ao checar se duas arestas contraídas `a` e `b` pertencem ao mesmo triângulo `(a, b, c)`, exige `c > ring`. Além disso, `test_configuration` não chama `validate_sparse_contract` | Permite contrair os dois lados internos de um triângulo que apoia uma aresta do anel `c <= ring` (ex.: `(5,9)` e `(6,9)` sobre a aresta `(5,6)` em `p_0.7322_3`). Isso identifica dois vértices adjacentes do anel, cria um auto-laço (*self-loop*) e faz `check_contract` retornar `true` para qualquer grafo (19/21 no catálogo de 21 e 21/25 no catálogo de 25\) |
| **2** | [src/graph.rs](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/src/graph.rs) (linha 158), [08\_pesquisa\_2flips/discharge.c](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/08_pesquisa_2flips/discharge.c) (linha 1415\) e [10\_recorde\_21\_configuracoes/discharge\_universal.c](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/10_recorde_21_configuracoes/discharge_universal.c) (linha 1603\) | Altera a Condição 6 de `ReadConf` do RSST de `if (k > 2)` para `if (k > 4)` | Permite vértices internos de articulação (*cut-vertices*) que tocam o anel externo em dois intervalos desconexos e estrangulam a ilha interna |
| **3** | [08\_pesquisa\_2flips/discharge.c](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/08_pesquisa_2flips/discharge.c) (linha 1716\) e [10\_recorde\_21\_configuracoes/discharge\_universal.c](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/10_recorde_21_configuracoes/discharge_universal.c) (linhas 1344 e 1907\) | Substitui o erro fatal `printf("Error in getquestions\n"); exit(1);` por `continue;` dentro de `GetQuestion` e `GetQuestionPair` | Quando uma configuração viola a Condição 6, o algoritmo pula vértices do anel no meio do leque e gera perguntas de *peeling* corrompidas ou incompletas |
| **4** | [08\_pesquisa\_2flips/discharge.c](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/08_pesquisa_2flips/discharge.c) (linha 997\) e [10\_recorde\_21\_configuracoes/discharge\_universal.c](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/10_recorde_21_configuracoes/discharge_universal.c) (linhas 1011 e 2036\) | Enfraquece `if (worried) Error("Isomorphism error 5")` em `CheckIso` e remove `image[j] = -1` em `RootedSubConf` | Aceita arestas internas que não pertencem a triângulos internos da configuração e deixa lixo de memória no vetor `image` |

# **Instruções Passo a Passo para o Modelo Codificador**

## **Tarefa 1: Corrigir a Verificação de Contratos (C-Redutibilidade) em `src/graph.rs`, `src/fusion.rs` e `src/reducibility.rs`**

1. Abra [src/graph.rs](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/src/graph.rs) na função `find_angles` (linhas 473–480) e substitua o bloco:

if a \> 0

    && b \> 0

    && angles.contract\[a\] \!= 0

    && angles.contract\[b\] \!= 0

    && c \> conf.ring()

{

    is\_sparse \= false;

}

por uma verificação completa que proíba quaisquer duas arestas contraídas no mesmo triângulo (mesmo quando a terceira aresta pertence ao anel `1..=ring`):let ca \= if a \> 0 && a \<= EDGES { angles.contract\[a\] \!= 0 } else { false };

let cb \= if b \> 0 && b \<= EDGES { angles.contract\[b\] \!= 0 } else { false };

let cc \= if c \> 0 && c \<= EDGES { angles.contract\[c\] \!= 0 } else { false };

if (ca && cb) || (ca && cc) || (cb && cc) {

    is\_sparse \= false;

}

2. Ainda em [src/graph.rs](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/src/graph.rs), na função `build_contract_angles` (linhas 568–572), faça a mesma substituição no laço `for &Triangle { a, b, c } in triangles`:

let ca \= if a \> 0 && a \<= EDGES { angles.contract\[a\] \!= 0 } else { false };

let cb \= if b \> 0 && b \<= EDGES { angles.contract\[b\] \!= 0 } else { false };

let cc \= if c \> 0 && c \<= EDGES { angles.contract\[c\] \!= 0 } else { false };

if (ca && cb) || (ca && cc) || (cb && cc) {

    angles.is\_sparse \= false;

}

3. Ainda em [src/graph.rs](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/src/graph.rs), na função `validate_sparse_contract` (linhas 669–684), adicione uma verificação de que as arestas contraídas não conectam dois vértices adjacentes do anel `1..=ring` (evitando a criação de auto-laços ao contrair componentes conexas do contrato).  
4. Abra [src/fusion.rs](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/src/fusion.rs) na função `synthesize_contract_with_live` (linhas 74–88) e remova as condições `c > ring`, `b > ring` e `a > ring` ao construir a matriz `conflicts`:

let mut conflicts \= \[\[false; EDGES \+ 1\]; EDGES \+ 1\];

for \&crate::graph::Triangle { a, b, c } in \&triangles {

    if a \> 0 && b \> 0 && a \<= EDGES && b \<= EDGES {

        conflicts\[a\]\[b\] \= true;

        conflicts\[b\]\[a\] \= true;

    }

    if a \> 0 && c \> 0 && a \<= EDGES && c \<= EDGES {

        conflicts\[a\]\[c\] \= true;

        conflicts\[c\]\[a\] \= true;

    }

    if b \> 0 && c \> 0 && b \<= EDGES && c \<= EDGES {

        conflicts\[b\]\[c\] \= true;

        conflicts\[c\]\[b\] \= true;

    }

}

5. Abra [src/reducibility.rs](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/src/reducibility.rs) na função `test_configuration` (linhas 124–130) e exija `validate_sparse_contract(conf, angles).is_ok()` antes de chamar `self.check_contract`:

if \!is\_d\_reducible

    && angles.contract\[0\] \> 0

    && crate::graph::validate\_sparse\_contract(conf, angles).is\_ok()

    && self.check\_contract(angles, \&live, nlive)

{

    is\_c\_reducible \= true;

}

## **Tarefa 2: Restaurar a Condição 6 Estrita (`k <= 2`) em `src/graph.rs` e nos Verificadores C**

1. Em [src/graph.rs](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/src/graph.rs) (linha 158, dentro de `check_rsst_admissibility`), altere:

if k \> 4 {

    return Err("Condition 6 violated: internal vertex has boundary contact \> 4");

}

para o limite canônico do RSST:if k \> 2 {

    return Err("Condition 6 violated: internal vertex has boundary contact \> 2 (cut-vertex)");

}

Ajuste ou remova os dois testes unitários no final de src/graph.rs que esperavam que unavoidable\_25.conf passasse na Condição 6, substituindo por um teste que valida que todas as 629 configurações de `01_modelo_629_rsst_canonico/unavoidable_629.conf` passam com `k <= 2`.

2. Nos arquivos C de descarregamento ([02\_modelo\_394\_recorde\_compacto/README.md](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/02_modelo_394_recorde_compacto/README.md), [08\_pesquisa\_2flips/discharge.c](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/08_pesquisa_2flips/discharge.c) linha 1415 e [10\_recorde\_21\_configuracoes/discharge\_universal.c](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/10_recorde_21_configuracoes/discharge_universal.c) linha 1603), restaure:

if (k \> 2\)

    ReadErr(6, name);

## **Tarefa 3: Restaurar a Integridade de `GetQuestion`, `GetQuestionPair`, `CheckIso` e `RootedSubConf` em C**

1. Em [08\_pesquisa\_2flips/discharge.c](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/08_pesquisa_2flips/discharge.c) (linhas 1709–1718) e [10\_recorde\_21\_configuracoes/discharge\_universal.c](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/10_recorde_21_configuracoes/discharge_universal.c) (linhas 1336–1346 em `GetQuestionPair` e linhas 1897–1908 em `GetQuestion`), restaure o tratamento de erro original do RSST:

for (g \= (h \== 1\) ? d : h \- 1; g \!= j; g \= (g \== 1\) ? d : g \- 1\) {

   t \= L\[v\]\[g\];

   if ((t \<= ring) || (found\[t\])) {

      (void) printf("Error in getquestions\\n");

      exit(1);

   }

   Q\[nfound\].z \= t;

   Q\[nfound\].xi \= t \> ring ? L\[t\]\[0\] : 0;

   Q\[nfound\].u \= Q\[nfound \- 1\].z;

   Q\[nfound\].v \= v;

   nfound++;

   found\[t\] \= 1;

}

E adicione ao final de `GetQuestion` e `GetQuestionPair` a verificação de que todos os vértices internos foram alcançados:for (v \= ring \+ 1; v \<= nverts; v++) {

   if (\!found\[v\]) {

      (void) fprintf(stderr, "Error in getquestions: internal vertex %d not reached\!\\n", v);

      exit(1);

   }

}

2. Em `CheckIso` ([08\_pesquisa\_2flips/discharge.c](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/08_pesquisa_2flips/discharge.c) linha 997 e [10\_recorde\_21\_configuracoes/discharge\_universal.c](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/10_recorde_21_configuracoes/discharge_universal.c) linha 1011), restaure a linha original:

if (w \<= ring) {

   if (worried)

      Error("Isomorphism error 5", lineno);

   continue;

}

3. Em `RootedSubConf` ([10\_recorde\_21\_configuracoes/discharge\_universal.c](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/10_recorde_21_configuracoes/discharge_universal.c) linha 2036), adicione novamente a limpeza de `image`:

for (j \= 0; j \< CARTVERT; j++) image\[j\] \= \-1;

## **Tarefa 4: Adicionar Testes Unitários de Regressão e Auditoria em `src/reducibility.rs`**

Adicione no bloco `#[cfg(test)] mod tests` de [src/reducibility.rs](https://github.com/Kastikas/teoria-das-quatro-cores-25/blob/master/src/reducibility.rs) os dois testes abaixo e execute `cargo test`:\#\[test\]

fn test\_pruned\_p\_0\_7322\_3\_is\_not\_reducible() {

    let mut conf \= Configuration::new(1, 9, 6, 0);

    conf.name \= "p\_0.7322\_3".to\_string();

    conf.contract\_edges \= vec\!\[(6, 9), (5, 9)\];

    conf.set\_vertex(1, &\[2, 7, 9, 6\]);

    conf.set\_vertex(2, &\[3, 7, 1\]);

    conf.set\_vertex(3, &\[4, 8, 7, 2\]);

    conf.set\_vertex(4, &\[5, 8, 3\]);

    conf.set\_vertex(5, &\[6, 9, 8, 4\]);

    conf.set\_vertex(6, &\[1, 9, 5\]);

    conf.set\_vertex(7, &\[2, 3, 8, 9, 1\]);

    conf.set\_vertex(8, &\[3, 4, 5, 9, 7\]);

    conf.set\_vertex(9, &\[8, 5, 6, 1, 7\]);

    let angles \= find\_angles(\&conf);

    assert\!(

        validate\_sparse\_contract(\&conf, \&angles).is\_err(),

        "Contract \[(6,9), (5,9)\] shares a triangle with ring edge (5,6) and must be rejected\!"

    );

    let engine \= ReducibilityEngine::new();

    let report \= engine.test\_configuration(\&conf, \&angles);

    assert\!(\!report.is\_reducible(), "p\_0.7322\_3 (3 internal deg-5 vertices) cannot be reducible\!");

}

\#\[test\]

fn test\_canonical\_629\_all\_pass\_strict\_validation() {

    let configs \= crate::graph::read\_configurations("01\_modelo\_629\_rsst\_canonico/unavoidable\_629.conf")

        .expect("Failed to read unavoidable\_629.conf");

    assert\_eq\!(configs.len(), 629);

    let engine \= ReducibilityEngine::new();

    for conf in \&configs {

        assert\_eq\!(conf.check\_rsst\_admissibility(), Ok(()), "Failed admissibility: {}", conf.name);

        let angles \= find\_angles(conf);

        assert\_eq\!(validate\_sparse\_contract(conf, \&angles), Ok(()), "Invalid contract: {}", conf.name);

        let report \= engine.test\_configuration(conf, \&angles);

        assert\!(report.is\_reducible(), "Canonical config {} failed reducibility\!", conf.name);

    }

}  
