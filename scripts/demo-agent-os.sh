#!/usr/bin/env bash
# Demo: host talks to agent-os guest runtime via stdio (simulates future vsock).
set -euo pipefail

export PATH="/usr/local/cargo/bin:${PATH}"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

WS="$(mktemp -d)"
trap 'rm -rf "$WS"' EXIT

echo "==> agent-os pivot demo"
echo "workspace: $WS"
echo "hello from agent-os" > "$WS/hello.txt"

export AGENT_OS_WORKSPACE="$WS"

run_req() {
  local label="$1"
  local json="$2"
  echo
  echo "--- $label ---"
  echo ">> $json"
  resp="$(echo "$json" | cargo run -q -p agent-os 2>/dev/null | tail -1)"
  echo "<< $resp"
}

run_req "read_workspace" '{"op":"read_workspace","path":"hello.txt"}'
run_req "run_tool echo" '{"op":"run_tool","name":"echo","input":{"message":"pivot ok"}}'
run_req "emit_event" '{"op":"emit_event","kind":"agent.started","payload":{"model":"demo"}}'

echo
echo "==> demo complete (guest runtime v1 primitives OK)"
