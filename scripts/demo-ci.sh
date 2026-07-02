#!/usr/bin/env bash
# Demo harness for mobile/CI review — runs everything that works WITHOUT /dev/kvm.
set -uo pipefail

export PATH="/usr/local/cargo/bin:${PATH}"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

section() { echo; echo "════════════════════════════════════════"; echo "  $1"; echo "════════════════════════════════════════"; }

section "BUILD"
cargo build

section "CLI"
cargo run -p quick-os -- --help
echo
cargo run -p quick-os -- check-env || true

section "SMOKE TESTS (no KVM)"
cargo test -p quick-os-tools -- --nocapture

section "CONFIG"
echo "--- configs/quick-os.toml ---"
cat configs/quick-os.toml

section "KVM NOTE"
echo "Full Firecracker flow (snapshot-create / agent-spawn / serve with real VMs)"
echo "requires /dev/kvm + ./scripts/setup-dev.sh on a Linux host."
echo "Review PR #5 from phone; ask questions inline on the PR."
