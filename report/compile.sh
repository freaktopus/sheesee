#!/bin/bash
# Compile LaTeX Report for Sheesee Project
# This script compiles the report multiple times to resolve all references

echo "========================================"
echo "  Sheesee Report Compilation Script"
echo "========================================"
echo

# Check if pdflatex is available
if ! command -v pdflatex &> /dev/null; then
    echo "ERROR: pdflatex not found in PATH"
    echo "Please install TeX Live: sudo apt-get install texlive-full"
    echo
    exit 1
fi

echo "[1/3] First compilation pass..."
pdflatex -interaction=nonstopmode main.tex
if [ $? -ne 0 ]; then
    echo "ERROR: Compilation failed. Check main.log for details."
    exit 1
fi

echo
echo "[2/3] Second compilation pass (resolving references)..."
pdflatex -interaction=nonstopmode main.tex

echo
echo "[3/3] Final compilation pass..."
pdflatex -interaction=nonstopmode main.tex

echo
echo "========================================"
echo "  Compilation Complete!"
echo "========================================"
echo
echo "Output: main.pdf"
echo

# Check if PDF was created
if [ -f main.pdf ]; then
    echo "Success! PDF created successfully."
    
    # Try to open the PDF with default viewer
    if command -v xdg-open &> /dev/null; then
        xdg-open main.pdf
    elif command -v open &> /dev/null; then
        open main.pdf
    fi
else
    echo "ERROR: PDF was not created. Check main.log for errors."
    exit 1
fi

echo
