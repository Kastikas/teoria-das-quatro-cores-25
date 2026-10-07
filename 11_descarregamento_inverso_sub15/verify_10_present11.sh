#!/bin/bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$DIR"

echo "=========================================================================="
echo " CERTIFICAÇÃO DO CONJUNTO DE 10 CONFIGURAÇÕES (PRESENT11 100% RESOLVIDO)"
echo " Projeto Quatro Cores - Pasta 11 (Descarregamento Inverso)"
echo "=========================================================================="
echo ""

echo "[1] Compilando verificador/profiler C..."
gcc -O3 -Wall -Wextra -Wno-unused-variable -Wno-unused-parameter discharge_profiler.c -o discharge_profiler
echo ">> Compilação concluída."
echo ""

echo "[2] Verificação Algébrica em Rust (10/10 configurações)..."
cd "$DIR/.."
cargo run --release -- verify-file "$DIR/unavoidable_10.conf" 20
cd "$DIR"
echo ""

echo "[3] Verificação Euleriana de Discharging em C (present11)..."
./discharge_profiler --test 0x3ff present11 unavoidable_10.conf rules

echo ""
echo "=========================================================================="
echo " SUCESSO: 10 CONFIGURAÇÕES RESOLVEM INTEGRALMENTE O GRAU 11 (0 EIXOS ABERTOS)"
echo "=========================================================================="
