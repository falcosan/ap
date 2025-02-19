#!/usr/bin/env bash

set -euo pipefail

cleanup() {
    kill $(jobs -p) 2>/dev/null
    exit 0
}

trap cleanup SIGINT

cargo watch -q -i .gitignore -s "cargo run" &

npx @tailwindcss/cli -i ./src/assets/input.css -o ./src/assets/index.css --watch

wait
