Add relevant context to this document to get other Claude instances up to speed ASAP.
Keep comments and variable names short and a bit sparse.
Try to keep comments to one line.
---

# Biospy -- DARPA Bio-Attribution Challenge

Hybrid Rust + Python repo. Rust core in `rust-core/biospy-core/` handles streaming IO, 2-bit sequence encoding, k-mer counting, MinHash, Bloom filters, and feature extraction via `FeatureExtractor` trait. Python SDK in `python-sdk/biospy/` wraps Rust via pyo3/maturin and adds sklearn models for detection (Round 1) and attribution (Round 2).

## Build

- `cd rust-core && cargo build` -- Rust
- `cd python-sdk && maturin develop` -- pyo3 extension
- `just all` -- lint + test everything

## Test

- `cargo test --lib` -- Rust unit tests (NOT `cargo test`, pyo3 cdylib linking fails without Python)
- `pytest tests/python/ -v` -- Python tests (from repo root, after `maturin develop`)

## Key patterns

- Streaming iterator IO -- never buffer more than one record
- `FeatureExtractor` trait is the main extensibility point
- Canonical k-mer hashing (min of fwd/rc) for strand-agnostic analysis
- Fixed-seed ahash (via `RandomState::with_seeds`) for reproducible hashes
- Python has pure-python fallbacks when native extension unavailable
- `detect_pathogens` / `attribute_origin` read real sequences from FASTQ, extract per-read features, aggregate predictions
- `Detector` and `Attributor` are both aliases for `Classifier` (GradientBoostingClassifier wrapper)
- Config lives in `configs/*.toml`, loaded via `biospy.config.load_config()`
- Input paths validated via `utils.resolve_paths()` before processing
