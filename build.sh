#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SRC_DIR="${SCRIPT_DIR}/src"
WASM_DIR="${SRC_DIR}/wasm"

if ! command -v wasm-pack >/dev/null 2>&1; then
  rsbuild build
  exit 0
fi

echo "Building WebAssembly module..."
cd "${SRC_DIR}"
wasm-pack build --out-name index --target web --out-dir "${WASM_DIR}"

echo "Cleaning up unnecessary files..."
rm -f "${WASM_DIR}/.gitignore"
rm -f "${WASM_DIR}/package.json"

cd "${SCRIPT_DIR}"

rsbuild build