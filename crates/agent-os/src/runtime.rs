use quick_os_core::{AgentEvent, GuestRequest, GuestResponse};
use tracing::info;

use crate::primitives::BuiltinTool;
use crate::workspace::Workspace;

/// In-VM agent runtime — executes the three v1 primitives.
pub struct AgentRuntime {
    workspace: Workspace,
    events: Vec<AgentEvent>,
}

impl AgentRuntime {
    pub fn new(workspace: Workspace) -> Self {
        Self {
            workspace,
            events: Vec::new(),
        }
    }

    pub fn events(&self) -> &[AgentEvent] {
        &self.events
    }

    pub fn handle(&mut self, request: GuestRequest) -> GuestResponse {
        match request {
            GuestRequest::ReadWorkspace { path } => match self.workspace.read_file(&path) {
                Ok(content) => GuestResponse::success(serde_json::json!({
                    "path": path,
                    "content": content,
                })),
                Err(e) => GuestResponse::failure(e.to_string()),
            },
            GuestRequest::RunTool { name, input } => match BuiltinTool::run(&name, &input) {
                Ok(outcome) => GuestResponse::success(outcome.output),
                Err(e) => GuestResponse::failure(e),
            },
            GuestRequest::EmitEvent { kind, payload } => {
                let event = AgentEvent { kind, payload };
                info!(event_kind = %event.kind, "agent event emitted");
                self.events.push(event.clone());
                GuestResponse::success(serde_json::json!({ "recorded": true, "event": event }))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use quick_os_core::GuestRequest;
    use tempfile::tempdir;

    #[test]
    fn read_workspace_roundtrip() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("hello.txt"), "world").unwrap();

        let mut rt = AgentRuntime::new(Workspace::new(dir.path()));
        let resp = rt.handle(GuestRequest::ReadWorkspace {
            path: "hello.txt".into(),
        });
        assert!(resp.ok);
        assert_eq!(resp.result.unwrap()["content"], "world");
    }
}
