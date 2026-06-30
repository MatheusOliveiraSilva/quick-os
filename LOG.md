# quick-os — log de aprendizado

Projeto pessoal: agent dispatcher em Rust (Firecracker microVMs, snapshot/fork/restore, observable tool surface).

---

## Passos 1–5 (fundamentos Rust)

Cobertos na branch inicial: Cargo, `Result`, `?`, CLI args, `Option`, struct + ownership.
Ver histórico de commits `ce26e43..a057555`.

---

## Passo 6 — Scaffold completo do dispatcher

**O que construímos**

Workspace Cargo com 5 crates:

| Crate | Responsabilidade |
|-------|------------------|
| `quick-os-core` | `AppConfig`, `AgentId`, `SnapshotRef`, `QuickOsError` |
| `quick-os-firecracker` | HTTP client via Unix socket, boot/restore/snapshot VM |
| `quick-os-dispatcher` | Orquestra agents, fork via snapshot restore |
| `quick-os-tools` | Tool registry + Axum HTTP (`/tools`, `/agents`, `/events`) |
| `quick-os` | CLI: `check-env`, `snapshot-create`, `agent-spawn`, `serve` |

Ambiente de dev:

- `configs/quick-os.toml` — config runtime
- `scripts/setup-dev.sh` — download Firecracker, kernel Alpine, rootfs ext4
- `docker/docker-compose.yml` — dev container com `/dev/kvm` passthrough

**Conceito para lembrar**

> **Snapshot/fork** aqui é restore de microVM a partir de `vm_state.snap` + `mem.snap` — CoW via `enable_diff_snapshots: true` no load. Dispatcher não re-boota kernel; **restaura estado** e resume. Tool surface expõe cada operação como tool invocável + event log observável.

**Próximo**

- Guest agent in-VM (comunicação host ↔ microVM)
- Jailer + network namespace por agent
- Dirty-page tracking para snapshot incremental
