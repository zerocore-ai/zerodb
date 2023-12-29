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


## Design

### Strong Consistency

- Isolated handling replica selection
    - We use a fan-out approach for communication with replicas

        ## Fetch & Lock
    - Main coordinator requests for target data from randomly selected replicas in replica set
    - Main coordinator waits for all replica set to respond
    - Each selected replica acts as a sub-coordinator fetching the latest data from the replica set
    - Each sub-coordinator waits for all replica set to respond
    - Each sub-coordinator sends the latest data to the coordinator
    - Main coordinator waits for all sub-coordinators to respond
    - Main coordinator applies the transaction to the latest data and requests for more as necessary using the same process

        ## Coordinated Commit
    - Main coordinator sends the transaction updates to sub-coordinators that need to commit
    - ???
    - Sub-coordinators send commit successful to main coordinator


    - Problems:
        - This may not be performant
        - What if another transaction is issued to coordinator or replica set?:
            - Once the first transaction recieved, the participating replica will enter a lock state for the set of data involved, rejecting any other transaction that involves the same data.
            - The rejected transaction initiator can wait for the lock to be released
        - What about deadlocks?:
            - ???
            - One yields to one that started first. This is achieved by:
                - ???
        - What if majority of a replica set holding the latest data were down during txn, but came back online eventually?:
            - ???
            - I guess they are not representative of the latest data anymore. So they have to fetch the latest data from the minority.
        - What if majority gave the coordinator data but majority didn't take the final transaction?:
            - It doesn't matter. The majority will be in locked state for that data. So whenever they are back online, they fetch the latest data from the minority.
            - What if one of the replica set didn't take the final transaction but others did?:
                - Same as above.
            - What if they a replica set is dead?:
                - The main coordinator cancels the transaction.
                - But what happens to the replica set with the committed transaction?:
                    - ???
        - What if any of the replica set is down or takes long to respond?:
            - There has to be a timeout period for each participating replica expected response.
    - vs Paxos:
        - Has a promise phase

- Self-repair
    - For every read request, the coordinator asks the replica set for the latest data
    - Problems:
        - This may not be performant


### Design

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

- Partition Level:
    - Database
    - Table
    - Record

- Node key range assignment based on partition basis
