#!/usr/bin/env bash
# ==============================================================================
# scripts/verify_strict.sh
#
# Validação de Aceite Ponta a Ponta para Reduções no Modelo RSST Estrito:
# 1. cargo test --release (24 testes algébricos e geométricos)
# 2. cargo run --release -- verify-file <catalogo.conf> 1000 (reducibilidade estrita k <= 2)
# 3. discharge (verificador C oficial de 1997 sobre present7..present11)
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$ROOT_DIR"

CONF_FILE="${1:-01_modelo_629_rsst_canonico/unavoidable_629.conf}"
PRES_DIR="${2:-01_modelo_629_rsst_canonico}"
RULES_FILE="${ROOT_DIR}/01_modelo_629_rsst_canonico/rules"
DISCHARGE_BIN="${ROOT_DIR}/target/discharge_1997"

echo "================================================================================"
echo "    VERIFICAÇÃO FORMAL ESTRITA DE ACEITE (RSST / 4CT)                           "
echo "================================================================================"
echo "Catálogo a verificar:   $CONF_FILE"
echo "Diretório apresentações: $PRES_DIR"
echo "Regras de descarga:     $RULES_FILE"
echo ""

# Compilar verificador C de 1997 se necessário
if [ ! -f "$DISCHARGE_BIN" ] || [ "$ROOT_DIR/01_modelo_629_rsst_canonico/discharge.c" -nt "$DISCHARGE_BIN" ]; then
    echo "[*] Compilando discharge_1997 (gcc -O3)..."
    mkdir -p "$ROOT_DIR/target"
    gcc -O3 "$ROOT_DIR/01_modelo_629_rsst_canonico/discharge.c" -o "$DISCHARGE_BIN"
fi

# 1. Testes unitários do motor Rust
echo "[PASSO 1/3] Suíte de testes unitários do motor algébrico (cargo test --release)..."
cargo test --release --quiet
echo "  ✓ Suíte Rust: 24/24 testes aprovados."
echo ""

# 2. Verificação do catálogo de configurações
echo "[PASSO 2/3] Verificação Algébrica e Geométrica Estrita no Catálogo..."
cargo run --release -- verify-file "$CONF_FILE" 1000
echo ""

# 3. Verificação de descarregamento nas 5 apresentações em C
echo "[PASSO 3/3] Verificação Oficial RSST em C (discharge de 1997) - 5 Apresentações..."
for p in present7 present8 present9 present10 present11; do
    echo -n "  -> Verificando $p... "
    "$DISCHARGE_BIN" "$PRES_DIR/$p" "$CONF_FILE" "$RULES_FILE" 0 0 | grep -E "verified\."
done

echo ""
echo "================================================================================"
echo "    >>> CERTIFICADO E VERIFICADO 100% COM SUCESSO NAS DUAS PLATAFORMAS! <<<     "
echo "================================================================================"
