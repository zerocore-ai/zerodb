use std::{fs, path::Path};

use crate::{
    config::{DEFAULT_CLIENT_PORT, DEFAULT_HOST, DEFAULT_PEER_PORT},
    Result, ZerodbError,
};
use serde::{Deserialize, Serialize};
use structstruck::strike;
use typed_builder::TypedBuilder;

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

                /// The seeds to connect to.
                #[builder(default)]
                #[serde(default)]
                pub seeds: Vec<String>,
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
            seeds: vec![],
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ZerodbConfig::default();

        assert_eq!(config.network.host, DEFAULT_HOST);
        assert_eq!(config.network.peer_port, DEFAULT_PEER_PORT);
        assert_eq!(config.network.client_port, DEFAULT_CLIENT_PORT);
    }

    #[test]
    fn test_default_network_config() {
        let config = NetworkConfig::default();

        assert_eq!(config.host, DEFAULT_HOST);
        assert_eq!(config.peer_port, DEFAULT_PEER_PORT);
        assert_eq!(config.client_port, DEFAULT_CLIENT_PORT);
    }

    #[test]
    fn test_full_toml() -> anyhow::Result<()> {
        let toml = r#"
            [network]
            name = "alice"
            host = "127.0.0.1"
            peer_port = 7700
            client_port = 7711
            seeds = ["127.0.0.1:7800", "127.0.0.1:7900"]
        "#;

        let config: ZerodbConfig = toml::from_str(toml)?;

        assert_eq!(config.network.host, "127.0.0.1");
        assert_eq!(config.network.peer_port, 7700);
        assert_eq!(config.network.client_port, 7711);
        assert_eq!(
            config.network.seeds,
            vec!["127.0.0.1:7800", "127.0.0.1:7900"]
        );

        Ok(())
    }
}
