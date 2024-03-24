mod fixtures;

use std::time::Duration;

use tokio::time;

use crate::fixtures::{RaftNodeCluster, RaftNodeClusterConfig};

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[test_log::test(tokio::test)]
async fn test_cluster_can_choose_single_leader_from_start() -> anyhow::Result<()> {
    let mut cluster = RaftNodeCluster::new_with_config(
        3,
        RaftNodeClusterConfig {
            election_timeout_range: (100, 200),
            ..Default::default()
        },
    )?;

    // Start the cluster.
    cluster.start();

    // Wait for the cluster to work out leader.
    time::sleep(Duration::from_secs(1)).await;

    // Count leaders.
    let mut leaders = 0;
    for server in cluster.get_servers().values() {
        let node = server.get_node();
        if node.is_leader_state().await {
            leaders += 1;
        }
    }

    // There should be only one leader.
    assert_eq!(leaders, 1);

    Ok(())
}

// TODO: Candidate with incomplete log
// TODO: Candidate has stale term
// TODO: Voter already voted in candidate term
// TODO: Candidate restarts its vote session if election coundown times out without a majority vote.
// TODO: Candidate resends vote requests to peers that did not acknowledge the vote.
