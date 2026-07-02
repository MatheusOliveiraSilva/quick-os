# RwLock — como o quick-os protege o HashMap de agents

Diagramas PNG (mobile):

- **Sequência (list + spawn):** https://raw.githubusercontent.com/MatheusOliveiraSilva/quick-os/main/docs/images/rwlock-concurrency.png
- **Estados do lock:** https://raw.githubusercontent.com/MatheusOliveiraSilva/quick-os/main/docs/images/rwlock-states.png

---

## O problema

O `Dispatcher` guarda agents num `HashMap` compartilhado por **vários requests HTTP** ao mesmo tempo (Axum é async multi-thread).

Sem lock, duas tasks poderiam:

- uma lê enquanto outra escreve → **data race** (comportamento indefinido em Rust, nem compila com referências mutáveis soltas)
- duas escrevem ao mesmo tempo → corrupção do mapa

---

## O que é `Arc<RwLock<HashMap<...>>>`

| Peça | Função |
|------|--------|
| `HashMap` | os dados (agents vivos) |
| `RwLock` | coordena quem pode acessar e como |
| `Arc` | vários donos do mesmo RwLock (cada handler HTTP clona o `Arc`) |

Código real (`dispatcher.rs`):

```rust
agents: Arc<RwLock<HashMap<AgentId, AgentEntry>>>

// listar — leitura
let agents = self.agents.read().await;

// spawn — escrita
self.agents.write().await.insert(...);
```

---

## Regra do RwLock (Read-Write Lock)

**Ou** várias leituras ao mesmo tempo, **ou** uma escrita sozinha. Nunca leitura + escrita juntas.

| Operação | Quem entra | Bloqueia |
|----------|------------|----------|
| `read().await` | N leitores | espera se houver escritor |
| `write().await` | 1 escritor | espera todos os leitores e outros escritores |

### Por que não `Mutex`?

`Mutex` = só **um** acesso por vez, leitura ou escrita.

`GET /agents` (só lê) poderia rodar em paralelo — com `RwLock`, vários `list` simultâneos compartilham o lock de leitura. `spawn` (escreve) espera a vez.

---

## Passo a passo — dois `list` + um `spawn`

1. Request A chama `read()` → entra, lê o mapa
2. Request B chama `read()` → entra também (leitura compartilhada)
3. A e B terminam, soltam o guard → lock livre
4. Request C chama `write()` para `insert` → lock exclusivo
5. Enquanto C escreve, qualquer `read()` ou `write()` **espera** (`await` na fila)
6. C solta o guard → próximo da fila entra

O `.await` em `read()` / `write()` no Tokio = **não trava a thread** do runtime; outras tasks seguem rodando.

---

## Analogia Python

```python
import asyncio
from collections import defaultdict

class Dispatcher:
    def __init__(self):
        self._agents = {}
        self._lock = asyncio.Lock()  # Mutex — mais simples, menos paralelo em reads

    async def list_agents(self):
        async with self._lock:  # com RwLock seria read lock
            return list(self._agents.values())

    async def spawn(self):
        async with self._lock:  # write lock
            self._agents[id] = entry
```

Python stdlib não tem `RwLock` async nativo; por isso muita gente usa `Mutex` mesmo. Em Rust, reads frequentes no `HashMap` justificam `RwLock`.

---

## O guard é o que importa

```rust
{
    let agents = self.agents.read().await;  // guard nasce
    agents.len()                            // usa o mapa
}                                           // guard drop → libera o lock
```

O lock **não** fica preso o tempo todo no `Dispatcher` — só enquanto o guard existe. Segurança por escopo (RAII de novo).

---

## Onde mais usamos

`EventLog` em `quick-os-tools`:

```rust
inner: Arc<RwLock<VecDeque<ToolEvent>>>
```

Mesma ideia: vários handlers leem `/events`, `invoke` escreve eventos.

---

## Gerar diagramas

```bash
./scripts/render-diagrams.sh
```

Fontes: `docs/mermaid/rwlock-*.mmd`
