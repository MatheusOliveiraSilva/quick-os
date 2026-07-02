use std::collections::VecDeque;
use std::sync::Arc;

use quick_os_core::AgentRecord;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolEvent {
    ToolInvoked {
        id: Uuid,
        tool: String,
        input: serde_json::Value,
    },
    ToolCompleted {
        id: Uuid,
        tool: String,
        output: serde_json::Value,
    },
    ToolFailed {
        id: Uuid,
        tool: String,
        error: String,
    },
    AgentSpawned {
        agent: AgentRecord,
    },
}

#[derive(Clone)]
pub struct EventLog {
    inner: Arc<RwLock<VecDeque<ToolEvent>>>,
    capacity: usize,
}

impl EventLog {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: Arc::new(RwLock::new(VecDeque::with_capacity(capacity))),
            capacity,
        }
    }

    pub async fn push(&self, event: ToolEvent) {
        let mut log = self.inner.write().await;
        if log.len() >= self.capacity {
            log.pop_front();
        }
        log.push_back(event);
    }

    pub async fn list(&self) -> Vec<ToolEvent> {
        self.inner.read().await.iter().cloned().collect()
    }
}
