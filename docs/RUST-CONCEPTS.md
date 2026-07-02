# Rust para quem aprendeu no Python — traits, abstração, sem herança

Leitura complementar ao Rust Book (até smart pointers). Foco no que este repo usa e vai usar.

**Diagrama traits vs Python (PNG):**

https://raw.githubusercontent.com/MatheusOliveiraSilva/quick-os/main/docs/images/rust-traits-vs-python.png

---

## 1. Não existe herança de struct

### Python

```python
class Animal:
    def speak(self): ...

class Dog(Animal):
    def speak(self): return "woof"
```

`Dog` **herda** campos e métodos de `Animal`.

### Rust

```rust
struct Dog { name: String }

impl Dog {
    fn speak(&self) -> &str { "woof" }
}
```

`Dog` **não herda** de nada. Só tem o que você declarou.

**Regra mental:** em Rust, reutilização é por **composição** (ter um campo) e **traits** (implementar um contrato), não por `extends`.

---

## 2. O que é um trait?

**Trait ≈ interface / Protocol / ABC** — define *o que* um tipo precisa fazer, sem dizer *como*.

```rust
trait Tool {
    fn name(&self) -> &str;
    fn run(&self, input: &serde_json::Value) -> Result<serde_json::Value, String>;
}

struct EchoTool;

impl Tool for EchoTool {
    fn name(&self) -> &str { "echo" }
    fn run(&self, input: &serde_json::Value) -> Result<serde_json::Value, String> {
        // ...
        Ok(serde_json::json!({}))
    }
}
```

### Comparação direta

| Conceito | Python | Rust |
|----------|--------|------|
| Contrato | `class Tool(ABC)` + `@abstractmethod` | `trait Tool { fn run(...); }` |
| Implementar | `class EchoTool(Tool)` | `impl Tool for EchoTool` |
| Polimorfismo | `tool: Tool` (referência) | `&dyn Tool` ou `Box<dyn Tool>` |
| Onde ficam métodos | Dentro da classe | `impl` separado do `struct` |

### Por que `impl` separado?

Em Rust você pode:

- implementar métodos no mesmo crate que definiu o struct
- **não pode** implementar trait externo em tipo externo (regra do orphan)
- adicionar `impl Display for MeuTipo` sem editar a definição do struct

Para Python dev: pense em `impl` como um bloco `@classmethod` / métodos que você cola *ao lado* da dataclass, não dentro dela.

---

## 3. Abstração na prática — `dyn` e `Box`

Quando você quer uma **lista de coisas diferentes** que obedecem ao mesmo contrato:

### Python

```python
tools: list[Tool] = [EchoTool(), PwdTool()]
for t in tools:
    t.run({})
```

### Rust

```rust
let tools: Vec<Box<dyn Tool>> = vec![
    Box::new(EchoTool),
    Box::new(PwdTool),
];
for t in &tools {
    t.run(&input)?;
}
```

- `dyn Tool` = "algo que implementa Tool" (tipo dinâmico, como vtable em C++/Python)
- `Box<dyn Tool>` = ponteiro heap para esse algo (tamanhos diferentes na mesma lista)

**Quando usar:** registry de tools, transportes (stdio vs vsock), backends fake para teste.

**Quando NÃO usar:** hot path simples com um tipo só — aí struct concreta + generics (`fn foo<T: Tool>(t: &T)`) é mais rápido.

---

## 4. Enum ≠ class hierarchy

`GuestRequest` no repo:

```rust
pub enum GuestRequest {
    ReadWorkspace { path: String },
    RunTool { name: String, input: Value },
    EmitEvent { kind: String, payload: Value },
}
```

**Python equivalente moderno:**

```python
class ReadWorkspace(TypedDict):
    op: Literal["read_workspace"]
    path: str
# ou Pydantic discriminated union
```

Enum em Rust é **soma de tipos** — cada variante pode ter campos diferentes. O `match` obriga tratar todas:

```rust
match request {
    GuestRequest::ReadWorkspace { path } => { ... }
    GuestRequest::RunTool { name, input } => { ... }
    GuestRequest::EmitEvent { kind, payload } => { ... }
}
```

Esqueceu uma variante → **erro de compilação**. Em Python só descobre em runtime.

---

## 5. Smart pointers que o repo já usa

| Tipo | Python aproximado | Uso no repo |
|------|-------------------|-------------|
| `Arc<T>` | referência compartilhada thread-safe | `Arc<Dispatcher>`, `Arc<ToolRegistry>` |
| `RwLock<T>` | `threading.RLock` + async | `tokio::sync::RwLock<HashMap<...>>` |
| `Arc<RwLock<T>>` | estado global compartilhado | lista de agents, event log |

**Por que não só `&mut`?** Axum e async precisam que o estado do servidor seja `Clone` + `'static`. `Arc` permite vários handlers HTTP apontarem pro mesmo `Dispatcher`.

Fluxo mental:

```
Dispatcher (único) 
    → Arc::new 
    → ToolRegistry guarda Arc<Dispatcher>
    → ToolServer guarda Arc<ToolRegistry>
    → cada request HTTP clona Arc (barato, só contador)
```

---

## 6. O que o repo faz HOJE vs o que vamos fazer

### Hoje (concreto + match)

```rust
// guest
BuiltinTool::run(&name, &input)  // match em string

// host
ToolRegistry::invoke(name, input)  // match em string

// AgentRuntime::handle — match em GuestRequest
```

Funciona. Não é "errado" para v1. Mas **não é abstração** — cada tool nova = editar o match.

### Alvo (traits)

| Abstração | Implementações |
|-----------|----------------|
| `trait GuestTransport` | `VsockTransport`, `StdioTransport` |
| `trait HostTool` | `AgentsSpawnTool`, ... |
| `trait GuestPrimitive` | `ReadWorkspace`, `RunTool`, `EmitEvent` |
| `trait Tool` (guest) | `EchoTool`, `PwdTool` |
| `trait VmBackend` | `FirecrackerVmBackend`, `FakeVmBackend` |

Ver [REFACTOR-PLAN.md](REFACTOR-PLAN.md).

---

## 7. Cheat sheet rápido

| Você pensa em Python | Em Rust |
|----------------------|---------|
| `class Foo` | `struct Foo` + `impl Foo` |
| `class Foo(Bar)` herança | composição + `impl Trait for Foo` |
| `Protocol` / `ABC` | `trait` |
| `Optional[Foo]` | `Option<Foo>` |
| `Union[A,B]` / discriminated union | `enum` |
| `raise X` | `return Err(X)` ou `?` |
| `with open()` cleanup | `Drop` trait (RAII) |
| `list[Tool]` polimórfico | `Vec<Box<dyn Tool>>` |
| `dataclass` | `struct` + `derive` |

---

## 8. Ordem sugerida de prática neste repo

1. Ler `guest_protocol.rs` — enum + serde (contrato)
2. Ler `workspace.rs` — struct com métodos + erro próprio
3. Ler `vm.rs` — `Drop` em `VmHandle`
4. Ler `dispatcher.rs` — `Arc`, `RwLock`, serviço async
5. Implementar primeiro trait: `GuestTransport` (maior impacto)
6. Depois: `HostTool` + `GuestPrimitive`

Cada passo tem teste ou demo script — rode `./scripts/demo-agent-os.sh` sem KVM.
