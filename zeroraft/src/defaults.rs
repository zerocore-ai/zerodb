//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

/// The default interval at which heartbeats are sent.
pub const DEFAULT_HEARTBEAT_INTERVAL: u64 = 50;

/// The default range of election timeouts.
pub const DEFAULT_ELECTION_TIMEOUT_RANGE: (u64, u64) = (150, 300);
