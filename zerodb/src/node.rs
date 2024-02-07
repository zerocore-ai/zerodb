use zeroraft::{channels, MemRaftNode, OutsideChannels};

use crate::{configs::ZerodbConfig, Query, QueryResponse, Result};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Represents a ZerodbNode instance.
pub struct ZerodbNode {
    config: ZerodbConfig,
    outside_channels: Option<OutsideChannels<Query, QueryResponse>>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl ZerodbNode {
    /// Creates a new ZerodbNode instance.
    pub fn new(config: ZerodbConfig) -> Result<Self> {
        config.validate()?;
        Ok(Self {
            config,
            outside_channels: None,
        })
    }

    /// Starts the ZerodbNode instance.
    pub async fn start(&mut self) -> Result<()> {
        // Create channels.
        let (raft_channels, outside_channels) = channels::create();

        // Create Raft Node.
        let raft_node = MemRaftNode::<Query, QueryResponse>::builder()
            .channels(raft_channels)
            .build();

        // Save other channels.
        self.outside_channels = Some(outside_channels);

        // Start Raft Node.
        raft_node.start().await??;

        Ok(())
    }

    /// Returns the configuration of the ZerodbNode instance.
    pub fn get_config(&self) -> &ZerodbConfig {
        &self.config
    }

    /// Shuts down the ZerodbNode instance.
    pub async fn shutdown(&self) -> Result<()> {
        if let Some(outside_channels) = &self.outside_channels {
            outside_channels.shutdown_tx.send(()).await?;
        }

        Ok(())
    }
}
