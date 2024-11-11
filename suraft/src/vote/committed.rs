use std::cmp::Ordering;
use std::fmt;

use crate::vote::ref_vote::RefVote;
use crate::Vote;

/// Represents a committed Vote that has been accepted by a quorum.
///
/// The inner `Vote`'s attribute `committed` is always set to `true`
#[derive(Debug, Clone)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd)]
pub(crate) struct CommittedVote {
    vote: Vote,
}

/// The `CommittedVote` is totally ordered.
///
/// Because:
/// - any two quorums have common elements,
/// - and the `CommittedVote` is accepted by a quorum,
/// - and a `Vote` is granted if it is greater than the old one.
#[allow(clippy::derive_ord_xor_partial_ord)]
impl Ord for CommittedVote {
    fn cmp(&self, other: &Self) -> Ordering {
        self.vote.partial_cmp(&other.vote).unwrap()
    }
}

impl CommittedVote {
    pub(crate) fn new(mut vote: Vote) -> Self {
        vote.committed = true;
        Self { vote }
    }

    pub(crate) fn term(&self) -> u64 {
        self.vote.leader_id().term()
    }

    pub(crate) fn into_vote(self) -> Vote {
        self.vote
    }

    pub(crate) fn as_ref_vote(&self) -> RefVote<'_> {
        RefVote::new(&self.vote.leader_id, true)
    }
}

impl fmt::Display for CommittedVote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.vote.fmt(f)
    }
}
