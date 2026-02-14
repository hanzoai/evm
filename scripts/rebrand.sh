#!/usr/bin/env bash
set -euo pipefail

# Hanzo EVM Rebranding Script
# Converts hanzoai/evm → hanzoai/evm (Hanzo EVM)
#
# Run from repo root: bash scripts/rebrand.sh

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_ROOT"

echo "=== Phase 1: Rename directories ==="

# Rename bin directories
if [ -d "bin/reth" ]; then
    git mv bin/evm bin/evm
    echo "  Renamed bin/evm → bin/evm"
fi
if [ -d "bin/evm-bench" ]; then
    git mv bin/evm-bench bin/evm-bench
    echo "  Renamed bin/evm-bench → bin/evm-bench"
fi
if [ -d "bin/evm-bench-compare" ]; then
    git mv bin/evm-bench-compare bin/evm-bench-compare
    echo "  Renamed bin/evm-bench-compare → bin/evm-bench-compare"
fi

# Rename crates/ethereum/evm → crates/ethereum/evm
if [ -d "crates/ethereum/reth" ]; then
    git mv crates/ethereum/evm crates/ethereum/evm
    echo "  Renamed crates/ethereum/evm → crates/ethereum/evm"
fi

echo "=== Phase 2: Bulk text replacements (longest patterns first) ==="

# Function to do sed replacement on all relevant files
do_replace() {
    local pattern="$1"
    local replacement="$2"
    local desc="$3"

    # Find all text files (exclude .git, Cargo.lock, target, assets/images)
    find . \( -name ".git" -o -name "target" -o -name "Cargo.lock" \) -prune -o \
        \( -name "*.rs" -o -name "*.toml" -o -name "*.md" -o -name "*.yml" -o -name "*.yaml" \
           -o -name "*.json" -o -name "*.sh" -o -name "*.nix" -o -name "*.hcl" \
           -o -name "*.txt" -o -name "Dockerfile*" -o -name "Makefile" -o -name "*.cfg" \
           -o -name "*.html" -o -name "*.lock" \) -type f -print0 | \
        xargs -0 sed -i '' "s|${pattern}|${replacement}|g" 2>/dev/null || true

    echo "  $desc"
}

# --- URL and org-level replacements (most specific first) ---

do_replace \
    'hanzoai\.github\.io/reth' \
    'hanzoai.github.io/evm' \
    'hanzoai.github.io/evm → hanzoai.github.io/evm'

do_replace \
    'github\.com/hanzoai/reth' \
    'github.com/hanzoai/evm' \
    'github.com/hanzoai/evm → github.com/hanzoai/evm'

do_replace \
    'hanzoai' \
    'hanzoai' \
    'hanzoai → hanzoai'

do_replace \
    'evm' \
    'evm' \
    'evm → evm (telegram etc)'

do_replace \
    'Hanzo AI' \
    'Hanzo AI' \
    'Hanzo AI → Hanzo AI'

# --- Crate/binary name replacements ---

# Replace evm-bench-compare first (longest match)
do_replace \
    'evm-bench-compare' \
    'evm-bench-compare' \
    'evm-bench-compare → evm-bench-compare'

# Replace evm-bench (but not evm-bench-compare which is already done)
do_replace \
    'evm-bench' \
    'evm-bench' \
    'evm-bench → evm-bench'

# --- Module/crate replacements with underscores (Rust module names) ---
# These need to be done before the hyphenated versions to avoid conflicts

# evm_bench_compare → evm_bench_compare
do_replace \
    'evm_bench_compare' \
    'evm_bench_compare' \
    'evm_bench_compare → evm_bench_compare'

# evm_bench → evm_bench
do_replace \
    'evm_bench' \
    'evm_bench' \
    'evm_bench → evm_bench'

echo "=== Phase 3: Bulk crate name replacements (evm- → evm-) ==="

# Replace all evm- prefixed crate names in Cargo.toml files and source
# This is the big one - handles all ~100 crates
# Using word boundaries where possible

# In Cargo.toml: package names, dependency names
find . \( -name ".git" -o -name "target" \) -prune -o \
    -name "Cargo.toml" -type f -print0 | \
    xargs -0 sed -i '' 's/evm-/evm-/g' 2>/dev/null || true
echo "  Cargo.toml: evm- → evm-"

# In .rs files: evm_ module prefixes (extern crate, use statements, etc)
find . \( -name ".git" -o -name "target" \) -prune -o \
    -name "*.rs" -type f -print0 | \
    xargs -0 sed -i '' 's/evm_/evm_/g' 2>/dev/null || true
echo "  .rs files: evm_ → evm_"

# In .md files
find . \( -name ".git" -o -name "target" \) -prune -o \
    -name "*.md" -type f -print0 | \
    xargs -0 sed -i '' 's/evm-/evm-/g; s/evm_/evm_/g' 2>/dev/null || true
