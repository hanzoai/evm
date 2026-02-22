#!/usr/bin/env bash
set -uo pipefail

crates_to_check=(
    evm-codecs-derive
    evm-primitives
    evm-primitives-traits
    evm-network-peers
    evm-trie-common
    evm-trie-sparse
    evm-chainspec
    evm-consensus
    evm-consensus-common
    evm-prune-types
    evm-static-file-types
    evm-storage-errors
    evm-execution-errors
    evm-errors
    evm-execution-types
    evm-db-models
    evm-evm
    evm-revm
    evm-storage-api

    ## ethereum
    reth-evm-ethereum
    reth-ethereum-forks
    reth-ethereum-primitives
    reth-ethereum-consensus
)

any_failed=0
tmpdir=$(mktemp -d 2>/dev/null || mktemp -d -t evm-check)
trap 'rm -rf -- "$tmpdir"' EXIT INT TERM

for crate in "${crates_to_check[@]}"; do
  outfile="$tmpdir/$crate.log"
  if cargo +stable build -p "$crate" --target riscv32imac-unknown-none-elf --no-default-features --color never >"$outfile" 2>&1; then
    echo "✅ $crate"
  else
    echo "❌ $crate"
    sed 's/^/   /' "$outfile"
    echo ""
    any_failed=1
  fi
done

exit $any_failed
