use std::fmt;

use crate::display_ext::DisplayOption;
use crate::storage::SnapshotSignature;
use crate::LogId;
use crate::SnapshotId;
use crate::StoredMembership;

/// The metadata of a snapshot.
///
/// Including the last log id that included in this snapshot,
/// the last membership included,
/// and a snapshot id.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize), serde(bound = ""))]
pub struct SnapshotMeta {
    /// Log entries upto which this snapshot includes, inclusive.
    pub last_log_id: Option<LogId>,

    /// The last applied membership config.
    pub last_membership: StoredMembership,

    /// To identify a snapshot when transferring.
    /// Caveat: even when two snapshot is built with the same `last_log_id`, they still could be
    /// different in bytes.
    pub snapshot_id: SnapshotId,
}

impl fmt::Display for SnapshotMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{snapshot_id: {}, last_log:{}, last_membership: {}}}",
            self.snapshot_id,
            DisplayOption(&self.last_log_id),
            self.last_membership
        )
    }
}

impl SnapshotMeta {
    pub fn signature(&self) -> SnapshotSignature {
        SnapshotSignature {
            last_log_id: self.last_log_id.clone(),
            last_membership_log_id: self.last_membership.log_id().clone(),
            snapshot_id: self.snapshot_id.clone(),
        }
    }

    /// Returns a ref to the id of the last log that is included in this snapshot.
    pub fn last_log_id(&self) -> Option<&LogId> {
        self.last_log_id.as_ref()
    }
}
