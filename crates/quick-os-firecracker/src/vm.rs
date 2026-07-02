use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

use quick_os_core::{AppConfig, QuickOsError};
use serde::Serialize;
use tracing::info;

use crate::client::FirecrackerClient;

/// Temporary boot wait before snapshotting. Replace with guest health probe (vsock/serial).
pub const GUEST_BOOT_SETTLE_SECS: u64 = 2;

#[derive(Debug, Clone, Serialize)]
pub struct MachineConfig {
    pub vcpu_count: u8,
    pub mem_size_mib: u32,
    pub smt: bool,
    pub track_dirty_pages: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct BootSource {
    pub kernel_image_path: String,
    pub boot_args: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Drive {
    pub drive_id: String,
    pub path_on_host: String,
    pub is_root_device: bool,
    pub is_read_only: bool,
}

#[derive(Debug, Serialize)]
struct VmAction {
    action_type: &'static str,
}

#[derive(Debug, Serialize)]
struct SnapshotCreate {
    snapshot_type: &'static str,
    snapshot_path: String,
    mem_file_path: String,
}

#[derive(Debug, Serialize)]
struct SnapshotLoad {
    snapshot_path: String,
    mem_file_path: String,
    enable_diff_snapshots: bool,
}

pub struct VmHandle {
    pub id: String,
    pub socket_path: PathBuf,
    pub vm_dir: PathBuf,
    process: Child,
}

pub struct VmBuilder {
    config: AppConfig,
}

impl VmBuilder {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    pub async fn boot_fresh(&self, id: &str) -> Result<VmHandle, QuickOsError> {
        let vm_dir = self.config.agents_dir().join(id);
        std::fs::create_dir_all(&vm_dir)?;

        let socket_path = vm_dir.join("firecracker.sock");
        if socket_path.exists() {
            std::fs::remove_file(&socket_path)?;
        }

        let process = spawn_firecracker(&self.config.firecracker.binary, &socket_path)?;
        wait_for_socket(&socket_path).await?;

        let client = FirecrackerClient::new(&socket_path);
        let guest = &self.config.guest;

        client
            .put_json(
                "/machine-config",
                &MachineConfig {
                    vcpu_count: guest.vcpu_count,
                    mem_size_mib: guest.mem_size_mib,
                    smt: false,
                    track_dirty_pages: false,
                },
            )
            .await?;

        client
            .put_json(
                "/boot-source",
                &BootSource {
                    kernel_image_path: guest.kernel_path.display().to_string(),
                    boot_args: "console=ttyS0 reboot=k panic=1 pci=off".into(),
                },
            )
            .await?;

        client
            .put_json(
                "/drives",
                &Drive {
                    drive_id: "rootfs".into(),
                    path_on_host: guest.rootfs_path.display().to_string(),
                    is_root_device: true,
                    is_read_only: false,
                },
            )
            .await?;

        client
            .put_json(
                "/actions",
                &VmAction {
                    action_type: "InstanceStart",
                },
            )
            .await?;

        info!(agent_id = id, "firecracker vm started");

        Ok(VmHandle {
            id: id.to_string(),
            socket_path,
            vm_dir,
            process,
        })
    }

    pub async fn restore_from_snapshot(
        &self,
        id: &str,
        snapshot_path: &Path,
        mem_path: &Path,
    ) -> Result<VmHandle, QuickOsError> {
        let vm_dir = self.config.agents_dir().join(id);
        std::fs::create_dir_all(&vm_dir)?;

        let socket_path = vm_dir.join("firecracker.sock");
        if socket_path.exists() {
            std::fs::remove_file(&socket_path)?;
        }

        let process = spawn_firecracker(&self.config.firecracker.binary, &socket_path)?;
        wait_for_socket(&socket_path).await?;

        let client = FirecrackerClient::new(&socket_path);
        client
            .put_json(
                "/snapshot/load",
                &SnapshotLoad {
                    snapshot_path: snapshot_path.display().to_string(),
                    mem_file_path: mem_path.display().to_string(),
                    enable_diff_snapshots: true,
                },
            )
            .await?;

        client
            .put_json(
                "/actions",
                &VmAction {
                    action_type: "Resume",
                },
            )
            .await?;

        info!(agent_id = id, "firecracker vm restored from snapshot");

        Ok(VmHandle {
            id: id.to_string(),
            socket_path,
            vm_dir,
            process,
        })
    }

    pub async fn create_snapshot(
        &self,
        vm: &VmHandle,
        snapshot_dir: &Path,
    ) -> Result<(), QuickOsError> {
        std::fs::create_dir_all(snapshot_dir)?;

        let vm_state = snapshot_dir.join("vm_state.snap");
        let mem = snapshot_dir.join("mem.snap");

        let client = FirecrackerClient::new(&vm.socket_path);
        client
            .patch_json("/vm", &serde_json::json!({ "state": "Paused" }))
            .await?;

        client
            .put_json(
                "/snapshot/create",
                &SnapshotCreate {
                    snapshot_type: "Full",
                    snapshot_path: vm_state.display().to_string(),
                    mem_file_path: mem.display().to_string(),
                },
            )
            .await?;

        info!(snapshot_dir = %snapshot_dir.display(), "snapshot created");
        Ok(())
    }
}

impl Drop for VmHandle {
    fn drop(&mut self) {
        let _ = self.process.kill();
        let _ = self.process.wait();
    }
}

fn spawn_firecracker(binary: &Path, socket_path: &Path) -> Result<Child, QuickOsError> {
    Command::new(binary)
        .arg("--api-sock")
        .arg(socket_path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| QuickOsError::firecracker(format!("spawn {}: {e}", binary.display())))
}

async fn wait_for_socket(path: &Path) -> Result<(), QuickOsError> {
    for _ in 0..50 {
        if path.exists() {
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    Err(QuickOsError::firecracker(format!(
        "socket not ready: {}",
        path.display()
    )))
}
