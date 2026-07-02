use std::sync::Arc;

use quick_os_core::QuickOsError;
use quick_os_dispatcher::Dispatcher;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::observe::{EventLog, ToolEvent};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDescriptor {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInvokeRequest {
    pub input: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInvokeResponse {
    pub id: Uuid,
    pub output: Value,
}

pub struct ToolRegistry {
    dispatcher: Arc<Dispatcher>,
    events: EventLog,
}

impl ToolRegistry {
    pub fn new(dispatcher: Arc<Dispatcher>, events: EventLog) -> Self {
        Self { dispatcher, events }
    }

    pub fn dispatcher(&self) -> Arc<Dispatcher> {
        self.dispatcher.clone()
    }

    pub fn events(&self) -> EventLog {
        self.events.clone()
    }

    pub fn list_tools(&self) -> Vec<ToolDescriptor> {
        vec![
            ToolDescriptor {
                name: "agents.list".into(),
                description: "List running agent microVMs".into(),
                input_schema: json!({ "type": "object", "properties": {} }),
            },
            ToolDescriptor {
                name: "agents.spawn".into(),
                description: "Fork/restore an agent VM from a snapshot".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "snapshot_id": { "type": "string" }
                    }
                }),
            },
            ToolDescriptor {
                name: "snapshots.create".into(),
                description: "Boot a fresh VM and capture a full snapshot".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "snapshot_id": { "type": "string" }
                    },
                    "required": ["snapshot_id"]
                }),
            },
        ]
    }

    pub async fn invoke(
        &self,
        name: &str,
        input: Value,
    ) -> Result<ToolInvokeResponse, QuickOsError> {
        let id = Uuid::new_v4();
        self.events
            .push(ToolEvent::ToolInvoked {
                id,
                tool: name.to_string(),
                input: input.clone(),
            })
            .await;

        let result = match name {
            "agents.list" => {
                let agents = self.dispatcher.list_agents().await;
                json!({ "agents": agents })
            }
            "agents.spawn" => {
                let snapshot_id = input.get("snapshot_id").and_then(Value::as_str);
                let agent = self.dispatcher.spawn_from_snapshot(snapshot_id).await?;
                self.events
                    .push(ToolEvent::AgentSpawned {
                        agent: agent.clone(),
                    })
                    .await;
                json!({ "agent": agent })
            }
            "snapshots.create" => {
                let snapshot_id = input
                    .get("snapshot_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| QuickOsError::tool("missing snapshot_id"))?;
                let snapshot = self.dispatcher.create_base_snapshot(snapshot_id).await?;
                json!({ "snapshot": snapshot })
            }
            other => return Err(QuickOsError::tool(format!("unknown tool: {other}"))),
        };

        self.events
            .push(ToolEvent::ToolCompleted {
                id,
                tool: name.to_string(),
                output: result.clone(),
            })
            .await;

        Ok(ToolInvokeResponse { id, output: result })
    }
}
