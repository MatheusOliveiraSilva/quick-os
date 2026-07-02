//! Guest entrypoint — in production listens on vsock; dev mode uses JSON lines on stdio.
use std::io::{self, BufRead, Write};

use agent_os::{AgentRuntime, Workspace};
use quick_os_core::GuestRequest;
use tracing_subscriber::EnvFilter;

fn main() -> io::Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(io::stderr)
        .init();

    let workspace_root =
        std::env::var("AGENT_OS_WORKSPACE").unwrap_or_else(|_| "/workspace".into());
    let mut runtime = AgentRuntime::new(Workspace::new(workspace_root));

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let request: GuestRequest = match serde_json::from_str(&line) {
            Ok(req) => req,
            Err(e) => {
                write_response(
                    &mut stdout,
                    &quick_os_core::GuestResponse::failure(format!("invalid request json: {e}")),
                )?;
                continue;
            }
        };

        let response = runtime.handle(request);
        write_response(&mut stdout, &response)?;
    }

    Ok(())
}

fn write_response(
    stdout: &mut impl Write,
    response: &quick_os_core::GuestResponse,
) -> io::Result<()> {
    match serde_json::to_string(response) {
        Ok(encoded) => {
            writeln!(stdout, "{encoded}")?;
            stdout.flush()
        }
        Err(e) => {
            let fallback =
                quick_os_core::GuestResponse::failure(format!("serialize response: {e}"));
            writeln!(
                stdout,
                "{}",
                serde_json::to_string(&fallback).unwrap_or_else(|_| {
                    r#"{"ok":false,"error":"fatal serialize"}"#.to_string()
                })
            )?;
            stdout.flush()
        }
    }
}
