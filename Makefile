# Makefile unificado para Teoria das Quatro Cores
# Suporta compilação do motor algébrico (Rust) e dos verificadores de descarregamento (C)

CC ?= gcc
CFLAGS ?= -O3 -Wall -Wextra -Wno-unused-variable -Wno-unused-parameter -Wno-implicit-int -Wno-format -Wno-unused-result
CARGO ?= cargo

.PHONY: all build test verifiers clean verify-629 verify-exact verify-all help

all: build verifiers bin/discharge

bin:
	mkdir -p bin

bin/discharge: src/discharge.c | bin
	$(CC) $(CFLAGS) -o $@ $<

help:
	@echo "=========================================================================="
	@echo " Comandos Disponíveis no Repositório:"
	@echo "=========================================================================="
	@echo "  make build         Compila o motor algébrico em Rust (modo release)"
	@echo "  make test          Executa a suíte de testes unitários em Rust"
	@echo "  make verifiers     Compila o binário canônico de descarregamento (bin/discharge)"
	@echo "  make verify-629    Executa a verificação do Modelo Canônico 629 (Pasta 01)"
	@echo "  make verify-exact  Executa a verificação do Modelo Leaf Deepening (Pasta 02)"
	@echo "  make verify-all    Executa a verificação de ambos os modelos canônicos"
	@echo "  make clean         Remove todos os binários e arquivos intermediários"
	@echo "=========================================================================="

build:
	$(CARGO) build --release

test:
	$(CARGO) test

verifiers: bin/discharge

verify-629: build bin/discharge
	@echo ">>> Executando verificação do Modelo Canônico RSST 629 (Pasta 01)..."
	@bash 01_modelo_629_rsst_canonico/verify_629.sh

verify-exact: build bin/discharge
	@echo ">>> Executando verificação do Modelo Leaf Deepening (Pasta 02)..."
	@bash 02_modelo_exato_leaf_deepening/verify_629.sh

verify-all: verify-629 verify-exact

clean:
	$(CARGO) clean
	rm -rf bin/
	@find . -type f \( -name "discharge" -o -name "discharge_track" -o -name "outlet.et" \) -delete
	@echo "Ambiente limpo com sucesso."
