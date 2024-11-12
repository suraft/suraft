use std::cmp::Ordering;
use std::fmt;

use crate::display_ext::DisplayOptionExt;
use crate::NodeId;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize), serde(bound = ""))]
pub struct LeaderId {
    pub term: u64,

    pub voted_for: Option<NodeId>,
}

impl PartialOrd for LeaderId {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match PartialOrd::partial_cmp(&self.term, &other.term) {
            Some(Ordering::Equal) => {
                //
                match (&self.voted_for, &other.voted_for) {
                    (None, None) => Some(Ordering::Equal),
                    (Some(_), None) => Some(Ordering::Greater),
                    (None, Some(_)) => Some(Ordering::Less),
                    (Some(a), Some(b)) => {
                        if a == b {
                            Some(Ordering::Equal)
                        } else {
                            None
                        }
                    }
                }
            }
            cmp => cmp,
        }
    }
}

impl LeaderId {
    pub fn new(term: u64, node_id: NodeId) -> Self {
        Self {
            term,
            voted_for: Some(node_id),
        }
    }

    pub fn term(&self) -> u64 {
        self.term
    }

    pub fn voted_for(&self) -> Option<NodeId> {
        self.voted_for.clone()
    }

    /// Return if it is the same leader as the committed leader id.
    ///
    /// A committed leader may have less info than a non-committed.
    pub(crate) fn is_same_term(&self, other_term: u64) -> bool {
        self.term == other_term
    }
}

impl fmt::Display for LeaderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "T{}-N{}", self.term, self.voted_for.display())
    }
}

#[cfg(test)]
#[allow(clippy::nonminimal_bool)]
mod tests {
    use crate::engine::testing::s;
    use crate::LeaderId;

    #[test]
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    fn test_leader_id_partial_order() -> anyhow::Result<()> {
        #[allow(clippy::redundant_closure)]
        let lid = |term, node_id: u64| LeaderId::new(term, s(node_id));

        let lid_none = |term| LeaderId { term, voted_for: None };

        // Compare term first
        assert!(lid(2, 2) > lid(1, 2));
        assert!(lid(1, 2) < lid(2, 2));

        // Equal term, Some > None
        assert!(lid(2, 2) > lid_none(2));
        assert!(lid_none(2) < lid(2, 2));

        // Equal
        assert!(lid(2, 2) == lid(2, 2));
        assert!(lid(2, 2) >= lid(2, 2));
        assert!(lid(2, 2) <= lid(2, 2));

        // Incomparable
        assert!(!(lid(2, 2) > lid(2, 3)));
        assert!(!(lid(2, 2) < lid(2, 3)));
        assert!(!(lid(2, 2) == lid(2, 3)));

        Ok(())
    }
}
