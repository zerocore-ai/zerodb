use std::{collections::HashMap, ops::Deref, sync::Arc, time::Duration};

use tokio::{
    sync::{mpsc, Notify, RwLock},
    time,
};

use crate::{
    roles::common, AppendEntriesRequest, AppendEntriesResponse, AppendEntriesResponseReason,
    ClientRequest, Command, LogEntry, NodeId, PeerRpc, RaftNode, Request, Response, Result, Store,
    Timeout,
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The tasks that a leader performs.
#[derive(Debug)]
pub(crate) struct LeaderRole;

/// The replication session of a leader.
pub(crate) struct ReplicationSession<S, R, P>
where
    S: Store<R>,
    R: Request,
    P: Response,
{
    inner: Arc<ReplicationSessionInner<S, R, P>>,
}

/// The inner state of a replication session.
pub(crate) struct ReplicationSessionInner<S, R, P>
where
    S: Store<R>,
    R: Request,
    P: Response,
{
    peers: RwLock<HashMap<NodeId, (u64, Option<u64>)>>, // (node_id, (next_index, match_index))
    stop_signal: Arc<Notify>,
    node: RaftNode<S, R, P>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl LeaderRole {
    /// Starts the leader tasks.
    pub(crate) async fn start<S, R, P>(node: RaftNode<S, R, P>) -> Result<()>
    where
        S: Store<R> + Sync + Send + 'static,
        R: Request + Clone + Sync + Send + 'static,
        P: Response + Send + 'static,
    {
        // Create a heartbeat timeout.
        let mut heartbeat_timeout = Timeout::start(node.get_heartbeat_interval());

        // Create a append_entries_session session.
        let mut session = ReplicationSession::initialize(node.clone()).await;
        session.send_heartbeats(heartbeat_timeout.clone()).await;
        session.start_standby_update().await;

        // Get the channels.
        let channels = node.get_channels();
        let in_rpc_rx = &mut *channels.in_rpc_rx.lock().await;
        let in_client_request_rx = &mut *channels.in_client_request_rx.lock().await;
        let shutdown_rx: &mut mpsc::Receiver<()> = &mut *channels.shutdown_rx.lock().await;

        loop {
            if !node.is_leader_state().await {
                session.stop_signal.notify_waiters();
                break;
            }

            tokio::select! {
                _ = shutdown_rx.recv() => {
                    node.change_to_shutdown_state().await;
                },
                Some(request) = in_rpc_rx.recv() => match request {
                    PeerRpc::AppendEntries(request, response_tx) => {
                        common::respond_to_append_entries(node.clone(), request, response_tx).await?;
                    },
                    PeerRpc::RequestVote(request, response_tx) => {
                        common::respond_to_request_vote(node.clone(), request, response_tx).await?;
                    },
                    PeerRpc::Config(_, _) => {
                        unimplemented!("Config RPC not implemented"); // TODO(appcypher): Implement Config RPC.
                    },
                    PeerRpc::InstallSnapshot(_, _) => {
                        unimplemented!("InstallSnapshot RPC not implemented"); // TODO(appcypher): Implement InstallSnapshot RPC.
                    }
                },
                Some(ClientRequest(request, _response_tx)) = in_client_request_rx.recv() => {
                    let entries = vec![LogEntry {
                        term: node.get_current_term(),
                        command: Command::ClientRequest(request)
                    }];
                    node.inner.store.write().await.append_entries(entries)?;
                },
                _ = heartbeat_timeout.continuation() => {
                    heartbeat_timeout.reset();
                    session.send_heartbeats(heartbeat_timeout.clone()).await;
                },
            }
        }

        Ok(())
    }
}

impl<S, R, P> ReplicationSession<S, R, P>
where
    S: Store<R>,
    R: Request,
    P: Response,
{
    /// Initializes a new replication session.
    async fn initialize(node: RaftNode<S, R, P>) -> ReplicationSession<S, R, P> {
        let next_index = node.inner.store.read().await.get_last_commit_index() + 1;
        let peers = node
            .inner
            .store
            .read()
            .await
            .get_membership()
            .iter()
            .filter_map(|(id, _)| {
                if *id == node.get_id() {
                    None
                } else {
                    Some((*id, (next_index, None)))
                }
            })
            .collect();

        Self {
            inner: Arc::new(ReplicationSessionInner {
                peers: RwLock::new(peers),
                stop_signal: Arc::new(Notify::new()),
                node,
            }),
        }
    }

    /// Sends heartbeats to all peers.
    async fn send_heartbeats(&mut self, heartbeat: Timeout)
    where
        S: Store<R> + Send + Sync + 'static,
        R: Request + Send + 'static,
        P: Response + Send + 'static,
    {
        let session = self.clone();
        tokio::spawn(time::timeout(heartbeat.get_remaining(), async move {
            // NOTE: times out with the heartbeat.
            // Create a channel to receive append entries responses.
            let (append_entries_tx, mut append_entries_rx) = mpsc::unbounded_channel();

            // Send heartbeats to peers.
            session.send_heartbeat_to_peers(append_entries_tx).await?;

            while let Some(response) = append_entries_rx.recv().await {
                let mut peers = session.peers.write().await;
                match response.reason {
                    AppendEntriesResponseReason::LogMismatch => {
                        peers.get_mut(&response.id).unwrap().0 -= 1;
                    }
                    AppendEntriesResponseReason::StaleTerm => {
                        session.node.update_current_term(response.term).await?; // Update term
                        session.node.change_to_follower_state().await; // Steps down to follower
                        break;
                    }
                    AppendEntriesResponseReason::Ok => {}
                }
            }

            crate::Ok(())
        }));
    }

    async fn send_heartbeat_to_peers(
        &self,
        append_entries_tx: mpsc::UnboundedSender<AppendEntriesResponse>,
    ) -> Result<()>
    where
        S: Store<R> + Send + Sync + 'static,
        R: Request + Send + 'static,
        P: Response + Send + 'static,
    {
        let last_commit_index = self.node.inner.store.read().await.get_last_commit_index();
        let peers = self.peers.read().await;

        for (peer, _) in peers.iter() {
            let request = AppendEntriesRequest {
                term: self.node.get_current_term(),
                leader_id: self.node.get_id(),
                last_commit_index,
                prev_log_index: 0,
                prev_log_term: 0,
                entries: vec![],
            };

            let append_entries_tx = append_entries_tx.clone();
            let session = self.clone();
            let peer = *peer;

            tokio::spawn(async move {
                session
                    .node
                    .send_append_entries_rpc(request, peer, append_entries_tx)
                    .await
            });
        }

        Ok(())
    }

    /// Starts a standby task to update the peers continuously.
    async fn start_standby_update(&mut self)
    where
        S: Store<R> + Send + Sync + 'static,
        R: Request + Send + Clone + 'static,
        P: Response + Send + 'static,
    {
        let session = self.clone();
        tokio::spawn(async move {
            // Create a channel to receive append entries responses.
            let (append_entries_tx, mut append_entries_rx) = mpsc::unbounded_channel();

            // Start the peer update tasks.
            session
                .start_standby_update_for_peers(append_entries_tx)
                .await?;

            // TODO: Reason through this logic
            while let Some(response) = append_entries_rx.recv().await {
                let mut peers = session.peers.write().await;
                match response.reason {
                    AppendEntriesResponseReason::LogMismatch => {
                        peers.get_mut(&response.id).unwrap().0 -= 1;
                    }
                    AppendEntriesResponseReason::Ok => {
                        let peer = peers.get_mut(&response.id).unwrap();
                        let next_index = peer.0 + response.len;

                        peer.1 = Some(next_index - 1); // Set match_index
                        peer.0 += next_index; // Increment next_index

                        // Fetch the leader's last commit index
                        let our_last_commit_index = session
                            .node
                            .inner
                            .store
                            .read()
                            .await
                            .get_last_commit_index();

                        // Calculate the last commit index
                        let last_commit_index = session.calculate_last_commit_index().await;

                        // Update last commit index if necessary
                        if let Some(last_commit_index) = last_commit_index {
                            if our_last_commit_index < last_commit_index {
                                session
                                    .node
                                    .inner
                                    .store
                                    .write()
                                    .await
                                    .set_last_commit_index(last_commit_index)?;
                            }
                        }
                    }
                    AppendEntriesResponseReason::StaleTerm => {
                        session.node.update_current_term(response.term).await?; // Update term
                        session.node.change_to_follower_state().await; // Steps down to follower
                        break;
                    }
                }
            }

            crate::Ok(())
        });
    }

    /// Starts each peer update task.
    async fn start_standby_update_for_peers(
        &self,
        append_entries_tx: mpsc::UnboundedSender<AppendEntriesResponse>,
    ) -> Result<()>
    where
        S: Store<R> + Send + Sync + 'static,
        R: Request + Send + Clone + 'static,
        P: Response + Send + 'static,
    {
        let stale_peers = self.get_stale_peers().await?;
        for peer in stale_peers {
            let session = self.clone();
            let append_entries_tx = append_entries_tx.clone();

            tokio::spawn(async move {
                loop {
                    if session.peer_matches(peer).await {
                        break;
                    }

                    // Compose append entries request
                    let request = session.compose_append_entries_request(peer).await;

                    // Send entries
                    session
                        .node
                        .send_append_entries_rpc(request, peer, append_entries_tx.clone())
                        .await?;

                    // Stop loop if signaled
                    if time::timeout(Duration::from_millis(1), session.stop_signal.notified())
                        .await
                        .is_ok()
                    {
                        break;
                    }
                }

                crate::Ok(())
            });
        }

        Ok(())
    }

    /// Composes an append entries request.
    async fn compose_append_entries_request(&self, peer: NodeId) -> AppendEntriesRequest<R>
    where
        R: Clone,
    {
        let peers = self.peers.read().await;
        let (next_index, _) = peers.get(&peer).unwrap();

        let last_commit_index = self.node.inner.store.read().await.get_last_commit_index();
        let prev_log_index = next_index - 1;

        let prev_log_term = self
            .node
            .inner
            .store
            .read()
            .await
            .get_entry(prev_log_index)
            .unwrap()
            .term;

        let entries = self.node.inner.store.read().await;
        let entries = entries.get_entries(*next_index, None).cloned().collect();

        AppendEntriesRequest {
            term: self.node.get_current_term(),
            leader_id: self.node.get_id(),
            last_commit_index,
            prev_log_index,
            prev_log_term,
            entries,
        }
    }

    /// Gets peers that are behind the leader.
    async fn get_stale_peers(&self) -> Result<Vec<NodeId>> {
        let peers = self.peers.read().await;
        let our_last_log_index = self.node.inner.store.read().await.get_last_commit_index();

        // Compare the each peer's next_index with the leader's last log index.
        let stale_peers = peers
            .iter()
            .filter_map(|(node_id, (next_index, _))| {
                if *next_index <= our_last_log_index {
                    Some(*node_id)
                } else {
                    None
                }
            })
            .collect();

        Ok(stale_peers)
    }

    /// Check if peer's log has caught up and matches the leader's log.
    async fn peer_matches(&self, peer: NodeId) -> bool {
        let peers = self.peers.read().await;
        let our_last_log_index = self.node.inner.store.read().await.get_last_commit_index();
        let (_, match_index) = peers.get(&peer).unwrap();

        match_index
            .map(|match_index| match_index == our_last_log_index)
            .unwrap_or(false)
    }

    async fn calculate_last_commit_index(&self) -> Option<u64> {
        let current_term = self.node.get_current_term();

        // Match indices of peers + the leader's last log index.
        let mut match_indices = self
            .peers
            .read()
            .await
            .values()
            .filter_map(|(_, match_index)| *match_index)
            .collect::<Vec<u64>>();

        if match_indices.is_empty() {
            return None;
        }

        // Sort the match indices.
        match_indices.sort_unstable();

        // Get the median index.
        let median_index = match_indices[match_indices.len() / 2];

        // Get the median index term.
        let median_index_term = self
            .node
            .inner
            .store
            .read()
            .await
            .get_entry(median_index)
            .map(|entry| entry.term);

        if let Some(term) = median_index_term {
            if term == current_term {
                return Some(median_index);
            }
        }

        None
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<S, R, P> Deref for ReplicationSession<S, R, P>
where
    S: Store<R>,
    R: Request,
    P: Response,
{
    type Target = ReplicationSessionInner<S, R, P>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<S, R, P> Clone for ReplicationSession<S, R, P>
where
    S: Store<R>,
    R: Request,
    P: Response,
{
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}
