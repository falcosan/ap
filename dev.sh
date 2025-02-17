#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WASM_DIR="${SCRIPT_DIR}/rust"
PKG_DIR="${WASM_DIR}/pkg"

cd "${WASM_DIR}"
cargo watch -i .gitignore -i "pkg/*" -s "wasm-pack build --out-name index --target web"


cd "${SCRIPT_DIR}"
rsbuild rsbuild dev --open