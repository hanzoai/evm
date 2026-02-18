# Hanzo EVM Development Guide

This guide provides comprehensive instructions for AI agents working on the Hanzo EVM codebase.

## Project Overview

Hanzo EVM is a high-performance EVM execution engine written in Rust, forked from reth and rebranded for the Hanzo AI / Lux ecosystem. It serves as:

- **Rust EVM for hanzod**: The EVM plugin for `~/work/hanzo/node` (Rust) and `~/work/hanzo/dev` (Rust+Bazel)
- **Lux-compatible**: Works with Lux consensus via `~/work/lux/node` (Go) and `~/work/lux/evm` (Go)
- **Shared C++ acceleration**: Leverages `~/work/luxcpp/` for GPU-accelerated crypto, FHE, and AI/ML compute
- **AI-native**: Designed for GPU-accelerated EVM execution, AI/ML inference on-chain, and crypto acceleration

### Ecosystem Integration

```
hanzod (~/work/hanzo/node)    <- Rust node with EVM plugin
  |-- evm (this repo)   <- Rust EVM execution engine
  |-- luxcpp (~/work/luxcpp)  <- C++ GPU/CUDA/Metal acceleration
  +-- lux node (~/work/lux/node) <- Go consensus (Lux Snow*)

hanzo-dev (~/work/hanzo/dev)  <- Bazel-based dev environment
  +-- Uses evm as dependency
```

### Key Design Principles

- **Modularity**: Each crate can be used as a standalone library
- **Performance**: Parallelism, memory-mapped I/O, GPU acceleration via luxcpp
- **Extensibility**: Traits and generics allow different chain implementations
- **AI Acceleration**: GPU-accelerated crypto (CUDA/Metal), FHE, AI/ML inference
- **Lux Compatibility**: Plugin architecture compatible with Lux Snow* consensus

## Architecture Overview

### Core Components

1. **Consensus (`crates/consensus/`)**: Validates blocks, integrates with Lux consensus
2. **Storage (`crates/storage/`)**: Hybrid database using MDBX + static files
3. **Networking (`crates/net/`)**: P2P networking with discovery, sync, tx propagation
4. **RPC (`crates/rpc/`)**: JSON-RPC server supporting all standard Ethereum APIs
5. **Execution (`crates/evm/`, `crates/ethereum/`)**: Transaction execution and state transitions
6. **Pipeline (`crates/stages/`)**: Staged sync architecture
7. **Trie (`crates/trie/`)**: Merkle Patricia Trie with parallel state root computation
8. **Node Builder (`crates/node/`)**: High-level node orchestration
9. **Engine (`crates/engine/`)**: Consensus engine (Engine API)

## Development Workflow

### Code Style

```bash
# Format
cargo +nightly fmt --all

# Lint
cargo +nightly clippy --workspace --lib --examples --tests --benches --all-features

# Test
cargo nextest run --workspace

# Build optimized binary
cargo build --release --features "jemalloc asm-keccak"
```

### GPU/AI Acceleration Integration

The Hanzo EVM integrates with luxcpp for GPU-accelerated operations:

- **CUDA/Metal crypto**: Accelerated hashing (keccak256), signature verification
- **FHE compute**: Fully Homomorphic Encryption for private computation
- **AI/ML inference**: On-chain model inference via GPU
- **Lattice crypto**: Post-quantum cryptographic operations

These are accessed via FFI bindings to the C++ libraries in `~/work/luxcpp/`.

### Important Rules

- **ALWAYS** use `hanzoai` packages/imports, NOT `paradigmxyz` or upstream `reth`
- Crate names use `evm-*` prefix (e.g., `evm-primitives`)
- Module names use `evm_*` prefix (e.g., `evm_primitives`)
- Binary name is `evm` (not `reth`)
- Config file is `evm.toml` (not `reth.toml`)
- Data directory is `evm/` (not `reth/`)
- Environment variables use `EVM_` prefix
- GitHub org: `hanzoai` (not `paradigmxyz`)
- Repository: `github.com/hanzoai/evm`

### CI Requirements

1. **Format Check**: `cargo +nightly fmt --all --check`
2. **Clippy**: No warnings
3. **Tests Pass**: All unit and integration tests
4. **Documentation**: Update relevant docs

### Testing

```bash
# Run all tests
cargo nextest run --workspace

# Run specific crate tests
cargo nextest run -p evm-consensus

# Run benchmarks
cargo bench --bench bench_name
```

## Quick Reference

| What | Old (reth) | New (Hanzo EVM) |
|------|-----------|-----------------|
| Binary | `reth` | `evm` |
| Crate prefix | `reth-*` | `evm-*` |
| Module prefix | `reth_*` | `evm_*` |
| Config file | `reth.toml` | `evm.toml` |
| Data dir | `~/.local/share/reth/` | `~/.local/share/evm/` |
| Env prefix | `RETH_*` | `EVM_*` |
| IPC socket | `reth.ipc` | `evm.ipc` |
| GitHub | `paradigmxyz/reth` | `hanzoai/evm` |
| Org | Paradigm | Hanzo AI |
