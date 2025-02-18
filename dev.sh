#!/usr/bin/env bash

set -euo pipefail

cleanup() {
    kill $(jobs -p) 2>/dev/null
    exit 0
}

trap cleanup SIGINT

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
WASM_DIR="${SCRIPT_DIR}/src"

cd "${WASM_DIR}"
cargo watch -q -i .gitignore -i "wasm/*" -s "wasm-pack build --out-name index --target web --out-dir wasm" &

cd "${SCRIPT_DIR}"
rsbuild dev &

wait
