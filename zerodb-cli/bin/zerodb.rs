use clap::{CommandFactory, Parser};
use zerodb::{
    configs::{ConsensusConfig, NetworkConfig, ZerodbConfig},
    ZerodbNode,
};
use zerodb_cli::{SubCommand, ZerodbArgs};

//--------------------------------------------------------------------------------------------------
// Main
//--------------------------------------------------------------------------------------------------

#[tokio::main]
async fn main() -> zerodb_cli::Result<()> {
    // Initialize the zerodb.
    zerodb::init()?;

    // Parse CLI arguments.
    let args = ZerodbArgs::parse();

    // Run the subcommand.
    match args.subcommand {
        Some(SubCommand::Serve {
            file,
            name,
            host,
            peer_port,
            client_port,
            peers,
            heartbeat_interval,
            election_timeout_min,
            election_timeout_max,
        }) => {
            let config = if let Some(file) = file {
                toml::from_str(&std::fs::read_to_string(file)?)?
            } else {
                ZerodbConfig::builder()
                    .network(
                        NetworkConfig::builder()
                            .name(name)
                            .host(host)
                            .peer_port(peer_port)
                            .client_port(client_port)
                            .peers(peers)
                            .consensus(
                                ConsensusConfig::builder()
                                    .heartbeat_interval(heartbeat_interval)
                                    .election_timeout_range((
                                        election_timeout_min,
                                        election_timeout_max,
                                    ))
                                    .build(),
                            )
                            .build(),
                    )
                    .build()
            };
            ZerodbNode::new(config)?.start().await?;
        }
        None => ZerodbArgs::command().print_help()?,
    }

    Ok(())
}
