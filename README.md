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

# Development
cargo +nightly fmt --all
cargo +nightly clippy --workspace --all-features
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
