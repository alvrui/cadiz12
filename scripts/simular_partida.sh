#!/bin/bash
set -e

echo "=== Simulando partida de Cadiz 1812 ==="
echo ""

cd $(dirname "$0")/..

# Verificar compilacion
echo "1. Verificando compilacion..."
cargo check --quiet 2>&1 | grep -v "Compiling\|Finished" || true

# Ejecutar simulacion
echo ""
echo "2. Ejecutando simulacion de 10 jornadas..."
cargo run --quiet --example simular configs/basic/completa.json 2>&1 || cargo run --example simular configs/basic/completa.json 2>&1

echo ""
echo "=== Simulacion completada ==="