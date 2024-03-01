use std::{
    collections::HashMap,
    fs,
    net::{IpAddr, SocketAddr},
    path::Path,
};

use serde::{Deserialize, Serialize};
use structstruck::strike;
use zeroraft::{NodeId, DEFAULT_ELECTION_TIMEOUT_RANGE, DEFAULT_HEARTBEAT_INTERVAL};

use crate::{
    configs::{DEFAULT_CLIENT_PORT, DEFAULT_HOST, DEFAULT_PEER_PORT},
    Result, ZerodbError,
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

strike! {
    /// The configuration for the `Zerodb` instance.
    #[strikethrough[derive(Debug, Deserialize, Serialize)]]
    #[derive(Default)]
    pub struct ZerodbConfig {
        /// The network configuration for cluster communication.
        #[serde(default)]
        pub network:
            /// The network configuration for cluster communication.
            pub struct NetworkConfig {
                /// The id of the node.
                #[serde(default)]
                pub id: NodeId,

                /// Name of the node.
                #[serde(default)]
                pub name: String,

                /// The host to listen on.
                #[serde(default = "super::serde::default_host")]
                pub host: IpAddr,

                /// The port to listen on for peers.
                #[serde(default = "super::serde::default_peer_port")]
                pub peer_port: u16,

                /// The port to listen on for clients.
                #[serde(default = "super::serde::default_client_port")]
                pub client_port: u16,

                /// The peers to connect to.
                #[serde(default)]
                pub seeds: HashMap<NodeId, SocketAddr>,

                // /// A passive node does not partake in consensus.
                // #[builder(default)]
                // #[serde(default)]
                // pub passive: bool,

                /// The consensus configuration.
                pub consensus:
                    /// The consensus configuration.
                    pub struct ConsensusConfig {
                        /// The interval at which heartbeats are sent.
                        #[serde(default = "super::serde::default_heartbeat_interval")]
                        pub heartbeat_interval: u64,

                        /// The range of election timeouts.
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
    pub fn get_peer_address(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.peer_port)
    }

    /// Gets the client address.
    pub fn get_client_address(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.client_port)
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            id: NodeId::new_v4(),
            name: String::new(),
            host: DEFAULT_HOST,
            peer_port: DEFAULT_PEER_PORT,
            client_port: DEFAULT_CLIENT_PORT,
            seeds: HashMap::new(),
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
    use std::{net::Ipv4Addr, str::FromStr};

    use super::*;

    #[test]
    fn test_default_config() {
        let config = ZerodbConfig::default();

        assert_eq!(config.network.host, DEFAULT_HOST);
        assert_eq!(config.network.peer_port, DEFAULT_PEER_PORT);
        assert_eq!(config.network.client_port, DEFAULT_CLIENT_PORT);
        assert_eq!(config.network.seeds, HashMap::new());
        assert_eq!(config.network.consensus.heartbeat_interval, 50);
        assert_eq!(config.network.consensus.election_timeout_range, (150, 300));
    }

    #[test]
    fn test_full_toml() -> anyhow::Result<()> {
        let toml = r#"
        [network]
        id = "4b72a445-d90d-4fd7-9711-b0e587ab6a21"
        name = "alice"
        host = "127.0.0.1"
        peer_port = 7700
        client_port = 7711

        [network.seeds]
        4b72a445-d90d-4fd7-9711-b0e587ab6a21 = "127.0.0.1:7800"
        0713a29e-9197-448a-9d34-e4ab1aa07eea = "127.0.0.1:7900"

        [network.consensus]
        heartbeat_interval = 1000
        election_timeout_range = [150, 300]
        "#;

        let config: ZerodbConfig = toml::from_str(toml)?;

        assert_eq!(config.network.host, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        assert_eq!(config.network.peer_port, 7700);
        assert_eq!(config.network.client_port, 7711);
        assert_eq!(config.network.seeds, {
            let mut peers = HashMap::new();
            peers.insert(
                NodeId::from_str("4b72a445-d90d-4fd7-9711-b0e587ab6a21")?,
                SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), 7800),
            );
            peers.insert(
                NodeId::from_str("0713a29e-9197-448a-9d34-e4ab1aa07eea")?,
                SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), 7900),
            );
            peers
        });
        assert_eq!(config.network.consensus.heartbeat_interval, 1000);
        assert_eq!(config.network.consensus.election_timeout_range, (150, 300));

        Ok(())
    }
}
