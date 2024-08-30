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

**`zerodb`** is a modern multi-model local-first database for distributed high-performance applications.

### Key Features

This project shares the same [core philosophies][key-features] as zerocore, and in addition to that, it is also:

#### Multi-Model

`zerodb` supports multiple data models including relational, key-value, graph, document, and vector. This allows developers to select the most suitable model for their specific use case, eliminating the need to use multiple databases.

#### Local-First Partition

This feature enables data to be stored locally, reducing reliance on network requests. It is particularly beneficial for mobile and edge devices where network connectivity may be limited, ensuring better performance and offline access.

#### Developer Experience

`zerodb` enhances productivity with an interactive shell, Language Server Protocol (LSP) server, and formatter, providing a comprehensive set of tools for a seamless development experience.

</br>

> [!WARNING]
> This project is in early development and is not yet ready for production use.

##

## Outline

- [Acknowledgments](#acknowledgments)
- [License](#license)

## Acknowledgments

The query language is inspired by the [SurrealQL][surrealql] syntax.

## License

This project is licensed under the [Apache License 2.0](./LICENSE), or
[http://www.apache.org/licenses/LICENSE-2.0][apache].

[key-features]: https://github.com/zerocore-ai/zerocore/tree/main?tab=readme-ov-file#key-features
[surrealql]: https://github.com/surrealdb/surrealdb
[apache]: http://www.apache.org/licenses/LICENSE-2.0
