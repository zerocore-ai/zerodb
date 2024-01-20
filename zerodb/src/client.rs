use crate::{
    message::{Ping, Pong},
    Result, ZerodbError,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc, vec};
use tokio::{
    io::AsyncReadExt,
    net::TcpListener,
    sync::mpsc::{self, Receiver},
    task::JoinHandle,
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Represents the different types of requests that a client can send to a Raft node.

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ClientRequest {
    Ping(Ping),
    Pong(Pong),
}

/// This server listens for client requests and forwards them to a receiving channel.
#[derive(Debug, Default)]
pub(crate) struct ClientServer {}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl ClientServer {
    // TODO(appcypher): Server should not crash when handling a request.
    /// Starts the server.
    ///
    /// It spawns a task that listens for client requests and forwards them to a receiving channel.
    pub(crate) fn start(
        &self,
        addr: SocketAddr,
    ) -> Result<(JoinHandle<Result<()>>, Receiver<ClientRequest>)> {
        let (tx, rx) = mpsc::channel(32);

        let handle = tokio::spawn(async move {
            let listener = TcpListener::bind(addr).await?;

            tracing::info!("Listening for client requests on: {addr}");

            let tx = Arc::new(tx);
            loop {
                // Get the tcp stream
                let (mut socket, addr) = listener.accept().await?;

                tracing::debug!("Got a client connection: {addr}");

                let tx = Arc::clone(&tx);

                // Spawn a task to read the bytes from the socket
                let handle = tokio::spawn(async move {
                    // Read all the bytes from the socket
                    let mut buf = vec![];

                    tracing::debug!("Reading all the bytes from the socket");

                    socket.read_to_end(&mut buf).await?;

                    tracing::debug!("Read all the bytes from the socket");

                    // Create a request type from the bytes
                    let request = ClientRequest::from_cbor(buf)?;

                    tracing::debug!("Created a request type from the bytes: {:?}", request);

                    // Send the request to the channel
                    tx.send(request).await?;

                    tracing::debug!("Forwarded the client request to the channel");

                    Ok::<(), ZerodbError>(())
                });

                let (r,) = tokio::join!(handle);
                r.unwrap().unwrap();

                tracing::debug!("Spawned a task to read the bytes from the socket");
            }
        });

        Ok((handle, rx))
    }
}

impl ClientRequest {
    /// Creates a ClientRequest from CBOR bytes.
    pub(crate) fn from_cbor(data: impl AsRef<[u8]>) -> Result<Self> {
        Ok(cbor4ii::serde::from_slice(data.as_ref())?)
    }

    /// Converts the ClientRequest to CBOR bytes.
    pub(crate) fn to_cbor(&self) -> Result<Vec<u8>> {
        Ok(cbor4ii::serde::to_vec(vec![], self)?)
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::fixture;
    use std::time::Duration;
    use tokio::{io::AsyncWriteExt, net::TcpStream, time};

    procspawn::enable_test_support!();

    #[rstest::rstest]
    #[tokio::test]
    async fn test_client_server(addr: SocketAddr) -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();

        let h1 = procspawn::spawn!(
            (addr) || {
                tokio_test::block_on(async move {
                    tracing_subscriber::fmt::init();

                    let (_, mut rx) = ClientServer::default().start(addr).unwrap();

                    time::sleep(Duration::from_secs(3)).await;

                    let forwarded_req = rx.recv().await.unwrap();

                    assert_eq!(ClientRequest::Ping(Ping::default()), forwarded_req);
                });
            }
        );

        let h2 = procspawn::spawn!(
            (addr) || {
                tokio_test::block_on(async move {
                    tracing_subscriber::fmt::init();

                    let mut socket = TcpStream::connect(addr).await.unwrap();

                    socket
                        .write_all(&ClientRequest::Ping(Ping::default()).to_cbor().unwrap())
                        .await
                        .unwrap();

                    time::sleep(Duration::from_secs(1)).await;
                });
            }
        );

        h1.join().unwrap();
        h2.join().unwrap();

        // let (handle, mut rx) = ClientServer::default().start(addr)?;

        // time::sleep(Duration::from_secs(1)).await;

        // let mut socket = TcpStream::connect(addr).await?;
        // let original_request = ClientRequest::Ping(Ping::default());

        // socket.write_all(&original_request.to_cbor()?).await?;

        // time::sleep(Duration::from_secs(1)).await;

        // let forwarded_req = rx.recv().await.unwrap();

        // assert_eq!(original_request, forwarded_req);

        Ok(())
    }

    #[fixture]
    fn addr() -> SocketAddr {
        "127.0.0.1:12345".parse().unwrap()
    }
}
