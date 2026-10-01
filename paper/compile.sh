#!/bin/bash
set -e

echo "=== [1/2] Compilando versão em Inglês (paper/main.pdf) ==="
pdflatex -interaction=nonstopmode main.tex
bibtex main
pdflatex -interaction=nonstopmode main.tex
pdflatex -interaction=nonstopmode main.tex

echo "=== [2/2] Compilando versão em Português PT-BR (paper/main_pt.pdf) ==="
pdflatex -interaction=nonstopmode main_pt.tex
bibtex main_pt
pdflatex -interaction=nonstopmode main_pt.tex
pdflatex -interaction=nonstopmode main_pt.tex

echo "=========================================================="
echo "Sucesso! Artigos gerados com perfeição:"
echo "  - Inglês:    paper/main.pdf"
echo "  - Português: paper/main_pt.pdf"
echo "=========================================================="
