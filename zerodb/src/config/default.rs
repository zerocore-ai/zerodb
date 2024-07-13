use serde::{Deserialize, Serialize};
use zeroutils_config::network::PortDefaults;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The default zerodb port configuration.
#[derive(Debug, Deserialize, Serialize)]
pub struct DbPortDefaults;

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl PortDefaults for DbPortDefaults {
    fn default_peer_port() -> u16 {
        7700
    }

    fn default_user_port() -> u16 {
        7711
    }
}
