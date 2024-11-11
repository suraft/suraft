use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

use validit::Validate;

use crate::display_ext::DisplayOptionExt;
use crate::LogId;
use crate::LogIdOptionExt;

// TODO: I need just a range, but not a log id range.

/// A log id range of continuous series of log entries.
///
/// The range of log to send is left open right close: `(prev, last]`.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq)]
pub(crate) struct LogIdRange {
    /// The prev log id before the first to send, exclusive.
    pub(crate) prev: Option<LogId>,

    /// The last log id to send, inclusive.
    pub(crate) last: Option<LogId>,
}

impl Display for LogIdRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}]", self.prev.display(), self.last.display())
    }
}

impl Validate for LogIdRange {
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        validit::less_equal!(&self.prev, &self.last);
        Ok(())
    }
}

impl LogIdRange {
    pub(crate) fn new(prev: Option<LogId>, last: Option<LogId>) -> Self {
        Self { prev, last }
    }

    #[allow(dead_code)]
    pub(crate) fn len(&self) -> u64 {
        self.last.next_index() - self.prev.next_index()
    }
}

#[cfg(test)]
mod tests {
    use validit::Valid;

    use crate::log_id_range::LogIdRange;
    use crate::LogId;

    fn log_id(index: u64) -> LogId {
        LogId { term: 1, index }
    }

    #[test]
    fn test_log_id_range_validate() -> anyhow::Result<()> {
        let res = std::panic::catch_unwind(|| {
            let r = Valid::new(LogIdRange::new(Some(log_id(5)), None));
            let _x = &r.last;
        });
        tracing::info!("res: {:?}", res);
        assert!(res.is_err(), "prev(5) > last(None)");

        let res = std::panic::catch_unwind(|| {
            let r = Valid::new(LogIdRange::new(Some(log_id(5)), Some(log_id(4))));
            let _x = &r.last;
        });
        tracing::info!("res: {:?}", res);
        assert!(res.is_err(), "prev(5) > last(4)");

        Ok(())
    }
}
