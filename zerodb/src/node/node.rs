use std::sync::Arc;

use tokio::sync::{mpsc, Mutex};
use zeroraft::{channels, ClientRequest, NodeId, PeerRpc, RaftNode};

use crate::{
    configs::ZerodbConfig, server, store::MemoryStore, Query, QueryResponse, Result,
    ZerodbNodeBuilder,
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

// TODO(appcypher): To be replaced by a proper implementation.
/// This is a convenience type alias for a Raft node with an in-memory store.
pub type MemRaftNode<R, P> = RaftNode<MemoryStore<R>, R, P>;

type OutRpcReciever = Arc<Mutex<mpsc::UnboundedReceiver<(NodeId, PeerRpc<Query>)>>>;
type InClientRequestSender = mpsc::UnboundedSender<ClientRequest<Query, QueryResponse>>;

/// A `zerodb` node.
pub struct ZerodbNode {
    config: ZerodbConfig,
    node: MemRaftNode<Query, QueryResponse>,
    _in_rpc_tx: mpsc::UnboundedSender<PeerRpc<Query>>,
    out_rpc_rx: OutRpcReciever,
    in_client_request_tx: InClientRequestSender,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl ZerodbNode {
    /// Creates a new `ZerodbNode` builder.
    pub fn builder() -> ZerodbNodeBuilder {
        ZerodbNodeBuilder::default()
    }

    /// Creates a new `ZerodbNode` instance with the given configuration.
    pub fn with_config(config: ZerodbConfig) -> Result<Self> {
        // Create channels.
        let (raft_channels, outside_channels) = channels::create();

        // Create in-memory store.
        let store = MemoryStore::default();

        // Create Raft Node.
        let raft_node = MemRaftNode::<Query, QueryResponse>::builder()
            .id(config.network.id)
            .store(store)
            .channels(raft_channels)
            .election_timeout_range(config.network.consensus.election_timeout_range)
            .heartbeat_interval(config.network.consensus.heartbeat_interval)
            .seeds(config.network.seeds.clone())
            .build()?;

        Ok(Self {
            config,
            node: raft_node,
            _in_rpc_tx: outside_channels.in_rpc_tx,
            out_rpc_rx: Arc::new(outside_channels.out_rpc_rx),
            in_client_request_tx: outside_channels.in_client_request_tx,
        })
    }

    /// Starts the ZerodbNode instance.
    pub async fn start(&self) -> Result<()> {
        // Start Raft Node.
        let raft_handle = self.node.start();

        // TCP server for client connections.
        server::start_client_server(
            self.config.network.get_client_address(),
            self.in_client_request_tx.clone(),
        );

        // Forward outgoing requests.
        server::forward_outgoing_requests(self.node.clone(), Arc::clone(&self.out_rpc_rx));

        // Wait for Raft Node to stop.
        raft_handle.await??;

        Ok(())
    }

    /// Returns the configuration of the ZerodbNode instance.
    pub fn get_config(&self) -> &ZerodbConfig {
        &self.config
    }

    /// Shuts down the ZerodbNode instance.
    pub async fn shutdown(&self) -> Result<()> {
        self.node.shutdown().await?;
        Ok(())
    }
}
