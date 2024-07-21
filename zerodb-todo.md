- [ ] Working

  - [x] Change Countdown to Timeout
  - [x] Replication Session
  - [ ] zeroql: Change syntax to SQL-like
    - [ ] Surrealdb syntax
    - [ ] ANSI SQL syntax
    - [ ] Land on a syntax that is easy to read and write
      - [ ] General query syntax
      - [ ] Meta syntax like create, delete, define, insert, etc.
      - [ ] Text search syntax
      - [ ] Regex syntax
      - [ ] JSON syntax
      - [ ] Vector syntax
    - [ ] No procedural elements in syntax
    - [ ] Figure how the language can import functions compiled to wasm
  - [x] zeroql: Reimplement the Lexer
    - [ ] Change keywords
    - [ ] Implement new lexer
  - [ ] zeroql: Parser

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

  - [ ] zeroql: Reimplement AST
  - [ ] zeroql: Semantic Analysis
    - [ ] Symbol Table
    - [ ] Type checking
    - [ ] Type inference
    - [ ] Signature checking
    - [ ] ...
  - [ ] zerodb: Use libp2p for comms

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

- [ ] Query Language

  - [x] Grammar
  - [ ] Lexer
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
  - [ ] list
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
