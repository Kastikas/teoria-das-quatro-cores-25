\# Plano Técnico v2: Limpeza, Aposentadoria de Modelos Falhos e Otimização Exata RSST

Este documento contém o plano de execução determinístico (projetado para ser executado passo a passo por um agente com acesso ao terminal \`bash\`) para:  
1\. \*\*Colher\*\* todas as configurações individualmente redutíveis geradas nos experimentos anteriores (\`01\` a \`08\`).  
2\. \*\*Aposentar (remover)\*\* os modelos \`02\` a \`08\` cujas provas de inevitabilidade apresentaram falhas matemáticas de cobertura estática, \*flips\* ou contratos \$K\_4\$.  
3\. \*\*Unificar\*\* a arquitetura C em \`src/\` e \`bin/\` via \`Makefile\` (Princípio DRY).  
4\. \*\*Implementar\*\* o novo método de \*\*Otimização Exata sobre as Folhas de \`present7..11\`\*\* utilizando \`src/discharge.c\` como oráculo exato e Programação Linear Inteira (\`PuLP\`).

\---

\#\# 1\. Motivação Matemática: Por que Aposentar os Modelos 02 a 08 (e o que Reaproveitar Deles)?

\#\#\# 1.1. Por que as demonstrações dos Modelos 03 a 08 são inválidas?  
No método de Robertson, Sanders, Seymour e Thomas (RSST), a prova de inevitabilidade ocorre sobre uma árvore de decisão axial (\`present7\` a \`present11\`). Cada folha crítica \$L\_k\$ dessa árvore representa uma vizinhança local parcialmente especificada \$W\$ (um \*cartwheel\*) ao redor de um vértice central \$v\$ cuja carga final ainda é positiva (\$c\_f(v) \> 0\$). Para eliminar a folha \$L\_k\$, o programa \`discharge.c\` precisa provar que existe uma configuração \$C\_i\$ no arquivo \`.conf\` tal que:  
1\. \$C\_i\$ é isomorfa a um subgrafo de \$W\$ (\$C\_i \\subseteq W\$);  
2\. Os graus de \*\*todos os vértices internos\*\* de \$C\_i\$ coincidem \*\*exatamente\*\* com os graus fixados na folha \$W\$;  
3\. O contrato de \$C\_i\$ (se C-redutível, com no máximo 3 arestas) é topologicamente compatível com as restrições da folha \$W\$ (\`CheckContract\`).

Os modelos \`03\` a \`08\` quebraram essa estrutura formal por três razões:  
\* \*\*Quebra de Transitividade e Graus Internos (Modelos 243, 177 e 137):\*\* Substituíram a verificação na árvore \`present7..11\` por uma cobertura estática (\`rsst\_coverage.txt\`) sobre as 633 configurações originais via \`checkiso\`. Porém, se uma configuração candidata \$C\_{\\text{nova}}\$ possui vértices extras (\$C\_{\\text{RSST}} \\subsetneq C\_{\\text{nova}}\$), saber que \$C\_{\\text{RSST}} \\subseteq W\$ \*\*não implica\*\* que \$C\_{\\text{nova}} \\subseteq W\$. E se \$C\_{\\text{nova}}\$ foi obtida removendo vértices de \$C\_{\\text{RSST}}\$, os graus internos dos vértices adjacentes mudam, fazendo o \`discharge.c\` rejeitar o mapeamento na folha \$W\$.  
\* \*\*Invariância Quebrada por \*Flips\* (Modelos 07 e 08 — 41 configurações e 2-\*flips\*):\*\* Um \*flip\* de diagonal substitui uma aresta \$(a,c)\$ existente na folha \$W\$ por uma aresta \$(b,d)\$ que \*\*não existe\*\* no contraexemplo mínimo \$G\$, além de alterar os graus de 4 vértices simultaneamente (invalidando as 32 regras de descarga \`rules\`).  
\* \*\*Colapso Indutivo em Contratos \$K\_4\$ (Modelo 05 — 149 configurações):\*\* Contrair 4 arestas simultaneamente pode fechar ciclos separadores de comprimento 4 ou 5 pelo exterior da configuração em \$G\$, gerando laços (\*loops\*) no grafo contraído \$G'\$ e invalidando a hipótese de indução.

\#\#\# 1.2. O que AINDA É ÚTIL nos Modelos 02 a 08? (A Estratégia de "Colheita")  
Embora os \*\*modelos de cobertura\*\* (\`rsst\_coverage.txt\`, \`solve\_sub140\_ilp.py\`, etc.) estejam matematicamente incorretos e devam ser aposentados, os \*\*arquivos \`.conf\` gerados naquelas pastas\*\* contêm centenas de configurações candidatas válidas que já foram sintetizadas computacionalmente\!  
\* Muitas dessas configurações individuais (como a superconfiguração \`07322\` do Modelo 394 e diversas subconfigurações podadas das pastas \`03\`, \`04\` e \`06\`) são \*\*grafos planares perfeitamente D-redutíveis ou C-redutíveis (com contrato seguro \$\\le 3\$)\*\*.  
\* Se reunirmos todas essas configurações candidatas em um único \*\*Super-Pool Certificado\*\* (filtrando cada uma pelo \`./reduce\` canônico estrito do RSST) e passarmos esse Super-Pool pelo \`discharge.c\` em modo matriz (\`-m\`), essas configurações "coringas" poderão cobrir múltiplas folhas das árvores \`present7..11\` de forma \*\*100% legal e rigorosa\*\*\!

Portanto, o procedimento correto é:  
1\. \*\*Colher\*\* todos os blocos \`.conf\` das pastas \`01\` a \`08\` e filtrá-los pelo \`./reduce\` canônico estrito.  
2\. \*\*Aposentar (\`git rm \-rf\`)\*\* todas as pastas \`02\` a \`08\` após a colheita, limpando definitivamente o repositório.

\---

\#\# 2\. Roteiro Passo a Passo para o Agente (Comandos Bash Determinísticos)

\> \*\*Instrução para o Agente Executor:\*\* Execute as fases abaixo sequencialmente. Não pule etapas e valide o código de retorno (\`\$? \== 0\`) ao final de cada fase antes de prosseguir.

\#\#\# Fase 1: Preparação da Estrutura Central (\`src/\`, \`bin/\`, \`scripts/\`) e \`Makefile\`

Execute no terminal na raiz do repositório:

\`\`\`bash  
mkdir \-p src bin scripts data/pool  
\`\`\`

Crie o arquivo \`Makefile\` na raiz do repositório:

\`\`\`bash  
cat \<\< 'EOF' \> Makefile  
CC \= gcc  
CFLAGS \= \-O3 \-Wall

all: bin/discharge bin/reduce bin/checkiso

bin:  
	mkdir \-p bin

bin/discharge: src/discharge.c | bin  
	\$(CC) \$(CFLAGS) src/discharge.c \-o bin/discharge

bin/reduce: src/reduce.c | bin  
	\$(CC) \$(CFLAGS) src/reduce.c \-o bin/reduce

bin/checkiso: src/checkiso.c | bin  
	\$(CC) \$(CFLAGS) src/checkiso.c \-o bin/checkiso

clean:  
	rm \-rf bin/  
EOF  
\`\`\`

Garanta que \`bin/\` esteja no \`.gitignore\`:

\`\`\`bash  
grep \-qxF "bin/" .gitignore || echo "bin/" \>\> .gitignore  
\`\`\`

\---

\#\#\# Fase 2: Instrumentação Segura de \`src/discharge.c\` (Patch Automatizado)

Para evitar erros de edição manual em C, vamos usar como base o arquivo \`01\_modelo\_629\_rsst\_canonico/discharge.c\` e comparar com \`03\_modelo\_243\_podas\_2a\_ordem/discharge\_track.c\` (que já possui a marcação exata do ponto onde uma configuração resolve uma folha da árvore axial).

1\. Primeiro, inspecione o \`diff\` entre \`discharge.c\` e \`discharge\_track.c\` para visualizar a função \`Reduce\`:

\`\`\`bash  
diff \-u 01\_modelo\_629\_rsst\_canonico/discharge.c 03\_modelo\_243\_podas\_2a\_ordem/discharge\_track.c || true  
\`\`\`

2\. Copie o \`01\_modelo\_629\_rsst\_canonico/discharge.c\` para \`src/discharge.c\` e execute o script Python abaixo para injetar de forma determinística as flags \`-t\` (modo \`track\`) e \`-m\` (modo \`matrix\`, que não interrompe o laço \`for\` na primeira configuração encontrada, listando todas as configurações que cobrem a folha atual):

\`\`\`bash  
cp 01\_modelo\_629\_rsst\_canonico/discharge.c src/discharge.c

python3 \- \<\< 'EOF'  
with open("src/discharge.c", "r", encoding="utf-8") as f:  
    code \= f.read()

\# 1\. Adicionar variaveis globais de controle logo apos os \#includes  
globals\_inject \= """  
\#include \<string.h\>  
int track\_mode \= 0;  
int matrix\_mode \= 0;  
long leaf\_counter \= 0;  
"""

\# Inserir apos o ultimo \#include  
last\_inc \= code.rfind("\#include")  
line\_end \= code.find("\\n", last\_inc)  
code \= code\[:line\_end+1\] \+ globals\_inject \+ code\[line\_end+1:\]

\# 2\. Adicionar parsing de flags \-t e \-m no inicio de main(argc, argv)  
main\_target \= "int main(argc,argv)"  
if main\_target not in code:  
    main\_target \= "main(argc,argv)"

main\_pos \= code.find(main\_target)  
brace\_pos \= code.find("{", main\_pos)

 CLI\_inject \= """  
   while (argc \> 1 && argv\[1\]\[0\] \== '-') {  
      if (strcmp(argv\[1\], "-t") \== 0 || strcmp(argv\[1\], "--track") \== 0\) {  
         track\_mode \= 1;  
      } else if (strcmp(argv\[1\], "-m") \== 0 || strcmp(argv\[1\], "--matrix") \== 0\) {  
         matrix\_mode \= 1;  
      }  
      argc--;  
      argv++;  
   }  
"""  
code \= code\[:brace\_pos+1\] \+ CLI\_inject \+ code\[brace\_pos+1:\]

\# 3\. Instrumentar a funcao Reduce() onde CheckIso e chamado  
\# No discharge.c padrao do RSST, dentro de Reduce(), ha o laco:  
\# for (i \= 0; i \< nconf; i++) {  
\#    if (\!CheckIso(G\[i\],A,B,C)) continue;  
\#    ...  
\#    return(1);  
\# }  
old\_loop \= """   for (i \= 0; i \< nconf; i++) {  
      if (\!CheckIso(G\[i\],A,B,C)) continue;  
      if (lineno \> 0\) {  
         printf("Configuration %d present\\\\n", i \+ 1);  
         fflush(stdout);  
      }  
      return(1);  
   }"""

new\_loop \= """   int found\_any \= 0;  
   long current\_leaf \= \++leaf\_counter;  
   if (matrix\_mode) {  
      printf("LEAF\_MATRIX %ld:", current\_leaf);  
   }  
   for (i \= 0; i \< nconf; i++) {  
      if (\!CheckIso(G\[i\],A,B,C)) continue;  
      if (matrix\_mode) {  
         printf(" %d", i \+ 1);  
         found\_any \= 1;  
         continue;  
      }  
      if (track\_mode) {  
         printf("USED\_CONFIG\_ID: %d\\\\n", i \+ 1);  
      }  
      if (lineno \> 0\) {  
         printf("Configuration %d present\\\\n", i \+ 1);  
         fflush(stdout);  
      }  
      return(1);  
   }  
   if (matrix\_mode) {  
      printf("\\\\n");  
      fflush(stdout);  
      if (found\_any) return(1);  
   }"""

if old\_loop not in code:  
    raise RuntimeError("Padrao do laco for em Reduce() nao encontrado exatamente. Verifique com diff.")

code \= code.replace(old\_loop, new\_loop, 1\)

with open("src/discharge.c", "w", encoding="utf-8") as f:  
    f.write(code)

print("src/discharge.c instrumentado com sucesso\!")  
EOF  
\`\`\`

3\. Compile todos os binários com \`make all\` e verifique se o \`bin/discharge\` passa no teste canônico de 629 configurações:

\`\`\`bash  
make all  
for deg in 7 8 9 10 11; do  
    ./bin/discharge data/rsst\_presentations/present\$deg data/rsst\_presentations/rules 01\_modelo\_629\_rsst\_canonico/unavoidable\_629.conf  
done  
\`\`\`

\---

\#\#\# Fase 3: Colheita de Configurações Candidatas e Certificação Estrita (\`scripts/harvest\_superpool.py\`)

Antes de apagar as pastas \`02\` a \`08\`, vamos extrair todas as configurações únicas de todos os arquivos \`.conf\` do repositório e submeter cada uma ao \`./bin/reduce\` canônico estrito (garantindo que apenas configurações matematicamente válidas e com contratos seguros \$\\le 3\$ entrem no Super-Pool).

Crie e execute o script \`scripts/harvest\_superpool.py\`:

\`\`\`bash  
cat \<\< 'EOF' \> scripts/harvest\_superpool.py  
\#\!/usr/bin/env python3  
"""  
Varre todos os arquivos .conf do repositorio, extrai blocos unicos (ignorando IDs),  
valida a estrutura sintatica e certifica cada configuracao individualmente no  
binario canonico ./bin/reduce.  
"""  
import glob  
import os  
import re  
import subprocess  
import tempfile

def normalize\_topology(block\_lines):  
    """  
    Remove a primeira linha (ID da configuracao) e espacos redundantes  
    para criar uma chave unica da topologia \+ coordenadas \+ contrato.  
    """  
    cleaned \= \[re.sub(r'\\s+', ' ', line.strip()) for line in block\_lines\[1:\] if line.strip()\]  
    return "\\n".join(cleaned)

def extract\_blocks\_from\_file(filepath):  
    with open(filepath, "r", encoding="utf-8", errors="ignore") as f:  
        text \= f.read()  
    raw\_blocks \= re.split(r'\\n\\s\*\\n', text.strip())  
    valid\_blocks \= \[\]  
    for rb in raw\_blocks:  
        lines \= \[l.rstrip() for l in rb.strip().splitlines() if l.strip()\]  
        if len(lines) \< 4:  
            continue  
        \# Verifica se a linha 2 tem o formato "n r p c" (inteiros)  
        header\_parts \= lines\[1\].split()  
        if len(header\_parts) \!= 4 or not all(p.isdigit() for p in header\_parts):  
            continue  
        n, r, p, c \= map(int, header\_parts)  
        \# Rejeita imediatamente contratos de tamanho \>= 4 (Ex: K4 do modelo 05\)  
        contract\_parts \= lines\[2\].split()  
        if not contract\_parts or not contract\_parts\[0\].isdigit():  
            continue  
        num\_contract\_edges \= int(contract\_parts\[0\])  
        if num\_contract\_edges \> 3:  
            continue  
        valid\_blocks.append(lines)  
    return valid\_blocks

def certify\_with\_reduce(block\_lines, reduce\_bin="./bin/reduce"):  
    block\_str \= "\\n".join(block\_lines) \+ "\\n\\n"  
    with tempfile.NamedTemporaryFile("w", suffix=".conf", delete=False) as tmp:  
        tmp.write(block\_str)  
        tmp\_path \= tmp.name  
    try:  
        res \= subprocess.run(  
            \[reduce\_bin, tmp\_path\],  
            stdout=subprocess.PIPE,  
            stderr=subprocess.PIPE,  
            text=True,  
            timeout=20  
        )  
        return res.returncode \== 0  
    except Exception:  
        return False  
    finally:  
        if os.path.exists(tmp\_path):  
            os.remove(tmp\_path)

def main():  
    \# 1\. Comecar sempre pelas 629 configuracoes canonicas na ordem original  
    canonical\_path \= "01\_modelo\_629\_rsst\_canonico/unavoidable\_629.conf"  
    all\_files \= \[canonical\_path\] \+ sorted(  
        f for f in glob.glob("\*\_\*/\*.conf") \+ glob.glob("\*\_\*/data/\*.conf")  
        if f \!= canonical\_path  
    )

    seen\_topologies \= set()  
    candidates \= \[\]

    for fpath in all\_files:  
        for lines in extract\_blocks\_from\_file(fpath):  
            topo\_key \= normalize\_topology(lines)  
            if topo\_key not in seen\_topologies:  
                seen\_topologies.add(topo\_key)  
                candidates.append((fpath, lines))

    print(f"Total de topologias unicas extraidas (com contrato \<= 3): {len(candidates)}")

    certified \= \[\]  
    for idx, (src\_file, lines) in enumerate(candidates, 1):  
        \# Renumerar ID sequencialmente na linha 0  
        new\_id \= len(certified) \+ 1  
        lines\_copy \= \[str(new\_id)\] \+ lines\[1:\]  
        if certify\_with\_reduce(lines\_copy):  
            certified.append(lines\_copy)  
        if idx % 100 \== 0 or idx \== len(candidates):  
            print(f"Testadas {idx}/{len(candidates)} | Aprovadas no ./bin/reduce: {len(certified)}")

    out\_path \= "data/pool/certified\_superpool.conf"  
    with open(out\_path, "w", encoding="utf-8") as f:  
        for block in certified:  
            f.write("\\n".join(block) \+ "\\n\\n")

    print(f"Super-Pool salvo com sucesso em {out\_path} ({len(certified)} configuracoes).")

if \_\_name\_\_ \== "\_\_main\_\_":  
    main()  
EOF  
chmod \+x scripts/harvest\_superpool.py  
python3 scripts/harvest\_superpool.py  
\`\`\`

\---

\#\#\# Fase 4: Aposentadoria Definitiva dos Modelos Falhos (\`02\` a \`08\`) e Limpeza da Pasta \`01\`

Agora que todas as configurações genuinamente redutíveis já foram colhidas e certificadas em \`data/pool/certified\_superpool.conf\`, aposente os diretórios \`02\` a \`08\` e limpe os arquivos duplicados da pasta \`01\`:

\`\`\`bash  
\# 1\. Remover as pastas dos modelos 02 a 08 que usavam heuristicas falhas ou superconfiguracoes nao auditadas  
git rm \-rf \\  
    02\_modelo\_394\_recorde\_compacto \\  
    03\_modelo\_243\_podas\_2a\_ordem \\  
    04\_modelo\_177\_sub200\_otimizacao\_global \\  
    05\_pesquisa\_contratos\_k4\_podas\_profundas \\  
    06\_pesquisa\_podas\_4a\_ordem\_sub140 \\  
    07\_pesquisa\_flips\_sub130 \\  
    08\_pesquisa\_2flips

\# 2\. Limpar symlinks, duplicatas e codigo C redundante dentro de 01\_modelo\_629\_rsst\_canonico  
git rm \-f \\  
    01\_modelo\_629\_rsst\_canonico/discharge.c \\  
    01\_modelo\_629\_rsst\_canonico/present7 \\  
    01\_modelo\_629\_rsst\_canonico/present8 \\  
    01\_modelo\_629\_rsst\_canonico/present9 \\  
    01\_modelo\_629\_rsst\_canonico/present10 \\  
    01\_modelo\_629\_rsst\_canonico/present11 \\  
    01\_modelo\_629\_rsst\_canonico/rules \\  
    01\_modelo\_629\_rsst\_canonico/data/unavoidable\_629.conf

\# 3\. Atualizar 01\_modelo\_629\_rsst\_canonico/verify\_629.sh para usar bin/discharge e data/rsst\_presentations  
cat \<\< 'EOF' \> 01\_modelo\_629\_rsst\_canonico/verify\_629.sh  
\#\!/bin/bash  
set \-e  
DIR="\$(cd "\$(dirname "\$0")" && pwd)"  
ROOT="\$(cd "\$DIR/.." && pwd)"

make \-C "\$ROOT" bin/discharge  
CONF="\$DIR/unavoidable\_629.conf"  
DATA="\$ROOT/data/rsst\_presentations"

for deg in 7 8 9 10 11; do  
    echo "Verificando present\$deg com unavoidable\_629.conf..."  
    "\$ROOT/bin/discharge" "\$DATA/present\$deg" "\$DATA/rules" "\$CONF"  
done  
echo "Modelo 629 verificado com sucesso\!"  
EOF  
chmod \+x 01\_modelo\_629\_rsst\_canonico/verify\_629.sh  
bash 01\_modelo\_629\_rsst\_canonico/verify\_629.sh  
\`\`\`

\---

\#\#\# Fase 5: Extração da Matriz Exata de Folhas e Otimização via ILP (\`PuLP\`)

1\. Gere os logs da matriz exata Folha \$\\times\$ Configuração para os graus 7, 8, 9, 10 e 11 executando \`./bin/discharge \-m\` sobre o \`certified\_superpool.conf\`:

\`\`\`bash  
for deg in 7 8 9 10 11; do  
    echo "Extraindo matriz exata de folhas para present\$deg..."  
    ./bin/discharge \-m "data/rsst\_presentations/present\$deg" "data/rsst\_presentations/rules" "data/pool/certified\_superpool.conf" \> "data/pool/matrix\_deg\${deg}.log"  
done  
\`\`\`

2\. Crie e execute o solver ILP exato (\`scripts/solve\_exact\_leaves\_ilp.py\`) mantendo a biblioteca \`pulp\`:

\`\`\`bash  
cat \<\< 'EOF' \> scripts/solve\_exact\_leaves\_ilp.py  
\#\!/usr/bin/env python3  
"""  
Resolve o Set Cover Exato diretamente sobre as folhas das arvores present7..11  
usando a matriz gerada pelo proprio ./bin/discharge \-m.  
"""  
import glob  
import os  
import re  
import subprocess  
import pulp

def load\_superpool(filepath="data/pool/certified\_superpool.conf"):  
    with open(filepath, "r", encoding="utf-8") as f:  
        text \= f.read().strip()  
    blocks \= re.split(r'\\n\\s\*\\n', text)  
    pool \= {}  
    for b in blocks:  
        lines \= \[l.rstrip() for l in b.strip().splitlines() if l.strip()\]  
        if lines:  
            cid \= int(lines\[0\].strip())  
            pool\[cid\] \= lines  
    return pool

def load\_leaf\_constraints(pattern="data/pool/matrix\_deg\*.log"):  
    constraints \= \[\]  
    active\_cids \= set()  
    for path in sorted(glob.glob(pattern)):  
        deg \= os.path.basename(path).replace("matrix\_deg", "").replace(".log", "")  
        with open(path, "r", encoding="utf-8") as f:  
            for line in f:  
                if line.startswith("LEAF\_MATRIX "):  
                    head, tail \= line.strip().split(":")  
                    leaf\_num \= head.split()\[1\]  
                    cids \= \[int(x) for x in tail.strip().split()\]  
                    if not cids:  
                        raise RuntimeError(f"ERRO CRITICO: Folha sem cobertura em deg={deg}, leaf={leaf\_num}")  
                    constraints.append((f"d{deg}\_l{leaf\_num}", sorted(set(cids))))  
                    active\_cids.update(cids)  
    return constraints, sorted(active\_cids)

def main():  
    pool \= load\_superpool()  
    constraints, active\_cids \= load\_leaf\_constraints()

    \# Remover constraints duplicadas (folhas cobertas exatamente pelo mesmo subconjunto de configuracoes)  
    unique\_sets \= {}  
    for tag, cids in constraints:  
        key \= tuple(cids)  
        if key not in unique\_sets:  
            unique\_sets\[key\] \= tag

    print(f"Total de folhas criticas nas arvores present7..11: {len(constraints)}")  
    print(f"Restricoes unicas de cobertura apos desduplicacao: {len(unique\_sets)}")  
    print(f"Configuracoes candidatas que atuam nas arvores: {len(active\_cids)}")

    prob \= pulp.LpProblem("RSST\_Exact\_Tree\_Leaves\_Cover", pulp.LpMinimize)  
    x \= {cid: pulp.LpVariable(f"x\_{cid}", cat=pulp.LpBinary) for cid in active\_cids}

    \# Objetivo: minimizar o numero total de configuracoes  
    prob \+= pulp.lpSum(x\[cid\] for cid in active\_cids)

    \# Restricao: cada folha da arvore deve ter pelo menos 1 configuracao valida  
    for cids\_tuple, tag in unique\_sets.items():  
        prob \+= pulp.lpSum(x\[cid\] for cid in cids\_tuple) \>= 1, f"cov\_{tag}"

    prob.solve(pulp.PULP\_CBC\_CMD(msg=True))  
    status \= pulp.LpStatus\[prob.status\]  
    if status \!= "Optimal":  
        raise RuntimeError(f"Solver nao encontrou solucao otima. Status: {status}")

    selected\_cids \= sorted(cid for cid in active\_cids if pulp.value(x\[cid\]) \> 0.5)  
    n\_opt \= len(selected\_cids)  
    print(f"\\n\>\>\> SUCESSO\! Cardinalidade Exata Otima Encontrada: {n\_opt} configuracoes \<\<\<")

    out\_dir \= f"02\_modelo\_{n\_opt}\_otimo\_exato\_ilp"  
    os.makedirs(out\_dir, exist\_ok=True)  
    out\_conf \= os.path.join(out\_dir, f"unavoidable\_{n\_opt}.conf")

    with open(out\_conf, "w", encoding="utf-8") as f:  
        for new\_idx, cid in enumerate(selected\_cids, 1):  
            lines \= \[str(new\_idx)\] \+ pool\[cid\]\[1:\]  
            f.write("\\n".join(lines) \+ "\\n\\n")

    \# Criar script de verificacao formal independente  
    verify\_sh \= os.path.join(out\_dir, f"verify\_{n\_opt}.sh")  
    with open(verify\_sh, "w", encoding="utf-8") as f:  
        f.write(f"""\#\!/bin/bash  
set \-e  
DIR="\$(cd "\$(dirname "\$0")" && pwd)"  
ROOT="\$(cd "\$DIR/.." && pwd)"

make \-C "\$ROOT" all  
CONF="\$DIR/unavoidable\_{n\_opt}.conf"  
DATA="\$ROOT/data/rsst\_presentations"

echo "1. Certificando redutibilidade estrita (./bin/reduce)..."  
"\$ROOT/bin/reduce" "\$CONF"

echo "2. Certificando inevitabilidade nas arvores axiais (./bin/discharge)..."  
for deg in 7 8 9 10 11; do  
    "\$ROOT/bin/discharge" "\$DATA/present\$deg" "\$DATA/rules" "\$CONF"  
done  
echo "Prova formal completa verificada com sucesso para {n\_opt} configuracoes\!"  
""")  
    os.chmod(verify\_sh, 0o755)  
    print(f"Modelo salvo em {out\_dir}. Executando verificacao formal...")  
    subprocess.run(\["bash", verify\_sh\], check=True)

if \_\_name\_\_ \== "\_\_main\_\_":  
    main()  
EOF  
chmod \+x scripts/solve\_exact\_leaves\_ilp.py  
python3 scripts/solve\_exact\_leaves\_ilp.py  
\`\`\`

\---

\#\#\# Fase 6: Atualização do \`README.md\` Principal e do CI (\`.github/workflows/ci.yml\`)

1\. Atualize \`.github/workflows/ci.yml\` para compilar via \`Makefile\` e rodar os scripts \`verify\_\*.sh\` tanto do Modelo 629 quanto do novo modelo ótimo exato gerado na Fase 5\.  
2\. Atualize o \`README.md\` da raiz documentando:  
   \- A aposentadoria das abordagens heurísticas anteriores (subgrafos indiretos, contratos \$K\_4\$ e \*flips\* de diagonal) com a justificativa matemática;  
   \- A arquitetura limpa em \`src/\` e \`bin/\`;  
   \- O novo método exato de cobertura das folhas de \`present7..11\` via Programação Linear Inteira.  
