# quick-os + agent-os

**Host orchestrator** (quick-os) + **Agent Operating Environment** (agent-os) on Firecracker microVMs.

> **Review PR #5:** [REVIEW.md](REVIEW.md) · **Architecture:** [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)  
> **Mobile / diagramas PNG:** [docs/DIAGRAMS.md](docs/DIAGRAMS.md) · **Python→Rust:** [docs/RUST-CONCEPTS.md](docs/RUST-CONCEPTS.md)

## Pivot (Path A)

| Component | What |
|-----------|------|
| `agent-os` | Guest runtime — sandboxed workspace, tools, events (**the product**) |
| `quick-os` | Host — spawn, snapshot, HTTP tool surface |

## Quick demo (no KVM)

```bash
cargo build
./scripts/demo-agent-os.sh
```

## Full stack (Linux + KVM)

```bash
./scripts/setup-dev.sh
cargo run -p quick-os -- check-env
cargo run -p quick-os -- snapshot-create --id base
cargo run -p quick-os -- serve
```

## v1 primitives

- `read_workspace` — sandboxed file read
- `run_tool` — builtin tool executor (`echo`, `pwd`)
- `emit_event` — structured agent events

Protocol: `quick-os-core::guest_protocol`

## Workspace

```
crates/agent-os/          guest runtime ★
crates/quick-os/          host CLI
crates/quick-os-core/     shared protocol
crates/quick-os-firecracker/
crates/quick-os-dispatcher/
crates/quick-os-tools/
```

## CI

`fmt` · `clippy -D warnings` · `test` · `demo-ci.sh` · `demo-agent-os.sh`
