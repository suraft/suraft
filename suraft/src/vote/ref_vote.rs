use std::cmp::Ordering;
use std::fmt::Formatter;

use crate::LeaderId;

/// Same as [`Vote`] but with a reference to the [`LeaderId`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RefVote<'a> {
    pub(crate) leader_id: &'a LeaderId,
    pub(crate) committed: bool,
}

impl<'a> RefVote<'a> {
    pub(crate) fn new(leader_id: &'a LeaderId, committed: bool) -> Self {
        Self { leader_id, committed }
    }

    pub(crate) fn is_committed(&self) -> bool {
        self.committed
    }
}

// Commit vote have a total order relation with all other votes
impl<'a> PartialOrd for RefVote<'a> {
    #[inline]
    fn partial_cmp(&self, other: &RefVote<'a>) -> Option<Ordering> {
        match PartialOrd::partial_cmp(self.leader_id, other.leader_id) {
            Some(Ordering::Equal) => PartialOrd::partial_cmp(&self.committed, &other.committed),
            None => {
                // If two leader_id are not comparable, they won't both be granted(committed).
                // Therefore use `committed` to determine greatness to minimize election conflict.
                match (self.committed, other.committed) {
                    (false, false) => None,
                    (true, false) => Some(Ordering::Greater),
                    (false, true) => Some(Ordering::Less),
                    (true, true) => {
                        unreachable!("two incomparable leaders can not be both committed: {} {}", self, other)
                    }
                }
            }
            // Some(non-equal)
            cmp => cmp,
        }
    }
}

impl<'a> std::fmt::Display for RefVote<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}:{}>",
            self.leader_id,
            if self.is_committed() { "Q" } else { "-" }
        )
    }
}
