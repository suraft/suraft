use crate::proposer::Candidate;
use crate::proposer::Leader;
use crate::quorum::Joint;
use crate::NodeId;

/// The quorum set type used by `Leader`.
pub(crate) type LeaderQuorumSet = Joint<NodeId, Vec<NodeId>, Vec<Vec<NodeId>>>;

pub(crate) type LeaderState<C> = Option<Box<Leader<C, LeaderQuorumSet>>>;
pub(crate) type CandidateState<C> = Option<Candidate<C, LeaderQuorumSet>>;
