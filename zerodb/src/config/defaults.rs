//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

/// The default host to bind for the database server.
pub const DEFAULT_HOST: &str = "127.0.0.1";

/// The default port to bind to for listening to peers.
pub const DEFAULT_PEER_PORT: u16 = 6600;

/// The default port to bind to for listening to clients.
pub const DEFAULT_CLIENT_PORT: u16 = 6611;

//--------------------------------------------------------------------------------------------------
// Modules
//--------------------------------------------------------------------------------------------------

pub(crate) mod serde {
    pub(crate) fn default_host() -> String {
        super::DEFAULT_HOST.to_string()
    }

    pub(crate) fn default_peer_port() -> u16 {
        super::DEFAULT_PEER_PORT
    }

    pub(crate) const fn default_client_port() -> u16 {
        super::DEFAULT_CLIENT_PORT
    }
}
