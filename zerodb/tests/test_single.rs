mod fixtures;

use std::time::Duration;

use tokio::time;
use tracing_test::traced_test;

use crate::fixtures::RaftNodeServer;

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[tokio::test]
#[traced_test]
async fn test_single_server_can_shutdown() -> anyhow::Result<()> {
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
