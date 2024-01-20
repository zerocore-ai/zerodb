use super::RaftRpc;
use crate::{client::ClientRequest, raft::timeouts, Result};
use std::{net::SocketAddr, time::Duration};
use tokio::{sync::mpsc::Receiver, task::JoinHandle};
use uuid::Uuid;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A Raft node.
pub(crate) struct RaftNode {
    id: Uuid,
    term: u64,
    role: Role,
}

/// A Raft node's role.
pub(crate) enum Role {
    Follower,
    Candidate,
    Leader,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl RaftNode {
    /// Starts the Raft node.
    pub(crate) fn start(
        &self,
        addr: SocketAddr,
        mut client_rx: Receiver<ClientRequest>,
    ) -> JoinHandle<Result<()>> {
        tokio::spawn(async move {
            let (_, mut rpc_rx) = RaftRpc::default().start(addr)?;
            loop {
                tokio::select! {
                    // _ = timeouts::election(Duration::from_secs(5)) => {
                    //     tracing::info!("Election timeout");
                    // }

                    // _ = timeouts::heartbeat(Duration::from_secs(1)) => {
                    //     tracing::info!("Heartbeat timeout");
                    // }

                    _ = rpc_rx.recv() => {
                        tracing::info!("Got an RPC request");
                    }

                    _ = client_rx.recv() => {
                        tracing::info!("Got a client request");
                    }
                }
            }
        })
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for RaftNode {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            term: 0,
            role: Role::Follower,
        }
    }
}
