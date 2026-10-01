#!/bin/bash
set -e

echo "Compilando artigo formal em LaTeX..."
pdflatex -interaction=nonstopmode main.tex
bibtex main
pdflatex -interaction=nonstopmode main.tex
pdflatex -interaction=nonstopmode main.tex

echo "Sucesso! Artigo gerado em paper/main.pdf"
