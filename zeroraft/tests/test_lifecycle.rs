use std::time::Duration;

use tokio::time;

use crate::fixtures::{RaftNodeCluster, RaftNodeServer};

mod fixtures;

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[test_log::test(tokio::test)]
async fn test_cluster_can_shutdown() -> anyhow::Result<()> {
    let mut cluster = RaftNodeCluster::new(3)?;

    // Start the cluster.
    let handle = cluster.start();

    // Shutdown the cluster.
    cluster.shutdown().await?;

    // Wait for shutdown to complete.
    time::sleep(Duration::from_millis(100)).await;

    for server in cluster.get_servers().values() {
        let node = server.get_node();
        // Assert that the server is in shutdown state.
        assert!(node.is_shutdown_state().await);
    }

    handle.await??;

    Ok(())
}

#[tokio::test]
async fn test_single_server_can_shutdown() -> anyhow::Result<()> {
    let server = RaftNodeServer::builder().build()?;

    // Start the server.
    server.start();

    // Shutdown the server.
    server.shutdown().await?;

    // Wait for shutdown to complete.
    time::sleep(Duration::from_millis(100)).await;

    // Assert that the server is in shutdown state.
    assert!(server.get_node().is_shutdown_state().await);

    Ok(())
}
