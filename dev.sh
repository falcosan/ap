#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

readonly CARGO_CMD="cargo run"
readonly LOG_PREFIX="[DEV]"
readonly PORT=8888

log() {
  echo "${LOG_PREFIX} $1"
}

kill_port_process() {
  local pid
  pid=$(lsof -ti:"$PORT" 2>/dev/null)
  if [[ -n "$pid" ]]; then
    log "Killing process using port $PORT (PID: $pid)"
    kill -9 "$pid" 2>/dev/null || true
  fi
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

  if [[ "${RUST_BACKTRACE:-}" == "1" ]]; then
    kill_port_process
  fi

  log "Starting cargo run"
  cargo watch -q -i .gitignore -s "${CARGO_CMD}" &
  cargo_watch_pid=$!

  log "cargo run running with PID: $cargo_watch_pid"
  wait "$cargo_watch_pid"
}

main "$@"