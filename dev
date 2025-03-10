#!/usr/bin/env bash

set -euo pipefail
IFS=$'\n\t'

readonly CARGO_CMD="cargo run"
readonly LOG_PREFIX="[DEV]"
readonly PORT=8000

log() {
  echo "${LOG_PREFIX} $1"
}

kill_old_port_process() {
  local pids
  pids=$(lsof -ti:"$PORT" 2>/dev/null || true)
  if [[ -n "$pids" ]]; then
    for pid in $pids; do
      log "Killing process using port $PORT (PID: $pid)"
      kill -9 "$pid" 2>/dev/null || true
    done

    kill_old_port_process
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

  kill_old_port_process

  log "Starting cargo run"
  cargo watch -q -i .gitignore -s "${CARGO_CMD}" &
  cargo_watch_pid=$!
  log "cargo run running with PID: $cargo_watch_pid"
  
  wait "$cargo_watch_pid"
}

main "$@"