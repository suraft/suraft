use std::fmt;

use crate::display_ext::DisplayOptionExt;
use crate::LogId;
use crate::Vote;
use crate::NID;

#[derive(Clone, Debug)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize), serde(bound = ""))]
pub struct TransferLeaderRequest {
    /// The vote of the Leader that is transferring the leadership.
    pub(crate) from_leader: Vote,

    /// The assigned node to be the next Leader.
    pub(crate) to_node_id: NID,

    /// The last log id the `to_node_id` node should at least have to become Leader.
    pub(crate) last_log_id: Option<LogId>,
}

impl TransferLeaderRequest {
    pub fn new(from: Vote, to: NID, last_log_id: Option<LogId>) -> Self {
        Self {
            from_leader: from,
            to_node_id: to,
            last_log_id,
        }
    }

    /// From which Leader the leadership is transferred.
    pub fn from_leader(&self) -> &Vote {
        &self.from_leader
    }

    /// To which node the leadership is transferred.
    pub fn to_node_id(&self) -> &NID {
        &self.to_node_id
    }

    /// The last log id on the `to_node_id` node should at least have to become Leader.
    ///
    /// This is the last log id on the Leader when the leadership is transferred.
    pub fn last_log_id(&self) -> Option<&LogId> {
        self.last_log_id.as_ref()
    }
}

impl fmt::Display for TransferLeaderRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(from_leader={}, to={}, last_log_id={})",
            self.from_leader,
            self.to_node_id,
            self.last_log_id.display()
        )
    }
}
