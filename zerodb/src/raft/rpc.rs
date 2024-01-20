use crate::{Result, ZerodbError};
use std::{net::SocketAddr, sync::Arc};
use tokio::{
    io::AsyncReadExt,
    net::TcpListener,
    sync::mpsc::{self, Receiver},
    task::JoinHandle,
};

use super::RaftRequest;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// This server listens for Raft requests and forwards them to a receiving channel.
#[derive(Debug, Default)]
pub(crate) struct RaftRpc {}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl RaftRpc {
    // TODO(appcypher): Server should not crash when handling a request.
    /// Starts the Raft RPC server.
    ///
    /// The server listens for Raft requests and forwards them to a receiving channel.
    pub(crate) fn start(
        &self,
        addr: SocketAddr,
    ) -> Result<(JoinHandle<Result<()>>, Receiver<RaftRequest>)> {
        let (tx, rx) = mpsc::channel(32);

        let handle = tokio::spawn(async move {
            let listener = TcpListener::bind(&addr).await?;

            tracing::info!("Listening for Raft RPCs on: {addr}");

            let tx = Arc::new(tx);
            loop {
                // Get the tcp stream
                let (mut socket, _) = listener.accept().await?;
                let tx = Arc::clone(&tx);

                // Spawn a task to read the bytes from the socket
                tokio::spawn(async move {
                    // Read all the bytes from the socket
                    let mut buf = vec![];
                    socket.read_to_end(&mut buf).await?;

                    // Create a request type from the bytes
                    let request = RaftRequest::from_cbor(buf)?;

                    // Send the request to the channel
                    tx.send(request).await?;

                    Ok::<(), ZerodbError>(())
                });
            }
        });

        Ok((handle, rx))
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------
