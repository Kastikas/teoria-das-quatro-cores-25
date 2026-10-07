#!/bin/bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$DIR"

echo "=========================================================================="
echo " PIPELINE DE DESCARREGAMENTO INVERSO (FRONTEIRA SUB-15)"
echo " Projeto Quatro Cores - Pasta 11"
echo "=========================================================================="
echo ""

# 1. Compila profiler se necessário
if [ ! -f "discharge_profiler" ] || [ "discharge_profiler.c" -nt "discharge_profiler" ]; then
    echo "[FASE 1] Compilando profiler C ('discharge_profiler')..."
    gcc -O3 -Wall -Wextra -Wno-unused-variable -Wno-unused-parameter discharge_profiler.c -o discharge_profiler
    echo ">> Compilação concluída."
else
    echo "[FASE 1] 'discharge_profiler' já compilado e atualizado."
fi
echo ""

# 2. Gera perfis se necessário
echo "[FASE 2] Verificando/Gerando perfis das 5 apresentações..."
for p in present7 present8 present9 present10 present11; do
    if [ ! -f "profile_${p}.json" ]; then
        echo "  Gerando profile_${p}.json..."
        ./discharge_profiler --profile "$p" unavoidable_21.conf rules "profile_${p}.json"
    else
        echo "  profile_${p}.json já existente."
    fi
done
echo ">> Todos os perfis prontos."
echo ""

# 3. Executa busca combinatória e relatório de Pareto
echo "[FASE 3] Executando varredura combinatória e modelagem inversa (Python)..."
python3 inverse_search.py

echo ""
echo "=========================================================================="
echo " PIPELINE CONCLUÍDO COM SUCESSO!"
echo " Consulte o relatório detalhado em: report_inverse_analysis.md"
echo "=========================================================================="
