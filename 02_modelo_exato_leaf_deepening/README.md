# Modelo 02: Otimização Exata via Leaf Deepening e Super-Pool Certificado (629 Configurações)

Este diretório contém o **Modelo 02** do Teorema das Quatro Cores, obtido através da metodologia formal de **Leaf Deepening** (aprofundamento estrutural de folhas em árvores axiais de descarregamento) e otimização global via Programação Linear Inteira Mista (**HiGHS MILP**).

---

## 1. Fundamentação Matemática e Arquitetura

O método de descarregamento de Robertson, Sanders, Seymour e Thomas (RSST, 1997) estabelece que todo mapa planar maximal contém pelo menos uma configuração de um conjunto inevitável redutível. 

Nas apresentações canônicas inalteradas (`present7` a `present11`), o catálogo clássico possui 629 configurações. Através da instrumentação de `src/discharge.c` e do motor `scripts/deepen_optimizer.py`:
1. **Super-Pool Estritamente Redutível (891 configurações)**:
   - 629 configurações canônicas do RSST.
   - 262 configurações extras geradas e certificadas via D-redutibilidade e C-redutibilidade (com contratos de Kempe $\le 3$).
2. **Resolução Global Exata via HiGHS MILP**:
   - Mapeamento exato das 162.699 folhas axiais.
   - O solver HiGHS substituiu 79 configurações canônicas por 79 candidatos do super-pool que possuem maior cobertura cruzada e menor dependência de singletons.
3. **Expansão Estrutural de Árvores (Leaf Deepening)**:
   - As sub-árvores são aprofundadas em nós folha (`L<k> R` $\to$ `L<k> C <v> <n>` $\to$ `L<k+1> ...`), com remapeamento sequencial exato de todas as 544 referências de simetria (`S <rot> <refl> <level> <line>`).
   - A árvore resultante executa no verificador oficial `./bin/discharge` **100% puro, sem flags especiais em linha de comando**.

---

## 2. Composição do Catálogo

- **Total de Configurações:** 629
- **Configurações Canônicas:** 550
- **Novos Candidatos Incorporados:** 79
- **Redutibilidade Estrita de Birkhoff/Kempe:** 100% (245 D-redutíveis, 384 C-redutíveis com contratos $\le 3$).
- **Inevitabilidade Canônica:** 100% nas 5 apresentações axiais (`present7` a `present11`).

---

## 3. Verificação Canônica Independente

Para reproduzir a prova formal completa deste modelo:

```bash
./02_modelo_exato_leaf_deepening/verify_629.sh
```

O script executa em duas etapas estritas:
1. `./bin/discharge` sobre as 5 apresentações locais contra o catálogo `unavoidable_exact_629.conf` e o arquivo canônico de regras `data/rsst_presentations/rules`.
2. `./target/release/quatro_cores verify-file unavoidable_exact_629.conf 630`, comprovando a redutibilidade algébrica estrita sem tolerâncias ou atalhos.
