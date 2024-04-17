<div align="center">
  <!-- <a href="https://github.com/zerocore-ai/zerodb" target="_blank">
    <img src="https://raw.githubusercontent.com/zerocore-ai/zerodb/main/assets/a_logo.png" alt="zerodb Logo" width="100"></img>
  </a> -->

  <h1 align="center">zerodb</h1>
<!--
  <p>
    <a href="https://crates.io/crates/zerodb">
      <img src="https://img.shields.io/crates/v/zerodb?label=crates" alt="Crate">
    </a>
    <a href="https://codecov.io/gh/zerocore-ai/zerodb">
      <img src="https://codecov.io/gh/zerocore-ai/zerodb/branch/main/graph/badge.svg?token=SOMETOKEN" alt="Code Coverage"/>
    </a>
    <a href="https://github.com/zerocore-ai/zerodb/actions?query=">
      <img src="https://github.com/zerocore-ai/zerodb/actions/workflows/tests_and_checks.yml/badge.svg" alt="Build Status">
    </a>
    <a href="https://github.com/zerocore-ai/zerodb/blob/main/LICENSE">
      <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="License">
    </a>
    <a href="https://docs.rs/zerodb">
      <img src="https://img.shields.io/static/v1?label=Docs&message=docs.rs&color=blue" alt="Docs">
    </a>
  </p> -->
</div>

**`zerodb`** is a modern multi-model database for distributed high-performance applications.

### Key Features

This project shares the same [core philosophies][key-features] as zerocore, and in addition to that, it also has these key features:

#### Multi-Model

zerodb supports multiple data models such as relational, key-value, graph, document, and vector. This flexibility allows developers to choose the most suitable model for their specific use case, without the need for multiple databases.

#### Content Addressable

Data in zerodb is stored based on its content, using a unique cryptographic hash. This content-addressable storage (CAS) ensures data integrity and immutability, facilitating efficient deduplication and integrity checks.

#### Versioning

zerodb features robust data versioning where each modification creates a new immutable version of the data, linked through hashes. This design allows for full historical traceability and simple rollback capabilities.

</br>

> [!WARNING]
> This project is in early development and is not yet ready for production use.

##

## Outline

- [Testing the Project](#testing-the-project)
- [License](#license)

## Testing the Project

- Run tests

  ```console
  cargo test
  ```

## License

This project is licensed under the [Apache License 2.0](./LICENSE), or
[http://www.apache.org/licenses/LICENSE-2.0][apache].

[key-features]: https://github.com/zerocore-ai/zerocore/edit/main/README.md#key-features
