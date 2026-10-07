# Makefile unificado para Teoria das Quatro Cores
# Suporta compilação do motor algébrico (Rust) e dos verificadores de descarregamento (C)

CC ?= gcc
CFLAGS ?= -O3 -Wall -Wextra -Wno-unused-variable -Wno-unused-parameter -Wno-implicit-int -Wno-format -Wno-unused-result
CARGO ?= cargo

.PHONY: all build test verifiers clean verify-25 verify-24 verify-21 verify-10 verify-all help

all: build verifiers

help:
	@echo "=========================================================================="
	@echo " Comandos Disponíveis no Repositório:"
	@echo "=========================================================================="
	@echo "  make build         Compila o motor algébrico em Rust (modo release)"
	@echo "  make test          Executa a suíte de testes unitários em Rust"
	@echo "  make verifiers     Compila todos os executáveis C de descarregamento"
	@echo "  make verify-21     Executa a verificação completa do Recorde de 21 Grafos"
	@echo "  make verify-10     Executa a verificação do Modelo Inverso de 10 Grafos"
	@echo "  make verify-25     Executa a verificação do modelo canônico de 25 Grafos"
	@echo "  make verify-all    Executa a verificação de todos os marcos"
	@echo "  make clean         Remove todos os binários e arquivos intermediários"
	@echo "=========================================================================="

build:
	$(CARGO) build --release

test:
	$(CARGO) test

verifiers: 08_pesquisa_2flips/discharge \
           09_perspectivas_futuras_3flips/discharge_universal \
           10_recorde_21_configuracoes/discharge_universal \
           11_descarregamento_inverso_sub15/discharge_universal \
           11_descarregamento_inverso_sub15/discharge_profiler

08_pesquisa_2flips/discharge: 08_pesquisa_2flips/discharge.c
	$(CC) $(CFLAGS) -o $@ $<

09_perspectivas_futuras_3flips/discharge_universal: 09_perspectivas_futuras_3flips/discharge_universal.c
	$(CC) $(CFLAGS) -o $@ $<

10_recorde_21_configuracoes/discharge_universal: 10_recorde_21_configuracoes/discharge_universal.c
	$(CC) $(CFLAGS) -o $@ $<

11_descarregamento_inverso_sub15/discharge_universal: 11_descarregamento_inverso_sub15/discharge_universal.c
	$(CC) $(CFLAGS) -o $@ $<

11_descarregamento_inverso_sub15/discharge_profiler: 11_descarregamento_inverso_sub15/discharge_profiler.c
	$(CC) $(CFLAGS) -o $@ $<

verify-25: build 08_pesquisa_2flips/discharge
	@echo ">>> Executando verificação do Modelo de 25 Configurações..."
	@cd 08_pesquisa_2flips && ./verify_25.sh

verify-24: build 09_perspectivas_futuras_3flips/discharge_universal
	@echo ">>> Executando verificação do Modelo de 24 Configurações..."
	@cd 09_perspectivas_futuras_3flips && ./verify_24.sh

verify-21: build 10_recorde_21_configuracoes/discharge_universal
	@echo ">>> Executando verificação do Recorde Mundial de 21 Configurações..."
	@cd 10_recorde_21_configuracoes && ./verify_21.sh

verify-10: build 11_descarregamento_inverso_sub15/discharge_profiler
	@echo ">>> Executando verificação do Modelo Inverso de 10 Configurações..."
	@cd 11_descarregamento_inverso_sub15 && ./verify_10_model.sh

verify-all: verify-21 verify-10 verify-25

clean:
	$(CARGO) clean
	@find . -type f \( -name "discharge" -o -name "discharge_track" -o -name "discharge_universal" -o -name "discharge_profiler" -o -name "outlet.et" \) -delete
	@echo "Ambiente limpo com sucesso."
