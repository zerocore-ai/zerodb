use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use anyhow::Ok;
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Mutex},
    task::JoinHandle,
};
use zeroraft::{
    channels,
    utils::mock::{MockRequest, MockResponse},
    AppendEntriesRequest, AppendEntriesResponse, ClientRequest, ClientResponse, NodeId, PeerRpc,
    RaftNode, RequestVoteRequest, RequestVoteResponse, DEFAULT_ELECTION_TIMEOUT_RANGE,
    DEFAULT_HEARTBEAT_INTERVAL,
};

use super::MemoryStore;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// This is a convenience type alias for a Raft node with an in-memory store.
pub type MemRaftNode<R, P> = RaftNode<MemoryStore<R>, R, P>;

/// `RaftNodeServer` wraps a Raft node and provides a server for client and peer connections.
pub struct RaftNodeServer {
    id: NodeId,
    client_addr: SocketAddr,
    peer_addr: SocketAddr,
    node: MemRaftNode<MockRequest, MockResponse>,
    in_rpc_tx: mpsc::UnboundedSender<PeerRpc<MockRequest>>,
    out_rpc_rx: Arc<Mutex<mpsc::UnboundedReceiver<(NodeId, PeerRpc<MockRequest>)>>>,
    in_client_request_tx: mpsc::UnboundedSender<ClientRequest<MockRequest, MockResponse>>,
}

/// `RaftNodeServerBuilder` is a builder for `RaftNodeServer`.
pub struct RaftNodeServerBuilder<N = (), C = ()> {
    seeds: HashMap<NodeId, SocketAddr>,
    id: NodeId,
    election_timeout_range: (u64, u64),
    heartbeat_interval: u64,
    client_addr: SocketAddr,
    peer_addr: SocketAddr,
    node: N,
    outside_channels: C,
}

/// `Rpc` is an enum representing the different types of RPC requests that can be sent to a Raft node.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Rpc {
    AppendEntries(AppendEntriesRequest<MockRequest>),
    RequestVote(RequestVoteRequest),
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl RaftNodeServer {
    /// Create a new RaftNodeServerBuilder.
    pub fn builder() -> RaftNodeServerBuilder {
        RaftNodeServerBuilder::default()
    }

    /// Start the Raft server.
    pub fn start(&self) -> JoinHandle<zeroraft::Result<()>> {
        // Start the Raft node.
        let raft_handle = self.node.start();

        tracing::debug!(id = self.id.to_string(), "Started Raft node");

        // TCP server for client connections.
        start_client_server(self.client_addr, self.in_client_request_tx.clone());

        tracing::debug!(
            id = self.id.to_string(),
            "Started client server: {}",
            self.client_addr
        );

        // TCP server for peer connections.
        start_peer_server(self.peer_addr, self.in_rpc_tx.clone());

        // Forward outgoing requests.
        forward_outgoing_requests(self.node.clone(), Arc::clone(&self.out_rpc_rx));

        tracing::debug!(
            id = self.id.to_string(),
            "Started peer server: {}",
            self.peer_addr
        );

        raft_handle
    }

    /// Shutdown the Raft server.
    pub async fn shutdown(&self) -> anyhow::Result<()> {
        self.node.shutdown().await?;
        Ok(())
    }

    /// Returns the Raft node.
    pub fn get_node(&self) -> &MemRaftNode<MockRequest, MockResponse> {
        &self.node
    }

    /// Returns the peer address of the Raft node.
    pub fn get_peer_addr(&self) -> SocketAddr {
        self.peer_addr
    }

    /// Returns the client address of the Raft node.
    pub fn get_client_addr(&self) -> SocketAddr {
        self.client_addr
    }
}

impl RaftNodeServerBuilder {
    /// Set the ID of the Raft node.
    pub fn id(mut self, id: NodeId) -> Self {
        self.id = id;
        self
    }

    /// Set the election timeout range of the Raft node.
    pub fn election_timeout_range(mut self, election_timeout_range: (u64, u64)) -> Self {
        self.election_timeout_range = election_timeout_range;
        self
    }

    /// Set the heartbeat interval of the Raft node.
    pub fn heartbeat_interval(mut self, heartbeat_interval: u64) -> Self {
        self.heartbeat_interval = heartbeat_interval;
        self
    }

    /// Set the peers of the Raft node.
    pub fn seeds(mut self, seeds: HashMap<NodeId, SocketAddr>) -> Self {
        self.seeds = seeds;
        self
    }

    /// Set the client address of the Raft node.
    pub fn client_addr(mut self, client_addr: SocketAddr) -> Self {
        self.client_addr = client_addr;
        self
    }

    /// Set the peer address of the Raft node.
    pub fn peer_addr(mut self, peer_addr: SocketAddr) -> Self {
        self.peer_addr = peer_addr;
        self
    }

