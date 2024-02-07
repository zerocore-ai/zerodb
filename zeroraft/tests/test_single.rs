mod fixtures;

use std::time::Duration;

use tokio::time;

use crate::fixtures::RaftNodeServer;

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[tokio::test]
async fn test_single_server_can_shutdown() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let server = RaftNodeServer::builder().build();

    // Start the server.
    server.start();

    // Shutdown the server.
    server.shutdown().await?;

    // Wait for shutdown to complete.
    time::sleep(Duration::from_millis(100)).await;

    assert!(server.get_node().is_shutdown_state().await);

    Ok(())
}
