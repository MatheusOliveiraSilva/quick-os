# Visão — quick-os (o que estamos construindo)

## TL;DR

**Não estamos montando um OS do zero.**

Estamos construindo um **dispatcher** que sobe **microVMs Linux mínimas** (via Firecracker) já prontas para rodar **agent runtimes** (Cursor SDK, Claude Agent SDK, etc.) — com **snapshot/fork** para spawn em ~150ms em vez de boot completo.

---

## Camadas (de baixo pra cima)

```
┌─────────────────────────────────────────────┐
│  Agent runtime (dentro da microVM)        │
│  Cursor SDK / Claude Agent SDK + tools      │
├─────────────────────────────────────────────┤
│  Firecracker microVM (1 por agent)          │
│  kernel mínimo + rootfs com runtime         │
├─────────────────────────────────────────────┤
│  quick-os (Rust, no host)                   │
│  dispatcher + snapshot pool + tool surface  │
├─────────────────────────────────────────────┤
│  Host Linux + KVM                           │
└─────────────────────────────────────────────┘
```

Ver também: `docs/images/quick-os-stack.png`

---

## Por que isso nos ajuda?

| Problema | Docker/container | Processo solto | **quick-os + Firecracker** |
|----------|------------------|----------------|----------------------------|
| Isolamento | namespace (shared kernel) | nenhum | **microVM** (kernel guest separado) |
| Spawn rápido | pull image + start (~s) | instantâneo mas inseguro | **snapshot restore (~ms)** |
| Agent pode escapar | risco se kernel bug | total | VM boundary |
| Observabilidade | logs espalhados | difícil | **tool surface + event log** |
| Multi-tenant agents | OK mas pesado | perigoso | 1 VM = 1 agent, descartável |

**Analogia:** snapshot é a **golden image** do agent — SO + runtime + deps já bootados. `agents.spawn` é **fork/restore**, não reinstall.

---

## O que já existe vs o que falta

### Já no repo (scaffold)

- [x] CLI dispatcher (`check-env`, `snapshot-create`, `agent-spawn`, `serve`)
- [x] Client Firecracker (boot, pause, snapshot, restore)
- [x] HTTP tool surface observável (`/tools`, `/events`)
- [x] Smoke tests sem KVM

### Falta (próximas fases)

- [ ] **Guest image** com agent SDK pré-instalado (rootfs real, não Alpine vazio)
- [ ] Comunicação host ↔ guest (vsock / serial / HTTP sidecar)
- [ ] Jailer + network namespace por agent
- [ ] Integração concreta Cursor SDK / Claude Agent SDK dentro da VM

---

## Fluxo snapshot → spawn

Ver: `docs/images/quick-os-spawn-flow.png`

1. **Uma vez:** boot VM → instala runtime → `snapshot-create` → `base.snap`
2. **Cada agent:** `agents.spawn` → restore snapshot (CoW) → agent já "warm"
3. **Observar:** tool surface registra cada invoke + estado dos agents

---

## Perguntas comuns no review

### "Por que kernel + rootfs nos configs?"

Firecracker **precisa** de um guest Linux mínimo (kernel + disco). Não é "criar OS" — é **escolher/buildar a imagem guest** onde o agent roda. Hoje usamos Alpine placeholder; depois vira imagem custom com SDK.

### "O que é `VmHandle` guardado no dispatcher?"

Mantém o **processo Firecracker vivo**. Sem isso, a VM morre quando o handle é dropped. Não é dead code — é **ownership intencional** do processo.

### "Por que `sleep(2)` no snapshot?"

**Hack temporário** — espera guest bootar antes de snapshotar. Substituir por health check (vsock/serial "ready").

### "Tool surface vs CLI?"

- **CLI** — operação humana / scripts
- **Tool surface** — mesma capacidade exposta como **tools invocáveis** (observable, auditável, integrável com outros agents)

---

## Como revisar pelo celular

1. `REVIEW.md` — guia do PR
2. `docs/VISION.md` — este arquivo
3. `./scripts/demo-ci.sh` — demo sem KVM (CI roda isso)
4. Perguntas inline no **PR #5**
