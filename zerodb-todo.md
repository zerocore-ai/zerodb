- [ ] Query Language

  - [ ] Language - https://gist.github.com/appcypher/6032834df3f835b611139c56d2b5a8d3
  - [ ] Grammar
  - [ ] Lexer
  - [ ] Parser
  - [ ] AST

- [ ] Backing Key-Value Store

  - [ ] RocksDB
  - [ ] TiKV

- [ ] Models

  - [ ] Relational
  - [ ] Key-Value
  - [ ] Graphs
  - [ ] Document
  - [ ] Vector

- [ ] Design

  - [ ] Vertices are also records
  - [ ] Json record
  - [ ] Schemafull and schemaless support
  - [ ] Multi-tenancy with namespaces
  - [ ] Strong and eventual consistency support per namespace

- [ ] Query Engine

  - [ ] Query Planner ?
  - [ ] Query Optimizer ?
  - [ ] Query Compiler
  - [ ] Query Executor

- [ ] Provide a simple API

  - [ ] API
    - [ ] Prepare
    - [ ] Execute (Query or Prepared)
  - [ ] A basic server
  - [ ] A simple CLI
  - [ ] WIT interface
  - [ ] A basic client
    - [ ] Rust client
    - [ ] zerosystem WIT and Guest SDK

- [ ] Tests
  - [ ] Proptests
  - [ ] Fuzzing
  - [ ] Jepsen

# Design

## Transactions w/ Strongly Consistent Consensus

### Steps

#### Coordinator Selection

- The client selects a coordinator of the replica set; ideally randomly.

#### Fetch & Lock

- The coordinator sends a request for expected data, as well as, lock on the data to its replica set
- The coordinator recieves the data and lock ack from the replica set
- The coordinator expects a quorum lock ack and it also determines what the latest data is
- The coordinator applies transaction ops to the data

#### Commit

- The coordinator sends a transaction result and update request to its replica set
- Each replica in the replica set applies the transaction to the data

### Problems

- What if another transaction is issued to coordinator or replica set?:
  - If it is for the same set of data, it is rejected with relevant details
- What about deadlocks on data? How do they yield?:
  - ???
- What if majority gave the coordinator data but majority didn't take the final transaction?:

  - It doesn't matter. The majority will have locks for the data. They must fetch the latest data from those without locks on the data.

- What if majority of a replica set holding the latest data were down during txn, but came back online eventually?:
  - Well, they were not representative of the latest data at txn time.
- What if any of the replica set is down or takes long to respond?:
  - There has to be a timeout period for each participating replica expected response.

### Comparisons

- vs Paxos:
  - No promise phase
  - ...
- vs Raft:
  - No expensive leader election phase
  - No expensive heartbeat mechanism
  - No expensive state machine replication and replays

## Self-repair

- For every read request, the coordinator asks the replica set for the latest data
- Problems:
  - This may not be performant

## Design

- Ring architecture as node connection topology
- Two rings:

  - Cluster ring
  - Replica ring (used in txns)

- Replica selection goes around the ring clockwise
- Gossip goes around the ring clockwise
- If connection is broken, the node will try to connect to the next node in the ring
- Consistent hashing for node selection
- Partition Basis:

  - Key (Record) / Name (Database/Table)
  - Geo + Key/Name

- A replica holds the entire database it represents. So if a database is in a replica, all its tables are in the same replica.

- Node key range assignment based on partition basis
