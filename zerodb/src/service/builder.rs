use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
};

use zeroraft::NodeId;

use crate::{
    configs::{
        ConsensusConfig, NetworkConfig, ZerodbConfig, DEFAULT_CLIENT_PORT, DEFAULT_HOST,
        DEFAULT_PEER_PORT,
    },
    Result, ZerodbService,
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A builder for the `ZerodbNode` type.
pub struct ZerodbServiceBuilder {
    id: NodeId,
    name: String,
    host: IpAddr,
    peer_port: u16,
    client_port: u16,
    seeds: HashMap<NodeId, SocketAddr>,
    raft_heartbeat_interval: u64,
    raft_election_timeout: (u64, u64),
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl ZerodbServiceBuilder {
    /// Sets the id of the ZerodbNode instance.
    pub fn id(mut self, id: NodeId) -> Self {
        self.id = id;
        self
    }

    /// Sets the name of the ZerodbNode instance.
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    /// Sets the host address of the ZerodbNode instance.
    pub fn host(mut self, host: IpAddr) -> Self {
        self.host = host;
        self
    }

    /// Sets the peer port of the ZerodbNode instance.
    pub fn peer_port(mut self, peer_port: u16) -> Self {
        self.peer_port = peer_port;
        self
    }

    /// Sets the client port of the ZerodbNode instance.
    pub fn client_port(mut self, client_port: u16) -> Self {
        self.client_port = client_port;
        self
    }

    /// Sets the seeds of the ZerodbNode instance.
    pub fn seeds(mut self, seeds: HashMap<NodeId, SocketAddr>) -> Self {
        self.seeds = seeds;
        self
    }

    /// Sets the Raft heartbeat interval of the ZerodbNode instance.
    pub fn raft_heartbeat_interval(mut self, raft_heartbeat_interval: u64) -> Self {
        self.raft_heartbeat_interval = raft_heartbeat_interval;
        self
    }

    /// Sets the Raft election timeout of the ZerodbNode instance.
    pub fn raft_election_timeout(mut self, raft_election_timeout: (u64, u64)) -> Self {
        self.raft_election_timeout = raft_election_timeout;
        self
    }

    /// Builds the ZerodbNode.
    pub fn build(self) -> Result<ZerodbService> {
        let config = ZerodbConfig {
            network: NetworkConfig {
                id: self.id,
                name: self.name,
                host: self.host,
                peer_port: self.peer_port,
                client_port: self.client_port,
                seeds: self.seeds,
                consensus: ConsensusConfig {
                    heartbeat_interval: self.raft_heartbeat_interval,
                    election_timeout_range: self.raft_election_timeout,
                },
            },
        };

        ZerodbService::with_config(config)
    }
}

//--------------------------------------------------------------------------------------------------
// Traits Implementations
//--------------------------------------------------------------------------------------------------

impl Default for ZerodbServiceBuilder {
    fn default() -> Self {
        Self {
            id: NodeId::new_v4(),
            name: String::default(),
            host: DEFAULT_HOST,
            peer_port: DEFAULT_PEER_PORT,
            client_port: DEFAULT_CLIENT_PORT,
            seeds: HashMap::new(),
            raft_heartbeat_interval: 0,
            raft_election_timeout: (0, 0),
        }
    }
}
