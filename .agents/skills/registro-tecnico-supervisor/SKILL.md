---
name: registro-tecnico-supervisor
description: >-
  Metodologia e diretrizes para estruturar relatórios técnicos de pesquisa e registros de experimentos
  voltados a modelos supervisores de IA e auditoria matemática de alto rigor. Use sempre que for
  documentar testes, buscas combinatórias, mutações de grafos, refatorações ou hipóteses no padrão formal
  de 4 etapas (Sugerido, Feito, Motivo, Resultados) com máxima densidade de informação e rigor algébrico.
---

# Registro Técnico de Experimentos para Modelo Supervisor

Esta skill estabelece o processo padronizado para documentar experimentos, testes computacionais e hipóteses teóricas em formato de alta densidade semiótica e fidelidade matemática, ideal para consumo por modelos de IA supervisores e auditorias técnicas rigorosas.

---

## 1. Princípios Norteadores

1. **Economia Cognitiva e Computacional:** O modelo supervisor opera em alta capacidade de raciocínio. O documento deve eliminar redundâncias prosaicas e focar em especificações quantitativas, teoremas de não-existência, invariantes e formalismo exato.
2. **Caminhos Fechados são Teoremas de Não-Existência:** Mesmo que uma abordagem não atinja o objetivo almejado (ex: uma mutação planar que não fecha uma folha residual), o resultado negativo é um fato matemático essencial que delimita e poda o espaço de busca futuro. Deve ser registrado com o mesmo rigor de um resultado positivo.
3. **Proibição Absoluta de Atalhos:** Nunca suavizar, omitir ou flexibilizar invariantes ou regras de validação para justificar um teste. Registre o comportamento exato dos verificadores e solvers.
4. **Reprodutibilidade Estrita:** Cada teste deve conter comandos executáveis exatos, parâmetros, sementes, arquivos envolvidos e métricas verificáveis.

---

## 2. A Estrutura Padronizada de 4 Etapas

Cada experimento, teste ou mutação investigada deve ser documentado obrigatoriamente seguindo o quarteto:

### 1. O que foi sugerido
* Descreva a hipótese inicial, intervenção proposta ou questionamento que originou o teste.
* Identifique os objetos de estudo (ex: IDs de configurações, números de linhas de apresentação, invariantes em análise).

### 2. O que foi feito
* Detalhe a ação prática executada, comandos executados no shell, algoritmos empregados e parâmetros de execução.
* Mencione os arquivos de entrada, transformações aplicadas e ferramentas utilizadas (ex: `discharge`, `quatro_cores`, solvers MILP, scripts Python).

### 3. O motivo pelo qual foi feito
* Exponha o embasamento teórico e a justificativa estratégica do método escolhido frente a alternativas.
* Explique a lógica combinatória/topológica (ex: por que usar diagonal flips em vez de expansão de vértices; por que testar configurações de baixa frequência primeiro).

### 4. Resultados
* Apresente os dados objetivos, números concretos, métricas de cobertura, tempo de execução e status de validação algébrica.
* Se falhou, analise a causa raiz geométrica ou combinatória (ex: perturbação de graus de vértices vizinhos, quebra de isomorfismo em anel fixo).
* Destaque o teorema de contorno ou invariante demonstrada pelo teste.

---

## 3. Estrutura Canônica do Documento `.md`

Ao produzir um relatório técnico para o supervisor (como em `registro_tecnico_supervisor_*.md`), utilize a seguinte espinha dorsal:

```markdown
# Registro Técnico de Arquitetura e Otimização: [Título Específico]

**Destinatário:** Modelo Supervisor / Agente Raciocinador Superior  
**Projeto:** [Nome do Projeto]  
**Data:** [Data de Atualização]  
**Status do Repositório:** Ramificação, commit atual, status de testes (`cargo test`).

---

## 1. Visão Executiva e Fundamentos Teóricos
* Formulação teórica do problema (ex: Teorema das Quatro Cores, conservação de carga de Euler, árvores de descarregamento).
* Equações governantes e invariantes estruturais.

## 2. Invariantes Críticas e Teoremas de Não-Existência
* Limites matemáticos já comprovados no projeto (ex: cotas inferiores via MILP).
* Proibições explícitas de falsas premissas ou caminhos já matematicamente invalidados.

## 3. O Motor Computacional e Ferramental
* Descrição das ferramentas, verificadores, flags de execução e pipelines de teste.

## 4. Registro Estruturado dos Testes e Experimentos
[Aplicações sistemáticas do formato de 4 etapas para cada teste individual: 4.1, 4.2, 4.3, etc.]

## 5. Tabela Comparativa de Métricas e Estados
* Tabela consolidando candidatos, coberturas percentuais, folhas residuais e tipologia geométrica.

## 6. Próximo Passo Cirúrgico
* Proposta da intervenção seguinte com procedimento passo a passo claro, invariantes a verificar e critérios de aceite.
```

---

## 4. Checklist de Validação do Relatório

Antes de concluir a redação ou atualização do registro:
- [ ] O cabeçalho reflete o commit e o resultado de `cargo test` mais recentes?
- [ ] Todos os testes utilizam os 4 tópicos numerados (*1. O que foi sugerido*, *2. O que foi feito*, *3. O motivo pelo qual foi feito*, *4. Resultados*)?
- [ ] Resultados negativos contêm o diagnóstico formal da causa do insucesso?
- [ ] Links clicáveis para arquivos e símbolos de código foram incluídos com protocolo `file://`?
- [ ] O documento preserva a precisão matemática sem atalhos ou simplificações imprecisas?
