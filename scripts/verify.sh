#!/usr/bin/env bash
# ==============================================================================
# CLI Unificada de Verificação do Teorema das Quatro Cores
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$ROOT_DIR"

# Cores ANSI para formatação elegante
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
RED='\033[0;31m'
BOLD='\033[1m'
NC='\033[0m'

print_header() {
    echo -e "${CYAN}==============================================================================${NC}"
    echo -e "${BOLD}${CYAN}   SISTEMA DE VERIFICAÇÃO FORMAL UNIFICADO - TEOREMA DAS QUATRO CORES        ${NC}"
    echo -e "${CYAN}==============================================================================${NC}"
}

usage() {
    echo -e "${BOLD}Uso:${NC} ./scripts/verify.sh [ALVO]"
    echo ""
    echo -e "${BOLD}Alvos disponíveis:${NC}"
    echo -e "  ${GREEN}exact${NC}  - [MODELO 02] Modelo Exato com Leaf Deepening & Super-Pool (Pasta 02)"
    echo -e "  ${GREEN}629${NC}    - [MODELO 01] Modelo Canônico Histórico RSST 629 (Pasta 01)"
    echo -e "  ${GREEN}all${NC}    - Executa a verificação completa de todos os modelos estritos"
    echo -e "  ${GREEN}test${NC}   - Executa a suíte de testes unitários do motor algébrico em Rust"
    echo ""
    exit 1
}

TARGET="${1:-exact}"

print_header

case "$TARGET" in
    exact|02)
        echo -e "${BOLD}${BLUE}>>> Executando Verificação do Modelo 02: Leaf Deepening (Pasta 02)...${NC}\n"
        make verify-exact
        ;;
    629|01)
        echo -e "${BOLD}${BLUE}>>> Executando Verificação do Modelo Canônico RSST 629 (Pasta 01)...${NC}\n"
        make verify-629
        ;;
    test)
        echo -e "${BOLD}${BLUE}>>> Executando Suíte de Testes Algébricos em Rust...${NC}\n"
        cargo test
        ;;
    all)
        echo -e "${BOLD}${BLUE}>>> Executando Verificação Completa de Todos os Modelos...${NC}\n"
        make verify-all
        ;;
    *)
        usage
        ;;
esac

echo ""
echo -e "${BOLD}${GREEN}==============================================================================${NC}"
echo -e "${BOLD}${GREEN}                   VERIFICAÇÃO FORMAL CONCLUÍDA COM SUCESSO!                  ${NC}"
echo -e "${BOLD}${GREEN}==============================================================================${NC}"
