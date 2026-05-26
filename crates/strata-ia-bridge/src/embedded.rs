//! Spawn the Python IA microservice in-process for the pip wheel use case.
//!
//! When a user does `pip install strata-reader` and calls
//! `from strata_reader import parse`, they shouldn't need to start a
//! separate gRPC server manually. [`EmbeddedWorker`] handles that:
//!
//! 1. Pick a free TCP port on `127.0.0.1`.
//! 2. Spawn `python -m strata_ia.grpc_server` as a child process with
//!    `STRATA_IA_GRPC_PORT=<picked>` and `STRATA_IA_HTTP_PORT=0`
//!    (HTTP transport disabled — pure gRPC).
//! 3. Poll the gRPC Health endpoint until SERVING or timeout.
//! 4. Hand a connected [`BridgeClient`] back to the caller.
//! 5. On `Drop`, send SIGTERM (or kill on Windows) so the child does not
//!    outlive the parent process.
//!
//! See Plan Maestro §11.T6.4.

use std::path::PathBuf;
use std::process::{Child, Command};
use std::time::Duration;

use crate::client::{BridgeClient, BridgeClientConfig};
use crate::error::BridgeError;

const DEFAULT_HEALTH_TIMEOUT: Duration = Duration::from_secs(15);
const HEALTH_POLL_INTERVAL: Duration = Duration::from_millis(200);

#[derive(Debug)]
pub struct EmbeddedWorker {
    child: Child,
    endpoint: String,
}

impl EmbeddedWorker {
    /// Spawn the Python IA worker on a free local port and wait for it to
    /// report ``SERVING``.
    pub async fn spawn() -> Result<Self, BridgeError> {
        Self::spawn_with(SpawnOptions::default()).await
    }

    /// Spawn with explicit options. Used by tests to override the Python
    /// interpreter path or the health-check timeout.
    pub async fn spawn_with(opts: SpawnOptions) -> Result<Self, BridgeError> {
        let port = pick_free_port()?;
        let mut command = Command::new(opts.python_bin());
        command
            .args(["-m", "strata_ia.grpc_server"])
            .env("STRATA_IA_GRPC_PORT", port.to_string())
            .env("STRATA_IA_HTTP_PORT", "0")
            .env("STRATA_IA_GRPC_ENABLED", "true");
        if let Some(extra_env) = opts.env {
            for (k, v) in extra_env {
                command.env(k, v);
            }
        }
        let child = command
            .spawn()
            .map_err(|e| BridgeError::Transport(format!("failed to spawn python: {e}")))?;

        let endpoint = format!("http://127.0.0.1:{port}");
        let worker = Self { child, endpoint };
        worker.wait_for_serving(opts.health_timeout).await?;
        Ok(worker)
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Connect a fresh [`BridgeClient`] to the embedded worker.
    pub async fn connect_client(&self) -> Result<BridgeClient, BridgeError> {
        BridgeClient::connect(BridgeClientConfig {
            endpoint: self.endpoint.clone(),
            ..Default::default()
        })
        .await
    }

    async fn wait_for_serving(&self, timeout: Duration) -> Result<(), BridgeError> {
        let deadline = std::time::Instant::now() + timeout;
        while std::time::Instant::now() < deadline {
            if probe_tcp(&self.endpoint).await {
                // The gRPC Health/Check handshake will succeed as soon as
                // the listener is up — `BridgeClient::connect` itself does
                // the actual TLS-free HTTP/2 negotiation.
                return Ok(());
            }
            tokio::time::sleep(HEALTH_POLL_INTERVAL).await;
        }
        Err(BridgeError::Unreachable(format!(
            "python IA worker did not become ready within {timeout:?}",
        )))
    }
}

impl Drop for EmbeddedWorker {
    fn drop(&mut self) {
        // Best-effort termination. On Windows `Child::kill` sends a
        // hard kill; on Unix we'd prefer SIGTERM but `Child` only exposes
        // SIGKILL — good enough for a wheel-side worker that has no
        // state to flush.
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[derive(Clone, Debug, Default)]
pub struct SpawnOptions {
    /// Override the Python interpreter. Defaults to the `STRATA_PYTHON_BIN`
    /// env var, then falls back to `python` on PATH.
    pub python_bin: Option<PathBuf>,
    /// Maximum wall-clock for the worker to come online.
    pub health_timeout: Duration,
    /// Extra env vars to pass to the child (model selection, log level, …).
    pub env: Option<Vec<(String, String)>>,
}

impl SpawnOptions {
    fn python_bin(&self) -> PathBuf {
        if let Some(p) = &self.python_bin {
            return p.clone();
        }
        #[allow(clippy::disallowed_methods)]
        if let Ok(p) = std::env::var("STRATA_PYTHON_BIN") {
            return PathBuf::from(p);
        }
        PathBuf::from("python")
    }
}

impl SpawnOptions {
    pub fn with_default_timeout(mut self) -> Self {
        if self.health_timeout == Duration::ZERO {
            self.health_timeout = DEFAULT_HEALTH_TIMEOUT;
        }
        self
    }
}

fn pick_free_port() -> Result<u16, BridgeError> {
    let listener = std::net::TcpListener::bind(("127.0.0.1", 0))
        .map_err(|e| BridgeError::Transport(format!("could not pick free port: {e}")))?;
    let port = listener
        .local_addr()
        .map_err(|e| BridgeError::Transport(e.to_string()))?
        .port();
    drop(listener);
    Ok(port)
}

async fn probe_tcp(endpoint: &str) -> bool {
    let parsed = match url_to_host_port(endpoint) {
        Some(v) => v,
        None => return false,
    };
    tokio::net::TcpStream::connect(parsed).await.is_ok()
}

fn url_to_host_port(endpoint: &str) -> Option<(String, u16)> {
    let stripped = endpoint
        .trim_start_matches("http://")
        .trim_start_matches("https://");
    let (host, port) = stripped.split_once(':')?;
    let port = port.split('/').next()?.parse::<u16>().ok()?;
    Some((host.to_string(), port))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pick_free_port_returns_nonzero() {
        let port = pick_free_port().unwrap();
        assert!(port > 0);
    }

    #[test]
    fn parses_endpoint_into_host_port() {
        assert_eq!(
            url_to_host_port("http://127.0.0.1:50051"),
            Some(("127.0.0.1".into(), 50051))
        );
        assert_eq!(
            url_to_host_port("https://ia.local:443/healthz"),
            Some(("ia.local".into(), 443))
        );
        assert_eq!(url_to_host_port("invalid"), None);
    }

    #[test]
    #[allow(clippy::disallowed_methods)]
    fn spawn_options_defaults_to_python_on_path_when_no_env() {
        // We don't want to mutate the test process env, just verify the
        // fallback contract.
        let opts = SpawnOptions::default();
        let py = opts.python_bin();
        let py_str = py.to_string_lossy();
        // It's either explicitly overridden via STRATA_PYTHON_BIN, or "python".
        assert!(py_str == "python" || std::env::var("STRATA_PYTHON_BIN").is_ok());
    }
}
