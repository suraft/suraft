//! Test the `declare_raft_types` macro with default values

use std::io::Cursor;

use crate::declare_raft_types;
use crate::impls::TokioRuntime;

declare_raft_types!(
    All:
        /// This is AppData
        AppData = (),
        #[allow(dead_code)]
        #[allow(dead_code)]
        AppResponse = (),
        Entry = crate::Entry<Self>,
        SnapshotData = Cursor<Vec<u8>>,
        AsyncRuntime = TokioRuntime,
        Responder = crate::impls::OneshotResponder<Self>,
);

declare_raft_types!(
    WithoutD:
        AppResponse = (),
        Entry = crate::Entry<Self>,
        SnapshotData = Cursor<Vec<u8>>,
        AsyncRuntime = TokioRuntime,
);

declare_raft_types!(
    WithoutR:
        AppData = (),
        Entry = crate::Entry<Self>,
        SnapshotData = Cursor<Vec<u8>>,
        AsyncRuntime = TokioRuntime,
);

declare_raft_types!(EmptyWithColon:);

declare_raft_types!(Empty);
