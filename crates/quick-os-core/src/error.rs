use std::path::{Path, PathBuf};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum QuickOsError {
    #[error("configuration: {0}")]
    Config(String),

    #[error("environment: {0}")]
    Environment(String),

    #[error("firecracker: {0}")]
    Firecracker(String),

    #[error("dispatcher: {0}")]
    Dispatcher(String),

    #[error("agent not found: {0}")]
    AgentNotFound(String),

    #[error("tool: {0}")]
    Tool(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Toml(#[from] toml::de::Error),
}

impl QuickOsError {
    pub fn config(msg: impl Into<String>) -> Self {
        Self::Config(msg.into())
    }

    pub fn environment(msg: impl Into<String>) -> Self {
        Self::Environment(msg.into())
    }

    pub fn firecracker(msg: impl Into<String>) -> Self {
        Self::Firecracker(msg.into())
    }

    pub fn dispatcher(msg: impl Into<String>) -> Self {
        Self::Dispatcher(msg.into())
    }

    pub fn tool(msg: impl Into<String>) -> Self {
        Self::Tool(msg.into())
    }
}

pub type Result<T> = std::result::Result<T, QuickOsError>;

pub fn ensure_path_exists(path: &Path, label: &str) -> Result<()> {
    if path.exists() {
        Ok(())
    } else {
        Err(QuickOsError::environment(format!(
            "missing {label}: {}",
            path.display()
        )))
    }
}

pub fn ensure_kvm() -> Result<()> {
    let kvm = PathBuf::from("/dev/kvm");
    if !kvm.exists() {
        return Err(QuickOsError::environment(
            "/dev/kvm not found — enable KVM on the host (bare metal or privileged VM with nested virt)",
        ));
    }
    Ok(())
}