echo "  .md files: evm-/evm_ → evm-/evm_"

# In yml/yaml files
find . \( -name ".git" -o -name "target" \) -prune -o \
    \( -name "*.yml" -o -name "*.yaml" \) -type f -print0 | \
    xargs -0 sed -i '' 's/evm-/evm-/g; s/evm_/evm_/g' 2>/dev/null || true
echo "  yml/yaml: evm-/evm_ → evm-/evm_"

# In Dockerfiles, Makefile, shell scripts, nix, hcl
find . \( -name ".git" -o -name "target" \) -prune -o \
    \( -name "Dockerfile*" -o -name "Makefile" -o -name "*.sh" -o -name "*.nix" -o -name "*.hcl" \) -type f -print0 | \
    xargs -0 sed -i '' 's/evm-/evm-/g; s/evm_/evm_/g' 2>/dev/null || true
echo "  Build files: evm-/evm_ → evm-/evm_"

echo "=== Phase 4: Standalone 'reth' replacements (careful - only in specific contexts) ==="

# Standalone reth as package name in Cargo.toml: name = "reth"
find . \( -name ".git" -o -name "target" \) -prune -o \
    -name "Cargo.toml" -type f -print0 | \
    xargs -0 sed -i '' 's/name = "reth"/name = "evm"/g' 2>/dev/null || true
echo '  Cargo.toml: name = "reth" → name = "evm"'

# Binary name references
find . \( -name ".git" -o -name "target" \) -prune -o \
    \( -name "Dockerfile*" -o -name "Makefile" -o -name "*.sh" -o -name "*.nix" -o -name "*.hcl" -o -name "*.yml" -o -name "*.yaml" \) -type f -print0 | \
    xargs -0 sed -i '' 's|/evm |/evm |g; s|/reth$|/evm|g' 2>/dev/null || true
echo "  Build files: /evm → /evm (binary path refs)"

echo "=== Phase 5: Display name replacements ==="

# Reth → Hanzo EVM in docs and comments
find . \( -name ".git" -o -name "target" \) -prune -o \
    -name "*.md" -type f -print0 | \
    xargs -0 sed -i '' 's/\bReth\b/Hanzo EVM/g' 2>/dev/null || true
echo "  .md files: Reth → Hanzo EVM"

# EVM_ env var prefix
find . \( -name ".git" -o -name "target" \) -prune -o \
    \( -name "*.rs" -o -name "*.toml" -o -name "*.md" -o -name "*.yml" -o -name "*.yaml" \
       -o -name "*.sh" -o -name "Dockerfile*" -o -name "Makefile" \) -type f -print0 | \
    xargs -0 sed -i '' 's/EVM_/EVM_/g' 2>/dev/null || true
echo "  All files: EVM_ → EVM_"

# evm:: tracing targets in .rs files (these didn't get caught by evm_ replacement since they use ::)
find . \( -name ".git" -o -name "target" \) -prune -o \
    -name "*.rs" -type f -print0 | \
    xargs -0 sed -i '' 's/"reth::/"evm::/g' 2>/dev/null || true
echo '  .rs files: "reth:: → "evm::'

echo "=== Phase 6: Fix over-replacements ==="

# Fix revm being corrupted (revm contains "re" + "vm", not "reth")
# Check if any revm references were broken - they shouldn't be since we only replace evm_ and evm-

# Fix book.toml title
sed -i '' 's/title = ".*"/title = "Hanzo EVM Book"/' book.toml 2>/dev/null || true

# Fix deny.toml if it has org references
sed -i '' 's|https://github\.com/hanzoai/evm|https://github.com/hanzoai/evm|g' deny.toml 2>/dev/null || true

echo "=== Phase 7: Workspace path references ==="

# The workspace Cargo.toml has path references like "bin/reth/" that need updating
sed -i '' 's|"bin/reth/"|"bin/evm/"|g' Cargo.toml
sed -i '' 's|"bin/evm-bench/"|"bin/evm-bench/"|g' Cargo.toml
sed -i '' 's|"bin/evm-bench-compare/"|"bin/evm-bench-compare/"|g' Cargo.toml
sed -i '' 's|"crates/ethereum/reth/"|"crates/ethereum/evm/"|g' Cargo.toml
echo "  Fixed workspace member paths in root Cargo.toml"

echo ""
echo "=== DONE ==="
echo "Next steps:"
echo "  1. Review changes with: git diff --stat"
echo "  2. Check for any remaining 'paradigm' references: grep -r paradigm --include='*.rs' --include='*.toml' | head"
echo "  3. Check for remaining 'reth' references that should have been replaced"
echo "  4. Run: cargo check --workspace"