    /// Build the Raft node.
    pub fn build(self) -> anyhow::Result<RaftNodeServer> {
        let (raft_channels, outside_channels) = channels::create();

        let node = MemRaftNode::<MockRequest, MockResponse>::builder()
            .id(self.id)
            .election_timeout_range(self.election_timeout_range)
            .heartbeat_interval(self.heartbeat_interval)
            .seeds(self.seeds.clone())
            .channels(raft_channels)
            .build()?;

        Ok(RaftNodeServer {
            id: self.id,
            client_addr: self.client_addr,
            peer_addr: self.peer_addr,
            node,
            in_rpc_tx: outside_channels.in_rpc_tx,
            out_rpc_rx: Arc::new(outside_channels.out_rpc_rx),
            in_client_request_tx: outside_channels.in_client_request_tx,
        })
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for RaftNodeServerBuilder {
    fn default() -> Self {
        Self {
            id: NodeId::new_v4(),
            election_timeout_range: DEFAULT_ELECTION_TIMEOUT_RANGE,
            heartbeat_interval: DEFAULT_HEARTBEAT_INTERVAL,
            peer_addr: SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 5550),
            client_addr: SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 5551),
            seeds: Default::default(),
            node: (),
            outside_channels: (),
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Start the client server. // TODO(appcypher): refactor
fn start_client_server(
    addr: SocketAddr,
    in_client_request_tx: mpsc::UnboundedSender<ClientRequest<MockRequest, MockResponse>>,
) -> JoinHandle<anyhow::Result<()>> {
    tokio::spawn(async move {
        let listener = TcpListener::bind(addr).await?;

        loop {
            let (mut stream, _) = listener.accept().await?;
            let in_client_request_tx = in_client_request_tx.clone();
            tokio::spawn(async move {
                let (mut read_stream, mut write_stream) = stream.split();

                let mut buf = vec![];
                read_stream.read_to_end(&mut buf).await?;

                let request: MockRequest = cbor4ii::serde::from_slice(&buf)?;
                let (response_tx, mut response_rx) =
                    mpsc::channel::<ClientResponse<MockResponse>>(1);

                in_client_request_tx.send(ClientRequest(request, response_tx))?;

                let response = response_rx.recv().await.ok_or(anyhow::anyhow!(
                    "Failed to receive response from Raft Node."
                ))?;

                let response = cbor4ii::serde::to_vec(vec![], &response)?;

                write_stream.write_all(&response).await?;
                write_stream.shutdown().await?;

                anyhow::Ok(())
            });
        }
    })
}

/// Start the peer server.
fn start_peer_server(
    addr: SocketAddr,
    in_rpc_tx: mpsc::UnboundedSender<PeerRpc<MockRequest>>,
) -> JoinHandle<anyhow::Result<()>> {
    tokio::spawn(async move {
        let listener = TcpListener::bind(addr).await?;

        loop {
            let (mut stream, _) = listener.accept().await?;
            let in_rpc_tx = in_rpc_tx.clone();

            tokio::spawn(async move {
                let (mut read_stream, mut write_stream) = stream.split();

                let mut buf = vec![];
                read_stream.read_to_end(&mut buf).await?;

                let request: Rpc = cbor4ii::serde::from_slice(&buf)?;
                let response = match request {
                    Rpc::AppendEntries(request) => {
                        let (response_tx, mut response_rx) =
                            mpsc::channel::<AppendEntriesResponse>(1);

                        in_rpc_tx.send(PeerRpc::AppendEntries(request, response_tx))?;

                        let response = response_rx.recv().await.ok_or(anyhow::anyhow!(
                            "Failed to receive response from Raft Node."
                        ))?;

                        cbor4ii::serde::to_vec(vec![], &response)?
                    }
                    Rpc::RequestVote(request) => {
                        let (response_tx, mut response_rx) =
                            mpsc::channel::<RequestVoteResponse>(1);

                        in_rpc_tx.send(PeerRpc::RequestVote(request, response_tx))?;

                        let response = response_rx.recv().await.ok_or(anyhow::anyhow!(
                            "Failed to receive response from Raft Node."
                        ))?;

                        cbor4ii::serde::to_vec(vec![], &response)?
                    }
                };

                write_stream.write_all(&response).await?;
                write_stream.shutdown().await?;

                anyhow::Ok(())
            });
        }
    })
}

/// Forward outgoing requests.
fn forward_outgoing_requests(
    node: MemRaftNode<MockRequest, MockResponse>,
    out_rpc_rx: Arc<Mutex<mpsc::UnboundedReceiver<(NodeId, PeerRpc<MockRequest>)>>>,
) -> JoinHandle<anyhow::Result<()>> {
    tokio::spawn(async move {
        while let Some((peer, request)) = out_rpc_rx.lock().await.recv().await {
            let addr = node
                .get_peer(&peer)
                .await
                .ok_or(anyhow::anyhow!("Node ID not found."))?;

            let mut stream = TcpStream::connect(addr).await?;
            let (mut read_stream, mut write_stream) = stream.split();

            match request {
                PeerRpc::AppendEntries(request, response_tx) => {
                    let request = Rpc::AppendEntries(request);
                    let request = cbor4ii::serde::to_vec(vec![], &request)?;

                    write_stream.write_all(&request).await?;
                    write_stream.shutdown().await?;

                    let mut buf = vec![];
                    read_stream.read_to_end(&mut buf).await?;

                    let response: AppendEntriesResponse = cbor4ii::serde::from_slice(&buf)?;

                    response_tx.send(response).await?;
                }
                PeerRpc::RequestVote(request, response_tx) => {
                    let request = Rpc::RequestVote(request);
                    let request = cbor4ii::serde::to_vec(vec![], &request)?;

                    write_stream.write_all(&request).await?;
                    write_stream.shutdown().await?;

                    let mut buf = vec![];
                    read_stream.read_to_end(&mut buf).await?;

                    let response: RequestVoteResponse = cbor4ii::serde::from_slice(&buf)?;

                    response_tx.send(response).await?;
                }
                PeerRpc::Config(_, _) => {
                    // Do nothing.
                }
                PeerRpc::InstallSnapshot(_, _) => {
                    // Do nothing.
                }
            };
        }

        anyhow::Ok(())
    })
}
