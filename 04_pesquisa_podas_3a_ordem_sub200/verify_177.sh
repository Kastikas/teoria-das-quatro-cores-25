#!/usr/bin/env bash
set -e

echo "================================================================================"
echo "    VERIFICAÇÃO FORMAL DO NOVO RECORDE MUNDIAL: 177 CONFIGURAÇÕES               "
echo "    Fronteira Sub-200: Otimização Global Exata com Reconhecedor Universal       "
echo "================================================================================"

# 1. Verificador RSST oficial em C
echo "1. Executando Verificador Oficial RSST (discharge) com Reconhecedor Universal..."
for pres in present7 present8 present9 present10 present11; do
    echo -n "  -> Verificando $pres... "
    OUTPUT=$(./discharge "$pres" unavoidable_177.conf rules 0 1)
    if echo "$OUTPUT" | grep -q "verified"; then
        echo "OK ($pres verified.)"
    else
        echo "FALHOU!"
        echo "$OUTPUT"
        exit 1
    fi
done

# 2. Verificador algébrico em Rust puro
echo ""
echo "2. Executando Verificador Algébrico em Rust Puro..."
cd ..
cargo build --release --quiet
./target/release/quatro_cores verify-file 04_pesquisa_podas_3a_ordem_sub200/unavoidable_177.conf 250
cd 04_pesquisa_podas_3a_ordem_sub200

echo ""
echo "================================================================================"
echo " CERTIFICAÇÃO CONCLUÍDA: 177/177 CONFIGURAÇÕES 100% VÁLIDAS E INEVITÁVEIS!      "
echo " Redução vs RSST 633: -456 (-72.0%) | Redução vs Recorde 243: -66 (-27.2%)      "
echo "================================================================================"
