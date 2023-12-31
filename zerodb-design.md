## Single Replica-Set Transaction w/ a Strongly Consistent Consensus

### Steps

#### Coordinator Selection

- The client (or some initial node) selects a new coordinator from the replica set; ideally randomly.

#### Fetch & Lock

- The coordinator sends a request for target data, as well as, lock on the data to its replica set
- Locked data is marked LOCK
- The coordinator expects the data and and a quorum lock ack from the replica set
- The coordinator determines what the latest data is
- The coordinator applies transaction ops to the data

#### Commit

- The coordinator sends a transaction result and update request to its replica set
- The coordinator expects a quorum update ack
- The coordinator sends the transaction result to the client

### Considerations

- Latest data decided on in a write operation, has to be available in at least one of the replicas in subsequent read operation, for strong consistency.
- Availability not only means being available to the client but also being available to other nodes.
- What constitutes latest is based on a logical clock.

### Problems

- What if any of the replica set is down or takes long to respond?:

  - There has to be a timeout period for each participating replica expected response.

- What if a replica tries to read data that is locked by some other replicas, with the locked data being the latest?:

  - Reading replica checks with the lock's coordinator for an active txn session.
  - If there is an active txn session, reading replica can wait.
  - Otherwise, the reading replica unwraps the lock and set that the data as latest.
  - Optionally, reading replica can send updates to stale replicas.

- What if another transaction for target data is issued to the same replica set?:

  - If there is an active txn session, the replica set will reject the transaction.

- What about deadlocks on data? How do they yield?:

  - ??? (Maybe timeout on locks?)

- What if the coordinator is unavailable before commiting a txn and some replicas are in lock state?:

  - They remain in lock state until a reading replica comes around.

- What if a chunk of a replica set holding the latest data is unavailable during txn, but becomes available after txn or during commit?:
  - The chunk is not representative of the latest data at txn time. The lock is now the latest data.

### Comparisons

- vs Paxos:
  - No promise phase
- vs Raft:
  - No leader election phase
  - No heartbeat pings
  - No state machine replication and replays

## Cross Replica-Set Transaction w/ a Strongly Consistent Consensus

### Steps

#### Coordinator Selection

- Unlike single replica-set txn, two types of coordinators are selected:
  - The main coordinator that the client (or some initial node) selects from one of the target replica sets
  - And the sub coordinators that the main coordinator selects from the other target replica sets
  - The main coordinator serves as the sub coordinator for its own replica set
- The main coordinator and the sub coordinators form the "transactional replica set".

#### Fetch & Lock

- The main coordinator sends a request for target data, as well as, lock on the data to sub coordinators who in turn send the request to their replica sets.
- Locked data is marked X-SET LOCK
- The main coordinator expects the data and and an ALL lock ack from the sub coordinators
- The main coordinator determines what the latest data is
- The main coordinator applies transaction ops to the data

#### Commit

- The main coordinator sends a transaction result and update request to sub coordinators who in turn send the request to their replica sets
- Committed data is marked X-SET COMMIT
- The main coordinator expects an ALL update ack from the sub coordinators
- The main coordinator sends the transaction result to the client

#### Success Acknowledgement

- The main coordinator sends a success ack to the sub coordinators

#### Problems

- What if a replica tries to read data that is locked by some other replicas, with the locked data being the latest?:

  - Reading node checks with the lock's main coodinator for an active txn session.
  - If there is an active txn session, reading node can wait.
  - Otherwise, the reading node unwraps the lock and set that the data as latest.
  - Optionally, reading node can send updates to stale replicas.

- How does a node read data that is committed by some other replicas, with the committed data being the latest?:
  - Reading node checks with the commit's transactional replica set for an active txn session.
  - If there is an active txn session, node can wait.
  - If transactional replica set is unavailable or txn unsuccessful, the reading node rolls the data back.
  - If txn successful, the reading node unwraps the commit and set that the data as latest.
  - Optionally, reading node can send updates to stale replicas.

### Considerations

- Latest data decided on in a write operation, has to be available in at least one of the replicas in subsequent read operation, for strong consistency.
- Availability not only means being available to the client but also being available to other nodes.
- What constitutes latest is based on a logical clock.

### Potential Optimizations

- In choosing a main coordinator, you could choose a node:
  - with the least number of target records for faster txn
  - with the largest intersection of target records for stronger consistency

---

## Single Replica-Set Non-Transactional Write

### Steps

#### Coordinator Selection

- The client (or some initial node) selects a new coordinator from the replica set; ideally randomly.

#### Write

- The coordinator sends a write request to its replica set
- The coordinator expects a quorum write ack
- The coordinator sends the write result to the client

### Problems

- What if another write with the same write count for target data is issued to the same replica set?:
  - The replica set will reject the write.

---

## Last Write Wins (LWW)

- Latest data is determined by a logical clock
- Time is attached to a data down to its scalar value. (This should truly prevent lost writes for LWW methods)

---

## Self-repair

### Partial Repair on Read

- For every read request, the reading coordinator asks the replica set for their data
- Determines the latest data and checks for active locks or commit state.
- Sends data to the client
- Sends update requests to stale replicas

### Full Repair

- Old and new nodes in a replica set can be repaired fully
- They fetch the latest data from the replica set

## Other Ideas

- Ring architecture as node connection topology
- Two rings:

  - Cluster ring
  - Replica ring

- Replica selection goes around the ring clockwise
- Gossip goes around the ring clockwise
- If connection is broken, the node will try to connect to the next node in the ring
- Consistent hashing for node selection
- Partition Basis:

  - Key (Record) / Name (Database/Table)
  - Geo + Key/Name

- A replica holds the entire database it represents. So if a database is in a replica, all its tables are in the same replica.

- Node key range assignment based on partition basis
