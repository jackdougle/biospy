Add relevant context to this document to get other Claude instances up to speed ASAP.
Keep comments and variable names short and a bit sparse.
Try to keep comments to one line.
---

# Biospy — DARPA Bio-Attribution Challenge

Hybrid Rust + Python repo. Rust core in `rust-core/biospy-core/` handles streaming IO, 2-bit sequence encoding, k-mer counting, MinHash, Bloom filters, and feature extraction via `FeatureExtractor` trait. Python SDK in `python-sdk/biospy/` wraps Rust via pyo3/maturin and adds sklearn models for detection (Round 1) and attribution (Round 2).

## Build

- `cd rust-core && cargo build` — Rust
- `cd python-sdk && maturin develop` — pyo3 extension
- `just all` — lint + test everything

## Key patterns

- Streaming iterator IO — never buffer more than one record
- `FeatureExtractor` trait is the main extensibility point
- Canonical k-mer hashing (min of fwd/rc) for strand-agnostic analysis
- Python has pure-python fallbacks when native extension unavailable
