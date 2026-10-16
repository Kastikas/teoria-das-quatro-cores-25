# Exemplo de Aplicação: Padrão de 4 Etapas

Este arquivo demonstra a aplicação prática da estrutura formal de 4 etapas para documentação de experimentos, utilizando o caso de mutação topológica de grafos por diagonal flips.

---

### Experimento: Modificação de Grafo via Diagonal Flips para Encaixe de Folha Residual

1. **O que foi sugerido:**  
   Alterar o grafo candidato (Candidato 3234) para tentar encaixar em uma folha residual solta (`present8`, linha 204), buscando uma configuração com mesmo número de vértices e anel.

2. **O que foi feito:**  
   Utilizou-se a operação de *diagonal flips* planares internos (`quatro_cores generate-flips`) em arestas internas do grafo, gerando 1-mutações, filtrando pelas 8 propriedades de triangulação planar bem-formada e testando contra o verificador `discharge`.

3. **O motivo pelo qual foi feito:**  
   É o método mais direto e minimalista para descobrir novos candidatos conexos no mesmo espaço topológico ($V=22, R=13$), preservando a planaridade e a integridade do anel sem a necessidade de desenhar grafos do zero.

4. **Resultados:**  
   Os grafos gerados e validados como D-redutíveis puras não conseguiram substituir os grafos em questão nem fechar a folha residual. O *flip* perturbou os graus dos vértices internos que serviam de âncora para os raios fixos (grau 5 e 6) da folha 204, quebrando os isomorfismos de subgrafo necessários para o encaixe. O resultado negativo provou que variações de 1-flip não resolvem o descompasso na vizinhança da linha 204.
