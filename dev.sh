#!/usr/bin/env bash

set -euo pipefail

cargo watch -q -i .gitignore -s "cargo run"