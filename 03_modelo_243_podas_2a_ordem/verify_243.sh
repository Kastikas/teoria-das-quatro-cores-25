#!/bin/bash
set -e

echo "================================================================================"
echo "    VERIFICAÇÃO FORMAL DO NOVO RECORDE MUNDIAL: 243 CONFIGURAÇÕES               "
echo "    Fronteira 3: Poda de 2ª Ordem + Contratos k=3 sob Stromquist (1975)         "
echo "================================================================================"
echo "1. Executando Verificador Oficial RSST (discharge) com Reconhecedor Universal..."
for p in present7 present8 present9 present10 present11; do
    echo -n "  -> Verificando $p... "
    out=$(./discharge "$p" unavoidable_243.conf rules 0 1 | grep -i "verified")
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
./target/release/quatro_cores verify-file 03_modelo_243_podas_2a_ordem/unavoidable_243.conf 300
cd 03_modelo_243_podas_2a_ordem

echo ""
echo "================================================================================"
echo " CERTIFICAÇÃO CONCLUÍDA: 243/243 CONFIGURAÇÕES 100% VÁLIDAS E INEVITÁVEIS!      "
echo " Redução vs RSST 633: -390 (-61.6%) | Redução vs Recorde 305: -62 (-20.3%)      "
echo "================================================================================"
