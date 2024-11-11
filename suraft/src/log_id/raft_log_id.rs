use crate::LogId;

/// Defines API to operate an object that contains a log-id, such as a log entry or a log id.
pub trait RaftLogId {
    fn term(&self) -> u64 {
        self.get_log_id().term()
    }

    /// Return a reference to the log-id it stores.
    fn get_log_id(&self) -> &LogId;

    /// Update the log id it contains.
    fn set_log_id(&mut self, log_id: &LogId);
}
