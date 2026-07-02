# Diagramas — links PNG para mobile

> **Regra do Matheus:** ao pedir diagrama (humano ou agent), **sempre incluir link PNG**
> `raw.githubusercontent.com` — o app GitHub no celular não renderiza mermaid de forma confiável.

## Como gerar novos diagramas

```bash
./scripts/render-diagrams.sh
```

Fonte: `docs/mermaid/*.mmd` → saída: `docs/images/*.png`

---

## Índice (clique no link no celular)

### Arquitetura e conexões

| Diagrama | PNG |
|----------|-----|
| **Como tudo se conecta (hoje)** | https://raw.githubusercontent.com/MatheusOliveiraSilva/quick-os/main/docs/images/current-connections.png |
| **Camadas (layered arch)** | https://raw.githubusercontent.com/MatheusOliveiraSilva/quick-os/main/docs/images/layered-architecture.png |
| **Alvo com traits (refatoração)** | https://raw.githubusercontent.com/MatheusOliveiraSilva/quick-os/main/docs/images/target-with-traits.png |
| **Traits Rust vs herança Python** | https://raw.githubusercontent.com/MatheusOliveiraSilva/quick-os/main/docs/images/rust-traits-vs-python.png |

### Fluxos existentes

| Diagrama | PNG |
|----------|-----|
| Stack geral | https://raw.githubusercontent.com/MatheusOliveiraSilva/quick-os/main/docs/images/quick-os-stack.png |
| Fluxo spawn | https://raw.githubusercontent.com/MatheusOliveiraSilva/quick-os/main/docs/images/quick-os-spawn-flow.png |
| Sequência agent-os | https://raw.githubusercontent.com/MatheusOliveiraSilva/quick-os/main/docs/images/agent-os-sequence.png |

---

## Markdown embed (para README/PR)

```markdown
![descrição](https://raw.githubusercontent.com/MatheusOliveiraSilva/quick-os/main/docs/images/current-connections.png)
```

---

## Docs relacionados

- [CONNECTIONS.md](CONNECTIONS.md) — narrativa dos fluxos
- [PATTERNS.md](PATTERNS.md) — layered, DTO, facade, RAII
- [RUST-CONCEPTS.md](RUST-CONCEPTS.md) — traits e abstração
- [REFACTOR-PLAN.md](REFACTOR-PLAN.md) — plano de implementação das melhorias
