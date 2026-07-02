# Mobile — revisar pelo celular

## Regra de diagramas (Matheus)

**Sempre pedir link PNG** — mermaid no app GitHub não renderiza de forma confiável.

Índice: **[DIAGRAMS.md](DIAGRAMS.md)** · gerar: `./scripts/render-diagrams.sh`

Exemplo:

https://raw.githubusercontent.com/MatheusOliveiraSilva/quick-os/main/docs/images/current-connections.png

---

## Guias novos (Python → Rust)

| Doc | O quê |
|-----|-------|
| [CONNECTIONS.md](CONNECTIONS.md) | Como tudo se conecta + PNG |
| [PATTERNS.md](PATTERNS.md) | Layered, DTO, Facade, RAII |
| [RUST-CONCEPTS.md](RUST-CONCEPTS.md) | Traits, sem herança |
| [REFACTOR-PLAN.md](REFACTOR-PLAN.md) | Próximas refatorações |

---

# Visão — quick-os (60 segundos)

## Não estamos montando um OS

**quick-os** = programa Rust no teu Linux que **spawna microVMs** (Firecracker) com **agent SDK** dentro.

```
TU (celular/laptop)
    │
    ▼
PR #5 / quick-os CLI / Tool HTTP
    │
    ▼
quick-os (Rust dispatcher)     ← código deste repo
    │
    ▼
Firecracker microVM por agent  ← isolamento forte
    │
    ▼
Cursor SDK / Claude SDK        ← roda DENTRO da VM
```

## Por que microVM + snapshot?

| | Docker | microVM + snapshot |
|---|--------|-------------------|
| Isolamento | namespace | kernel separado |
| Spawn agent | segundos | ~150ms (restore) |
| Use case | apps | **agents descartáveis** |

## O que falta ainda

- [x] Dispatcher + Firecracker client + tool HTTP
- [ ] Guest image com SDK instalado (hoje é Alpine vazio)
- [ ] Comunicação host ↔ guest

## Onde ver no celular

1. https://github.com/MatheusOliveiraSilva/quick-os/pull/5
2. Aba **Files changed**
3. Abre **`REVIEW.md`** ou **`docs/VISION.md`**

PNG primeiro: [DIAGRAMS.md](DIAGRAMS.md). Mermaid só como extra no browser desktop.
