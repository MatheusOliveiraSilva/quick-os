# Visão — agent-os (produto) + quick-os (host)

## TL;DR

**agent-os** = runtime leve onde agents operam (primitivas, sandbox, events).  
**quick-os** = host que spawna microVMs Firecracker e fala com agent-os.

Não estamos escrevendo um kernel do zero (Path B). Estamos construindo o **Agent Operating Environment** dentro de microVMs (Path A) — vendável como *"Agent OS"* sem pedir para empresas trocarem Linux no datacenter.

---

## Camadas

```
┌─────────────────────────────────────────┐
│  Cursor SDK / Claude SDK (opcional)     │
├─────────────────────────────────────────┤
│  agent-os (Rust)  ← PRODUTO             │
│    read_workspace | run_tool | emit_event│
├─────────────────────────────────────────┤
│  Linux kernel mínimo (commodity)        │
├─────────────────────────────────────────┤
│  Firecracker microVM                    │
├─────────────────────────────────────────┤
│  quick-os host (Rust)                   │
└─────────────────────────────────────────┘
```

Diagrama completo: [ARCHITECTURE.md](ARCHITECTURE.md)

---

## Demo sem KVM

```bash
./scripts/demo-agent-os.sh
# ou
cargo run -p quick-os -- demo-guest
```

---

## Million-dollar wedge

Infra de **execução de agents** com:
- isolamento microVM
- spawn via snapshot
- primitivas observáveis
- API + DX

Não vender "apague Linux" — vender **Agent OS runtime**.
