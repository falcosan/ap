#!/usr/bin/env bash

set -euo pipefail

command -v cargo >/dev/null

command -v cargo-upgrade >/dev/null || cargo install cargo-edit --locked

cargo upgrade -i allow

cargo update
