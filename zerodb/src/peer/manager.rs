use crate::{config::ZerodbConfig, util, util::HandlerFuture, Result, ZerodbError};
use futures::{FutureExt, SinkExt, StreamExt};
use http_body_util::Full;
use hyper::{
    body::{Bytes, Incoming},
    server::conn::http1,
    service, Request, Response,
};
use hyper_tungstenite::tungstenite::Message;
use hyper_util::rt::TokioIo;
use std::{collections::LinkedList, net::SocketAddr, panic::AssertUnwindSafe, sync::Arc};
use tokio::{
    io,
    net::{TcpListener, TcpStream},
    task::JoinHandle,
    time,
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Manages communication with peers.
pub(crate) struct PeerManager {
    /// Configuration for the `Zerodb` instance.
    config: Arc<ZerodbConfig>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl PeerManager {
    /// Creates a new `PeerManager`.
    pub(crate) fn new(config: Arc<ZerodbConfig>) -> Self {
        Self { config }
    }

    /// Connect to the seed nodes.
    pub(crate) fn establish_seed_connections(&self) -> JoinHandle<Result<()>> {
        let config = Arc::clone(&self.config);
        tokio::spawn(async move {
            // We rotate through the seeds attempting to connect.
            // Each seed is tried once before moving the next.
            // In total, each seed is tried 5 times before giving up on it.
            tracing::debug!("trying seeds: {:?}...", config.network.seeds);

            let mut seed_stack = config
                .network
                .seeds
                .iter()
                .map(|seed| (seed, 5))
                .collect::<LinkedList<_>>();

            while !seed_stack.is_empty() {
                let (seed, tries) = seed_stack.pop_back().unwrap();
                match PeerManager::connect_to_seed(seed).await {
                    Ok(_) => {
                        tracing::debug!("connected to seed {seed}");
                    }
                    Err(e) => {
                        tracing::error!("failed to connect to seed {seed}: {e}");

                        if tries > 0 {
                            seed_stack.push_front((seed, tries - 1));
                        }

                        // Sleep for 3 seconds before trying the next seed.
                        time::sleep(tokio::time::Duration::from_secs(3)).await;
                    }
                }
            }

            tracing::debug!("done trying to connect to seeds");

            Ok(())
        })
    }

    /// Connects to a seed node.
    async fn connect_to_seed(seed: &str) -> Result<()> {
        tracing::debug!("connecting to seed {seed}...");

        let (websocket, _) = tokio_tungstenite::connect_async(seed).await?;
        let (mut write, mut read) = websocket.split();

        tracing::debug!("connected to seed {seed}");

        // Send a ping to the seed.
        write.send(Message::Text("Hello".to_string())).await?;

        // Read the response.
        while let Some(message) = read.next().await {
            Self::handle_peer_message(message?).await?;
        }

        Ok(())
    }

    /// Listens for peer connections.
    pub(crate) async fn listen(&self) -> Result<JoinHandle<()>> {
        let addr = util::parse_address(&self.config.network.host, self.config.network.peer_port)?;
        let listener = TcpListener::bind(addr).await?;

        tracing::debug!("listening for peers on {addr}");

        Ok(tokio::spawn(async move {
            loop {
                PeerManager::handle_connection(listener.accept().await);
            }
        }))
    }

    /// Handles a peer connection.
    pub(crate) fn handle_connection(stream: io::Result<(TcpStream, SocketAddr)>) {
        tracing::debug!("handling peer connection...");
        match stream {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    let conn = http1::Builder::new()
                        .keep_alive(true)
                        .serve_connection(
                            TokioIo::new(stream),
                            service::service_fn(Self::make_handler()),
                        )
                        .with_upgrades()
                        .await;

                    if let Err(e) = conn {
                        tracing::error!("failed to serve connection: {e}");
                    }
                });
            }
            Err(e) => {
                // Failed http event connection should not be fatal.
                tracing::error!("failed to accept http event connection: {e}");
            }
        };
    }

    /// Creates the request handler service function.
    fn make_handler<'a>() -> impl Fn(Request<Incoming>) -> HandlerFuture<'a> + 'a {
        move |request| {
            let fut = async move {
                let result = AssertUnwindSafe(async move {
                    match Self::handle_request(request).await {
                        Ok(response) => Ok(response),
                        Err(e) => {
                            tracing::error!("error while handling request: {}", e);
                            Ok(Response::builder()
                                .status(500)
                                .body(Full::from(Bytes::from("Internal Server Error")))?)
                        }
                    }
                })
                .catch_unwind()
                .await;

                match result {
                    Ok(result) => result,
                    Err(e) => {
                        if let Some(e) = e.downcast_ref::<&str>() {
                            tracing::error!("panic while handling request: {}", e);
                        } else {
                            tracing::error!("panic while handling request");
                        }

                        Ok(Response::builder()
                            .status(500)
                            .body(Full::from(Bytes::from("Internal Server Error")))?)
                    }
                }
            };

            Box::pin(fut)
        }
    }

    /// Handles a peer request.
    async fn handle_request(mut request: Request<Incoming>) -> Result<Response<Full<Bytes>>> {
        if hyper_tungstenite::is_upgrade_request(&request) {
            tracing::debug!("upgrading request...");

            let (response, websocket) = hyper_tungstenite::upgrade(&mut request, None)?;
            tokio::spawn(async move {
                let mut websocket = websocket.await?;

                tracing::debug!("request upgraded");

                while let Some(message) = websocket.next().await {
                    Self::handle_peer_message(message?).await?;
                }

                Ok::<(), ZerodbError>(())
            });

            return Ok(response);
        }

        Err(ZerodbError::CannotUpgradePeerConnection)
    }

    /// Handles a peer message.
    async fn handle_peer_message(message: Message) -> Result<()> {
        match message {
            Message::Text(msg) => {
                tracing::debug!("received text message: {msg}");
            }
            Message::Binary(msg) => {
                tracing::debug!("received binary message: {msg:?}");
            }
            Message::Ping(msg) => {
                tracing::debug!("received ping message: {msg:?}");
            }
            Message::Pong(msg) => {
                tracing::debug!("received pong message: {msg:?}");
            }
            Message::Close(msg) => {
                if let Some(msg) = &msg {
                    tracing::debug!(
                        "received close message with code {} and message: {}",
                        msg.code,
                        msg.reason
                    );
                } else {
                    tracing::debug!("received close message");
                }
            }
            Message::Frame(_msg) => {
                unreachable!();
            }
        }

        Ok(())
    }
}
