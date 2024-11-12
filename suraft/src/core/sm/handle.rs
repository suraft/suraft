//! State machine control handle

use crate::async_runtime::MpscUnboundedSender;
use crate::async_runtime::SendError;
use crate::core::sm;
use crate::type_config::alias::JoinHandleOf;
use crate::type_config::alias::MpscUnboundedSenderOf;
use crate::RaftTypeConfig;

/// State machine worker handle for sending command to it.
pub(crate) struct Handle<C>
where C: RaftTypeConfig
{
    pub(in crate::core::sm) cmd_tx: MpscUnboundedSenderOf<C, sm::Command<C>>,

    #[allow(dead_code)]
    pub(in crate::core::sm) join_handle: JoinHandleOf<C, ()>,
}

impl<C> Handle<C>
where C: RaftTypeConfig
{
    pub(crate) fn send(&mut self, cmd: sm::Command<C>) -> Result<(), SendError<sm::Command<C>>> {
        tracing::debug!("sending command to state machine worker: {:?}", cmd);
        self.cmd_tx.send(cmd)
    }
}
