#!/bin/bash
set -e

DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$DIR/.." && pwd)"

echo "================================================================================"
echo "          VERIFICAÇÃO FORMAL DO MODELO CANÔNICO RSST (629 CONFIGURAÇÕES)        "
echo "================================================================================"
echo "1. Executando Verificador Oficial RSST (discharge) em C..."

make -C "$ROOT" bin/discharge
CONF="$DIR/unavoidable_629.conf"
DATA="$ROOT/data/rsst_presentations"

for p in present7 present8 present9 present10 present11; do
    echo -n "  -> Verificando $p... "
    out=$("$ROOT/bin/discharge" "$DATA/$p" "$CONF" "$DATA/rules" 0 1 | grep -i "verified")
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
echo " CERTIFICAÇÃO CONCLUÍDA: 629/629 CONFIGURAÇÕES 100% VÁLIDAS E INEVITÁVEIS!      "
echo "================================================================================"
