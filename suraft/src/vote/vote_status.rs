use crate::vote::CommittedVote;
use crate::vote::NonCommittedVote;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum VoteStatus {
    Committed(CommittedVote),
    Pending(NonCommittedVote),
}
