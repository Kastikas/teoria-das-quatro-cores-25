#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$DIR"

echo "================================================================================"
echo "    VERIFICAÇÃO COMPLETA: MODELO DE 10 CONFIGURAÇÕES (DESCARREGAMENTO INVERSO)"
echo "================================================================================"
echo ""

# 1. Compilar executável se necessário
if [ ! -f "discharge_profiler" ]; then
    echo ">> [1/4] Compilando discharge_profiler com gcc -O3..."
    gcc -O3 -o discharge_profiler discharge_profiler.c
else
    echo ">> [1/4] discharge_profiler já compilado e atualizado."
fi

# 2. Redutibilidade Algébrica em Rust
echo ""
echo ">> [2/4] Verificando Redutibilidade Algébrica em Rust (Birkhoff/Kempe/Stromquist)..."
cd ..
cargo run --release -- verify-file 11_descarregamento_inverso_sub15/unavoidable_10.conf 20
cd "$DIR"

# 3. Certificação de Inevitabilidade no Grau 11 (present11) com Regras Aumentadas
echo ""
echo ">> [3/4] Certificando Grau 11 (present11) com Regras Aumentadas (75 regras)..."
./discharge_profiler present11 unavoidable_10.conf rules_augmented 0 0

# 4. Auditoria dos Graus 7 a 10 e Compensação de Curvatura dos 117 Eixos
echo ""
echo ">> [4/4] Executando Auditoria Global de Cobertura e Curvatura (Graus 7 a 10)..."
python3 lp_rule_synthesizer.py

echo ""
echo "================================================================================"
echo "    CERTIFICAÇÃO CONCLUÍDA: 10 CONFIGURAÇÕES + 75 REGRAS (DESCARREGAMENTO INVERSO)"
echo "================================================================================"
