use std::{pin::Pin, sync::Arc};

use futures::Future;
use tokio::sync::mpsc;

use crate::{
    AppendEntriesRequest, AppendEntriesResponse, AppendEntriesResponseReason, Log, RaftNodeInner,
    Request, RequestVoteRequest, RequestVoteResponse, RequestVoteResponseReason, Response, Result,
};

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Responds to a request vote RPC.
pub(crate) async fn respond_to_request_vote<L, R, P>(
    node: Arc<RaftNodeInner<L, R, P>>,
    RequestVoteRequest {
        term: candidate_term,
        candidate_id,
        last_log_index: candidate_last_log_index,
        last_log_term: candidate_last_log_term,
    }: RequestVoteRequest,
    response_tx: mpsc::Sender<RequestVoteResponse>,
) -> Result<()>
where
    L: Log<R>,
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
    let our_last_log_index = node.log.lock().await.get_last_index().await;
    let our_last_log_term = node.log.lock().await.get_last_term().await;
    if our_last_log_term > candidate_last_log_term && our_last_log_index > candidate_last_log_index
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
pub(crate) async fn respond_to_append_entries<L, R, P>(
    node: Arc<RaftNodeInner<L, R, P>>,
    request: AppendEntriesRequest<R>,
    response_tx: mpsc::Sender<AppendEntriesResponse>,
    callback: fn(
        Arc<RaftNodeInner<L, R, P>>,
        AppendEntriesRequest<R>,
        mpsc::Sender<AppendEntriesResponse>,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>,
) -> Result<()>
where
    L: Log<R>,
    R: Request,
    P: Response,
{
    let leader_term = request.term;
    let our_term = node.get_current_term();
    let our_id = node.get_id();

    // Check if their term is stale.
    if our_term > leader_term {
        response_tx
            .send(AppendEntriesResponse {
                term: our_term,
                success: false,
                id: our_id,
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

    // Other operations we can do.
    callback(node, request, response_tx).await?;

    Ok(())
}
