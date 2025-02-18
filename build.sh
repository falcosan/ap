#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SRC_DIR="${SCRIPT_DIR}/src"
WASM_DIR="${SRC_DIR}/wasm"

if ! command -v wasm-pack >/dev/null 2>&1; then
  echo "Error: wasm-pack is not installed. Please install it before running this script." >&2
  exit 1
fi

echo "Building WebAssembly module..."
cd "${SRC_DIR}"
wasm-pack build --out-name index --target web --out-dir "${WASM_DIR}"

echo "Cleaning up unnecessary files..."
rm -f "${WASM_DIR}/.gitignore"
rm -f "${WASM_DIR}/package.json"

cd "${SCRIPT_DIR}"

echo "Building the Rust project..."
if ! command -v rsbuild >/dev/null 2>&1; then
  echo "Error: rsbuild is not installed. Please install it before running this script." >&2
  exit 1
fi

rsbuild build