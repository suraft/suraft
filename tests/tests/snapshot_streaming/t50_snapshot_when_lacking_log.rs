use std::sync::Arc;

use anyhow::Result;
use maplit::btreeset;
use suraft::Config;
use suraft::LogId;
use suraft::SnapshotPolicy;

use crate::fixtures::s;
use crate::fixtures::ut_harness;
use crate::fixtures::RaftRouter;

/// A leader switch to snapshot replication if a log a follower/learner needs but is already purged.
///
/// - build a stable single node cluster.
/// - send enough requests to the node that log compaction will be triggered.
/// - add learner and assert that they receive the snapshot and logs.
#[tracing::instrument]
#[test_harness::test(harness = ut_harness)]
async fn switch_to_snapshot_replication_when_lacking_log() -> Result<()> {
    let snapshot_threshold: u64 = 20;
    let log_cnt = snapshot_threshold + 11;

    let config = Arc::new(
        Config {
            snapshot_policy: SnapshotPolicy::LogsSinceLast(snapshot_threshold),
            max_in_snapshot_log_to_keep: 0,
            purge_batch_size: 1,
            enable_heartbeat: false,
            ..Default::default()
        }
        .validate()?,
    );
    let mut router = RaftRouter::new(config.clone());

    let mut log_index = router.new_cluster(btreeset! {s(0)}, btreeset! {}).await?;

    tracing::info!(log_index, "--- send just enough logs to trigger snapshot");
    {
        router.client_request_many(s(0), "0", (snapshot_threshold - 1 - log_index) as usize).await?;
        log_index = snapshot_threshold - 1;

        router.wait_for_log(&btreeset![s(0)], Some(log_index), None, "send log to trigger snapshot").await?;

        router.wait_for_snapshot(&btreeset![s(0)], LogId::new(1, log_index), None, "snapshot").await?;
        router
            .assert_storage_state(
                1,
                log_index,
                Some(s(0)),
                LogId::new(1, log_index),
                Some((log_index.into(), 1)),
            )
            .await?;
    }

    tracing::info!(
        log_index,
        "--- send logs to make distance between snapshot index and last_log_index"
    );
    {
        router.client_request_many(s(0), "0", (log_cnt - log_index) as usize).await?;
        log_index = log_cnt;
    }

    tracing::info!(log_index, "--- add learner to receive snapshot and logs");
    {
        router.new_raft_node(s(1)).await;
        router.add_learner(s(0), s(1)).await.expect("failed to add new node as learner");
        log_index += 1;

        router.wait_for_log(&btreeset! {s(0), s(1)}, Some(log_index), None, "add learner").await?;
        router.wait_for_snapshot(&btreeset![s(1)], LogId::new(1, snapshot_threshold - 1), None, "").await?;
        let expected_snap = Some(((snapshot_threshold - 1).into(), 1));
        router
            .assert_storage_state(
                1,
                log_index,
                None, /* learner does not vote */
                LogId::new(1, log_index),
                expected_snap,
            )
            .await?;
    }

    Ok(())
}
