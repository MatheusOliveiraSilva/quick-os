# quick-os — log de aprendizado

Projeto pessoal: agent dispatcher em Rust (Firecracker microVMs, snapshot/fork/restore, observable tool surface).

---

## Passo 1 — Cargo init + binary mínimo

**O que construímos**
- `Cargo.toml` — manifest do crate `quick-os` (edition 2021, sem dependências)
- `src/main.rs` — entry point `fn main()` que imprime uma linha
- Validado com `cargo build` e `cargo run`

**Conceito para lembrar**
> `Cargo.toml` declara *policy* (nome, edition, deps); o Cargo orquestra compile/run/test; `fn main()` é o entry point do **binary** — o OS carrega o executável e começa ali (como `_start` → `main` em C, só que o runtime Rust já inicializou antes).

**Próximo**
- ~~Primeiro `Result` real~~ → feito no passo 2

---

## Passo 2 — Primeiro `Result` (`fs::read_to_string`)

**O que construímos**
- `main.rs` lê `config.txt` via `std::fs::read_to_string`
- Tratamos sucesso (`Ok`) e falha (`Err`) com `match` — sem panic
- Falha → mensagem em stderr + `exit(1)`

**Conceito para lembrar**
> `Result<T, E>` é o WAL aplicado ao control flow: a função **não esconde** falha atrás de errno global ou exceção — ela **retorna** `Ok(valor)` ou `Err(motivo)` e você decide quando fazer commit (propagar, logar, abortar).

**Próximo**
- ~~Operador `?` para propagar `Err` sem `match` aninhado (e/ou `fn main() -> Result<...>`)~~ → feito no passo 3

---

## Passo 3 — Operador `?` + `main -> Result`

**O que construímos**
- `read_config()` extrai a leitura; retorna `Result<String, io::Error>`
- `main() -> Result<(), io::Error>` usa `?` para propagar `Err`
- Sucesso termina com `Ok(())` — runtime converte em exit 0

**Conceito para lembrar**
> `?` é **early return** tipado: se `Result` for `Err`, a função retorna imediatamente com esse erro (como `return Err(e)`); se for `Ok(v)`, desempacota `v`. Só funciona em funções que retornam `Result` (ou `Option`) compatível.

**Próximo**
- Path via CLI (`std::env::args`) — input externo sem hardcode
