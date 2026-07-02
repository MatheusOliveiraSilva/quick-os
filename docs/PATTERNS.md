# Padrões de design — guia para quem vem de Python

Este doc explica os padrões usados no quick-os **sem jargão desnecessário**.
Para diagramas no celular, use [DIAGRAMS.md](DIAGRAMS.md) (links PNG).

---

## Layered Architecture (arquitetura em camadas)

**Ideia:** dividir o código em **camadas** onde cada uma só fala com a de baixo (ou ao lado, no caso guest).

**Analogia Python:** imagine um projeto Django/FastAPI:

```
views/routers     →  recebe HTTP
services/         →  regra de negócio
repositories/     →  banco / APIs externas
models/schemas/   →  tipos de dados
```

No quick-os:

| Camada | Crate | Responsabilidade |
|--------|-------|------------------|
| Interface | `quick-os`, `quick-os-tools` | CLI, HTTP, expõe API |
| Aplicação | `quick-os-dispatcher`, `ToolRegistry` | orquestra agents |
| Infra | `quick-os-firecracker` | Firecracker, VMs |
| Domínio | `quick-os-core` | tipos, protocolo, erros |
| Guest (produto) | `agent-os` | runtime dentro da VM |

**Por que importa:** se você misturar HTTP com chamada Firecracker dentro do mesmo arquivo, vira spaghetti. Camadas deixam claro *onde* mudar quando algo evolui.

**Diagrama (PNG — abre no mobile):**

https://raw.githubusercontent.com/MatheusOliveiraSilva/quick-os/main/docs/images/layered-architecture.png

---

## DTO (Data Transfer Object)

**Ideia:** struct **só de dados** — sem lógica pesada, só campos + serialização.

**Analogia Python:**

```python
@dataclass
class AgentRecord:
    id: UUID
    state: str
    snapshot_id: str
```

ou Pydantic `BaseModel` usado só para passar dados entre camadas.

**No repo:** `AgentRecord`, `SnapshotRef`, `GuestRequest`, `GuestResponse`, `AppConfig`.

**Por que importa:** o host e o guest precisam falar a mesma língua. DTOs em `quick-os-core` são o **contrato** — mudou o protocolo, mudou um lugar só.

**O que NÃO é DTO:** `Dispatcher`, `AgentRuntime`, `VmBuilder` — esses **têm comportamento** (métodos, estado, side effects).

---

## Facade (fachada)

**Ideia:** uma classe/módulo **simples na frente** de um subsistema complexo.

**Analogia Python:**

```python
class ToolRegistry:
  def __init__(self, dispatcher, event_log):
    self._dispatcher = dispatcher

  def invoke(self, name, input):
    # esconde spawn, snapshot, list, logging...
```

O cliente HTTP não chama `Dispatcher` + `EventLog` direto — chama `ToolRegistry.invoke("agents.spawn", ...)`.

**No repo:** `ToolRegistry` é facade sobre `Dispatcher` + `EventLog`. `ToolServer` é outra fachada HTTP em cima do registry.

**Por que importa:** quem consome a API vê 3 tools (`agents.list`, `agents.spawn`, `snapshots.create`) em vez de 5 crates internos.

---

## RAII (Resource Acquisition Is Initialization)

**Ideia Rust:** você amarra um recurso (processo, arquivo, lock) ao **tempo de vida** de um valor. Quando o valor sai de escopo, o recurso é liberado automaticamente.

**Analogia Python:**

```python
with open("file") as f:   # abre
    ...                   # usa
# fecha automaticamente ao sair do with
```

ou um `__del__` confiável (Python é menos garantido que Rust aqui).

**No repo:** `VmHandle` guarda o processo `Child` do Firecracker. Quando o handle é dropado (`impl Drop for VmHandle`), o processo é morto.

```rust
struct AgentEntry {
    record: AgentRecord,
    _vm: VmHandle,  // VM viva enquanto entry existir no HashMap
}
```

Tirou o agent do mapa → `VmHandle` drop → microVM morre. Sem vazamento de processo zumbi.

**Por que importa:** em sistemas com processos/VMs, RAII evita "esqueci de dar kill". É um dos superpoderes do Rust.

---

## Outros padrões presentes (menção rápida)

| Padrão | Onde | Uma linha |
|--------|------|-----------|
| **Factory** | `VmBuilder::boot_fresh` | cria VMs |
| **Newtype** | `AgentId(Uuid)` | UUID com tipo próprio |
| **Tagged Union** | `GuestRequest` enum | JSON com `"op": "read_workspace"` |
| **Command** | `GuestRequest`, CLI `Command` | pedido como dado, não método |
| **Registry** | `ToolRegistry` | catálogo de tools (hoje via `match`, amanhã via trait) |

---

## Próximo passo (refatoração planejada)

Ver [REFACTOR-PLAN.md](REFACTOR-PLAN.md) e diagrama alvo:

https://raw.githubusercontent.com/MatheusOliveiraSilva/quick-os/main/docs/images/target-with-traits.png
