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
- Primeiro `Result` real: error handling explícito vs panic/errno implícito (provavelmente via `std::env::args` ou leitura de arquivo que pode falhar)
