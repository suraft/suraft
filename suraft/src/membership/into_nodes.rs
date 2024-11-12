use std::collections::BTreeMap;
use std::collections::BTreeSet;

use maplit::btreemap;

use crate::NodeId;
use crate::N;

/// Convert into a map of `Node`.
///
/// This is used as a user input acceptor when building a Membership, to convert various input types
/// into a map of `Node`.
pub trait IntoNodes {
    fn into_nodes(self) -> BTreeMap<NodeId, N>;
}

impl IntoNodes for () {
    fn into_nodes(self) -> BTreeMap<NodeId, N> {
        btreemap! {}
    }
}

impl IntoNodes for Option<BTreeSet<NodeId>> {
    fn into_nodes(self) -> BTreeMap<NodeId, N> {
        match self {
            Some(set) => set.into_nodes(),
            None => btreemap! {},
        }
    }
}
impl IntoNodes for BTreeSet<NodeId> {
    fn into_nodes(self) -> BTreeMap<NodeId, N> {
        self.into_iter().map(|id| (id, N::new(""))).collect()
    }
}

impl IntoNodes for BTreeMap<NodeId, N> {
    fn into_nodes(self) -> BTreeMap<NodeId, N> {
        self
    }
}
