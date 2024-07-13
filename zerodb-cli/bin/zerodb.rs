use std::{collections::HashMap, net::SocketAddr, str::FromStr};

use clap::{CommandFactory, Parser};
use zerodb::{raft::NodeId, ZerodbService};
use zerodb_cli::{Result, SubCommand, ZerodbArgs};

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
            // id,
            // name,
            // host,
            // peer_port,
            // user_port,
            // heartbeat_interval,
            peer,
            // election_timeout_min,
            // election_timeout_max,
            ..
        }) => {
            let config = if let Some(file) = file {
                toml::from_str(&std::fs::read_to_string(file)?)?
            } else {
                let _seeds = peer
                    .iter()
                    .map(|peer| {
                        let mut peer = peer.splitn(2, ':');
                        let id = NodeId::from_str(peer.next().unwrap())?; // TODO: Handle error.
                        let addr: SocketAddr = peer.next().unwrap().parse()?; // TODO: Handle error.
                        Ok((id, addr))
                    })
                    .collect::<Result<HashMap<_, _>>>()?;

                // TODO(appcypher): Need to support did IDs in zeroraft.
                // ZerodbConfig {
                //     network: NetworkConfig {
                //         id,
                //         name,
                //         host,
                //         peer_port,
                //         user_port,
                //         seeds,
                //         consensus: ConsensusConfig {
                //             heartbeat_interval,
                //             election_timeout_range: (election_timeout_min, election_timeout_max),
                //         },
                //     },
                // }

                todo!()
            };

            ZerodbService::with_config(config)?.start().await?;
        }
        None => ZerodbArgs::command().print_help()?,
    }

    Ok(())
}
