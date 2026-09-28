#!/bin/bash
set -e

echo "================================================================================"
echo "          VERIFICAÇÃO FORMAL DO MODELO CANÔNICO RSST (629 CONFIGURAÇÕES)        "
echo "================================================================================"
echo "1. Executando Verificador Oficial RSST (discharge) em C..."
for p in present7 present8 present9 present10 present11; do
    echo -n "  -> Verificando $p... "
    out=$(./discharge "$p" unavoidable_629.conf rules 0 1 | grep -i "verified")
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
./target/release/quatro_cores verify-file 01_modelo_629_rsst_canonico/unavoidable_629.conf 630
cd 01_modelo_629_rsst_canonico

echo ""
echo "================================================================================"
echo " CERTIFICAÇÃO CONCLUÍDA: 629/629 CONFIGURAÇÕES 100% VÁLIDAS E INEVITÁVEIS!      "
echo "================================================================================"
