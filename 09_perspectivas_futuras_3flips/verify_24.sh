#!/usr/bin/env bash
set -euo pipefail

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$DIR/.." && pwd)"

echo "=========================================================================="
echo " DUPLA CERTIFICAÇÃO DO CONJUNTO INEVITÁVEL DE 24 CONFIGURAÇÕES"
echo " (Extensão Universal Diédrica / Multi-Raiz de Robertson-Seymour-Sanders-Thomas)"
echo "=========================================================================="

echo ""
echo "[FASE 1] Compilando verificador C universal ('discharge_universal')..."
gcc -O3 -o "$DIR/discharge_universal" "$DIR/discharge_universal.c"
echo ">> Compilação concluída com sucesso."

echo ""
echo "[FASE 2] Verificação Euleriana de Discharging (C - Todas as 5 Apresentações)..."
for p in present7 present8 present9 present10 present11; do
    echo -n "  Verificando $p... "
    "$DIR/discharge_universal" "$DIR/$p" "$DIR/unavoidable_24.conf" "$DIR/rules" 0 0 > /dev/null
    echo "OK (0 déficit de carga)"
done

echo ""
echo "[FASE 3] Verificação Algébrica de Redutibilidade (Rust - D-reducibility & C-reducibility)..."
cargo run --manifest-path "$ROOT/Cargo.toml" --release -- verify-file "$DIR/unavoidable_24.conf" 30

echo ""
echo "=========================================================================="
echo " CERTIFICAÇÃO CONCLUÍDA COM SUCESSO: 24 CONFIGURAÇÕES COMPROVADAS"
echo "=========================================================================="
