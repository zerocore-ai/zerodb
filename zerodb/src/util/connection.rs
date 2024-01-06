use crate::Result;
use http_body_util::Full;
use hyper::{body::Bytes, Response};
use std::{future::Future, net::SocketAddr, pin::Pin};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The type expected by hyper for a request handler.
pub type HandlerFuture<'a> =
    Pin<Box<dyn Future<Output = Result<Response<Full<Bytes>>>> + Send + 'a>>;

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Parses a host and port into a `SocketAddr`.
pub fn parse_address(host: &str, port: u16) -> Result<SocketAddr> {
    let addr = format!("{}:{}", host, port);
    Ok(addr.parse()?)
}
