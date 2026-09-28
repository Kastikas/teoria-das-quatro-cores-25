#!/bin/bash
set -e

echo "================================================================================"
echo "    VERIFICAÇÃO FORMAL DO NOVO RECORDE MUNDIAL: 305 CONFIGURAÇÕES               "
echo "    Fronteira 1: Superconfigurações com 4 conexões ao anel + Stromquist         "
echo "================================================================================"
echo "1. Executando Verificador Oficial RSST (discharge) com Reconhecedor Universal..."
for p in present7 present8 present9 present10 present11; do
    echo -n "  -> Verificando $p... "
    out=$(./discharge "$p" unavoidable_305.conf rules 0 1 | grep -i "verified")
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
./target/release/quatro_cores verify-file 03_pesquisa_sub394_nova_fronteira/unavoidable_305.conf 350
cd 03_pesquisa_sub394_nova_fronteira

echo ""
echo "================================================================================"
echo " CERTIFICAÇÃO CONCLUÍDA: 305/305 CONFIGURAÇÕES 100% VÁLIDAS E INEVITÁVEIS!      "
echo " Redução vs RSST 633: -328 (-51.8%) | Redução vs Recorde 394: -89 (-22.6%)      "
echo "================================================================================"
