#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

readonly CARGO_CMD="cargo run"
readonly LOG_PREFIX="[DEV]"

log() {
  echo "${LOG_PREFIX} $1"
}

cleanup() {
  echo
  if [[ -n "${cargo_watch_pid:-}" ]]; then
    log "Terminating cargo watch with PID: $cargo_watch_pid"
    kill "$cargo_watch_pid" &>/dev/null || true
    wait "$cargo_watch_pid" &>/dev/null || true
  fi
  log "Cleanup complete"
  exit 0
}

main() {
  trap cleanup SIGINT SIGTERM SIGHUP

  log "Starting cargo watch"
  cargo watch -q -i .gitignore -s "${CARGO_CMD}" &
  cargo_watch_pid=$!

  log "Cargo watch running with PID: $cargo_watch_pid"
  wait "$cargo_watch_pid"
}

main "$@"