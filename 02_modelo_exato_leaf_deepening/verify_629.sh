#!/bin/bash
set -e

DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$DIR/.." && pwd)"

echo "================================================================================"
echo "   VERIFICAÇÃO FORMAL DO MODELO 02: LEAF DEEPENING & SUPER-POOL (629 CONFS)     "
echo "================================================================================"
echo "1. Executando Verificador Oficial RSST (discharge) em C sobre árvores aprofundadas..."

make -C "$ROOT" bin/discharge
CONF="$DIR/unavoidable_exact_629.conf"
RULES="$ROOT/data/rsst_presentations/rules"

for p in present7 present8 present9 present10 present11; do
    echo -n "  -> Verificando $p (Árvore com Leaf Deepening)... "
    out=$("$ROOT/bin/discharge" "$DIR/$p" "$CONF" "$RULES" 0 1 | grep -i "verified")
    if [ -n "$out" ]; then
        echo "OK ($out)"
    else
        echo "FALHA!"
        exit 1
    fi
done

echo ""
echo "2. Executando Verificador Algébrico em Rust Puro..."
cargo build --release --manifest-path "$ROOT/Cargo.toml" --quiet
"$ROOT/target/release/quatro_cores" verify-file "$CONF" 630

echo ""
echo "================================================================================"
echo " CERTIFICAÇÃO CONCLUÍDA: MODELO DE LEAF DEEPENING 100% VÁLIDO E CERTIFICADO!   "
echo "================================================================================"
