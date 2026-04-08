# Development Guide

## Prerequisites

- Rust 1.85+ (edition 2024)
- Python 3.13+
- maturin (`pip install maturin`)

## Quick Start

```bash
bash scripts/setup.sh
```

## Manual Setup

```bash
# Build Rust
cd rust-core && cargo build && cd ..

# Build Python extension
cd python-sdk && maturin develop && cd ..

# Run tests
just test
```

## Common Commands

```bash
just build-rust    # compile Rust
just test-rust     # run Rust tests
just dev           # build pyo3 extension
just test-py       # run Python tests
just lint          # fmt + clippy + ruff
just all           # lint + test
```

## Adding a New Feature Extractor

1. Implement `FeatureExtractor` trait in `rust-core/biospy-core/src/feat.rs`
2. Add pyo3 wrapper in `py.rs` if needed
3. Register in the default `Pipeline`
4. Add tests

## Project Layout

- `rust-core/` — Cargo workspace with core bioinformatics primitives
- `python-sdk/` — Python package with ML models and orchestration
- `tests/` — integration tests and fixtures
- `configs/` — TOML configuration files
