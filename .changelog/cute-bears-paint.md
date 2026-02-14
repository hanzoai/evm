---
reth: minor
evm-cli-commands: minor
evm-e2e-test-utils: minor
evm-ethereum-cli: minor
evm-node-core: minor
evm-optimism-bin: minor
evm-optimism-cli: minor
evm-prune: patch
evm-stages: patch
evm-storage-api: minor
evm-storage-db-api: minor
evm-storage-db-common: patch
evm-storage-provider: patch
---

Introduced `--storage.v2` flag to control storage mode defaults, replacing the `edge` feature flag with `rocksdb` feature. The new flag enables v2 storage settings (static files + RocksDB routing) while individual `--static-files.*` and `--rocksdb.*` flags can still override defaults. Updated feature gates from `edge` to `rocksdb` across all affected crates.
