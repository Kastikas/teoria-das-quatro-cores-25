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
    echo -e "  ${GREEN}21${NC}    - [RECORDE MUNDIAL] Modelo Canônico de 21 Configurações (Pasta 10)"
    echo -e "  ${GREEN}10${NC}    - [FRONTEIRA SUB-15] Modelo Inverso de 10 Configurações + 75 Regras (Pasta 11)"
    echo -e "  ${GREEN}25${NC}    - Modelo Canônico Clássico de 25 Configurações (Pasta 08)"
    echo -e "  ${GREEN}24${NC}    - Modelo com Isomorfismo D_22 de 24 Configurações (Pasta 09)"
    echo -e "  ${GREEN}all${NC}   - Executa a verificação completa de todos os modelos sequencialmente"
    echo -e "  ${GREEN}test${NC}  - Executa a suíte de testes unitários do motor algébrico em Rust"
    echo ""
    exit 1
}

TARGET="${1:-21}"

print_header

case "$TARGET" in
    21)
        echo -e "${BOLD}${BLUE}>>> Executando Verificação do Recorde Mundial: 21 Configurações (Pasta 10)...${NC}\n"
        make verify-21
        ;;
    10)
        echo -e "${BOLD}${BLUE}>>> Executando Verificação da Fronteira Inversa: 10 Configurações (Pasta 11)...${NC}\n"
        make verify-10
        ;;
    25)
        echo -e "${BOLD}${BLUE}>>> Executando Verificação do Modelo Canônico: 25 Configurações (Pasta 08)...${NC}\n"
        make verify-25
        ;;
    24)
        echo -e "${BOLD}${BLUE}>>> Executando Verificação do Modelo de 24 Configurações (Pasta 09)...${NC}\n"
        make verify-24
        ;;
    test)
        echo -e "${BOLD}${BLUE}>>> Executando Suíte de Testes Algébricos em Rust...${NC}\n"
        cargo test
        ;;
    all)
        echo -e "${BOLD}${BLUE}>>> Executando Verificação Geral de Todos os Modelos...${NC}\n"
        START_TIME=$(date +%s)
        
        echo -e "${YELLOW}[1/4] Suíte de Testes Rust...${NC}"
        cargo test --quiet
        echo -e "${GREEN}✓ Testes em Rust: OK${NC}\n"

        echo -e "${YELLOW}[2/4] Recorde Mundial de 21 Configurações (Pasta 10)...${NC}"
        make verify-21
        echo -e "${GREEN}✓ Modelo de 21 Configurações: OK${NC}\n"

        echo -e "${YELLOW}[3/4] Modelo Inverso de 10 Configurações (Pasta 11)...${NC}"
        make verify-10
        echo -e "${GREEN}✓ Modelo de 10 Configurações: OK${NC}\n"

        echo -e "${YELLOW}[4/4] Modelo Canônico de 25 Configurações (Pasta 08)...${NC}"
        make verify-25
        echo -e "${GREEN}✓ Modelo de 25 Configurações: OK${NC}\n"

        END_TIME=$(date +%s)
        ELAPSED=$((END_TIME - START_TIME))
        echo -e "${CYAN}==============================================================================${NC}"
        echo -e "${BOLD}${GREEN}✓ TODOS OS MARCOS FORAM AUDITADOS E VERIFICADOS COM SUCESSO EM ${ELAPSED}s!${NC}"
        echo -e "${CYAN}==============================================================================${NC}"
        ;;
    *)
        usage
        ;;
esac
