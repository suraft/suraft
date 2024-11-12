use crate::error::Operation;
use crate::NodeId;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize), serde(bound = ""))]
#[error("Node {node_id} not found when: ({operation})")]
pub struct NodeNotFound {
    pub node_id: NodeId,
    pub operation: Operation,
}

impl NodeNotFound {
    pub fn new(node_id: NodeId, operation: Operation) -> Self {
        Self { node_id, operation }
    }
}
