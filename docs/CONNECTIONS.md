# Como tudo se conecta — mapa do sistema

Documento para não se perder entre crates. **No celular, abra os PNGs** (mermaid do GitHub nem sempre renderiza no app).

Índice completo de imagens: [DIAGRAMS.md](DIAGRAMS.md)

---

## Diagrama principal — conexões atuais

![Como os crates se conectam hoje](https://raw.githubusercontent.com/MatheusOliveiraSilva/quick-os/main/docs/images/current-connections.png)

**Link direto (mobile):**

https://raw.githubusercontent.com/MatheusOliveiraSilva/quick-os/main/docs/images/current-connections.png

---

## Fluxo 1 — Demo sem KVM (funciona hoje)

```
scripts/demo-agent-os.sh
        │
        ▼ JSON line (GuestRequest)
agent-os main.rs (stdio)
        │
        ▼
AgentRuntime.handle()
        ├── Workspace.read_file()
        ├── BuiltinTool.run()
        └── events.push()
        │
        ▼ JSON line (GuestResponse)
stdout → script
```

**Não passa pelo host.** Simula o que o vsock fará depois.

---

## Fluxo 2 — Host spawn agent (precisa KVM)

```
curl POST /tools/agents.spawn/invoke
        │
        ▼
ToolServer → ToolRegistry.invoke()
        │
        ▼
Dispatcher.spawn_from_snapshot()
        ├── VmBuilder.restore_from_snapshot()
        │       └── FirecrackerClient → Firecracker API
        └── HashMap.insert(AgentId, AgentEntry { record, _vm })
        │
        ▼
microVM rodando (agent-os dentro do rootfs — futuro)
```

**Lacuna atual:** host spawna VM mas **ainda não envia** `GuestRequest` pra guest via vsock.

---

## Fluxo 3 — Criar snapshot base

```
quick-os snapshot-create --id base
        │
        ▼
Dispatcher.create_base_snapshot()
        ├── VmBuilder.boot_fresh()     → boot VM limpa
        ├── sleep (placeholder)        → TODO: health check guest
        ├── VmBuilder.create_snapshot()
        └── drop VmHandle              → RAII mata processo boot
```

---

## Tabela — quem chama quem

| De | Para | Como |
|----|------|------|
| `quick-os main` | `Dispatcher` | `Dispatcher::new(config)` |
| `quick-os main` | `ToolServer` | `Arc` chain: dispatcher → registry → server |
| `ToolServer` | `ToolRegistry` | Axum `State`, clone de `Arc` |
| `ToolRegistry` | `Dispatcher` | `Arc<Dispatcher>` |
| `ToolRegistry` | `EventLog` | `clone()` barato (`Arc` interno) |
| `Dispatcher` | `VmBuilder` | owned, criado no `new()` |
| `VmBuilder` | `FirecrackerClient` | por socket path da VM |
| `AgentEntry` | `VmHandle` | campo `_vm` — RAII |
| `agent-os main` | `AgentRuntime` | owned, loop stdin |
| `AgentRuntime` | `quick-os-core` | tipos `GuestRequest` / `GuestResponse` |
| guest ↔ host | — | **não conectado ainda** (vsock planejado) |

---

## Tabela — tipos que atravessam fronteiras

| Tipo | Definido em | Cruza fronteira |
|------|-------------|-----------------|
| `GuestRequest` | `quick-os-core` | host → guest (futuro vsock) |
| `GuestResponse` | `quick-os-core` | guest → host |
| `AgentRecord` | `quick-os-core` | dispatcher → HTTP JSON |
| `AppConfig` | `quick-os-core` | TOML → todos os crates host |
| `ToolEvent` | `quick-os-tools` | só host (observabilidade) |
| `WorkspaceError` | `agent-os` | só guest (convertido em `GuestResponse`) |

---

## Diagrama alvo — com traits (refatoração)

![Arquitetura alvo com traits](https://raw.githubusercontent.com/MatheusOliveiraSilva/quick-os/main/docs/images/target-with-traits.png)

https://raw.githubusercontent.com/MatheusOliveiraSilva/quick-os/main/docs/images/target-with-traits.png

Detalhes: [REFACTOR-PLAN.md](REFACTOR-PLAN.md)

---

## Diagramas legados (ainda válidos)

| Diagrama | Link PNG |
|----------|----------|
| Stack geral | https://raw.githubusercontent.com/MatheusOliveiraSilva/quick-os/main/docs/images/quick-os-stack.png |
| Fluxo spawn | https://raw.githubusercontent.com/MatheusOliveiraSilva/quick-os/main/docs/images/quick-os-spawn-flow.png |
| Sequência agent-os | https://raw.githubusercontent.com/MatheusOliveiraSilva/quick-os/main/docs/images/agent-os-sequence.png |
