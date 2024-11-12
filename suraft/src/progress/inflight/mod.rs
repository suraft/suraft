#[cfg(test)]
mod tests;

use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

use validit::Validate;

use crate::log_id_range::LogIdRange;
use crate::LogId;
use crate::LogIdOptionExt;

/// The inflight data being transmitting from leader to a follower/learner.
///
/// If inflight data is non-None, it's waiting for responses from a follower/learner.
/// The follower/learner respond with `ack()` or `conflict()` to update the state of inflight data.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq)]
pub(crate) enum Inflight {
    None,

    /// Being replicating a series of logs.
    Logs {
        log_id_range: LogIdRange,
    },
}

impl Validate for Inflight {
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Inflight::None => Ok(()),
            Inflight::Logs { log_id_range: r, .. } => r.validate(),
        }
    }
}

impl Display for Inflight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Inflight::None => write!(f, "None"),
            Inflight::Logs { log_id_range: r } => write!(f, "Logs:{}", r),
        }
    }
}

impl Inflight {
    /// Create inflight state for sending logs.
    pub(crate) fn logs(prev: Option<LogId>, last: Option<LogId>) -> Self {
        #![allow(clippy::nonminimal_bool)]
        if !(prev < last) {
            Self::None
        } else {
            Self::Logs {
                log_id_range: LogIdRange::new(prev, last),
            }
        }
    }

    pub(crate) fn is_none(&self) -> bool {
        &Inflight::None == self
    }

    // test it if used
    #[allow(dead_code)]
    pub(crate) fn is_sending_log(&self) -> bool {
        matches!(self, Inflight::Logs { .. })
    }

    /// Update inflight state when log upto `upto` is acknowledged by a follower/learner.
    pub(crate) fn ack(&mut self, upto: Option<LogId>) {
        match self {
            Inflight::None => {
                unreachable!("no inflight data")
            }
            Inflight::Logs { log_id_range } => {
                *self = {
                    debug_assert!(upto >= log_id_range.prev);
                    debug_assert!(upto <= log_id_range.last);
                    Inflight::logs(upto, log_id_range.last.clone())
                }
            }
        }
    }

    /// Update inflight state when a conflicting log id is responded by a follower/learner.
    pub(crate) fn conflict(&mut self, conflict: u64) {
        match self {
            Inflight::None => {
                unreachable!("no inflight data")
            }
            Inflight::Logs { log_id_range: logs } => {
                // if prev_log_id==None, it will never conflict
                debug_assert_eq!(Some(conflict), logs.prev.index());
                *self = Inflight::None
            }
        }
    }
}
