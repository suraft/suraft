use std::time::Duration;

use crate::engine::time_state;
use crate::type_config::alias::AsyncRuntimeOf;
use crate::Config;
use crate::RaftTypeConfig;
use crate::SnapshotPolicy;
use crate::NID;

/// Config for Engine
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq)]
pub(crate) struct EngineConfig {
    /// The id of this node.
    pub(crate) id: NID,

    /// The snapshot policy to use for a Raft node.
    pub(crate) snapshot_policy: SnapshotPolicy,

    /// The maximum number of applied logs to keep before purging.
    pub(crate) max_in_snapshot_log_to_keep: u64,

    /// The minimal number of applied logs to purge in a batch.
    pub(crate) purge_batch_size: u64,

    /// The maximum number of entries per payload allowed to be transmitted during replication
    pub(crate) max_payload_entries: u64,

    pub(crate) timer_config: time_state::Config,
}

impl EngineConfig {
    pub(crate) fn new<C: RaftTypeConfig>(id: NID, config: &Config) -> Self {
        let election_timeout = Duration::from_millis(config.new_rand_election_timeout::<AsyncRuntimeOf<C>>());
        Self {
            id,
            snapshot_policy: config.snapshot_policy.clone(),
            max_in_snapshot_log_to_keep: config.max_in_snapshot_log_to_keep,
            purge_batch_size: config.purge_batch_size,
            max_payload_entries: config.max_payload_entries,
            timer_config: time_state::Config {
                election_timeout,
                smaller_log_timeout: Duration::from_millis(config.election_timeout_max * 2),
                leader_lease: Duration::from_millis(config.election_timeout_max),
            },
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_default(id: NID) -> Self {
        Self {
            id,
            snapshot_policy: SnapshotPolicy::LogsSinceLast(5000),
            max_in_snapshot_log_to_keep: 1000,
            purge_batch_size: 256,
            max_payload_entries: 300,
            timer_config: time_state::Config::default(),
        }
    }
}
