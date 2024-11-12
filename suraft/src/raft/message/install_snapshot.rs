use std::fmt;

use crate::storage::SnapshotMeta;
use crate::Vote;

/// An RPC sent by the Raft leader to send chunks of a snapshot to a follower (§7).
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize), serde(bound = ""))]
pub struct InstallSnapshotRequest {
    pub vote: Vote,

    /// Metadata of a snapshot: snapshot_id, last_log_ed membership etc.
    pub meta: SnapshotMeta,

    /// The byte offset where this chunk of data is positioned in the snapshot file.
    pub offset: u64,
    /// The raw bytes of the snapshot chunk, starting at `offset`.
    pub data: Vec<u8>,

    /// Will be `true` if this is the last chunk in the snapshot.
    pub done: bool,
}

impl fmt::Display for InstallSnapshotRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "InstallSnapshotRequest {{ vote:{}, meta:{}, offset:{}, len:{}, done:{} }}",
            self.vote,
            self.meta,
            self.offset,
            self.data.len(),
            self.done
        )
    }
}

/// The response to an `InstallSnapshotRequest`.
#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(derive_more::Display)]
#[display("{{vote:{}}}", vote)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize), serde(bound = ""))]
pub struct InstallSnapshotResponse {
    pub vote: Vote,
}

/// The response to `Raft::install_full_snapshot` API.
#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(derive_more::Display)]
#[display("SnapshotResponse{{vote:{}}}", vote)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize), serde(bound = ""))]
pub struct SnapshotResponse {
    pub vote: Vote,
}

impl SnapshotResponse {
    pub fn new(vote: Vote) -> Self {
        Self { vote }
    }
}

impl From<SnapshotResponse> for InstallSnapshotResponse {
    fn from(snap_resp: SnapshotResponse) -> Self {
        Self { vote: snap_resp.vote }
    }
}
