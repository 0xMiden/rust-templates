#!/usr/bin/env bash
set -euo pipefail

# This script installs the latest released cargo-miden from crates.io
# and verifies that the default "new" project templates (fetched by
# cargo-miden from the rust-templates repo) can be scaffolded and built.

echo "=== Environment ==="
uname -a || true
rustc -V || true
cargo -V || true

echo "=== Installing released cargo-miden ==="
cargo install cargo-miden --locked --force
cargo miden -V

# Ensure cargo-miden uses default remote templates (no local override)
unset TEST_LOCAL_TEMPLATES_PATH || true

WORKDIR="$(mktemp -d)"
echo "Working directory: $WORKDIR"

cleanup() {
  rm -rf "$WORKDIR"
}
trap cleanup EXIT

echo "=== Testing templates with released cargo-miden (no --template-path) ==="

echo "\n--- Scaffolding 'test-program' (--program) ---"
(cd "$WORKDIR" && cargo miden new test-program --program)
echo "--- Building 'test-program' (debug) ---"
(cd "$WORKDIR/test-program" && cargo miden build)
echo "--- Building 'test-program' (release) ---"
(cd "$WORKDIR/test-program" && cargo miden build --release)

# note and tx-script templates require a sibling 'add-contract' account dependency
echo "\n--- Preparing shared account dependency 'add-contract' (--account) ---"
(cd "$WORKDIR" && cargo miden new add-contract --account)

echo "\n--- Scaffolding 'test-note' (--note) ---"
(cd "$WORKDIR" && cargo miden new test-note --note)
echo "--- Building 'test-note' (debug) ---"
(cd "$WORKDIR/test-note" && cargo miden build)
echo "--- Building 'test-note' (release) ---"
(cd "$WORKDIR/test-note" && cargo miden build --release)

echo "\n--- Scaffolding 'test-tx-script' (--tx-script) ---"
(cd "$WORKDIR" && cargo miden new test-tx-script --tx-script)
echo "--- Building 'test-tx-script' (debug) ---"
(cd "$WORKDIR/test-tx-script" && cargo miden build)
echo "--- Building 'test-tx-script' (release) ---"
(cd "$WORKDIR/test-tx-script" && cargo miden build --release)

echo "\n=== All templates built successfully with released cargo-miden ==="
