use std::path::{Path, PathBuf};

use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::{Method, Request, StatusCode};
use hyper_util::client::legacy::Client;
use hyperlocal::{UnixClientExt, Uri};
use quick_os_core::QuickOsError;
use tracing::debug;

type HttpClient = Client<hyperlocal::UnixConnector, Full<Bytes>>;

#[derive(Debug, Clone)]
pub struct FirecrackerClient {
    socket_path: PathBuf,
    client: HttpClient,
}

impl FirecrackerClient {
    pub fn new(socket_path: impl Into<PathBuf>) -> Self {
        let client = Client::unix();
        Self {
            socket_path: socket_path.into(),
            client,
        }
    }

    pub fn socket_path(&self) -> &Path {
        &self.socket_path
    }

    pub async fn put_json<T: serde::Serialize>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<(), QuickOsError> {
        let payload = serde_json::to_vec(body)
            .map_err(|e| QuickOsError::firecracker(format!("serialize request: {e}")))?;
        self.request(Method::PUT, path, Some(payload)).await
    }

    pub async fn patch_json<T: serde::Serialize>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<(), QuickOsError> {
        let payload = serde_json::to_vec(body)
            .map_err(|e| QuickOsError::firecracker(format!("serialize request: {e}")))?;
        self.request(Method::PATCH, path, Some(payload)).await
    }

    async fn request(
        &self,
        method: Method,
        path: &str,
        body: Option<Vec<u8>>,
    ) -> Result<(), QuickOsError> {
        let uri: Uri = Uri::new(self.socket_path.to_string_lossy().into_owned(), path);

        let mut builder = Request::builder().method(method).uri(uri);
        let request = if let Some(body) = body {
            builder = builder.header("Content-Type", "application/json");
            builder
                .body(Full::new(Bytes::from(body)))
                .map_err(|e| QuickOsError::firecracker(e.to_string()))?
        } else {
            builder
                .body(Full::new(Bytes::new()))
                .map_err(|e| QuickOsError::firecracker(e.to_string()))?
        };

        debug!(path, "firecracker api request");

        let response = self
            .client
            .request(request)
            .await
            .map_err(|e| QuickOsError::firecracker(format!("transport: {e}")))?;

        let status = response.status();
        let body = response
            .into_body()
            .collect()
            .await
            .map_err(|e| QuickOsError::firecracker(format!("read body: {e}")))?
            .to_bytes();

        if status.is_success() || status == StatusCode::NO_CONTENT {
            Ok(())
        } else {
            Err(QuickOsError::firecracker(format!(
                "{path} -> {status}: {}",
                String::from_utf8_lossy(&body)
            )))
        }
    }
}
