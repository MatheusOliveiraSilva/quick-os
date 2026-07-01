# quick-os

Agent dispatcher in Rust on **Firecracker microVMs** — snapshot/fork/restore for fast agent isolation and an **observable HTTP tool surface**.

> **Reviewing from mobile?** Start here → **[REVIEW.md](REVIEW.md)** (PR #5)

## Architecture

```
CLI (quick-os)
    │
    ├── Dispatcher ── fork/restore agents from snapshots
    │       └── Firecracker API (Unix socket)
    │
    └── Tool surface (Axum HTTP)
            ├── GET  /health
            ├── GET  /tools
            ├── POST /tools/{name}/invoke
            ├── GET  /agents
            └── GET  /events
```

## Prerequisites

- Linux with **KVM** (`/dev/kvm`)
- Rust stable (1.83+)
- `curl`, `extlinux`, `e2fsprogs` (for dev setup script)

## Quick start

```bash
# 1. Install Firecracker + guest assets (kernel, rootfs)
chmod +x scripts/setup-dev.sh
./scripts/setup-dev.sh

# 2. Build
cargo build

# 3. Check environment
cargo run -p quick-os -- check-env

# 4. Create base snapshot (boots a fresh microVM, pauses, snapshots)
cargo run -p quick-os -- snapshot-create --id base

# 5. Run dispatcher + tool surface
cargo run -p quick-os -- serve
```

## Docker (dev, requires host KVM)

```bash
docker compose -f docker/docker-compose.yml up --build
```

Passes `/dev/kvm` into a privileged container. Without KVM on the host, Firecracker will not run.

## CLI

| Command | Description |
|---------|-------------|
| `check-env` | Validate `/dev/kvm`, firecracker binary, guest assets |
| `snapshot-create --id <name>` | Boot fresh VM → full snapshot |
| `agent-spawn [--snapshot <id>]` | Fork/restore agent from snapshot |
| `serve` | HTTP tool surface on `tools.listen` (default `127.0.0.1:8080`) |

## Tool surface

Built-in tools (observable via `/events`):

| Tool | Description |
|------|-------------|
| `agents.list` | List running agent microVMs |
| `agents.spawn` | Restore VM from snapshot (`snapshot_id` optional) |
| `snapshots.create` | Capture a new base snapshot |

Example:

```bash
curl -s localhost:8080/tools | jq
curl -s -X POST localhost:8080/tools/agents.list/invoke \
  -H 'content-type: application/json' \
  -d '{"input":{}}' | jq
curl -s localhost:8080/events | jq
```

## Workspace layout

```
crates/
  quick-os-core/          # config, errors, agent/snapshot types
  quick-os-firecracker/   # Firecracker HTTP client + VM lifecycle
  quick-os-dispatcher/    # snapshot pool, agent spawn
  quick-os-tools/         # HTTP tool registry + event log
  quick-os/               # CLI binary
configs/quick-os.toml     # runtime config
scripts/setup-dev.sh      # download firecracker + guest assets
docker/                   # dev container with KVM passthrough
```

## Config

See `configs/quick-os.toml`. Paths are relative to the working directory.

## Learning log

See `LOG.md` for step-by-step Rust concepts covered during the build.
