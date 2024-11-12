use std::error::Error;

use crate::error::Fatal;
use crate::error::Infallible;
use crate::error::NetworkError;
use crate::error::RPCError;
use crate::error::RemoteError;
use crate::error::ReplicationClosed;
use crate::error::ReplicationError;
use crate::error::Timeout;
use crate::error::Unreachable;
use crate::StorageError;

/// Error occurred when streaming local data to a remote raft node.
///
/// Thus this error includes storage error, network error, and remote error.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(bound(serialize = "E: serde::Serialize")),
    serde(bound(deserialize = "E: for <'d> serde::Deserialize<'d>"))
)]
pub enum StreamingError<E: Error = Infallible> {
    /// The replication stream is closed intentionally.
    #[error(transparent)]
    Closed(#[from] ReplicationClosed),

    /// Storage error occurs when reading local data.
    #[error(transparent)]
    StorageError(#[from] StorageError),

    /// Timeout when streaming data to remote node.
    #[error(transparent)]
    Timeout(#[from] Timeout),

    /// The node is temporarily unreachable and should backoff before retrying.
    #[error(transparent)]
    Unreachable(#[from] Unreachable),

    /// Failed to send the RPC request and should retry immediately.
    #[error(transparent)]
    Network(#[from] NetworkError),

    /// Remote node returns an error.
    #[error(transparent)]
    RemoteError(#[from] RemoteError<E>),
}

impl From<StreamingError<Fatal>> for ReplicationError {
    fn from(e: StreamingError<Fatal>) -> Self {
        match e {
            StreamingError::Closed(e) => ReplicationError::Closed(e),
            StreamingError::StorageError(e) => ReplicationError::StorageError(e),
            StreamingError::Timeout(e) => ReplicationError::RPCError(RPCError::Timeout(e)),
            StreamingError::Unreachable(e) => ReplicationError::RPCError(RPCError::Unreachable(e)),
            StreamingError::Network(e) => ReplicationError::RPCError(RPCError::Network(e)),
            StreamingError::RemoteError(e) => {
                // Fatal on remote error is considered as unreachable.
                ReplicationError::RPCError(RPCError::Unreachable(Unreachable::new(&e.source)))
            }
        }
    }
}

impl From<RPCError> for StreamingError {
    fn from(value: RPCError) -> Self {
        #[allow(unreachable_patterns)]
        match value {
            RPCError::Timeout(e) => StreamingError::Timeout(e),
            RPCError::Unreachable(e) => StreamingError::Unreachable(e),
            RPCError::PayloadTooLarge(_e) => {
                unreachable!("PayloadTooLarge should not be converted to StreamingError")
            }
            RPCError::Network(e) => StreamingError::Network(e),
            RPCError::RemoteError(e) => StreamingError::RemoteError(e),
        }
    }
}

impl From<StreamingError> for ReplicationError {
    fn from(e: StreamingError) -> Self {
        #[allow(unreachable_patterns)]
        match e {
            StreamingError::Closed(e) => ReplicationError::Closed(e),
            StreamingError::StorageError(e) => ReplicationError::StorageError(e),
            StreamingError::Timeout(e) => ReplicationError::RPCError(RPCError::Timeout(e)),
            StreamingError::Unreachable(e) => ReplicationError::RPCError(RPCError::Unreachable(e)),
            StreamingError::Network(e) => ReplicationError::RPCError(RPCError::Network(e)),
            StreamingError::RemoteError(_e) => {
                unreachable!("Infallible error should not be converted to ReplicationError")
            }
        }
    }
}
