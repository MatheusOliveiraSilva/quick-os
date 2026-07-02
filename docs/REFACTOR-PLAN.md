# Plano de refatoração — traits, transport, testabilidade

Checklist das melhorias acordadas. Ordem sugerida por **impacto no produto**.

Diagrama alvo:

https://raw.githubusercontent.com/MatheusOliveiraSilva/quick-os/main/docs/images/target-with-traits.png

---

## Fase 1 — GuestTransport (desbloqueia o produto)

- [ ] `trait GuestTransport` em `quick-os-core` ou crate `quick-os-guest-client`
- [ ] `StdioTransport` — wrap do demo atual
- [ ] `VsockTransport` — produção
- [ ] `Dispatcher::invoke_primitive(agent_id, GuestRequest)` usa transport
- [ ] Testes de integração host→guest sem KVM (stdio)

```rust
#[async_trait::async_trait]
pub trait GuestTransport: Send + Sync {
    async fn invoke(&self, req: GuestRequest) -> Result<GuestResponse, QuickOsError>;
}
```

---

## Fase 2 — Traits de tools e primitivas

- [ ] `trait HostTool` + registro `HashMap<String, Arc<dyn HostTool>>`
- [ ] Migrar `agents.list`, `agents.spawn`, `snapshots.create`
- [ ] `trait GuestPrimitive` no `agent-os`
- [ ] `trait Tool` (guest) — substituir `BuiltinTool` match
- [ ] `AgentRuntime` despacha por registry, não god-match

---

## Fase 3 — Erros e observabilidade

- [ ] Erros tipados no guest (`GuestError` → `GuestResponse::failure`)
- [ ] `ToolRegistry` emite `ToolEvent::ToolFailed` em erro
- [ ] Reduzir mistura `anyhow` / `QuickOsError` na borda CLI

---

## Fase 4 — Testabilidade infra

- [ ] `trait VmBackend` injetado em `Dispatcher`
- [ ] `FakeVmBackend` para testes sem KVM
- [ ] `Arc<AppConfig>` em vez de clone em cadeia

---

## Fase 5 — Composition root

- [ ] `QuickOsApp::boot(config)` — wiring fora do `main.rs`
- [ ] Facilita SDK embarcado e testes e2e

---

## Critério de done por fase

| Fase | Demo |
|------|------|
| 1 | `invoke_primitive` no demo ou integration test |
| 2 | nova tool = novo arquivo + register, sem editar match central |
| 3 | `/events` mostra falhas |
| 4 | `cargo test` dispatcher sem `/dev/kvm` |
| 5 | `main.rs` < 30 linhas |
