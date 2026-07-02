use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Context;
use clap::{Parser, Subcommand};
use quick_os_core::AppConfig;
use quick_os_dispatcher::Dispatcher;
use quick_os_firecracker::{check_environment, print_report, require_environment};
use quick_os_tools::{EventLog, ToolRegistry, ToolServer};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(
    name = "quick-os",
    about = "Host orchestrator: Firecracker microVMs + agent-os guest runtime"
)]
struct Cli {
    #[arg(short, long, default_value = "configs/quick-os.toml")]
    config: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Validate host prerequisites (/dev/kvm, binaries, guest assets)
    CheckEnv,
    /// Boot a fresh VM and capture a base snapshot
    SnapshotCreate {
        #[arg(long)]
        id: String,
    },
    /// Restore/fork an agent VM from snapshot
    AgentSpawn {
        #[arg(long)]
        snapshot: Option<String>,
    },
    /// Run dispatcher + observable HTTP tool surface
    Serve,
    /// Demo agent-os guest primitives locally (no KVM)
    DemoGuest,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    let cli = Cli::parse();
    let config = AppConfig::from_file(&cli.config)
        .with_context(|| format!("load config {}", cli.config.display()))?;

    match cli.command {
        Command::CheckEnv => {
            let report = check_environment(&config);
            print_report(&report, &config);
            if !report.all_ok() {
                std::process::exit(1);
            }
        }
        Command::SnapshotCreate { id } => {
            require_environment(&config)?;
            let dispatcher = Dispatcher::new(config);
            let snapshot = dispatcher.create_base_snapshot(&id).await?;
            println!("snapshot created: {}", snapshot.id);
        }
        Command::AgentSpawn { snapshot } => {
            require_environment(&config)?;
            let dispatcher = Dispatcher::new(config);
            let agent = dispatcher.spawn_from_snapshot(snapshot.as_deref()).await?;
            println!("agent spawned: {agent:?}");
        }
        Command::Serve => {
            require_environment(&config)?;
            let listen: SocketAddr = config.tools.listen.parse().context("parse tools.listen")?;

            let dispatcher = Arc::new(Dispatcher::new(config));
            let events = EventLog::new(1024);
            let registry = Arc::new(ToolRegistry::new(dispatcher, events));
            let server = ToolServer::new(registry);
            server.serve(listen).await?;
        }
        Command::DemoGuest => {
            let status = std::process::Command::new("bash")
                .arg("scripts/demo-agent-os.sh")
                .status()
                .context("run demo-agent-os.sh")?;
            if !status.success() {
                std::process::exit(status.code().unwrap_or(1));
            }
        }
    }

    Ok(())
}
