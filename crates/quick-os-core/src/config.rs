use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub firecracker: FirecrackerConfig,
    pub guest: GuestConfig,
    pub dispatcher: DispatcherConfig,
    pub tools: ToolsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirecrackerConfig {
    pub binary: PathBuf,
    pub data_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuestConfig {
    pub kernel_path: PathBuf,
    pub rootfs_path: PathBuf,
    pub vcpu_count: u8,
    pub mem_size_mib: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispatcherConfig {
    pub base_snapshot_id: String,
    pub max_agents: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsConfig {
    pub listen: String,
}

impl AppConfig {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, crate::QuickOsError> {
        let raw = std::fs::read_to_string(path.as_ref())?;
        let config: AppConfig = toml::from_str(&raw)?;
        Ok(config)
    }

    pub fn snapshot_dir(&self, snapshot_id: &str) -> PathBuf {
        self.firecracker.data_dir.join("snapshots").join(snapshot_id)
    }

    pub fn agents_dir(&self) -> PathBuf {
        self.firecracker.data_dir.join("agents")
    }
}
