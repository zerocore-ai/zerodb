- [ ] Working

  - [x] Change Countdown to Timeout
  - [x] Replication Session
  - [x] zeroql: Lexer
  - [ ] zeroql: Parser
  - [x] zeroql: Reimplement the Lexer
    - [x] `memoize` macro
      - [x] Support use as impl attribute.
      - [x] Change value back to owned
      - [x] Provided cache should be <AnyKey, V>.
      - [x] Get rid of sha3.
      - [x] Change "key_extension" to "salt".
      - [x] HashInput is compiler output.
      - [x] Add better docs. Add note here that we are removing self. And fix implementation to reflect that.
      - [x] Arguments have to implement Hash?
      - [x] Add tests
      - [x] Add example in doc. Borrow from tests.
    - [x] `backtrack` macro
      - [x] Support use as impl attribute.
      - [x] Add tests
      - [x] Add example in doc. Borrow from tests.
    - [ ] Implement parser functions
      - [ ] `parse_ident`
      - [ ] Remove infix notation and rely on pipe operator. Infix notation is hard to read when combined with command notation.
  - [ ] zeroql: AST
  - [ ] zeroql: Semantic Analysis
    - [ ] Symbol Table
    - [ ] Type checking
    - [ ] Type inference
    - [ ] Signature checking
    - [ ] ...

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

- [ ] Replica Set

  - [ ] Each table has a keyspace.
  - [ ] The keyspace of a table is partitioned and stored in replica sets.
  - [ ] The replica set is a group of replicas that store the same data.
  - [ ] A node can be a member of multiple replica sets.

- [ ] Key space

  - [ ] A keyspace can be based on region, time, or any other criteria

- [ ] Query Language

  - [x] Grammar
  - [ ] Parser
  - [ ] AST
  - [ ] Semantic Analysis
  - [ ] Wasm Codegen
  - [ ] Language Server: The db doubling as a language server
  - [ ] Formatter
  - [ ] Linter

- [ ] Backing Key-Value Store

  - [x] Memstore
  - [ ] RocksDB

- [ ] Data Types
  - [ ] float
  - [ ] int
  - [ ] u8, u16, u32, u64, u128
  - [ ] i8, i16, i32, i64, i128
  - [ ] decimal
  - [ ] string
  - [ ] bool
  - [ ] datetime
  - [ ] uuid
  - [ ] array
  - [ ] vector - vec![]
  - [ ] stream
  - [ ] object
  - [ ] hashset
  - [ ] hashmap
  - [ ] tuple
  - [ ] option
  - [ ] result

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
