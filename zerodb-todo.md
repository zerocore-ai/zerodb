- [ ] Working

  - [x] Change Countdown to Timeout
  - [x] Replication Session
  - [x] zeroql: Change syntax to SQL-like
    - [x] Surrealdb syntax
    - [x] ANSI SQL syntax
    - [x] Land on a syntax that is easy to read and write
      - [x] General query syntax
      - [x] Meta syntax like create, delete, define, insert, etc.
      - [x] Text search syntax
      - [x] Regex syntax
      - [x] JSON syntax
      - [x] Vector syntax
    - [x] No procedural elements in syntax
    - [x] Figure how the language can import functions compiled to wasm
  - [x] zeroql: Reimplement the Lexer
    - [x] Change keywords
    - [x] Implement new lexer
  - [x] zeroql: Parser

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
    - [x] Combinator macros
      - [x] Single
      - [x] Alt
      - [x] Many0
      - [x] Many1
      - [x] Optional
      - [x] Sequence
      - [x] Permutation
    - [x] Complete Parser
      - [x] Operations
      - [x] Expressions
      - [x] Statements
      - [x] Program
    - [x] Support any case keyword and operators
    - [ ] Need strategy for preventing malicious code that lead to stack overflows
      - [ ] stacker::maybe_grow - grows the stack as needed or spills to heap - used by rustc https://docs.rs/stacker/latest/stacker/
      - [ ] thread isolation - running in a separate thread with set stack size and is the least intrusive
      - [ ] reblessive crate - problematic because it complicates the interface for backtrack and memoize macros
      - [ ] active prevention - setting nested expression limit, unary/binary op limit or recursion depth limit
    - [x] Implement `perm_ord` combinator for SELECT to preserve order of transforms perhaps as returned indexes, `SeqIndexX`?.
    - [x] Remove `perm` and make `perm_opt` the new `perm`
    - [ ] Rewrite combinator so that what needs to be extracted can be specified directly
      - [ ] For example
      ```
      (seq
        (arg parse_tok OpOpenParen)
        ident:parse_identifier
        idents:(many_0
          (seq (arg parse_tok OpComma) ident:parse_identifier)
        )
        (arg parse_tok OpCloseParen)
      )
      ```
      - [ ] extracted items should return as a flat tuple
      - [ ] supporting indexed and non-indexed perm macro versions
    - [ ] Move `REMOVE` expressions under statement because we don't want them in certain contexts like list constructor.
    - [ ] Introduce `REDEFINE` statements to modify schema and rename
      ```
      REDEFINE TABLE IF EXISTS table_name AS table_name_new FIELDS \
        field_name AS field_name_new,
        field_name2 TYPE int
      ```
    - [ ] Introduce `DEFINE FUNCTION` and `RETURN` statements
    - [ ] Remove `NAMESPACE`. `DATABASE` is now the new `NAMESPACE`.
    - [ ] Support `IN DATABASE` for `DEFINE DATABASE`, `DESCRIBE DATABASE`, ...
    - [ ] Change `USE` SYNTAX to `CHANGE`. Also support `CD` SYNTAX
    - [ ] Scoped Identifiers should allow `/` and `.` for database and table names.
    - [ ] Support both `ON DATABASE` and `IN DATABASE`
    - [ ] `DEFINE/DESCRIBE/REMOVE INDEX` should use just `ON TABLE` and `IN TABLE`

  - [ ] zeroql: Semantic Analysis
    - [ ] Symbol Table
    - [ ] Type checking
    - [ ] Type inference
    - [ ] Signature checking
    - [ ] ...
  - [x] zeroql: Reimplement AST
  - [ ] zerodb: Use libp2p for comms

- [x] Query Language

  - [x] Grammar
  - [x] Lexer
  - [x] Parser
  - [ ] AST
  - [ ] Semantic Analysis
  - [ ] Optimizer
  - [ ] Analyzer
  - [ ] Executor
  - [ ] Language Server: The db doubling as a language server
  - [ ] Formatter
  - [ ] Linter

- [ ] Backing Key-Value Store

  - [ ] Memstore
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

- [ ] Other Features

  - [ ] Virtual Tables
  - [ ] Reactive Queries
  - [ ] Full Text Search
  - [ ] Vector Search

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
