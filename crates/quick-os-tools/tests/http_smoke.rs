#[tokio::test]
async fn health_endpoint_returns_ok() {
    use std::sync::Arc;

    use axum::body::Body;
    use http_body_util::BodyExt;
    use quick_os_core::AppConfig;
    use quick_os_dispatcher::Dispatcher;
    use quick_os_tools::{EventLog, ToolRegistry, ToolServer};
    use tower::ServiceExt;

    let config: AppConfig = toml::from_str(include_str!("../../../configs/quick-os.toml"))
        .expect("parse test config");

    let dispatcher = Arc::new(Dispatcher::new(config));
    let registry = Arc::new(ToolRegistry::new(dispatcher, EventLog::new(64)));
    let app = ToolServer::new(registry).router();

    let response = app
        .oneshot(
            axum::http::Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "ok");
}

#[tokio::test]
async fn tools_endpoint_lists_builtin_tools() {
    use std::sync::Arc;

    use axum::body::Body;
    use http_body_util::BodyExt;
    use quick_os_core::AppConfig;
    use quick_os_dispatcher::Dispatcher;
    use quick_os_tools::{EventLog, ToolRegistry, ToolServer};
    use tower::ServiceExt;

    let config: AppConfig = toml::from_str(include_str!("../../../configs/quick-os.toml"))
        .expect("parse test config");

    let dispatcher = Arc::new(Dispatcher::new(config));
    let registry = Arc::new(ToolRegistry::new(dispatcher, EventLog::new(64)));
    let app = ToolServer::new(registry).router();

    let response = app
        .oneshot(
            axum::http::Request::builder()
                .uri("/tools")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let tools: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    let names: Vec<_> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert!(names.contains(&"agents.list"));
    assert!(names.contains(&"agents.spawn"));
    assert!(names.contains(&"snapshots.create"));
}
