#!/bin/bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$DIR"

echo "=========================================================================="
echo " DUPLA CERTIFICAÇÃO DO CONJUNTO INEVITÁVEL DE 21 CONFIGURAÇÕES"
echo " (Recorde Mundial sob Extensão Universal Diédrica & Mutações de 3-Flips)"
echo "=========================================================================="
echo ""

echo "[FASE 1] Compilando verificador C universal ('discharge_universal')..."
gcc -O3 -Wall -Wextra -Wno-unused-variable -Wno-unused-parameter discharge_universal.c -o discharge_universal
echo ">> Compilação concluída com sucesso."
echo ""

echo "[FASE 2] Verificação Euleriana de Discharging (C - Todas as 5 Apresentações)..."
for p in present7 present8 present9 present10 present11; do
    echo -n "  Verificando $p... "
    OUTPUT=$(./discharge_universal "$p" unavoidable_21.conf rules 0 0)
    if echo "$OUTPUT" | grep -q "0 deficit"; then
        echo "OK (0 déficit de carga)"
    else
        echo "FALHA!"
        echo "$OUTPUT"
        exit 1
    fi
done
echo ""

echo "[FASE 3] Verificação Algébrica de Redutibilidade (Rust - D-reducibility & C-reducibility)..."
cd "$DIR/.."
cargo run --release -- verify-file "$DIR/unavoidable_21.conf" 30
cd "$DIR"

echo ""
echo "=========================================================================="
echo " CERTIFICAÇÃO CONCLUÍDA COM SUCESSO: 21 CONFIGURAÇÕES COMPROVADAS"
echo "=========================================================================="
