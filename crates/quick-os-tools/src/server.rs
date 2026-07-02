use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use quick_os_core::QuickOsError;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::registry::{ToolInvokeRequest, ToolRegistry};

#[derive(Clone)]
pub struct ToolServer {
    registry: Arc<ToolRegistry>,
}

impl ToolServer {
    pub fn new(registry: Arc<ToolRegistry>) -> Self {
        Self { registry }
    }

    pub fn router(&self) -> Router {
        Router::new()
            .route("/health", get(health))
            .route("/tools", get(list_tools))
            .route("/tools/:name/invoke", post(invoke_tool))
            .route("/agents", get(list_agents))
            .route("/events", get(list_events))
            .with_state(self.clone())
            .layer(CorsLayer::permissive())
            .layer(TraceLayer::new_for_http())
    }

    pub async fn serve(self, listen: SocketAddr) -> anyhow::Result<()> {
        let app = self.router();
        let listener = tokio::net::TcpListener::bind(listen).await?;
        tracing::info!(%listen, "tool surface listening");
        axum::serve(listener, app).await?;
        Ok(())
    }
}

async fn health() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok" }))
}

async fn list_tools(State(server): State<ToolServer>) -> impl IntoResponse {
    Json(server.registry.list_tools())
}

async fn list_agents(State(server): State<ToolServer>) -> impl IntoResponse {
    let agents = server.registry.dispatcher().list_agents().await;
    Json(serde_json::json!({ "agents": agents }))
}

async fn list_events(State(server): State<ToolServer>) -> impl IntoResponse {
    let events = server.registry.events().list().await;
    Json(serde_json::json!({ "events": events }))
}

async fn invoke_tool(
    State(server): State<ToolServer>,
    Path(name): Path<String>,
    Json(body): Json<ToolInvokeRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let response = server.registry.invoke(&name, body.input).await?;
    Ok(Json(serde_json::json!(response)))
}

struct AppError(QuickOsError);

impl From<QuickOsError> for AppError {
    fn from(value: QuickOsError) -> Self {
        Self(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": self.0.to_string() })),
        )
            .into_response()
    }
}
