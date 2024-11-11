use std::collections::BTreeMap;
use std::collections::BTreeSet;

use maplit::btreemap;

use crate::Node;
use crate::NID;

/// Convert into a map of `Node`.
///
/// This is used as a user input acceptor when building a Membership, to convert various input types
/// into a map of `Node`.
pub trait IntoNodes<N>
where N: Node
{
    fn into_nodes(self) -> BTreeMap<NID, N>;
}

impl<N> IntoNodes<N> for ()
where N: Node
{
    fn into_nodes(self) -> BTreeMap<NID, N> {
        btreemap! {}
    }
}

impl<N> IntoNodes<N> for BTreeSet<NID>
where N: Node
{
    fn into_nodes(self) -> BTreeMap<NID, N> {
        self.into_iter().map(|node_id| (node_id, N::default())).collect()
    }
}

impl<N> IntoNodes<N> for Option<BTreeSet<NID>>
where N: Node
{
    fn into_nodes(self) -> BTreeMap<NID, N> {
        match self {
            None => BTreeMap::new(),
            Some(s) => s.into_iter().map(|node_id| (node_id, N::default())).collect(),
        }
    }
}

impl<N> IntoNodes<N> for BTreeMap<NID, N>
where N: Node
{
    fn into_nodes(self) -> BTreeMap<NID, N> {
        self
    }
}
