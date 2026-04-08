#!/usr/bin/env bash
set -euo pipefail

echo "=== Biospy dev setup ==="

# Rust
if ! command -v cargo &>/dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Python
if ! command -v python3 &>/dev/null; then
    echo "ERROR: Python 3.13+ required"
    exit 1
fi

# Maturin + Python deps
pip install maturin numpy scikit-learn polars pytest ruff

# Build native extension
cd python-sdk && maturin develop && cd ..

# Verify
cd rust-core && cargo test && cd ..
pytest tests/python/ -v

echo "=== Setup complete ==="
