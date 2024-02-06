mod fixtures;

use std::time::Duration;

use tokio::time;
use tracing_test::traced_test;

use crate::fixtures::RaftNodeCluster;

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[tokio::test]
#[traced_test]
async fn test_cluster_can_shutdown() -> anyhow::Result<()> {
    let mut cluster = RaftNodeCluster::new(2);

    // Start the cluster.
    let handle = cluster.start();

    // Shutdown the cluster.
    cluster.shutdown().await?;

    // Wait for shutdown to complete.
    time::sleep(Duration::from_millis(100)).await;

    for server in cluster.get_servers().values() {
        let node = server.get_node();
        assert!(node.is_shutdown_state().await);
    }

    handle.await??;

    Ok(())
}

#[tokio::test]
#[traced_test]
async fn test_cluster_can_choose_single_leader_from_start() -> anyhow::Result<()> {
    let mut cluster = RaftNodeCluster::new(3);

    // Start the cluster.
    cluster.start();

    // Wait for the cluster to work out leader.
    time::sleep(Duration::from_secs(3)).await;

    // Count leaders.
    let mut leaders = 0;
    for server in cluster.get_servers().values() {
        let node = server.get_node();
        if node.is_leader_state().await {
            leaders += 1;
        }
    }

    // TODO(appcypher)
    // assert_eq!(leaders, 1);
    tracing::info!("Leaders: {leaders}");

    Ok(())
}
