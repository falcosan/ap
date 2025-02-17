#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WASM_DIR="${SCRIPT_DIR}/wasm"
PKG_DIR="${WASM_DIR}/pkg"
LIB_DIR="${PKG_DIR}/lib"

# Build WebAssembly package
cd "${WASM_DIR}"
wasm-pack build --out-name index --target web
