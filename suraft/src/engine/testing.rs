use std::fmt::Display;
use std::io::Cursor;

use crate::impls::TokioRuntime;
use crate::RaftTypeConfig;

/// Trivial Raft type config for Engine related unit tests,
/// with an optional custom node type `N` for Node type.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub(crate) struct UTConfig {}

impl RaftTypeConfig for UTConfig {
    type AppData = ();
    type AppResponse = ();
    type Entry = crate::Entry<Self>;
    type SnapshotData = Cursor<Vec<u8>>;
    type AsyncRuntime = TokioRuntime;
    type Responder = crate::impls::OneshotResponder<Self>;
}

pub(crate) fn s(x: impl Display) -> String {
    format!("{}", x)
}
