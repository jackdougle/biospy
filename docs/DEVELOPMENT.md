# Development Guide

## Prerequisites

- Rust 1.85+ (edition 2024)
- Python 3.11+
- maturin (`pip install maturin`)

## Quick Start

```bash
# one-liner setup
bash scripts/setup.sh

# or manually:
pip install maturin numpy scikit-learn pytest ruff
cd python-sdk && maturin develop && cd ..
```

## Build Commands

```bash
just build-rust    # compile Rust core
just test-rust     # run Rust unit tests
just dev           # build pyo3 extension into current Python env
just test-py       # run Python tests
just lint          # fmt + clippy + ruff
just all           # lint + test everything
```

## Running Tests

Rust (unit tests only, avoids pyo3 linker issues):

```bash
cd rust-core && cargo test --lib
```

Python (requires the native extension to be built first):

```bash
cd python-sdk && maturin develop && cd ..
pytest tests/python/ -v
```

## Tutorial: Detect Pathogens in a FASTQ File

```python
import numpy as np
from biospy import detect_pathogens, extract_features_batch, read_fastq_seqs
from biospy.models import Detector

# 1. prepare training data
train_seqs = read_fastq_seqs("train.fastq")
X_train = np.array(extract_features_batch(train_seqs, k=21, top_n=50))
y_train = np.array([0, 0, 1, 1, ...])  # 0 = benign, 1 = pathogen

# 2. train a detector
det = Detector(n_estimators=100, max_depth=5)
det.fit(X_train, y_train)

# 3. run detection on new files
results = detect_pathogens(
    ["sample_a.fastq", "sample_b.fastq"],
    model=det,
    threshold=0.5,
    k=21,
    top_n=50,
)
for r in results:
    print(f"{r['path']}: detected={r['detected']} confidence={r['confidence']:.3f}")
```

## Tutorial: Attribute Origin of Engineered Sequences

```python
import numpy as np
from biospy import attribute_origin, extract_features_batch, read_fastq_seqs
from biospy.models import Attributor

# 1. prepare training data with lab labels
train_seqs = read_fastq_seqs("labeled.fastq")
X_train = np.array(extract_features_batch(train_seqs, k=21, top_n=50))
y_train = np.array(["lab_a", "lab_b", "lab_c", ...])

# 2. train an attributor
attr = Attributor(n_estimators=100, max_depth=5)
attr.fit(X_train, y_train)

# 3. run attribution
results = attribute_origin(
    ["unknown_sample.fastq"],
    model=attr,
    k=21,
    top_n=50,
)
for r in results:
    print(f"{r['path']}: origin={r['origin']} confidence={r['confidence']:.3f}")
```

## Tutorial: Low-Level Feature Extraction

```python
from biospy.core import extract_features, read_fasta_seqs

# single sequence
feats = extract_features(b"ACGTACGTACGTACGT", k=5, top_n=10)
# feats = [16.0, 0.5, 0.34, 0.12, 0.09, ...]
#          ^len  ^gc  ^complexity  ^top k-mer freqs...

# batch from file
seqs = read_fasta_seqs("sequences.fasta")
from biospy.core import extract_features_batch
feat_matrix = extract_features_batch(seqs, k=21, top_n=50)
```

## Adding a New Feature Extractor

1. Implement `FeatureExtractor` trait in `rust-core/biospy-core/src/feat.rs`:

```rust
pub struct MyFeatures { /* config fields */ }

impl FeatureExtractor for MyFeatures {
    fn name(&self) -> &str { "my_features" }
    fn dim(&self) -> usize { /* output size */ }
    fn extract(&self, seq: &[u8]) -> Vec<f64> { /* compute features */ }
}
```

2. Add pyo3 wrapper in `py.rs` if it needs direct Python access.
3. Add to the default `Pipeline` in `py.rs::extract_features` if it should be included by default.
4. Add tests.

## Configuration

Config files live in `configs/`. Load them in Python:

```python
from biospy.config import load_config

cfg = load_config("default")  # loads configs/default.toml
k = cfg["kmer"]["k"]          # 21
threshold = cfg["detection"]["threshold"]  # 0.5
```

Available config keys (see `configs/default.toml`):

| Section       | Key              | Default     | Description                          |
|---------------|------------------|-------------|--------------------------------------|
| kmer          | k                | 21          | k-mer length                         |
| sketch        | n                | 1000        | MinHash sketch size                  |
| bloom         | expected_items   | 1,000,000   | expected items for Bloom filter      |
| bloom         | fp_rate          | 0.001       | target false-positive rate           |
| detection     | threshold        | 0.5         | detection confidence threshold       |
| detection     | top_n_features   | 50          | number of k-mer features             |
| attribution   | top_n_features   | 50          | number of k-mer features             |

## Project Layout

```
rust-core/              Cargo workspace
  biospy-core/          core bioinformatics primitives
    src/
      lib.rs            module declarations
      seq.rs            2-bit encoding, ambiguity detection
      io.rs             streaming FASTQ/FASTA parsers
      kmer.rs           canonical k-mer counting
      sketch.rs         MinHash sketching
      bloom.rs          Bloom filter
      feat.rs           FeatureExtractor trait + Pipeline
      py.rs             pyo3 bindings
python-sdk/             Python package
  biospy/
    __init__.py         public API exports
    core.py             Rust wrappers + pure-python fallbacks
    detection.py        Round 1: pathogen detection
    attribution.py      Round 2: origin attribution
    config.py           TOML config loading
    utils.py            path validation, list chunking
    models/
      classifier.py     shared GradientBoosting wrapper
      detector.py       Detector alias
      attributor.py     Attributor alias
tests/                  test suites + fixtures
configs/                TOML config files
scripts/                dev setup + benchmarking
```
