AppendEntriesSession

- new session, send heartbeat to all peers
  |
- send AppendEntries to incomplete peers
  |
- send AppendEntries to incomplete peers
  |
- new session, send heartbeat to all peers (heartbeat timeout)

RequestVoteSession

- new session, send RequestVote to all peers
  |
- resend RequestVote to no-ack peers (ack timeout)
  |
- resend RequestVote to no-ack peers (ack timeout)
  |
- new session, send RequestVote to all peers (election timeout)

- [ ] Distributed Consensus

  - [ ] Raft

    - [ ] State transitions

      - [ ] Follower
        - [ ] Election timeout
        - [ ] Vote request
      - [ ] Leader
      - [ ] Candidate

    - [ ] Client comms
    - [ ] Peer RPC comms
    - [ ] Timeouts

  - [ ]

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
