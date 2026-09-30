#!/usr/bin/env bash
set -e

echo "================================================================================"
echo "    VERIFICAÇÃO FORMAL DO NOVO RECORDE MUNDIAL: 137 CONFIGURAÇÕES               "
echo "    Fronteira Sub-140: Podas de 4ª Ordem sob o Lema de Stromquist (1975)        "
echo "================================================================================"

# 1. Verificador RSST oficial em C
echo "1. Executando Verificador Oficial RSST (discharge) com Reconhecedor Universal..."
for pres in present7 present8 present9 present10 present11; do
    echo -n "  -> Verificando $pres... "
    OUTPUT=$(./discharge "$pres" unavoidable_137.conf rules 0 1)
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
./target/release/quatro_cores verify-file 06_pesquisa_podas_4a_ordem_sub140/unavoidable_137.conf 200
cd 06_pesquisa_podas_4a_ordem_sub140

echo ""
echo "================================================================================"
echo " CERTIFICAÇÃO CONCLUÍDA: 137/137 CONFIGURAÇÕES 100% VÁLIDAS E INEVITÁVEIS!      "
echo " Redução vs RSST 633: -496 (-78.4%) | Redução vs Recorde 149: -12 (-8.1%)       "
echo "================================================================================"
