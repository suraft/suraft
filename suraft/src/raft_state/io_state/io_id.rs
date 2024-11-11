use std::cmp::Ordering;
use std::fmt;

use crate::raft_state::io_state::log_io_id::LogIOId;
use crate::vote::ref_vote::RefVote;
use crate::vote::CommittedVote;
use crate::vote::NonCommittedVote;
use crate::ErrorSubject;
use crate::ErrorVerb;
use crate::LogId;
use crate::Vote;

/// An ID to uniquely identify a monotonic increasing io operation to [`RaftLogStorage`].
///
/// Not all IO to [`RaftLogStorage`] are included by this struct:
/// only the IOs make progress in the raft log are included.
/// Including [`save_vote()`] and [`append()`] logs.
///
/// [`purge()`] and [`truncate()`] are not included,
/// because [`truncate()`] just remove junks: the logs that are conflict with the leader.
/// And [`purge()`] just remove logs that are already committed.
///
/// [`RaftLogStorage`]: `crate::storage::RaftLogStorage`
/// [`save_vote()`]: `crate::storage::RaftLogStorage::save_vote()`
/// [`append()`]: `crate::storage::RaftLogStorage::append()`
/// [`truncate()`]: `crate::storage::RaftLogStorage::truncate()`
/// [`purge()`]: `crate::storage::RaftLogStorage::purge()`
#[derive(Debug, Clone)]
#[derive(PartialEq, Eq)]
pub(crate) enum IOId {
    /// Saving a non-committed vote, this kind of IO is not related to any log entries.
    Vote(NonCommittedVote),

    /// Saving log entries by a Leader, which is identified by a committed vote.
    Log(LogIOId),
}

impl fmt::Display for IOId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Vote(vote) => write!(f, "({})", vote),
            Self::Log(log_id) => write!(f, "({})", log_id),
        }
    }
}

/// Implement `PartialOrd` for `IOId`
///
/// Compare the `vote` first, if votes are equal, compare the `last_log_id`.
impl PartialOrd for IOId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let res = self.as_ref_vote().partial_cmp(&other.as_ref_vote())?;
        match res {
            Ordering::Less => Some(Ordering::Less),
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Equal => self.last_log_id().partial_cmp(&other.last_log_id()),
        }
    }
}

impl IOId {
    pub(crate) fn new(vote: &Vote) -> Self {
        if vote.is_committed() {
            Self::new_log_io(vote.clone().into_committed(), None)
        } else {
            Self::new_vote_io(vote.clone().into_non_committed())
        }
    }

    pub(crate) fn new_vote_io(vote: NonCommittedVote) -> Self {
        Self::Vote(vote)
    }

    pub(crate) fn new_log_io(committed_vote: CommittedVote, last_log_id: Option<LogId>) -> Self {
        Self::Log(LogIOId::new(committed_vote, last_log_id))
    }

    /// Returns the vote the io operation is submitted by.
    #[allow(clippy::wrong_self_convention)]
    // The above lint is disabled because in future Vote may not be `Copy`
    pub(crate) fn to_vote(&self) -> Vote {
        match self {
            Self::Vote(non_committed_vote) => non_committed_vote.clone().into_vote(),
            Self::Log(log_io_id) => log_io_id.committed_vote.clone().into_vote(),
        }
    }

    pub(crate) fn as_ref_vote(&self) -> RefVote<'_> {
        match self {
            Self::Vote(non_committed_vote) => non_committed_vote.as_ref_vote(),
            Self::Log(log_io_id) => log_io_id.committed_vote.as_ref_vote(),
        }
    }

    pub(crate) fn last_log_id(&self) -> Option<&LogId> {
        match self {
            Self::Vote(_) => None,
            Self::Log(log_io_id) => log_io_id.log_id.as_ref(),
        }
    }

    /// Return the `subject` of this io operation, such as `Log` or `Vote`.
    pub(crate) fn subject(&self) -> ErrorSubject {
        match self {
            Self::Vote(_vote) => ErrorSubject::Vote,
            Self::Log(log_io_id) => {
                if let Some(log_id) = &log_io_id.log_id {
                    ErrorSubject::Log(log_id.clone())
                } else {
                    ErrorSubject::Logs
                }
            }
        }
    }

    /// Return the `verb` of this io operation, such as `Write` or `Read`.
    pub(crate) fn verb(&self) -> ErrorVerb {
        ErrorVerb::Write
    }
}
