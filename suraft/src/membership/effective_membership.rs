use std::collections::BTreeSet;
use std::fmt;
use std::fmt::Debug;
use std::sync::Arc;

use crate::display_ext::DisplayOptionExt;
use crate::log_id::RaftLogId;
use crate::quorum::Joint;
use crate::quorum::QuorumSet;
use crate::LogId;
use crate::Membership;
use crate::Node;
use crate::NodeId;
use crate::StoredMembership;

/// The currently active membership config.
///
/// It includes:
/// - the id of the log that sets this membership config,
/// - and the config.
///
/// An active config is just the last seen config in raft spec.
#[derive(Clone, Default, Eq)]
pub struct EffectiveMembership {
    stored_membership: Arc<StoredMembership>,

    /// The quorum set built from `membership`.
    quorum_set: Joint<NodeId, Vec<NodeId>, Vec<Vec<NodeId>>>,

    /// Cache of union of all members
    voter_ids: BTreeSet<NodeId>,
}

impl Debug for EffectiveMembership {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EffectiveMembership")
            .field("log_id", self.log_id())
            .field("membership", self.membership())
            .field("voter_ids", &self.voter_ids)
            .finish()
    }
}

impl PartialEq for EffectiveMembership {
    fn eq(&self, other: &Self) -> bool {
        self.stored_membership == other.stored_membership && self.voter_ids == other.voter_ids
    }
}

impl<LID> From<(&LID, Membership)> for EffectiveMembership
where LID: RaftLogId
{
    fn from(v: (&LID, Membership)) -> Self {
        EffectiveMembership::new(Some(v.0.get_log_id().clone()), v.1)
    }
}

impl EffectiveMembership {
    pub(crate) fn new_arc(log_id: Option<LogId>, membership: Membership) -> Arc<Self> {
        Arc::new(Self::new(log_id, membership))
    }

    pub fn new(log_id: Option<LogId>, membership: Membership) -> Self {
        let voter_ids = membership.voter_ids().collect();

        let configs = membership.get_joint_config();
        let mut joint = vec![];
        for c in configs {
            joint.push(c.iter().cloned().collect::<Vec<_>>());
        }

        let quorum_set = Joint::from(joint);

        Self {
            stored_membership: Arc::new(StoredMembership::new(log_id, membership)),
            quorum_set,
            voter_ids,
        }
    }

    pub(crate) fn new_from_stored_membership(stored: StoredMembership) -> Self {
        Self::new(stored.log_id().clone(), stored.membership().clone())
    }

    pub(crate) fn stored_membership(&self) -> &Arc<StoredMembership> {
        &self.stored_membership
    }

    pub fn log_id(&self) -> &Option<LogId> {
        self.stored_membership.log_id()
    }

    pub fn membership(&self) -> &Membership {
        self.stored_membership.membership()
    }
}

/// Membership API
impl EffectiveMembership {
    #[allow(dead_code)]
    pub(crate) fn is_voter(&self, nid: &NodeId) -> bool {
        self.membership().is_voter(nid)
    }

    /// Returns an Iterator of all voter node ids. Learners are not included.
    pub fn voter_ids(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.voter_ids.iter().cloned()
    }

    /// Returns an Iterator of all learner node ids. Voters are not included.
    pub(crate) fn learner_ids(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.membership().learner_ids()
    }

    /// Get a node(either voter or learner) by node id.
    pub fn get_node(&self, node_id: &NodeId) -> Option<&Node> {
        self.membership().get_node(node_id)
    }

    /// Returns an Iterator of all nodes(voters and learners).
    pub fn nodes(&self) -> impl Iterator<Item = (&NodeId, &Node)> {
        self.membership().nodes()
    }

    /// Returns reference to the joint config.
    ///
    /// Membership is defined by a joint of multiple configs.
    /// Each config is a vec of node-id.
    pub fn get_joint_config(&self) -> &Vec<Vec<NodeId>> {
        self.quorum_set.children()
    }
}

impl fmt::Display for EffectiveMembership {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "EffectiveMembership{{log_id: {}, membership: {}}}",
            self.log_id().display(),
            self.membership()
        )
    }
}

/// Implement node-id joint quorum set.
impl QuorumSet<NodeId> for EffectiveMembership {
    type Iter = std::collections::btree_set::IntoIter<NodeId>;

    fn is_quorum<'a, I: Iterator<Item = &'a NodeId> + Clone>(&self, ids: I) -> bool {
        self.quorum_set.is_quorum(ids)
    }

    fn ids(&self) -> Self::Iter {
        self.quorum_set.ids()
    }
}
