use serde_json::{json, Value};

#[derive(Debug, Clone)]
pub struct ToolOutcome {
    pub output: Value,
}

pub struct BuiltinTool;

impl BuiltinTool {
    pub fn run(name: &str, input: &Value) -> Result<ToolOutcome, String> {
        match name {
            "echo" => {
                let message = input.get("message").and_then(Value::as_str).unwrap_or("");
                Ok(ToolOutcome {
                    output: json!({ "message": message }),
                })
            }
            "pwd" => Ok(ToolOutcome {
                output: json!({ "cwd": "/workspace" }),
            }),
            other => Err(format!("unknown tool: {other}")),
        }
    }
}
