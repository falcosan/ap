#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
WASM_DIR="${SCRIPT_DIR}/src"

cd "${WASM_DIR}" &>/dev/null
cargo watch -q -i .gitignore -i "pkg/*" -s "wasm-pack build --out-name index --target web"
