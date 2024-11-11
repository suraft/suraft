use crate::proposer::Candidate;
use crate::proposer::Leader;
use crate::quorum::Joint;
use crate::NID;

/// The quorum set type used by `Leader`.
pub(crate) type LeaderQuorumSet = Joint<NID, Vec<NID>, Vec<Vec<NID>>>;

pub(crate) type LeaderState<C> = Option<Box<Leader<C, LeaderQuorumSet>>>;
pub(crate) type CandidateState<C> = Option<Candidate<C, LeaderQuorumSet>>;
