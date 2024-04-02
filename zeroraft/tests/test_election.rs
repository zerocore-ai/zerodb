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
            heartbeat_interval: 25,
        },
    )?;

    cluster.start(); // Start the cluster.

    time::sleep(Duration::from_secs(1)).await; // Wait for the cluster to work out leader.

    // Count leaders.
    let mut leaders = 0;
    for server in cluster.get_servers().values() {
        let node = server.get_node();
        if node.is_leader_state().await {
            leaders += 1;
        }
    }

    assert_eq!(leaders, 1); // There should be only one leader.

    Ok(())
}

// TODO: Candidate with incomplete log. Candidate steps down
// TODO: Candidate has stale term. Candidate steps down
// TODO: Voter already voted in candidate term. Candidate waits
// TODO: Election coundown times out without a majority vote. Candidate retries
// TODO: Peers did not acknowledge the vote. Candidate resends
