@echo off
REM Compile LaTeX Report for Sheesee Project
REM This script compiles the report multiple times to resolve all references

echo ========================================
echo   Sheesee Report Compilation Script
echo ========================================
echo.

REM Check if pdflatex is available
where pdflatex >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: pdflatex not found in PATH
    echo Please install MiKTeX or TeX Live
    echo.
    pause
    exit /b 1
)

echo [1/3] First compilation pass...
pdflatex -interaction=nonstopmode main.tex
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Compilation failed. Check main.log for details.
    pause
    exit /b 1
)

echo.
echo [2/3] Second compilation pass (resolving references)...
pdflatex -interaction=nonstopmode main.tex

echo.
echo [3/3] Final compilation pass...
pdflatex -interaction=nonstopmode main.tex

echo.
echo ========================================
echo   Compilation Complete!
echo ========================================
echo.
echo Output: main.pdf
echo.

REM Check if PDF was created
if exist main.pdf (
    echo Success! Opening PDF...
    start main.pdf
) else (
    echo ERROR: PDF was not created. Check main.log for errors.
)

echo.
pause
