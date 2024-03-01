use std::{cmp::max, collections::HashSet, sync::Arc};

use tokio::sync::{mpsc, Mutex};

use crate::{
    task::common, AppendEntriesRequest, AppendEntriesResponse, AppendEntriesResponseReason,
    ClientRequest, Command, Countdown, LogEntry, NodeId, PeerRpc, RaftNode, Request, Response,
    Result, Store,
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The tasks that a leader performs.
#[derive(Debug)]
pub(crate) struct LeaderTasks;

/// This type is used to track the state of the heartbeat session for each peer.
pub(crate) struct AppendEntriesSession<S, R, P>
where
    S: Store<R>,
    R: Request,
    P: Response,
{
    reached_peers: Arc<Mutex<HashSet<NodeId>>>, // Arc<Mutex<HashMap<NodeId, u64>>>
    // peers: Arc<Mutex<HashMap<NodeId, u64>>>
    node: RaftNode<S, R, P>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl LeaderTasks {
    /// Starts the leader tasks.
    pub(crate) async fn start<S, R, P>(node: RaftNode<S, R, P>) -> Result<()>
    where
        S: Store<R> + Sync + Send + 'static,
        R: Request + Sync + Send + 'static,
        P: Response + Send + 'static,
    {
        // Create a heartbeat countdown.
        let mut heartbeat_countdown = Countdown::start(node.get_heartbeat_interval());

        // Create a append_entries_session session.
        let mut append_entries_session = AppendEntriesSession::initialize(node.clone());
        append_entries_session.send_heartbeats_to_all();

        // Get the channels.
        let channels = node.get_channels();
        let in_rpc_rx = &mut *channels.in_rpc_rx.lock().await;
        let in_client_request_rx = &mut *channels.in_client_request_rx.lock().await;
        let shutdown_rx: &mut mpsc::Receiver<()> = &mut *channels.shutdown_rx.lock().await;

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
                    PeerRpc::AppendEntries(request, response_tx) => {
                        common::respond_to_append_entries(node.clone(), request, response_tx, |node, _, response_tx| Box::pin(async move {
                            response_tx
                                .send(AppendEntriesResponse {
                                    term: node.get_current_term(),
                                    success: false,
                                    id: node.get_id(),
                                    reason: AppendEntriesResponseReason::NotAFollower,
                                })
                                .await?;

                            Ok(())
                        })).await?;
                    },
                    PeerRpc::RequestVote(request, response_tx) => {
                        common::respond_to_request_vote(node.clone(), request, response_tx).await?;
                    },
                    PeerRpc::Config(_, _) => {
                        // TODO(appcypher): Implement Config RPC.
                        unimplemented!("Config RPC not implemented");
                    },
                    PeerRpc::InstallSnapshot(_, _) => {
                        // TODO(appcypher): Implement InstallSnapshot RPC.
                        unimplemented!("InstallSnapshot RPC not implemented");
                    }
                },
                Some(ClientRequest(request, _response_tx)) = in_client_request_rx.recv() => {
                    let entries = vec![LogEntry {
                        term: node.get_current_term(),
                        command: Command::ClientRequest(request)
                    }];

                    // Append the command to the log.
                    node.inner.store.lock().await.append_entries(entries)?;

                    // // Send entries to all peers for replication.
                    // append_entries_session.send_entries_to_all(entries).await?;

                    todo!();
                },
                _ = heartbeat_countdown.continuation() => {
                    // Reset the heartbeat countdown.
                    heartbeat_countdown.reset();

                    // Reset the append_entries_session session.
                    append_entries_session.reset().await;
                    append_entries_session.send_heartbeats_to_all();
                },
            }
        }

        Ok(())
    }
}

impl<S, R, P> AppendEntriesSession<S, R, P>
where
    S: Store<R> + Sync + Send + 'static,
    R: Request + Sync + Send + 'static,
    P: Response + Send + 'static,
{
    /// Initializes a new append_entries_session session.
    pub fn initialize(node: RaftNode<S, R, P>) -> Self {
        Self {
            reached_peers: Arc::new(Mutex::new(HashSet::new())),
            node,
        }
    }

    /// Resets the append_entries_session session.
    pub async fn reset(&mut self) {
        *self.reached_peers.lock().await = HashSet::new();
    }

    /// Sends heartbeats to all peers.
    pub fn send_heartbeats_to_all(&self) {
        let reached_peers = Arc::clone(&self.reached_peers);
        let node = self.node.clone();

        tokio::spawn(async move {
            let peers: HashSet<NodeId> = node
                .inner
                .store
                .lock()
                .await
                .get_membership()
                .keys()
                .cloned()
                .filter(|id| *id != node.get_id())
                .collect::<HashSet<_>>();

            // Early exit to if there are no peers.
            if peers.is_empty() {
                return crate::Ok(());
            }

            let reached_peers = &mut *reached_peers.lock().await;
            let unreached_peers: HashSet<NodeId> = peers
                .difference(&reached_peers.clone())
                .cloned()
                .collect::<HashSet<_>>();

            // Early exit if there are no unreached peers.
            if unreached_peers.is_empty() {
                return crate::Ok(());
            }

            // Create a channel to receive the vote responses.
            let (append_entries_tx, mut append_entries_rx) =
                mpsc::channel::<AppendEntriesResponse>(max(unreached_peers.len(), 1));

            // Send heartbeats to all unreached peers.
            for peer in unreached_peers {
                let append_entries_tx = append_entries_tx.clone();
                let node = node.clone();

                // Send the RequestVote RPC in a separate task.
                tokio::spawn(async move {
                    // Send basic AppendEntries RPC to peer.
                    node.send_append_entries_rpc(
                        AppendEntriesRequest {
                            term: node.get_current_term(),
                            leader_id: node.get_id(),
                            // TODO(appcypher): Fix this.
                            prev_log_index: 0,
                            prev_log_term: 0,
                            entries: vec![],
                            last_commit_index: 0,
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
