use std::cmp;

use tokio::sync::mpsc;

use crate::{
    AppendEntriesRequest, AppendEntriesResponse, AppendEntriesResponseReason, RaftNode, Request,
    RequestVoteRequest, RequestVoteResponse, RequestVoteResponseReason, Response, Result, Store,
};

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Responds to a request vote RPC.
pub(crate) async fn respond_to_request_vote<S, R, P>(
    node: RaftNode<S, R, P>,
    RequestVoteRequest {
        term: candidate_term,
        candidate_id,
        last_log_index: candidate_last_log_index,
        last_log_term: candidate_last_log_term,
    }: RequestVoteRequest,
    response_tx: mpsc::Sender<RequestVoteResponse>,
) -> Result<()>
where
    S: Store<R>,
    R: Request,
    P: Response,
{
    let our_term = node.get_current_term();
    let our_id = node.get_id();

    // Check if we already voted for someone else.
    if our_term == candidate_term && node.get_voted_for().await.is_some() {
        response_tx
            .send(RequestVoteResponse {
                term: our_term,
                vote_granted: false,
                id: our_id,
                reason: RequestVoteResponseReason::AlreadyVoted,
            })
            .await?;

        return Ok(());
    }

    // Check if their term is stale.
    if our_term > candidate_term {
        response_tx
            .send(RequestVoteResponse {
                term: our_term,
                vote_granted: false,
                id: our_id,
                reason: RequestVoteResponseReason::StaleTerm,
            })
            .await?;

        return Ok(());
    }

    // Update the term and voted for.
    node.update_current_term_and_voted_for(candidate_term, candidate_id)
        .await?;

    // Change to follower state.
    node.change_to_follower_state().await;

    // Check candidate's completeness.
    let our_last_log_index = node.inner.store.read().await.get_last_index();
    let our_last_log_term = node.inner.store.read().await.get_last_term();
    if (our_last_log_term > candidate_last_log_term)
        || (our_last_log_term == candidate_last_log_term
            && our_last_log_index > candidate_last_log_index)
    {
        response_tx
            .send(RequestVoteResponse {
                term: our_term,
                vote_granted: false,
                id: our_id,
                reason: RequestVoteResponseReason::IncompleteLog,
            })
            .await?;

        return Ok(());
    }

    // Send granted response.
    response_tx
        .send(RequestVoteResponse {
            term: our_term,
            vote_granted: true,
            id: our_id,
            reason: RequestVoteResponseReason::Ok,
        })
        .await?;

    Ok(())
}

/// Responds to an append entries RPC and takes a callback function that is called after the first common checks.
pub(crate) async fn respond_to_append_entries<S, R, P>(
    node: RaftNode<S, R, P>,
    request: AppendEntriesRequest<R>,
    response_tx: mpsc::Sender<AppendEntriesResponse>,
) -> Result<()>
where
    S: Store<R>,
    R: Request,
    P: Response,
{
    let leader_term = request.term;
    let our_term = node.get_current_term();
    let our_id = node.get_id();
    let our_last_log_index = node.inner.store.read().await.get_last_index();
    let len = request.entries.len() as u64;

    // Check if their term is stale.
    if our_term > leader_term {
        response_tx
            .send(AppendEntriesResponse {
                term: our_term,
                success: false,
                id: our_id,
                len: 0,
                reason: AppendEntriesResponseReason::StaleTerm,
            })
            .await?;

        return Ok(());
    }

    // Update current term.
    node.update_current_term(leader_term).await?;

    // Update last heard from leader.
    node.update_last_heard_from_leader().await;

    // Update leader id.
    node.update_leader_id(request.leader_id).await;

    // Change to follower state.
    node.change_to_follower_state().await;

    // Check if we don't have the prev log index the leader is trying to append to.
    if our_last_log_index < request.prev_log_index {
        response_tx
            .send(AppendEntriesResponse {
                term: our_term,
                success: false,
                id: our_id,
                len: 0,
                reason: AppendEntriesResponseReason::LogMismatch,
            })
            .await?;

        return Ok(());
    }

    // Check if log index exists but the term doesn't match.
    if let Some(entry) = node
        .inner
        .store
        .read()
        .await
        .get_entry(request.prev_log_index)
    {
        if entry.term != request.prev_log_term {
            response_tx
                .send(AppendEntriesResponse {
                    term: our_term,
                    success: false,
                    id: our_id,
                    len: 0,
                    reason: AppendEntriesResponseReason::LogMismatch,
                })
                .await?;

            return Ok(());
        }
    }

    // Remove extraneous entries.
    node.inner
        .store
        .write()
        .await
        .remove_entries_after(request.prev_log_index)?;

    // Append the entries.
    node.inner
        .store
        .write()
        .await
        .append_entries(request.entries)?;

    // Update the commit index.
    let our_last_log_index = node.inner.store.read().await.get_last_index();
    node.inner
        .store
        .write()
        .await
        .set_last_commit_index(cmp::min(request.last_commit_index, our_last_log_index))?;

    // Respond to the append entries request.
    response_tx
        .send(AppendEntriesResponse {
            term: our_term,
            success: true,
            id: our_id,
            len,
            reason: AppendEntriesResponseReason::Ok,
        })
        .await?;

    Ok(())
}
