use std::{fs, net::SocketAddr, path::Path};

use serde::{Deserialize, Serialize};
use structstruck::strike;
use typed_builder::TypedBuilder;

use crate::{
    config::{DEFAULT_CLIENT_PORT, DEFAULT_HOST, DEFAULT_PEER_PORT},
    Result, ZerodbError,
};

use super::{DEFAULT_ELECTION_TIMEOUT_RANGE, DEFAULT_HEARTBEAT_INTERVAL};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

strike! {
    /// The configuration for the `Zerodb` instance.
    #[strikethrough[derive(Debug, Deserialize, Serialize)]]
    #[derive(TypedBuilder, Default)]
    pub struct ZerodbConfig {
        /// The network configuration for cluster communication.
        #[builder(default)]
        #[serde(default)]
        pub network:
            /// The network configuration for cluster communication.
            #[derive(TypedBuilder)]
            pub struct NetworkConfig {
                /// Name of the node.
                #[builder(default)]
                #[serde(default)]
                pub name: String,

                /// The host to listen on.
                #[builder(default = DEFAULT_HOST.to_string())]
                #[serde(default = "super::serde::default_host")]
                pub host: String,

                /// The port to listen on for peers.
                #[builder(default = DEFAULT_PEER_PORT)]
                #[serde(default = "super::serde::default_peer_port")]
                pub peer_port: u16,

                /// The port to listen on for clients.
                #[builder(default = DEFAULT_CLIENT_PORT)]
                #[serde(default = "super::serde::default_client_port")]
                pub client_port: u16,

                /// The peers to connect to.
                #[builder(default)]
                #[serde(default)]
                pub peers: Vec<SocketAddr>,

                // /// A passive node does not partake in consensus.
                // #[builder(default)]
                // #[serde(default)]
                // pub passive: bool,

                /// The consensus configuration.
                pub consensus:
                    /// The consensus configuration.
                    #[derive(TypedBuilder)]
                    pub struct ConsensusConfig {
                        /// The interval at which heartbeats are sent.
                        #[builder(default)]
                        #[serde(default = "super::serde::default_heartbeat_interval")]
                        pub heartbeat_interval: u64,

                        /// The range of election timeouts.
                        #[builder(default)]
                        #[serde(default = "super::serde::default_election_timeout_range")]
                        pub election_timeout_range: (u64, u64),
                    }
            }
    }
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl ZerodbConfig {
    /// Creates a new `ZerodbConfig` from a toml file.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let config = fs::read_to_string(path)?;
        let config = toml::from_str(&config)?;
        Ok(config)
    }

    /// Creates a new `ZerodbConfig` from a toml string.
    pub fn from_string(config: impl AsRef<str>) -> Result<Self> {
        let config = toml::from_str(config.as_ref())?;
        Ok(config)
    }

    /// Validates the configuration.
    pub fn validate(&self) -> Result<()> {
        self.network.validate()
    }
}

impl NetworkConfig {
    /// Validates the configuration.
    pub fn validate(&self) -> Result<()> {
        if self.peer_port == self.client_port {
            return Err(ZerodbError::EqualPeerClientPorts(self.peer_port));
        }

        Ok(())
    }

    /// Gets the peer address.
    pub fn get_peer_address(&self) -> Result<SocketAddr> {
        Self::parse_address(&self.host, self.peer_port)
    }

    /// Gets the client address.
    pub fn get_client_address(&self) -> Result<SocketAddr> {
        Self::parse_address(&self.host, self.client_port)
    }

    /// Parses a host and port into a `SocketAddr`.
    pub fn parse_address(host: &str, port: u16) -> Result<SocketAddr> {
        let addr = format!("{}:{}", host, port);
        Ok(addr.parse()?)
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            host: DEFAULT_HOST.to_string(),
            peer_port: DEFAULT_PEER_PORT,
            client_port: DEFAULT_CLIENT_PORT,
            peers: vec![],
            consensus: ConsensusConfig::default(),
        }
    }
}

impl Default for ConsensusConfig {
    fn default() -> Self {
        Self {
            heartbeat_interval: DEFAULT_HEARTBEAT_INTERVAL,
            election_timeout_range: DEFAULT_ELECTION_TIMEOUT_RANGE,
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use super::*;

    #[test]
    fn test_default_config() {
        let config = ZerodbConfig::default();

        assert_eq!(config.network.host, DEFAULT_HOST);
        assert_eq!(config.network.peer_port, DEFAULT_PEER_PORT);
        assert_eq!(config.network.client_port, DEFAULT_CLIENT_PORT);
        assert_eq!(config.network.peers, Vec::<SocketAddr>::new());
        assert_eq!(config.network.consensus.heartbeat_interval, 100);
        assert_eq!(config.network.consensus.election_timeout_range, (150, 300));
    }

    #[test]
    fn test_full_toml() -> anyhow::Result<()> {
        let toml = r#"
            [network]
            name = "alice"
            host = "127.0.0.1"
            peer_port = 7700
            client_port = 7711
            peers = ["127.0.0.1:7800", "127.0.0.1:7900"]

            [network.consensus]
            heartbeat_interval = 1000
            election_timeout_range = [150, 300]
        "#;

        let config: ZerodbConfig = toml::from_str(toml)?;

        assert_eq!(config.network.host, "127.0.0.1");
        assert_eq!(config.network.peer_port, 7700);
        assert_eq!(config.network.client_port, 7711);
        assert_eq!(
            config.network.peers,
            vec![
                SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 7800),
                SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 7900),
            ]
        );
        assert_eq!(config.network.consensus.heartbeat_interval, 1000);
        assert_eq!(config.network.consensus.election_timeout_range, (150, 300));

        Ok(())
    }
}
