# Architecture

## System Overview

```
FASTQ/FASTA files
    │
    ▼
┌─────────────────┐
│  Rust Core       │  io.rs → streaming parser
│  (biospy-core)   │  seq.rs → 2-bit encoding
│                  │  kmer.rs → k-mer counting
│                  │  sketch.rs → MinHash
│                  │  bloom.rs → Bloom filter
│                  │  feat.rs → FeatureExtractor trait
│                  │  py.rs → pyo3 bindings
└────────┬────────┘
         │ pyo3/maturin (~10ns call overhead)
         ▼
┌─────────────────┐
│  Python SDK      │  core.py → Rust wrappers
│  (biospy)        │  detection.py → Round 1
│                  │  attribution.py → Round 2
│                  │  models/ → sklearn classifiers
└─────────────────┘
```

## Data Flow

1. Raw FASTQ/FASTA → streaming IO (never buffer full file)
2. Records → 2-bit encoded `Seq` objects
3. Sequences → k-mer counts, MinHash sketches, Bloom filters
4. Features extracted via `FeatureExtractor` trait pipeline
5. Feature vectors → Python ML models for classification

## Key Design Decisions

- **Streaming IO**: records processed one at a time, constant memory
- **FeatureExtractor trait**: add new extractors for each challenge round
- **pyo3 bridge**: zero-copy where possible, ~10ns per call
- **Canonical k-mers**: forward/RC min hash for strand-agnostic analysis
