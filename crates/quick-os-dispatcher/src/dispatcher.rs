use std::collections::HashMap;
use std::sync::Arc;

use quick_os_core::{AgentId, AgentRecord, AgentState, AppConfig, QuickOsError, SnapshotRef};
use quick_os_firecracker::{VmBuilder, VmHandle, GUEST_BOOT_SETTLE_SECS};
use tokio::sync::RwLock;
use tracing::info;

pub struct Dispatcher {
    config: AppConfig,
    vm_builder: VmBuilder,
    agents: Arc<RwLock<HashMap<AgentId, AgentEntry>>>,
}

struct AgentEntry {
    record: AgentRecord,
    /// Keeps the Firecracker child process alive for this agent.
    _vm: VmHandle,
}

impl Dispatcher {
    pub fn new(config: AppConfig) -> Self {
        let vm_builder = VmBuilder::new(config.clone());
        Self {
            config,
            vm_builder,
            agents: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    pub async fn list_agents(&self) -> Vec<AgentRecord> {
        self.agents
            .read()
            .await
            .values()
            .map(|entry| entry.record.clone())
            .collect()
    }

    pub async fn spawn_from_snapshot(
        &self,
        snapshot_id: Option<&str>,
    ) -> Result<AgentRecord, QuickOsError> {
        let snapshot_id = snapshot_id
            .unwrap_or(&self.config.dispatcher.base_snapshot_id)
            .to_string();

        let snapshot = self.load_snapshot_ref(&snapshot_id)?;

        {
            let agents = self.agents.read().await;
            if agents.len() >= self.config.dispatcher.max_agents {
                return Err(QuickOsError::dispatcher(format!(
                    "max agents ({}) reached",
                    self.config.dispatcher.max_agents
                )));
            }
        }

        let agent_id = AgentId::new();
        let agent_str = agent_id.to_string();

        let vm = self
            .vm_builder
            .restore_from_snapshot(&agent_str, &snapshot.vm_state_path, &snapshot.mem_path)
            .await?;

        let record = AgentRecord {
            id: agent_id,
            state: AgentState::Running,
            snapshot_id: snapshot_id.clone(),
            socket_path: vm.socket_path.display().to_string(),
            vm_dir: vm.vm_dir.display().to_string(),
        };

        self.agents.write().await.insert(
            agent_id,
            AgentEntry {
                record: record.clone(),
                _vm: vm,
            },
        );

        info!(%agent_id, snapshot_id, "agent spawned from snapshot");
        Ok(record)
    }

    pub async fn create_base_snapshot(
        &self,
        snapshot_id: &str,
    ) -> Result<SnapshotRef, QuickOsError> {
        let snapshot_dir = self.config.snapshot_dir(snapshot_id);
        if snapshot_dir.exists() {
            return Err(QuickOsError::dispatcher(format!(
                "snapshot already exists: {snapshot_id}"
            )));
        }

        let boot_id = format!("boot-{snapshot_id}");
        let vm = self.vm_builder.boot_fresh(&boot_id).await?;

        // TODO(vsock): poll guest agent-runtime /health instead of fixed sleep.
        tokio::time::sleep(std::time::Duration::from_secs(GUEST_BOOT_SETTLE_SECS)).await;

        self.vm_builder.create_snapshot(&vm, &snapshot_dir).await?;

        drop(vm);

        Ok(SnapshotRef {
            id: snapshot_id.to_string(),
            vm_state_path: snapshot_dir.join("vm_state.snap"),
            mem_path: snapshot_dir.join("mem.snap"),
        })
    }

    fn load_snapshot_ref(&self, snapshot_id: &str) -> Result<SnapshotRef, QuickOsError> {
        let dir = self.config.snapshot_dir(snapshot_id);
        let vm_state_path = dir.join("vm_state.snap");
        let mem_path = dir.join("mem.snap");

        if !vm_state_path.exists() || !mem_path.exists() {
            return Err(QuickOsError::dispatcher(format!(
                "snapshot not found: {snapshot_id} (expected files under {})",
                dir.display()
            )));
        }

        Ok(SnapshotRef {
            id: snapshot_id.to_string(),
            vm_state_path,
            mem_path,
        })
    }
}
