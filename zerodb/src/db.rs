use crate::{client::ClientManager, config::ZerodbConfig, peer::PeerManager, Result};
use std::sync::Arc;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// An instance of the zerodb.
pub struct Zerodb {
    /// The configuration for the `Zerodb` instance.
    config: Arc<ZerodbConfig>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Zerodb {
    /// Creates a new `Zerodb` instance.
    pub fn new(config: ZerodbConfig) -> Result<Self> {
        config.validate()?;
        Ok(Self {
            config: Arc::new(config),
        })
    }

    /// Starts the `Zerodb` instance.
    pub async fn start(&self) -> Result<()> {
        let peer_mgr = PeerManager::new(Arc::clone(&self.config));
        let client_mgr = ClientManager::new(Arc::clone(&self.config));

        // Connect to the seed nodes.
        peer_mgr.establish_seed_connections();

        // Start listening for connections.
        let peer_handle = peer_mgr.listen().await?;
        let client_handle = client_mgr.listen().await?;

        // Join the handles.
        let (peer_task, client_task) = tokio::join!(peer_handle, client_handle);

        // Propagate errors.
        peer_task?;
        client_task?;

        Ok(())
    }

    /// Gets the config for the `Zerodb` instance.
    pub fn get_config(&self) -> &ZerodbConfig {
        &self.config
    }
}
