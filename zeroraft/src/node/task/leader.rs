use std::{cmp::max, collections::HashSet, sync::Arc};

use tokio::sync::{mpsc, Mutex};

use crate::{
    AppendEntriesRequest, AppendEntriesResponse, Countdown, Log, NodeId, PeerRpc, RaftNodeInner,
    Request, RequestVoteRequest, RequestVoteResponse, Response, Result,
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The tasks that a leader performs.
#[derive(Debug)]
pub(crate) struct LeaderTasks;

/// This type is used to track the state of the heartbeat session for each peer.
pub(crate) struct AppendEntriesSession<L, R, P>
where
    L: Log<R>,
    R: Request,
    P: Response,
{
    reached_peers: Arc<Mutex<HashSet<NodeId>>>,
    node: Arc<RaftNodeInner<L, R, P>>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl LeaderTasks {
    /// Starts the leader tasks.
    pub(crate) async fn start<L, R, P>(node: Arc<RaftNodeInner<L, R, P>>) -> Result<()>
    where
        L: Log<R> + Sync + Send + 'static,
        R: Request + Sync + Send + 'static,
        P: Response + Send + 'static,
    {
        // Create a heartbeat countdown.
        let mut heartbeat_countdown = Countdown::start(node.get_heartbeat_interval());

        // Create a append_entries_session session.
        let mut append_entries_session = AppendEntriesSession::new(Arc::clone(&node));
        append_entries_session.send_heartbeats_to_all(); // TODO(appcypher): Send empty hearbeats or send the actual entries?

        // Get the channels.
        let channels = node.get_channels();
        let in_rpc_rx = &mut *channels.in_rpc_rx.lock().await;
        let in_client_request_rx = &mut *channels.in_client_request_rx.lock().await;
        let shutdown_rx: &mut tokio::sync::mpsc::Receiver<()> =
            &mut *channels.shutdown_rx.lock().await;

        loop {
            if !node.is_leader_state().await {
                break;
            }

            tokio::select! {
                _ = shutdown_rx.recv() => {
                    // Shutdown.
                    node.change_to_shutdown_state().await;
                },
                Some(request) = in_rpc_rx.recv() => match request {
                    PeerRpc::AppendEntries(AppendEntriesRequest { .. }, _response_tx) => {
                        // TODO(appcypher): ...
                    },
                    PeerRpc::RequestVote(RequestVoteRequest { term, candidate_id }, response_tx) => {
                        // Check if our term is stale.
                        if term > node.get_current_term() {
                            // Send granted response.
                            response_tx.send(RequestVoteResponse {
                                term, // TODO(appcypher): should we return our term instead?
                                vote_granted: true,
                                id: node.get_id(),
                            }).await?;

                            node.change_to_follower_state().await;
                            node.update_term_and_voted_for(term, candidate_id).await;
                            continue;
                        }

                        // We have either already voted or the request term is stale.
                        // Send rejected response.
                        response_tx.send(RequestVoteResponse {
                            term: node.get_current_term(),
                            vote_granted: false,
                            id: node.get_id(),
                        }).await?;
                    }
                },
                Some(_) = in_client_request_rx.recv() => {
                    tracing::debug!(id = node.id.to_string(), "Received client request");
                },
                _ = heartbeat_countdown.continuation() => {
                    // Reset the heartbeat countdown.
                    heartbeat_countdown.reset();

                    // Reset the append_entries_session session.
                    append_entries_session = AppendEntriesSession::new(Arc::clone(&node));
                    append_entries_session.send_heartbeats_to_all();
                },
            }
        }

        Ok(())
    }
}

impl<L, R, P> AppendEntriesSession<L, R, P>
where
    L: Log<R> + Sync + Send + 'static,
    R: Request + Sync + Send + 'static,
    P: Response + Send + 'static,
{
    pub fn new(node: Arc<RaftNodeInner<L, R, P>>) -> Self {
        Self {
            reached_peers: Arc::new(Mutex::new(HashSet::new())),
            node,
        }
    }

    pub fn send_heartbeats_to_all(&self) {
        let reached_peers = Arc::clone(&self.reached_peers);
        let node = Arc::clone(&self.node);

        tokio::spawn(async move {
            let valid_peers = node.get_valid_peers().cloned().collect::<HashSet<_>>();
            let reached_peers = &mut *reached_peers.lock().await;
            let unreached_peers = &valid_peers - reached_peers;

            // Create a channel to receive the vote responses.
            let (append_entries_tx, mut append_entries_rx) =
                mpsc::channel::<AppendEntriesResponse>(max(unreached_peers.len(), 1));

            // Send heartbeats to all that have not been sent to.
            for peer in unreached_peers {
                let append_entries_tx = append_entries_tx.clone();
                let node = Arc::clone(&node);

                // Send the RequestVote RPC in a separate task.
                tokio::spawn(async move {
                    // Send basic AppendEntries RPC to peer.
                    node.send_append_entries_rpc(
                        AppendEntriesRequest {
                            term: node.get_current_term(),
                            leader_id: node.get_id(),
                        },
                        peer,
                        append_entries_tx,
                    )
                    .await?;

                    crate::Ok(())
                });
            }

            // Drop the vote_tx so that we can wait for all the responses.
            drop(append_entries_tx);

            // Wait for all the responses.
            while let Some(response) = append_entries_rx.recv().await {
                reached_peers.insert(response.id);
            }

            crate::Ok(())
        });
    }
}
