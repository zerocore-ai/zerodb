
- [ ] Working
  - [x] Change Countdown to Timeout
  - [x] Replication Session
  - [ ] zeroql: Lexer


- [x] Distributed Consensus

  - [x] Raft

    - [x] State transitions

      - [x] Follower
        - [x] Election timeout
        - [x] Vote request
      - [x] Leader
      - [x] Candidate

    - [x] Client comms
    - [x] Peer RPC comms
    - [x] Timeouts

- [ ] Raft continuation

  - [ ] Use binary search with some lower bound paired with exponential backoff to find the next index to send to a follower
  - [ ] Implement Install Snapshot
  - [ ] Implement Log compaction
  - [ ] Implement Configuration changes

- [ ] Query Language

  - [ ] Grammar
  - [ ] Parser
  - [ ] AST
  - [ ] Wasm Codegen

- [ ] Backing Key-Value Store

  - [ ] Memstore
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

  - [ ] Query Executor (wasmtime)

- [ ] Provide a simple API

  - [ ] API
    - [ ] Prepare
    - [ ] Execute (Query or Prepared)
  - [ ] A basic server
  - [ ] A simple CLI
  - [ ] A basic client
    - [ ] Rust client
    - [ ] zerosys:db WIT and Guest SDK

- [ ] Tests
  - [ ] Proptests
  - [ ] Fuzzing
  - [ ] Jepsen
