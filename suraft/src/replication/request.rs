use std::fmt;

/// A replication request sent by RaftCore leader state to replication stream.
#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub(crate) enum Replicate {
    /// Inform replication stream to forward the committed log id to followers/learners.
    Committed(Option<LogId>),

    /// Send a chunk of data, e.g., logs or snapshot.
    Data(Data),
}

impl Replicate {
    pub(crate) fn logs(log_id_range: LogIdRange) -> Self {
        Self::Data(Data::new_logs(log_id_range))
    }
}

impl fmt::Display for Replicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Committed(c) => write!(f, "Committed({})", c.display()),
            Self::Data(d) => write!(f, "Data({})", d),
        }
    }
}

use crate::display_ext::DisplayOptionExt;
use crate::log_id_range::LogIdRange;
use crate::LogId;

/// Request to replicate a chunk of data, logs or snapshot.
///
/// It defines what data to send to a follower/learner and an id to identify who is sending this
/// data.
/// Thd data is either a series of logs or a snapshot.
#[derive(PartialEq, Eq)]
pub(crate) enum Data {
    Committed,
    Logs(LogIdRange),
}

impl fmt::Debug for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Data::Committed => {
                write!(f, "Data::Committed")
            }
            Self::Logs(l) => f.debug_struct("Data::Logs").field("log_id_range", l).finish(),
        }
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Data::Committed => {
                write!(f, "Committed")
            }
            Self::Logs(l) => {
                write!(f, "Logs{{log_id_range: {}}}", l)
            }
        }
    }
}

impl Data {
    pub(crate) fn new_committed() -> Self {
        Self::Committed
    }

    pub(crate) fn new_logs(log_id_range: LogIdRange) -> Self {
        Self::Logs(log_id_range)
    }

    /// Return true if the data includes any payload, i.e., not a heartbeat.
    pub(crate) fn has_payload(&self) -> bool {
        match self {
            Self::Committed => false,
            Self::Logs(_) => true,
        }
    }
}
