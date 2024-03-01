use std::net::{IpAddr, Ipv4Addr};

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

/// The default host to bind for the database server.
pub const DEFAULT_HOST: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

/// The default port to bind to for listening to peers.
pub const DEFAULT_PEER_PORT: u16 = 6600;

/// The default port to bind to for listening to clients.
pub const DEFAULT_CLIENT_PORT: u16 = 6611;

//--------------------------------------------------------------------------------------------------
// Modules
//--------------------------------------------------------------------------------------------------

pub(crate) mod serde {
    use std::net::IpAddr;

    use zeroraft::{DEFAULT_ELECTION_TIMEOUT_RANGE, DEFAULT_HEARTBEAT_INTERVAL};

    pub(crate) fn default_host() -> IpAddr {
        super::DEFAULT_HOST
    }

    pub(crate) fn default_peer_port() -> u16 {
        super::DEFAULT_PEER_PORT
    }

    pub(crate) const fn default_client_port() -> u16 {
        super::DEFAULT_CLIENT_PORT
    }

    pub(crate) const fn default_heartbeat_interval() -> u64 {
        DEFAULT_HEARTBEAT_INTERVAL
    }

    pub(crate) const fn default_election_timeout_range() -> (u64, u64) {
        DEFAULT_ELECTION_TIMEOUT_RANGE
    }
}
