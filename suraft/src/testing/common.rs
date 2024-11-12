//! Testing utilities used by all kinds of tests.

use std::collections::BTreeSet;

use crate::entry::RaftEntry;
use crate::LogId;
use crate::NodeId;
use crate::RaftTypeConfig;

/// Builds a log id, for testing purposes.
pub fn log_id(term: u64, index: u64) -> LogId {
    LogId { term, index }
}

/// Create a blank log entry for test.
pub fn blank_ent<C: RaftTypeConfig>(term: u64, index: u64) -> crate::Entry<C> {
    crate::Entry::<C>::new_blank(LogId::new(term, index))
}

/// Create a membership log entry without learner config for test.
pub fn membership_ent<C: RaftTypeConfig>(term: u64, index: u64, config: Vec<BTreeSet<NodeId>>) -> crate::Entry<C> {
    crate::Entry::new_membership(LogId::new(term, index), crate::Membership::new(config, None))
}
