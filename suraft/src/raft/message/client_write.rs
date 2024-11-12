use std::fmt;
use std::fmt::Debug;

use suraft_macros::since;

use crate::display_ext::DisplayOptionExt;
use crate::error::ClientWriteError;
use crate::LogId;
use crate::Membership;
use crate::RaftTypeConfig;

/// The result of a write request to Raft.
pub type ClientWriteResult<C> = Result<ClientWriteResponse<C>, ClientWriteError>;

/// The response to a client-request.
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(bound = "C::AppResponse: crate::AppResponse")
)]
pub struct ClientWriteResponse<C: RaftTypeConfig> {
    /// The id of the log that is applied.
    pub log_id: LogId,

    /// Application specific response data.
    pub data: C::AppResponse,

    /// If the log entry is a change-membership entry.
    pub membership: Option<Membership>,
}

impl<C> ClientWriteResponse<C>
where C: RaftTypeConfig
{
    /// Create a new instance of `ClientWriteResponse`.
    #[allow(dead_code)]
    #[since(version = "0.9.5")]
    pub(crate) fn new_app_response(log_id: LogId, data: C::AppResponse) -> Self {
        Self {
            log_id,
            data,
            membership: None,
        }
    }

    #[since(version = "0.9.5")]
    pub fn log_id(&self) -> &LogId {
        &self.log_id
    }

    #[since(version = "0.9.5")]
    pub fn response(&self) -> &C::AppResponse {
        &self.data
    }

    /// Return membership config if the log entry is a change-membership entry.
    #[since(version = "0.9.5")]
    pub fn membership(&self) -> &Option<Membership> {
        &self.membership
    }
}

impl<C: RaftTypeConfig> Debug for ClientWriteResponse<C>
where C::AppResponse: Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClientWriteResponse")
            .field("log_id", &self.log_id)
            .field("data", &self.data)
            .field("membership", &self.membership)
            .finish()
    }
}

impl<C> fmt::Display for ClientWriteResponse<C>
where C: RaftTypeConfig
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ClientWriteResponse{{log_id:{}, membership:{}}}",
            self.log_id,
            self.membership.display()
        )
    }
}
