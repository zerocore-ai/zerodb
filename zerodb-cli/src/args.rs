use std::net::SocketAddr;

use clap::Parser;
use zerodb::configs::{
    DEFAULT_CLIENT_PORT, DEFAULT_ELECTION_TIMEOUT_RANGE, DEFAULT_HEARTBEAT_INTERVAL, DEFAULT_HOST,
    DEFAULT_PEER_PORT,
};

use crate::styles;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Arguments for managing the zerodb serverless engine.
#[derive(Debug, Parser)]
#[command(name = "zerodb", author, about, version, styles=styles::styles())]
pub struct ZerodbArgs {
    /// The subcommand to run.
    #[command(subcommand)]
    pub subcommand: Option<SubCommand>,
}

/// Zerodb has many functionalities. These subcommands lets you use the different functions of the engines
#[derive(Debug, Parser)]
pub enum SubCommand {
    /// Starts zerodb as a server.
    Serve {
        /// The path to the configuration file.
        #[arg(short, long)]
        file: Option<String>,

        /// The name of the node.
        #[arg(short, long, default_value_t = String::new())]
        name: String,

        /// The host to listen on.
        #[arg(long, default_value_t = DEFAULT_HOST.to_string())]
        host: String,

        /// The port to listen on for peer requests.
        #[arg(long, default_value_t = DEFAULT_PEER_PORT)]
        peer_port: u16,

        /// The port to listen on for client requests.
        #[arg(long, default_value_t = DEFAULT_CLIENT_PORT)]
        client_port: u16,

        /// The list of seed nodes to connect to.
        #[arg(short, long, use_value_delimiter = true, value_delimiter = ',')]
        peers: Vec<SocketAddr>,

        /// The interval between heartbeats.
        #[arg(long, default_value_t = DEFAULT_HEARTBEAT_INTERVAL)]
        heartbeat_interval: u64,

        /// The minimum election timeout range.
        #[arg(long, default_value_t = DEFAULT_ELECTION_TIMEOUT_RANGE.0)]
        election_timeout_min: u64,

        /// The maximum election timeout range.
        #[arg(long, default_value_t = DEFAULT_ELECTION_TIMEOUT_RANGE.1)]
        election_timeout_max: u64,
    },
    // /// Starts the zerodb interactive shell.
    // Shell {
    //     /// The url of the zerodb server.
    //     #[arg(short, long, default_value_t = DEFAULT_URL.to_string())]
    //     url: String,
    // },
}
