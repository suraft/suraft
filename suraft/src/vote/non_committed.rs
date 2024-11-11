use std::fmt;

use crate::vote::ref_vote::RefVote;
use crate::LeaderId;
use crate::Vote;

/// Represents a non-committed Vote that has **NOT** been granted by a quorum.
///
/// The inner `Vote`'s attribute `committed` is always set to `false`
#[derive(Debug, Clone)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd)]
pub(crate) struct NonCommittedVote {
    vote: Vote,
}

impl NonCommittedVote {
    pub(crate) fn new(vote: Vote) -> Self {
        debug_assert!(!vote.committed);
        Self { vote }
    }

    pub(crate) fn leader_id(&self) -> &LeaderId {
        &self.vote.leader_id
    }

    pub(crate) fn into_vote(self) -> Vote {
        self.vote
    }

    pub(crate) fn as_ref_vote(&self) -> RefVote<'_> {
        RefVote::new(&self.vote.leader_id, false)
    }
}

impl fmt::Display for NonCommittedVote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.vote.fmt(f)
    }
}
