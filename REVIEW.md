# PR #4 — Review guide (mobile-friendly)

> **Branch:** `cursor/quick-os-bb04` → `main`  
> **Commits:** 7 (learning steps 1–5 + full scaffold + demo harness)

Este arquivo existe porque a **descrição do PR no GitHub ficou desatualizada** (API do agent não consegue editar). **Tudo que importa está aqui e no diff.**

---

## O que mudou (TL;DR)

De um `main.rs` de tutorial → **workspace completo** de agent dispatcher:

| Crate | O que faz |
|-------|-----------|
| `quick-os-core` | Config TOML, errors, `AgentId`, `SnapshotRef` |
| `quick-os-firecracker` | Client HTTP via Unix socket, boot/restore/snapshot VM |
| `quick-os-dispatcher` | Pool de agents, spawn via snapshot load |
| `quick-os-tools` | Tool surface HTTP (Axum) + event log |
| `quick-os` | CLI binary |

+ `scripts/setup-dev.sh`, `scripts/demo-ci.sh`, `docker/`, `configs/quick-os.toml`

---

## Diagrama — arquitetura

```mermaid
flowchart TB
    subgraph CLI["quick-os CLI"]
        CE[check-env]
        SC[snapshot-create]
        AS[agent-spawn]
        SV[serve]
    end

    subgraph Core["quick-os-core"]
        CFG[AppConfig]
        AG[AgentId / AgentRecord]
    end

    subgraph FC["quick-os-firecracker"]
        API[FirecrackerClient]
        VM[VmBuilder]
    end

    subgraph DISP["quick-os-dispatcher"]
        D[Dispatcher]
    end

    subgraph TOOLS["quick-os-tools"]
        REG[ToolRegistry]
        HTTP["Axum /health /tools /agents /events"]
        LOG[EventLog]
    end

    CE --> CFG
    SC --> D
    AS --> D
    SV --> D
    SV --> HTTP
    D --> VM --> API
    HTTP --> REG --> D
    REG --> LOG
```

---

## Diagrama — snapshot / fork

```mermaid
sequenceDiagram
    participant U as User ou Tool surface
    participant D as Dispatcher
    participant FC as Firecracker
    participant S as Snapshot files

    Note over U,S: snapshot-create (uma vez)
    U->>D: create_base_snapshot("base")
    D->>FC: boot VM + pause + snapshot/create
    FC->>S: vm_state.snap + mem.snap

    Note over U,S: agent-spawn (fork)
    U->>D: spawn_from_snapshot("base")
    D->>FC: novo processo + snapshot/load (CoW)
    D->>FC: Resume
    D-->>U: AgentRecord
```

---

## Demo rodada (CI — sem KVM)

```text
════════════════════════════════════════
  BUILD
════════════════════════════════════════
    Finished `dev` profile [unoptimized + debuginfo] target(s)

════════════════════════════════════════
  CLI
════════════════════════════════════════
Usage: quick-os [OPTIONS] <COMMAND>

Commands:
  check-env        Validate host prerequisites
  snapshot-create  Boot a fresh VM and capture a base snapshot
  agent-spawn      Restore/fork an agent VM from snapshot
  serve            Run dispatcher + observable HTTP tool surface

════════════════════════════════════════
  CHECK-ENV (CI)
════════════════════════════════════════
  /dev/kvm:              MISSING      ← esperado no CI
  firecracker binary:    MISSING
  guest kernel:          MISSING
  guest rootfs:          MISSING

════════════════════════════════════════
  SMOKE TESTS (no KVM)
════════════════════════════════════════
test health_endpoint_returns_ok ... ok
test tools_endpoint_lists_builtin_tools ... ok
```

Reproduzir: `./scripts/demo-ci.sh`

---

## Onde olhar no diff (ordem sugerida)

1. **`REVIEW.md`** (este arquivo)
2. **`LOG.md`** — log de aprendizado + harness mobile/PR
3. **`README.md`** — quick start com KVM
4. **`crates/quick-os/src/main.rs`** — CLI entry
5. **`crates/quick-os-dispatcher/src/dispatcher.rs`** — orquestração
6. **`crates/quick-os-firecracker/src/vm.rs`** — lifecycle Firecracker
7. **`crates/quick-os-tools/src/server.rs`** — HTTP tool surface
8. **`scripts/demo-ci.sh`** — demo sem KVM

---

## Na tua máquina (com KVM)

```bash
./scripts/setup-dev.sh
cargo run -p quick-os -- check-env
cargo run -p quick-os -- snapshot-create --id base
cargo run -p quick-os -- serve
curl localhost:8080/tools
```

---

## Perguntas

Deixa inline **neste PR** — respondo no comment ou aqui no agent.
