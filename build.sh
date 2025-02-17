#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WASM_DIR="${SCRIPT_DIR}/src"

cd "${WASM_DIR}"
wasm-pack build --out-name index --target web

cd "${SCRIPT_DIR}"
rsbuild build
