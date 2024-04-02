//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[test_log::test(tokio::test)]
async fn test_leader_can_replicate_entries_on_followers() {
    // todo!("Write test");
}

// TODO: Follower with log mismatch. Leader decrements next index until match is reached. Leader sends entries
// TODO: Candidate gets append entries from leader with higher term. Candidate steps down and append entries
// TODO: Leader X gets append entries from leader Y with higher term. Leader X steps down and append entries
// TODO: Leader has stale term. Leader steps down
// TODO: Leader steps down. Update to followers stops
// TODO: Follower removed due to membership config change. Follower stops updating
// TODO: Follower added due to membership config change. Follower starts updating
// TODO: When majority replicates an entry. Leaders commit index is updated
