# Architecture

## System Overview

```
FASTQ/FASTA files
    |
    v
+-------------------+
|  Rust Core         |  io.rs     -> streaming FASTQ/FASTA parser (validates format)
|  (biospy-core)     |  seq.rs    -> 2-bit encoding, ambiguity detection
|                    |  kmer.rs   -> canonical k-mer counting (fixed-seed ahash)
|                    |  sketch.rs -> MinHash via BinaryHeap + AHashSet
|                    |  bloom.rs  -> Bloom filter (double-hashing scheme)
|                    |  feat.rs   -> FeatureExtractor trait + Pipeline
|                    |  py.rs     -> pyo3 bindings
+--------+----------+
         | pyo3/maturin
         v
+-------------------+
|  Python SDK        |  core.py        -> Rust wrappers + pure-python fallbacks
|  (biospy)          |  detection.py   -> Round 1: pathogen detection
|                    |  attribution.py -> Round 2: origin attribution
|                    |  config.py      -> TOML config loading
|                    |  utils.py       -> path validation, chunking
|                    |  models/        -> sklearn GradientBoosting classifiers
+-------------------+
```

## Data Flow

1. Raw FASTQ/FASTA -> streaming IO parser (one record at a time, validates format)
2. Sequences read in bulk via `read_fastq_seqs` / `read_fasta_seqs`
3. Per-sequence feature extraction via `FeatureExtractor` pipeline:
   - `BasicFeatures`: sequence length, GC fraction, linguistic complexity
   - `KmerFeatures`: top-N canonical k-mer frequencies (normalized by total k-mer count)
4. Feature matrix -> Python sklearn classifiers (detection or attribution)
5. Per-read predictions aggregated into per-file results

## Key Design Decisions

- **Streaming IO**: FASTQ parser validates `+` separator and seq/qual length match
- **FeatureExtractor trait**: add new extractors for each challenge round
- **Canonical k-mers**: min(hash(fwd), hash(rc)) for strand-agnostic analysis
- **Fixed-seed hashing**: deterministic ahash seeds for reproducible results across runs
- **Pure-python fallbacks**: every Rust binding has a Python fallback when the native extension is unavailable
- **MinHash**: O(m log n) insertion via BinaryHeap (replaced original O(n^2) sorted-vec approach)
- **Bloom filter**: double-hashing scheme for independent hash functions
