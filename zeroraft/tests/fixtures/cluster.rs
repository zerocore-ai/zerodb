use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use anyhow::Ok;
use futures::channel::oneshot;
use tokio::task::JoinHandle;
use zeroraft::{NodeId, DEFAULT_ELECTION_TIMEOUT_RANGE, DEFAULT_HEARTBEAT_INTERVAL};

use super::RaftNodeServer;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// `RaftNodeCluster` is a struct representing a cluster of Raft node servers.
pub struct RaftNodeCluster {
    servers: HashMap<NodeId, Arc<RaftNodeServer>>,
    kill_tx: Option<oneshot::Sender<()>>,
}

pub struct RaftNodeClusterConfig {
    pub election_timeout_range: (u64, u64),
    pub heartbeat_interval: u64,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl RaftNodeCluster {
    /// Create a new `RaftNodeCluster` with `count` number of servers.
    pub fn new(count: u16) -> anyhow::Result<Self> {
        Self::new_with_config(count, RaftNodeClusterConfig::default())
    }

    /// Create a new `RaftNodeCluster` with `count` number of servers and a custom configuration.
    pub fn new_with_config(num_nodes: u16, config: RaftNodeClusterConfig) -> anyhow::Result<Self> {
        // Create `count` number of servers with new ids and let them take `count` addresses (SocketAddr) as peers.
        let peer_addrs = (0..num_nodes)
            .map(|i| {
                (
                    NodeId::new_v4(),
                    SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 5550 + i * 2),
                )
            })
            .collect::<HashMap<_, _>>();

        let mut servers = HashMap::new();
        for (id, peer_addr) in peer_addrs.iter() {
            let server = RaftNodeServer::builder()
                .id(*id)
                .peer_addr(peer_addr.clone())
                .client_addr(SocketAddr::new(peer_addr.ip(), peer_addr.port() + 1))
                .seeds(peer_addrs.clone())
                .election_timeout_range(config.election_timeout_range)
                .heartbeat_interval(config.heartbeat_interval)
                .build()?;

            servers.insert(*id, Arc::new(server));
        }

        Ok(Self {
            servers,
            kill_tx: None,
        })
    }

    /// Start the cluster.
    pub fn start(&mut self) -> JoinHandle<anyhow::Result<()>> {
        for server in self.servers.values() {
            let server = Arc::clone(server);
            tokio::spawn(async move {
                server.start().await??;
                Ok(())
            });
        }

        let (kill_tx, kill_rx) = oneshot::channel::<()>();
        self.kill_tx = Some(kill_tx);

        tokio::spawn(async move {
            kill_rx
                .await
                .map_err(|_| anyhow::anyhow!("Failed to receive kill signal"))
        })
    }

    /// Get the servers in the cluster.
    pub fn get_servers(&self) -> &HashMap<NodeId, Arc<RaftNodeServer>> {
        &self.servers
    }

    /// Shutdown the cluster.
    pub async fn shutdown(&mut self) -> anyhow::Result<()> {
        for server in self.servers.values() {
            server.shutdown().await?;
        }

        if let Some(kill_tx) = self.kill_tx.take() {
            kill_tx
                .send(())
                .map_err(|_| anyhow::anyhow!("Failed to send kill signal"))?;
        }

        Ok(())
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for RaftNodeClusterConfig {
    fn default() -> Self {
        Self {
            election_timeout_range: DEFAULT_ELECTION_TIMEOUT_RANGE,
            heartbeat_interval: DEFAULT_HEARTBEAT_INTERVAL,
        }
    }
}
