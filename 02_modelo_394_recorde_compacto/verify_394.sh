#!/bin/bash
set -e

echo "================================================================================"
echo "          VERIFICAÇÃO FORMAL DO MODELO COMPACTO RECORDE (394 CONFIGURAÇÕES)     "
echo "================================================================================"
echo "1. Executando Verificador Oficial RSST (discharge) em C..."
for p in present7 present8 present9 present10 present11; do
    echo -n "  -> Verificando $p... "
    out=$(./discharge "$p" unavoidable_394.conf rules 0 1 | grep -i "verified")
    if [ -n "$out" ]; then
        echo "OK ($out)"
    else
        echo "FALHA!"
        exit 1
    fi
done

echo ""
echo "2. Executando Verificador Algébrico em Rust Puro..."
cd ..
./target/release/quatro_cores verify-file 02_modelo_394_recorde_compacto/unavoidable_394.conf 400
cd 02_modelo_394_recorde_compacto

echo ""
echo "================================================================================"
echo " CERTIFICAÇÃO CONCLUÍDA: 394/394 CONFIGURAÇÕES 100% VÁLIDAS E INEVITÁVEIS!      "
echo "================================================================================"
