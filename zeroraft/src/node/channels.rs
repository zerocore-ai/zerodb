//! The communication channels for a Raft node.

use tokio::sync::{mpsc, Mutex};

use crate::{ClientRequest, NodeId, PeerRpc, Request, Response};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The communication channels for a Raft node.
pub struct RaftSideChannels<R, P>
where
    R: Request,
    P: Response,
{
    /// Channel for raft to receive rpc requests from the outside.
    pub in_rpc_rx: Mutex<mpsc::UnboundedReceiver<PeerRpc>>,

    /// Channel for raft to send rpc requests to the outside.
    pub out_rpc_tx: mpsc::UnboundedSender<(NodeId, PeerRpc)>,

    /// Channel for raft to recieve client requests from the outside.
    pub in_client_request_rx: Mutex<mpsc::UnboundedReceiver<ClientRequest<R, P>>>,

    /// Channel for raft to receive shutdown signal from the outside.
    pub shutdown_rx: Mutex<mpsc::Receiver<()>>,

    /// Channel for raft to send shutdown signal to itself.
    pub shutdown_tx: mpsc::Sender<()>,
}

/// The channels for communicating with the Raft node from the outside.
pub struct OutsideChannels<R, P>
where
    R: Request,
    P: Response,
{
    /// Channel for sending incoming rpc requests to the Raft node.
    pub in_rpc_tx: mpsc::UnboundedSender<PeerRpc>,

    /// Channel for capturing outgoing rpc requests from the Raft node.
    pub out_rpc_rx: Mutex<mpsc::UnboundedReceiver<(NodeId, PeerRpc)>>,

    /// Channel for sending client requests to the Raft node.
    pub in_client_request_tx: mpsc::UnboundedSender<ClientRequest<R, P>>,

    /// Channel for sending shutdown signal to the Raft node.
    pub shutdown_tx: mpsc::Sender<()>,
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Creates a new set of channels for a Raft node.
pub fn create<R, P>() -> (RaftSideChannels<R, P>, OutsideChannels<R, P>)
where
    R: Request,
    P: Response,
{
    let (in_rpc_tx, in_rpc_rx) = mpsc::unbounded_channel();
    let (out_rpc_tx, out_rpc_rx) = mpsc::unbounded_channel();
    let (in_client_request_tx, in_client_request_rx) = mpsc::unbounded_channel();
    let (shutdown_tx, shutdown_rx) = mpsc::channel(1);
    (
        RaftSideChannels {
            in_rpc_rx: Mutex::new(in_rpc_rx),
            out_rpc_tx,
            in_client_request_rx: Mutex::new(in_client_request_rx),
            shutdown_rx: Mutex::new(shutdown_rx),
            shutdown_tx: shutdown_tx.clone(),
        },
        OutsideChannels {
            in_rpc_tx,
            out_rpc_rx: Mutex::new(out_rpc_rx),
            in_client_request_tx,
            shutdown_tx,
        },
    )
}
