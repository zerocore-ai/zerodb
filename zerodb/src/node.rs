use crate::{client::ClientServer, config::ZerodbConfig, raft::RaftNode, Result};
use std::time::Duration;
use tokio::time;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Represents a ZerodbNode instance.
pub struct ZerodbNode {
    config: ZerodbConfig,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl ZerodbNode {
    /// Creates a new ZerodbNode instance.
    pub fn new(config: ZerodbConfig) -> Result<Self> {
        config.validate()?;
        Ok(Self { config })
    }

    /// Starts the ZerodbNode instance.
    pub async fn start(&self) -> Result<()> {
        let (handle, rx) =
            ClientServer::default().start(self.config.network.get_client_address()?)?;

        let (result,) = tokio::join!(handle);
        result??;

        // let (_, client_rx) =
        //     ClientServer::default().start(self.config.network.get_client_address()?)?;

        // let raft_service_handle =
        //     RaftNode::default().start(self.config.network.get_peer_address()?, client_rx);

        // tokio::select! {
        //     _ = raft_service_handle => (),
        //     _ = Self::shutdown_signal() => ()
        // }

        Ok(())
    }

    /// Sets up shutdown signal for the ZerodbNode instance.
    async fn shutdown_signal() {
        time::sleep(Duration::from_secs(120)).await;
    }

    /// Returns the configuration of the ZerodbNode instance.
    pub fn get_config(&self) -> &ZerodbConfig {
        &self.config
    }
}
