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
use zerodb::{
    channels, AppendEntriesRequest, AppendEntriesResponse, ClientRequest, ClientResponse,
    MemRaftNode, NodeId, PeerRpc, Query, QueryResponse, RequestVoteRequest, RequestVoteResponse,
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// `RaftNodeServer` wraps a Raft node and provides a server for client and peer connections.
pub struct RaftNodeServer {
    peers: Arc<HashMap<NodeId, SocketAddr>>,
    id: NodeId,
    client_addr: SocketAddr,
    peer_addr: SocketAddr,
    node: MemRaftNode<Query, QueryResponse>,
    in_rpc_tx: mpsc::UnboundedSender<PeerRpc>,
    out_rpc_rx: Arc<Mutex<mpsc::UnboundedReceiver<(NodeId, PeerRpc)>>>,
    in_client_request_tx: mpsc::UnboundedSender<ClientRequest<Query, QueryResponse>>,
}

/// `RaftNodeServerBuilder` is a builder for `RaftNodeServer`.
pub struct RaftNodeServerBuilder<N = (), C = ()> {
    peers: Arc<HashMap<NodeId, SocketAddr>>,
    id: NodeId,
    client_addr: SocketAddr,
    peer_addr: SocketAddr,
    node: N,
    other_channels: C,
}

/// `Rpc` is an enum representing the different types of RPC requests that can be sent to a Raft node.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Rpc {
    AppendEntries(AppendEntriesRequest),
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
    pub fn start(&self) -> JoinHandle<zerodb::Result<()>> {
        // Start the Raft node.
        let raft_handle = self.node.start();

        tracing::debug!("Started Raft Node.");

        // TCP server for client connections.x
        start_client_server(self.client_addr, self.in_client_request_tx.clone());

        tracing::debug!("Started Client Server: {}", self.client_addr);

        // TCP server for peer connections.
        start_peer_server(self.peer_addr, self.in_rpc_tx.clone());

        // Forward outgoing requests.
        forward_outgoing_requests(Arc::clone(&self.peers), Arc::clone(&self.out_rpc_rx));

        tracing::debug!("Started Peer Server: {}", self.peer_addr);

        // Return
        raft_handle
    }

    /// Shutdown the Raft server.
    pub async fn shutdown(&self) -> anyhow::Result<()> {
        self.node.shutdown().await?;
        Ok(())
    }

    /// Returns the Raft node.
    pub fn get_node(&self) -> &MemRaftNode<Query, QueryResponse> {
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

    /// Set the peers of the Raft node.
    pub fn peers(mut self, peers: HashMap<NodeId, SocketAddr>) -> Self {
        self.peers = Arc::new(peers);
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
    pub fn build(self) -> RaftNodeServer {
        let (raft_channels, outside_channels) = channels::create();

        let node = MemRaftNode::<Query, QueryResponse>::builder()
            .id(self.id)
            .peers(self.peers.keys().copied().collect())
            .channels(raft_channels)
            .build();

        RaftNodeServer {
            peers: self.peers,
            id: self.id,
            client_addr: self.client_addr,
            peer_addr: self.peer_addr,
            node,
            in_rpc_tx: outside_channels.in_rpc_tx,
            out_rpc_rx: Arc::new(outside_channels.out_rpc_rx),
            in_client_request_tx: outside_channels.in_client_request_tx,
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for RaftNodeServerBuilder {
    fn default() -> Self {
        Self {
            peers: Default::default(),
            id: NodeId::new_v4(),
            peer_addr: SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 5550),
            client_addr: SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 5551),
            node: (),
            other_channels: (),
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Start the client server. // TODO(appcypher): refactor
fn start_client_server(
    addr: SocketAddr,
    in_client_request_tx: mpsc::UnboundedSender<ClientRequest<Query, QueryResponse>>,
) -> JoinHandle<anyhow::Result<()>> {
    // ClientServer::new(addr, in_client_request_tx)::start()
    tokio::spawn(async move {
        let listener = TcpListener::bind(addr).await?;

        loop {
            let (mut stream, _) = listener.accept().await?;
            let in_client_request_tx = in_client_request_tx.clone();
            tokio::spawn(async move {
                let (mut read_stream, mut write_stream) = stream.split();

                let mut buf = vec![];
                read_stream.read_to_end(&mut buf).await?;

                let request: Query = cbor4ii::serde::from_slice(&buf)?;
                let (response_tx, mut response_rx) =
                    mpsc::channel::<ClientResponse<QueryResponse>>(1);

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
    in_rpc_tx: mpsc::UnboundedSender<PeerRpc>,
) -> JoinHandle<anyhow::Result<()>> {
    // PeerServer::new(addr, in_rpc_tx)::start()
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
    peers: Arc<HashMap<NodeId, SocketAddr>>,
    out_request_rx: Arc<Mutex<mpsc::UnboundedReceiver<(NodeId, PeerRpc)>>>,
) -> JoinHandle<anyhow::Result<()>> {
    tokio::spawn(async move {
        while let Some((peer, request)) = out_request_rx.lock().await.recv().await {
            let addr = peers
                .get(&peer)
                .ok_or(anyhow::anyhow!("Node ID not found."))?;

            // TODO(appcypher): Remove this sleep.
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

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
                    read_stream.read_to_end(&mut buf).await?; // Stuck here waiting for data.

                    let response: RequestVoteResponse = cbor4ii::serde::from_slice(&buf)?;

                    response_tx.send(response).await?;
                }
            };
        }

        anyhow::Ok(())
    })
}
