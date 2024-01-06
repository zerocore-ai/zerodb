use crate::{
    config::ZerodbConfig,
    util::{self, HandlerFuture},
    Result,
};
use futures::FutureExt;
use http_body_util::Full;
use hyper::{
    body::{Bytes, Incoming},
    server::conn::http1,
    service, Request, Response,
};
use hyper_util::rt::TokioIo;
use std::{io, net::SocketAddr, panic::AssertUnwindSafe, sync::Arc};
use tokio::{
    net::{TcpListener, TcpStream},
    task::JoinHandle,
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Manages communication with clients.
pub(crate) struct ClientManager {
    /// Configuration for the `Zerodb` instance.
    config: Arc<ZerodbConfig>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl ClientManager {
    /// Creates a new `ClientManager`.
    pub(crate) fn new(config: Arc<ZerodbConfig>) -> Self {
        Self { config }
    }

    /// Listens for client connections.
    pub(crate) async fn listen(&self) -> Result<JoinHandle<()>> {
        let addr = util::parse_address(&self.config.network.host, self.config.network.client_port)?;
        let listener = TcpListener::bind(addr).await?;

        tracing::debug!("listening for clients on {addr}");

        Ok(tokio::spawn(async move {
            loop {
                ClientManager::handle_connection(listener.accept().await);
            }
        }))
    }

    /// Handles a client connection.
    pub(crate) fn handle_connection(stream: io::Result<(TcpStream, SocketAddr)>) {
        match stream {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    let conn = http1::Builder::new()
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
                            tracing::error!("error while handling request: {e}");
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
                            tracing::error!("panic while handling request: {e}");
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
    async fn handle_request(request: Request<Incoming>) -> Result<Response<Full<Bytes>>> {
        let path = request.uri().path();

        Ok(Response::new(Full::new(Bytes::from(format!(
            "Client handler: {}\n",
            path
        )))))
    }
}
