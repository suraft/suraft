use std::sync::Arc;

use anyhow::Result;
use maplit::btreeset;
use suraft::error::Fatal;
use suraft::testing::log_id;
use suraft::Config;

use crate::fixtures::ut_harness;
use crate::fixtures::RaftRouter;

/// Access Raft state via [`Raft::with_raft_state()`](suraft::Raft::with_raft_state)
#[tracing::instrument]
#[test_harness::test(harness = ut_harness)]
async fn with_raft_state() -> Result<()> {
    let config = Arc::new(
        Config {
            enable_heartbeat: false,
            ..Default::default()
        }
        .validate()?,
    );

    let mut router = RaftRouter::new(config.clone());

    tracing::info!("--- initializing cluster");
    let log_index = router.new_cluster(btreeset! {s(0),s(1),s(2)}, btreeset! {}).await?;

    let n0 = router.get_raft_handle(&s(0))?;

    let committed = n0.with_raft_state(|st| st.committed).await?;
    assert_eq!(committed, Some(log_id(1, log_index)));

    tracing::info!("--- shutting down node 0");
    n0.shutdown().await?;

    let res = n0.with_raft_state(|st| st.committed).await;
    assert_eq!(Err(Fatal::Stopped), res);

    Ok(())
}
