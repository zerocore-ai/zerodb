- [ ] Query Language
    - [ ] Language - https://gist.github.com/appcypher/6032834df3f835b611139c56d2b5a8d3
    - [ ] Grammar
    - [ ] Lexer
    - [ ] Parser
    - [ ] AST

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

### Consistency

#### Strong Consistency via Transactions
- Isolated handling replica selection
    - Coordinator sends a transaction to one of concerned replicas randomly
    - Replica passes transaction along the replica ring all the way back to itself. Each replica setting its state to transaction handling
    - Replica selects the last replica with the latest target records to handle the transaction
    - The handling replica handles the transaction and passes the result along the replica ring back to itself
    - The handling replica sends the result back to the coordinator
    - Problems:
        - What if a replica in the replica ring is down?: The replica will try to connect to the next replica in the ring
        - What if the replica with the latest target records is down?: Lost data!
        - What if another transaction is sent to the same replica ring?: If the new transaction gets to a replica in transaction handling state, it return an error to the coordinator.
        - It is sequential and slow

- Self-repair


#### Eventual Consistency

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
