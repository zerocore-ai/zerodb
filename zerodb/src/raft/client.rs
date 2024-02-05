use std::net::SocketAddr;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::{RequestVoteRequest, RequestVoteResponse, Result};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct RaftClient {
    addr: SocketAddr,

    // stream: TcpStream,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl RaftClient {
    /// Connects to a Raft node.
    pub async fn connect(addr: SocketAddr) -> Result<Self> {
        // Establish tcp connection.
        let stream = TcpStream::connect(addr).await?;
        Ok(Self { addr, stream })
    }

    /// Returns the address of the Raft node.
    pub fn get_addr(&self) -> SocketAddr {
        self.addr
    }
}
