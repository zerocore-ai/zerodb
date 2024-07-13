use std::{fs, path::Path};

use serde::{Deserialize, Serialize};
use structstruck::strike;
use typed_builder::TypedBuilder;
use zeroutils_config::{network::NetworkConfig, ConfigResult, MainConfig};

use super::DbPortDefaults;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

strike! {
    /// The configuration for the `Zerodb` instance.
    #[strikethrough[derive(Debug, Deserialize, Serialize, TypedBuilder)]]
    #[derive(Default)]
    pub struct ZerodbConfig {
        /// The network configuration.
        #[serde(default)]
        #[builder(default)]
        pub network: ZerodbNetworkConfig,
    }
}

/// The zerodb network configuration.
pub type ZerodbNetworkConfig = NetworkConfig<'static, DbPortDefaults>;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl ZerodbConfig {
    /// Creates a new `ZerodbConfig` from a toml file.
    pub fn from_file(path: impl AsRef<Path>) -> ConfigResult<Self> {
        let config = fs::read_to_string(path)?;
        let config = toml::from_str(&config)?;
        Ok(config)
    }

    /// Creates a new `ZerodbConfig` from a toml string.
    pub fn from_string(config: impl AsRef<str>) -> ConfigResult<Self> {
        let config = toml::from_str(config.as_ref())?;
        Ok(config)
    }

    /// Validates the configuration.
    pub fn validate(&self) -> ConfigResult<()> {
        self.network.validate()
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl MainConfig for ZerodbConfig {
    fn validate(&self) -> ConfigResult<()> {
        self.network.validate()
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        net::{IpAddr, Ipv4Addr, SocketAddr},
        str::FromStr,
    };

    use zeroutils_config::default::{DEFAULT_ELECTION_TIMEOUT_RANGE, DEFAULT_HEARTBEAT_INTERVAL};
    use zeroutils_did_wk::WrappedDidWebKey;

    use super::*;

    #[test]
    fn test_toml_full() -> anyhow::Result<()> {
        let toml = r#"
        [network]
        id = "did:wk:z6MkoVs2h6TnfyY8fx2ZqpREWSLS8rBDQmGpyXgFpg63CSUb"
        name = "alice"
        host = "127.0.0.1"
        peer_port = 6600
        user_port = 6611

        [network.seeds]
        "did:wk:m7QFAoSJPFzmaqQiTkLrWQ6pbYrmI6L07Fkdg8SCRpjP1Ig" = "127.0.0.1:7800"
        "did:wk:z6MknLif7jhwt6jUfn14EuDnxWoSHkkajyDi28QMMH5eS1DL" = "127.0.0.1:7900"

        [network.consensus]
        heartbeat_interval = 1000
        election_timeout_range = [150, 300]
        "#;

        let config: ZerodbConfig = toml::from_str(toml)?;

        assert_eq!(
            config.network.id,
            WrappedDidWebKey::from_str("did:wk:z6MkoVs2h6TnfyY8fx2ZqpREWSLS8rBDQmGpyXgFpg63CSUb")?
        );
        assert_eq!(config.network.host, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        assert_eq!(config.network.peer_port, 6600);
        assert_eq!(config.network.user_port, 6611);
        assert_eq!(config.network.seeds, {
            let mut peers = HashMap::new();
            peers.insert(
                WrappedDidWebKey::from_str(
                    "did:wk:m7QFAoSJPFzmaqQiTkLrWQ6pbYrmI6L07Fkdg8SCRpjP1Ig",
                )?,
                SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), 7800),
            );
            peers.insert(
                WrappedDidWebKey::from_str(
                    "did:wk:z6MknLif7jhwt6jUfn14EuDnxWoSHkkajyDi28QMMH5eS1DL",
                )?,
                SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), 7900),
            );
            peers
        });
        assert_eq!(config.network.consensus.heartbeat_interval, 1000);
        assert_eq!(config.network.consensus.election_timeout_range, (150, 300));

        Ok(())
    }

    #[test]
    fn test_toml_defaults() -> anyhow::Result<()> {
        let config: ZerodbConfig = toml::from_str("")?;

        assert_eq!(config.network.host, IpAddr::V4(Ipv4Addr::LOCALHOST));
        assert_eq!(config.network.peer_port, 6600);
        assert_eq!(config.network.user_port, 6611);
        assert!(config.network.seeds.is_empty());
        assert_eq!(
            config.network.consensus.heartbeat_interval,
            DEFAULT_HEARTBEAT_INTERVAL
        );
        assert_eq!(
            config.network.consensus.election_timeout_range,
            DEFAULT_ELECTION_TIMEOUT_RANGE
        );

        Ok(())
    }
}
