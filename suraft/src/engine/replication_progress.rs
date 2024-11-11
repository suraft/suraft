use std::fmt;

use crate::progress::entry::ProgressEntry;
use crate::NID;

#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub(crate) struct ReplicationProgress(pub NID, pub ProgressEntry);

impl fmt::Display for ReplicationProgress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ReplicationProgress({}={})", self.0, self.1)
    }
}
