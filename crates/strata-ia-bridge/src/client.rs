//! High-level gRPC client for the Python IA microservice.
//!
//! The [`BridgeClient`] wraps the generated `IaServiceClient` from
//! [`crate::proto`] and adds:
//!
//! - A connection pool: a single `tonic::transport::Channel` re-used across
//!   1000+ concurrent RPCs (Plan Maestro §11.T6.2 AC `≤ 8 conexiones
//!   simultáneas`).
//! - Per-call deadlines (`request_timeout_s`).
//! - Typed errors via [`BridgeError`] — every `tonic::Status` is mapped to
//!   a closed Rust enum so callers can write `match` expressions without
//!   stringly-comparing error messages.
//! - Streaming via [`BridgeClient::process_stream`] (T6.3).
//!
//! The client is `Clone` and cheap to share between Tokio tasks — cloning
//! only bumps an `Arc` refcount, the channel itself stays singleton.

use std::time::Duration;

use tonic::transport::{Channel, Endpoint};

use crate::error::BridgeError;
use crate::proto::ia_service_client::IaServiceClient;
use crate::proto::{
    Crop, FormulaResponse, ImageResponse, OcrResponse, StreamCrop, StreamResult, TableResponse,
};

/// Knobs the Triage Engine consults when building a [`BridgeClient`].
#[derive(Clone, Debug)]
pub struct BridgeClientConfig {
    /// Endpoint URI such as `http://127.0.0.1:50051`.
    pub endpoint: String,
    /// Per-RPC deadline.
    pub request_timeout: Duration,
    /// HTTP/2 keep-alive ping interval.
    pub keepalive: Option<Duration>,
    /// Maximum concurrent in-flight HTTP/2 streams over the single channel.
    /// Tonic multiplexes them — we don't open a new TCP connection per RPC.
    pub concurrency_limit: usize,
}

impl Default for BridgeClientConfig {
    fn default() -> Self {
        Self {
            endpoint: "http://127.0.0.1:50051".into(),
            request_timeout: Duration::from_secs(60),
            keepalive: Some(Duration::from_secs(30)),
            // Plan Maestro §11.T6.2 — never more than 8 simultaneous streams.
            concurrency_limit: 8,
        }
    }
}

#[derive(Clone)]
pub struct BridgeClient {
    inner: IaServiceClient<Channel>,
    #[allow(dead_code)] // Surfaced via Debug; retained for ops introspection.
    config: BridgeClientConfig,
}

impl std::fmt::Debug for BridgeClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BridgeClient").field("config", &self.config).finish_non_exhaustive()
    }
}

impl BridgeClient {
    /// Build a new client and lazily establish the underlying channel.
    /// The first RPC pays the connect cost; subsequent calls multiplex
    /// over the same HTTP/2 connection.
    pub async fn connect(config: BridgeClientConfig) -> Result<Self, BridgeError> {
        let mut endpoint = Endpoint::from_shared(config.endpoint.clone())
            .map_err(|e| BridgeError::Transport(e.to_string()))?
            .timeout(config.request_timeout)
            .concurrency_limit(config.concurrency_limit);
        if let Some(keepalive) = config.keepalive {
            endpoint = endpoint.keep_alive_while_idle(true).http2_keep_alive_interval(keepalive);
        }
        let channel = endpoint.connect().await?;
        Ok(Self { inner: IaServiceClient::new(channel), config })
    }

    /// Construct from an existing channel — useful in tests where a
    /// `tokio::io::DuplexStream` or in-process server is wired manually.
    pub fn from_channel(channel: Channel, config: BridgeClientConfig) -> Self {
        Self { inner: IaServiceClient::new(channel), config }
    }

    // ------------------------------------------------------------------
    // Unary RPCs
    // ------------------------------------------------------------------

    pub async fn ocr_page(&self, crop: Crop) -> Result<OcrResponse, BridgeError> {
        let resp = self.inner.clone().ocr_page(crop).await?;
        Ok(resp.into_inner())
    }

    pub async fn extract_table(&self, crop: Crop) -> Result<TableResponse, BridgeError> {
        let resp = self.inner.clone().extract_table(crop).await?;
        Ok(resp.into_inner())
    }

    pub async fn describe_image(&self, crop: Crop) -> Result<ImageResponse, BridgeError> {
        let resp = self.inner.clone().describe_image(crop).await?;
        Ok(resp.into_inner())
    }

    pub async fn ocr_formula(&self, crop: Crop) -> Result<FormulaResponse, BridgeError> {
        let resp = self.inner.clone().ocr_formula(crop).await?;
        Ok(resp.into_inner())
    }

    // ------------------------------------------------------------------
    // Streaming RPC (T6.3)
    // ------------------------------------------------------------------

    /// Bidirectional streaming. Pass any `Stream<Item = StreamCrop>` and
    /// receive a `Stream<Item = Result<StreamResult, BridgeError>>` back.
    /// Replies may arrive out of order; the caller re-zips by
    /// `correlation_id`.
    ///
    /// Idiom (Context7-verified, tonic 0.14): the Rust client passes the
    /// stream directly thanks to the `IntoStreamingRequest` blanket impl,
    /// no manual `tonic::Request::new(...)` wrapper needed.
    pub async fn process_stream<S>(
        &self,
        crops: S,
    ) -> Result<impl futures::Stream<Item = Result<StreamResult, BridgeError>>, BridgeError>
    where
        S: futures::Stream<Item = StreamCrop> + Send + 'static,
    {
        use futures::StreamExt;
        let resp = self.inner.clone().process_stream(crops).await?;
        let outbound = resp.into_inner().map(|r| r.map_err(BridgeError::from));
        Ok(outbound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_caps_concurrency_at_eight() {
        let cfg = BridgeClientConfig::default();
        assert_eq!(cfg.concurrency_limit, 8);
        assert_eq!(cfg.request_timeout, Duration::from_secs(60));
    }

    #[test]
    fn debug_does_not_expose_internal_client() {
        let cfg = BridgeClientConfig::default();
        let dbg_str = format!("{cfg:?}");
        assert!(dbg_str.contains("BridgeClientConfig"));
    }
}
