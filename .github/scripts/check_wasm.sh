#!/usr/bin/env bash
set -uo pipefail

readarray -t crates < <(
  cargo metadata --format-version=1 --no-deps | jq -r '.packages[].name' | grep '^reth' | sort
)

# shellcheck disable=SC2034
exclude_crates=(
  # The following require investigation if they can be fixed
  evm-basic-payload-builder
  evm-bench
  evm-bench-compare
  evm-cli
  evm-cli-commands
  evm-cli-runner
  evm-consensus-debug-client
  evm-db-common
  evm-discv4
  evm-discv5
  evm-dns-discovery
  evm-downloaders
  evm-e2e-test-utils
  evm-engine-service
  evm-engine-tree
  evm-engine-util
  evm-eth-wire
  evm-ethereum-cli
  evm-ethereum-payload-builder
  evm-etl
  evm-exex
  evm-exex-test-utils
  evm-ipc
  evm-net-nat
  evm-network
  evm-node-api
  evm-node-builder
  evm-node-core
  evm-node-ethereum
  evm-node-events
  evm-node-metrics
  evm-rpc
  evm-rpc-api
  evm-rpc-api-testing-util
  evm-rpc-builder
  evm-rpc-convert
  evm-rpc-e2e-tests
  evm-rpc-engine-api
  evm-rpc-eth-api
  evm-rpc-eth-types
  evm-rpc-layer
  evm-stages
  evm-engine-local
  evm-ress-protocol
  evm-ress-provider
  # The following are not supposed to be working
  reth # all of the crates below
  reth-storage-rpc-provider
  reth-invalid-block-hooks # reth-provider
  reth-libmdbx # mdbx
  reth-mdbx-sys # mdbx
  reth-payload-builder # reth-metrics
  reth-provider # tokio
  reth-prune # tokio
  reth-prune-static-files # reth-provider
  reth-tasks # tokio rt-multi-thread
  reth-stages-api # reth-provider, reth-prune
  reth-static-file # tokio
  reth-transaction-pool # c-kzg
  reth-payload-util # reth-transaction-pool
  reth-trie-parallel # tokio
  reth-trie-sparse-parallel # rayon
  reth-testing-utils
  reth-era-downloader # tokio
  reth-era-utils # tokio
  reth-tracing-otlp
  reth-node-ethstats
)

any_failed=0
tmpdir=$(mktemp -d 2>/dev/null || mktemp -d -t evm-check)
trap 'rm -rf -- "$tmpdir"' EXIT INT TERM

contains() {
  local array="$1[@]"
  local seeking="$2"
  local element
  for element in "${!array}"; do
    [[ "$element" == "$seeking" ]] && return 0
  done
  return 1
}

for crate in "${crates[@]}"; do
  if contains exclude_crates "$crate"; then
    echo "⏭️ $crate"
    continue
  fi

  outfile="$tmpdir/$crate.log"
  if cargo +stable build -p "$crate" --target wasm32-wasip1 --no-default-features --color never >"$outfile" 2>&1; then
    echo "✅ $crate"
  else
    echo "❌ $crate"
    sed 's/^/   /' "$outfile"
    echo ""
    any_failed=1
  fi
done

exit $any_failed
