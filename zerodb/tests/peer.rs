use zerodb::{
    config::{NetworkConfig, ZerodbConfig},
    ZerodbNode,
};

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[tokio::test]
async fn pinging_nodes() -> anyhow::Result<()> {
    // Initialize tracing.
    zerodb::init()?;

    // TODO(appcypher): Use helper to create db instances.
    // Configs
    let alice_conf = ZerodbConfig::builder()
        .network(
            NetworkConfig::builder()
                .name("alice".to_string())
                .seeds(vec!["ws://127.0.0.1:6700".to_string()])
                .build(),
        )
        .build();

    let bob_conf = ZerodbConfig::builder()
        .network(
            NetworkConfig::builder()
                .name("bob".to_string())
                .peer_port(6700)
                .client_port(6711)
                .seeds(vec!["ws://127.0.0.1:6600".to_string()])
                .build(),
        )
        .build();

    // Starting the nodes.
    let _alice_db = ZerodbNode::new(alice_conf)?.start().await?;
    let _bob_db = ZerodbNode::new(bob_conf)?.start().await?;

    Ok(())
}
