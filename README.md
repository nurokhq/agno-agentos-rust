# agno-agentos-client

[![Build](https://github.com/nurokhq/agno-agentos-rust/actions/workflows/build.yml/badge.svg)](https://github.com/nurokhq/agno-agentos-rust/actions/workflows/build.yml)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org/)
[![Code style: rustfmt](https://img.shields.io/badge/code%20style-rustfmt-000000.svg)](https://github.com/rust-lang/rustfmt)
[![Linter: clippy](https://img.shields.io/badge/linter-clippy-blue.svg)](https://github.com/rust-lang/rust-clippy)

Rust API Client SDK for [Agno AgentOS API](https://docs.agno.com/reference-api/overview)

## Overview

This is an auto-generated Rust client library for the Agno AgentOS API. It provides type-safe bindings for interacting with the AgentOS platform, which manages AI agents, teams, workflows, sessions, and more.

## Features

- **Type-safe API client**: Auto-generated from OpenAPI specification
- **Async/await support**: Built on `reqwest` with async/await
- **Flexible TLS backends**: Choose between `rustls-tls` (default) or `native-tls`
- **Comprehensive API coverage**: Full support for all AgentOS endpoints including:
  - Agent management and execution
  - Team and workflow orchestration
  - Session management
  - Memory and knowledge base operations
  - Configuration management

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
agno-agentos-client = { version = "0.1.0", features = ["rustls-tls"] }
```

Or with native TLS:

```toml
[dependencies]
agno-agentos-client = { version = "0.1.0", features = ["native-tls"] }
```

## Version Compatibility

The following table shows the version correspondence between the client and AgnoOS API:

| Client Version | AgnoOS API Version |
|----------------|-------------------|
| 0.1.0          | 2.3.10             |

Please ensure you use a compatible client version for your AgnoOS API version to avoid compatibility issues.

## Usage

```rust
use agno_agentos_client::apis::*;
use agno_agentos_client::models::*;
...
```

## Features

- `default`: Enables `rustls-tls` feature
- `rustls-tls`: Use rustls for TLS (default, recommended)
- `native-tls`: Use native TLS implementation

## Development

This SDK is generated from the OpenAPI specification using [OpenAPI Generator](https://openapi-generator.tech).

To regenerate the SDK:

```bash
./scripts/generate_api_sdk.sh
```

This script:
1. Generates the SDK from `api/agno-agentos-openapi.json`
2. Copies the generated code to `src/generated/`
3. Formats the code with `cargo fmt`

## Requirements

- Rust 1.70 or later
- OpenAPI Generator CLI (for regeneration)

## License

See [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please ensure that:
- Code is formatted with `cargo fmt`
- All tests pass with `cargo test`
- Clippy checks pass with `cargo clippy`
