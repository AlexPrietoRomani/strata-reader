//! Typed error surface for the bridge client.
//!
//! Every variant maps cleanly to a Provenance reason on the Rust side so
//! the PRISMA trail records *why* a block ended up using which backend.

use thiserror::Error;
use tonic::Status;

#[derive(Debug, Error)]
pub enum BridgeError {
    /// Could not even connect to the IA server. The Rust scheduler should
    /// back off (AIMD) when this fires repeatedly.
    #[error("ia server unreachable: {0}")]
    Unreachable(String),

    /// The IA server replied `RESOURCE_EXHAUSTED` — VRAM admission denied.
    /// The scheduler should reduce concurrency before re-trying.
    #[error("ia resource exhausted: {0}")]
    ResourceExhausted(String),

    /// The IA server replied `UNAVAILABLE` (Ollama unreachable from the
    /// Python side, model not loaded, etc.).
    #[error("ia transient failure: {0}")]
    Transient(String),

    /// The IA server replied with a non-retryable error (malformed VLM
    /// JSON, internal exception, etc.).
    #[error("ia permanent failure: {0}")]
    Permanent(String),

    /// Network / transport level error not classified above.
    #[error("transport error: {0}")]
    Transport(String),
}

impl From<Status> for BridgeError {
    fn from(value: Status) -> Self {
        use tonic::Code;
        let msg = value.message().to_string();
        match value.code() {
            Code::Unavailable => Self::Unreachable(msg),
            Code::ResourceExhausted => Self::ResourceExhausted(msg),
            Code::DeadlineExceeded | Code::Aborted => Self::Transient(msg),
            Code::InvalidArgument | Code::FailedPrecondition | Code::Internal => {
                Self::Permanent(msg)
            }
            _ => Self::Permanent(msg),
        }
    }
}

impl From<tonic::transport::Error> for BridgeError {
    fn from(value: tonic::transport::Error) -> Self {
        Self::Transport(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_to_bridge_error_mapping() {
        let cases = [
            (tonic::Code::Unavailable, "u"),
            (tonic::Code::ResourceExhausted, "r"),
            (tonic::Code::Internal, "i"),
            (tonic::Code::InvalidArgument, "a"),
        ];
        let mapped: Vec<_> = cases
            .iter()
            .map(|(code, msg)| BridgeError::from(Status::new(*code, *msg)))
            .collect();
        assert!(matches!(mapped[0], BridgeError::Unreachable(_)));
        assert!(matches!(mapped[1], BridgeError::ResourceExhausted(_)));
        assert!(matches!(mapped[2], BridgeError::Permanent(_)));
        assert!(matches!(mapped[3], BridgeError::Permanent(_)));
    }

    #[test]
    fn error_display_includes_inner_message() {
        let err = BridgeError::Unreachable("boom".into());
        assert!(err.to_string().contains("boom"));
    }
}
