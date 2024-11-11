use std::fmt;
use std::ops::RangeInclusive;

use crate::LogId;

/// The first and the last log id belonging to a Leader.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LeaderLogIds {
    log_id_range: Option<RangeInclusive<LogId>>,
}

impl fmt::Display for LeaderLogIds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.log_id_range {
            None => write!(f, "None"),
            Some(rng) => write!(f, "({}, {})", rng.start(), rng.end()),
        }
    }
}

impl LeaderLogIds {
    pub(crate) fn new(log_id_range: Option<RangeInclusive<LogId>>) -> Self {
        Self { log_id_range }
    }

    /// Used only in tests
    #[allow(dead_code)]
    pub(crate) fn new_single(log_id: LogId) -> Self {
        Self {
            log_id_range: Some(log_id.clone()..=log_id),
        }
    }

    /// Used only in tests
    #[allow(dead_code)]
    pub(crate) fn new_start_end(first: LogId, last: LogId) -> Self {
        Self {
            log_id_range: Some(first..=last),
        }
    }

    pub(crate) fn first(&self) -> Option<&LogId> {
        self.log_id_range.as_ref().map(|x| x.start())
    }

    pub(crate) fn last(&self) -> Option<&LogId> {
        self.log_id_range.as_ref().map(|x| x.end())
    }
}
