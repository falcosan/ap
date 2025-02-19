#!/usr/bin/env bash

set -euo pipefail

cleanup() {
    kill $(jobs -p) 2>/dev/null
    exit 0
}

trap cleanup SIGINT

npx @tailwindcss/cli -i ./src/assets/input.css -o ./src/assets/index.css --watch &
cargo watch -s "cargo run" &

wait
