use std::{net::SocketAddr, sync::Arc};

use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Mutex},
    task::JoinHandle,
};
use zeroraft::{
    AppendEntriesRequest, AppendEntriesResponse, ClientRequest, ClientResponse, NodeId, PeerRpc,
    RequestVoteRequest, RequestVoteResponse,
};

use crate::{MemRaftNode, Query, QueryResponse, Result, ZerodbError};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

type OutRpcReciever = mpsc::UnboundedReceiver<(NodeId, PeerRpc<Query>)>;

/// `Rpc` is an enum representing the different types of RPC requests that can be sent to a Raft node.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Rpc {
    AppendEntries(AppendEntriesRequest<Query>),
    RequestVote(RequestVoteRequest),
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Start the client server. // TODO(appcypher): refactor
pub(crate) fn start_client_server(
    addr: SocketAddr,
    in_client_request_tx: mpsc::UnboundedSender<ClientRequest<Query, QueryResponse>>,
) -> JoinHandle<Result<()>> {
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

                let response = response_rx.recv().await.ok_or(ZerodbError::ChannelClosed)?;
                let response = cbor4ii::serde::to_vec(vec![], &response)?;

                write_stream.write_all(&response).await?;
                write_stream.shutdown().await?;

                crate::Ok(())
            });
        }
    })
}

/// Forward outgoing requests.
pub(crate) fn forward_outgoing_requests(
    node: MemRaftNode<Query, QueryResponse>,
    out_rpc_rx: Arc<Mutex<OutRpcReciever>>,
) -> JoinHandle<Result<()>> {
    tokio::spawn(async move {
        while let Some((peer, request)) = out_rpc_rx.lock().await.recv().await {
            let addr = node
                .get_peer(&peer)
                .await
                .ok_or(ZerodbError::PeerNotFound)?;

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

        Ok(())
    })
}
