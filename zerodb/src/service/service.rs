use std::sync::Arc;

use tokio::sync::{mpsc, Mutex};
use zeroraft::{channels, ClientRequest, NodeId, PeerRpc, RaftNode};

use crate::{config::ZerodbConfig, server, MemoryState, Query, QueryResponse, ZerodbResult};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

// TODO(appcypher): To be replaced by a proper implementation.
/// This is a convenience type alias for a Raft node with an in-memory store.
pub type MemRaftNode = RaftNode<MemoryState<Query>, Query, QueryResponse>;

type OutRpcReciever = Arc<Mutex<mpsc::UnboundedReceiver<(NodeId, PeerRpc<Query>)>>>;
type InClientRequestSender = mpsc::UnboundedSender<ClientRequest<Query, QueryResponse>>;

/// A `zerodb` node.
pub struct ZerodbService {
    config: ZerodbConfig,
    node: MemRaftNode,
    _in_rpc_tx: mpsc::UnboundedSender<PeerRpc<Query>>,
    out_rpc_rx: OutRpcReciever,
    in_client_request_tx: InClientRequestSender,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl ZerodbService {
    // TODO(appcypher): Need to support did IDs in zeroraft.
    // /// Creates a new `ZerodbService` builder.
    // pub fn builder() -> ZerodbServiceBuilder {
    //     ZerodbServiceBuilder::default()
    // }

    /// Creates a new `ZerodbService` instance with the given configuration.
    pub fn with_config(config: ZerodbConfig) -> ZerodbResult<Self> {
        // Create channels.
        let (raft_channels, outside_channels) = channels::create();

        // Create in-memory state.
        let state = MemoryState::default();

        // TODO(appcypher): Need to support did IDs in zeroraft.
        // Create Raft Node.
        let raft_node = MemRaftNode::builder()
            // .id(config.network.id)
            .channels(raft_channels)
            .state(state)
            .election_timeout_range(config.network.consensus.election_timeout_range)
            .heartbeat_interval(config.network.consensus.heartbeat_interval)
            // .seeds(config.network.seeds.clone())
            .build()?;

        Ok(Self {
            config,
            node: raft_node,
            _in_rpc_tx: outside_channels.in_rpc_tx,
            out_rpc_rx: Arc::new(outside_channels.out_rpc_rx),
            in_client_request_tx: outside_channels.in_client_request_tx,
        })
    }

    /// Returns the configuration of the ZerodbService instance.
    pub fn get_config(&self) -> &ZerodbConfig {
        &self.config
    }

    /// Shuts down the ZerodbService instance.
    pub async fn shutdown(&self) -> ZerodbResult<()> {
        self.node.shutdown().await?;
        Ok(())
    }

    /// Starts the ZerodbService instance.
    pub async fn start(&self) -> ZerodbResult<()> {
        // Start Raft Node.
        let raft_handle = self.node.start();

        // TCP server for client connections.
        server::start_client_server(
            self.config.network.get_user_address(),
            self.in_client_request_tx.clone(),
        );

        // Forward outgoing requests.
        server::forward_outgoing_requests(self.node.clone(), Arc::clone(&self.out_rpc_rx));

        // Wait for Raft Node to stop.
        raft_handle.await??;

        Ok(())
    }
}
