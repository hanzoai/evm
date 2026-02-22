# Hanzo EVM

[![CI status](https://github.com/hanzoai/evm/workflows/unit/badge.svg)][gh-ci]
[![cargo-lint status](https://github.com/hanzoai/evm/actions/workflows/lint.yml/badge.svg)][gh-lint]

**High-performance Rust EVM execution engine for the Hanzo AI / Lux ecosystem**

[gh-ci]: https://github.com/hanzoai/evm/actions/workflows/unit.yml
[gh-lint]: https://github.com/hanzoai/evm/actions/workflows/lint.yml

## What is Hanzo EVM?

Hanzo EVM is a modular, high-performance Ethereum Virtual Machine execution engine written in Rust. It serves as the EVM plugin for the Hanzo node (`hanzod`) and is compatible with the Lux blockchain consensus layer.

Originally forked from [reth](https://github.com/paradigmxyz/reth), Hanzo EVM extends the base with:

- **GPU-accelerated cryptography** via CUDA/Metal (keccak256, signature verification)
- **AI/ML on-chain inference** through GPU compute integration
- **Post-quantum cryptography** via lattice-based schemes
- **FHE (Fully Homomorphic Encryption)** for private computation
- **Lux consensus compatibility** for the Lux Snow* protocol family

Built and maintained by [Hanzo AI](https://hanzo.ai/) (Techstars '17).

## Ecosystem

```
Hanzo Node (hanzod)           Lux Node
  |-- evm (Rust)          |-- lux-evm (Go)
  |-- hanzo-dev (Bazel)         |-- lux-cli
  +-- luxcpp (C++ accel)        +-- luxcpp (shared)
```

| Component | Language | Purpose |
|-----------|----------|---------|
| `evm` (this) | Rust | EVM execution engine |
| `lux-evm` | Go | Go EVM for Lux node |
| `luxcpp` | C++ | GPU/CUDA/Metal/FHE acceleration |
| `hanzod` | Rust | Hanzo node with EVM plugin |
| `lux-node` | Go | Lux consensus node |

## Goals

1. **Modularity**: Every component is a standalone library crate
2. **Performance**: Parallelism, MDBX, GPU acceleration, optimized data structures
3. **AI-Native**: GPU-accelerated crypto and on-chain AI/ML inference
4. **Lux Compatible**: Plugin architecture for Lux Snow* consensus
5. **Post-Quantum Ready**: Lattice-based cryptography via luxcpp
6. **Open Source**: Apache/MIT dual-licensed

## Quick Start

```bash
# Build
cargo build --release --features "jemalloc asm-keccak"

# Run
./target/release/evm node

Reth is production ready, and suitable for usage in mission-critical environments such as staking or high-uptime services. We also actively recommend professional node operators to switch to Reth in production for performance and cost reasons in use cases where high performance with great margins is required such as RPC, MEV, Indexing, Simulations, and P2P activities.

More historical context below:

- We released 1.0 "production-ready" stable Reth in June 2024.
  - Reth completed an audit with [Sigma Prime](https://sigmaprime.io/), the developers of [Lighthouse](https://github.com/sigp/lighthouse), the Rust Consensus Layer implementation. Find it [here](./audit/sigma_prime_audit_v2.pdf).
  - Revm (the EVM used in Reth) underwent an audit with [Guido Vranken](https://x.com/guidovranken) (#1 [Ethereum Bug Bounty](https://ethereum.org/en/bug-bounty)). We will publish the results soon.
- We released multiple iterative beta versions, up to [beta.9](https://github.com/paradigmxyz/reth/releases/tag/v0.2.0-beta.9) on Monday June 3, 2024, the last beta release.
- We released [beta](https://github.com/paradigmxyz/reth/releases/tag/v0.2.0-beta.1) on Monday March 4, 2024, our first breaking change to the database model, providing faster query speed, smaller database footprint, and allowing "history" to be mounted on separate drives.
- We shipped iterative improvements until the last alpha release on February 28, 2024, [0.1.0-alpha.21](https://github.com/paradigmxyz/reth/releases/tag/v0.1.0-alpha.21).
- We [initially announced](https://www.paradigm.xyz/2023/06/reth-alpha) [0.1.0-alpha.1](https://github.com/paradigmxyz/reth/releases/tag/v0.1.0-alpha.1) on June 20, 2023.

### Database compatibility

We do not have any breaking database changes since beta.1, and we do not plan any in the near future.

Reth [v0.2.0-beta.1](https://github.com/paradigmxyz/reth/releases/tag/v0.2.0-beta.1) includes
a [set of breaking database changes](https://github.com/paradigmxyz/reth/pull/5191) that makes it impossible to use database files produced by earlier versions.

If you had a database produced by alpha versions of Reth, you need to drop it with `reth db drop`
(using the same arguments such as `--config` or `--datadir` that you passed to `reth node`), and resync using the same `reth node` command you've used before.

## For Users

See the [Reth documentation](https://reth.rs/) for instructions on how to install and run Reth.

## For Developers

### Using reth as a library

You can use individual crates of reth in your project.

The crate docs can be found [here](https://reth.rs/docs/).

For a general overview of the crates, see [Project Layout](./docs/repo/layout.md).

### Contributing

If you want to contribute, or follow along with contributor discussion, you can use our [main telegram](https://t.me/paradigm_reth) to chat with us about the development of Reth!

- Our contributor guidelines can be found in [`CONTRIBUTING.md`](./CONTRIBUTING.md).
- See our [contributor docs](./docs) for more information on the project. A good starting point is [Project Layout](./docs/repo/layout.md).

### Building and testing

<!--
When updating this, also update:
- Cargo.toml
- .github/workflows/lint.yml
-->

The Minimum Supported Rust Version (MSRV) of this project is [1.93.0](https://blog.rust-lang.org/2026/01/22/Rust-1.93.0/).

See the docs for detailed instructions on how to [build from source](https://reth.rs/installation/source/).

To fully test Reth, you will need to have [Geth installed](https://geth.ethereum.org/docs/getting-started/installing-geth), but it is possible to run a subset of tests without Geth.

First, clone the repository:

```sh
git clone https://github.com/paradigmxyz/reth
cd reth
```

Next, run the tests:

```sh
cargo nextest run --workspace
```

## Configuration

- Config file: `evm.toml`
- Data directory: `~/.local/share/evm/<network>/`
- IPC socket: `evm.ipc`
- Environment prefix: `EVM_*`

## Architecture

### Core Components

| Component | Path | Description |
|-----------|------|-------------|
| Consensus | `crates/consensus/` | Block validation, Lux integration |
| Storage | `crates/storage/` | MDBX + static files hybrid DB |
| Networking | `crates/net/` | P2P stack (discv4/v5, eth-wire) |
| RPC | `crates/rpc/` | JSON-RPC server (all Ethereum APIs) |
| EVM | `crates/evm/` | Transaction execution |
| Pipeline | `crates/stages/` | Staged sync architecture |
| Trie | `crates/trie/` | Parallel Merkle Patricia Trie |
| Node | `crates/node/` | Node orchestration |
| Engine | `crates/engine/` | Consensus Engine API |

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

## Security

Please report vulnerabilities to [security@hanzo.ai](mailto:security@hanzo.ai).
