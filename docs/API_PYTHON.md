# Python API Reference

## Core (`biospy.core`)

### `count_fastq(path: str) -> int`

Count records in a FASTQ file.

- **path**: filesystem path to a `.fastq` file
- **returns**: number of records
- **raises**: `ValueError` on malformed FASTQ, `OSError` on file errors

### `read_fastq_seqs(path: str) -> list[bytes]`

Read all sequences from a FASTQ file.

- **path**: filesystem path to a `.fastq` file
- **returns**: list of raw sequence byte strings
- **raises**: `OSError` on file errors or format violations (missing `+` separator, seq/qual length mismatch)

### `read_fasta_seqs(path: str) -> list[bytes]`

Read all sequences from a FASTA file. Handles multi-line sequences.

- **path**: filesystem path to a `.fasta` file
- **returns**: list of raw sequence byte strings

### `extract_features(seq: bytes, k: int = 21, top_n: int = 50) -> list[float]`

Extract a feature vector from a single DNA sequence.

- **seq**: raw DNA sequence bytes (e.g. `b"ACGTACGT"`)
- **k**: k-mer length for frequency features (default 21)
- **top_n**: number of top k-mer frequencies to include (default 50)
- **returns**: float vector of length `3 + top_n` (3 basic features + top_n k-mer frequencies)

The 3 basic features are: sequence length, GC fraction, linguistic complexity (unique 3-mers / 64).

### `extract_features_batch(seqs: list[bytes], k: int = 21, top_n: int = 50) -> list[list[float]]`

Extract features from multiple sequences.

- **seqs**: list of raw DNA sequence byte strings
- **k**: k-mer length (default 21)
- **top_n**: number of top k-mer frequencies (default 50)
- **returns**: list of feature vectors, one per sequence

## Detection (`biospy.detection`)

### `detect_pathogens(fastq_paths, model=None, threshold=0.5, k=21, top_n=50) -> list[dict]`

Detect pathogens across one or more FASTQ files.

- **fastq_paths**: `list[str | Path]` -- paths to FASTQ files. Validated for existence.
- **model**: fitted `Detector` (or any sklearn-compatible classifier). If `None`, returns zero confidence.
- **threshold**: `float` -- confidence threshold for positive detection (default 0.5)
- **k**: `int` -- k-mer length for feature extraction (default 21)
- **top_n**: `int` -- number of top k-mer features (default 50)
- **returns**: list of dicts, one per file:

```python
{
    "path": str,        # resolved file path
    "n_reads": int,     # number of reads in the file
    "detected": bool,   # True if confidence >= threshold
    "confidence": float # mean positive-class probability across reads (0.0-1.0)
}
```

- **raises**: `FileNotFoundError` if any path doesn't exist, `ValueError` if model is not fitted

## Attribution (`biospy.attribution`)

### `attribute_origin(fastq_paths, model=None, k=21, top_n=50) -> list[dict]`

Attribute origin/lab for sequences in FASTQ files.

- **fastq_paths**: `list[str | Path]` -- paths to FASTQ files. Validated for existence.
- **model**: fitted `Attributor` (or any sklearn-compatible multi-class classifier). If `None`, returns "unknown".
- **k**: `int` -- k-mer length for feature extraction (default 21)
- **top_n**: `int` -- number of top k-mer features (default 50)
- **returns**: list of dicts, one per file:

```python
{
    "path": str,        # resolved file path
    "origin": str,      # predicted origin label (majority vote across reads)
    "confidence": float # mean max-class probability across reads (0.0-1.0)
}
```

- **raises**: `FileNotFoundError` if any path doesn't exist, `ValueError` if model is not fitted

## Models (`biospy.models`)

### `Classifier(n_estimators: int = 100, max_depth: int = 5)`

Gradient-boosted classifier wrapper around `sklearn.ensemble.GradientBoostingClassifier`.

- **n_estimators**: number of boosting stages (default 100)
- **max_depth**: max depth of individual trees (default 5)

Methods:

- `.fit(X: np.ndarray, y: np.ndarray) -> None` -- train the model. `X` is feature matrix (n_samples, n_features), `y` is label array.
- `.predict(X: np.ndarray) -> np.ndarray` -- predict labels. Raises `ValueError` if not fitted.
- `.predict_proba(X: np.ndarray) -> np.ndarray` -- predict class probabilities. Raises `ValueError` if not fitted.
- `.fitted: bool` -- whether `fit()` has been called

### `Detector`

Alias for `Classifier`. Used for binary pathogen detection (Round 1).

### `Attributor`

Alias for `Classifier`. Used for multi-class origin attribution (Round 2).

## Config (`biospy.config`)

### `load_config(name: str = "default") -> dict`

Load a TOML config file from `configs/<name>.toml`. Falls back to built-in defaults if the file doesn't exist.

- **name**: config name without extension (default "default")
- **returns**: parsed config dict

## Utils (`biospy.utils`)

### `resolve_paths(paths: list[str | Path]) -> list[Path]`

Resolve and validate input paths.

- **paths**: list of file paths (strings or Path objects)
- **returns**: list of resolved `Path` objects
- **raises**: `FileNotFoundError` if any path doesn't exist

### `chunk_list(lst: list, n: int) -> list[list]`

Split a list into chunks of size `n`.

- **lst**: input list
- **n**: chunk size
- **returns**: list of sub-lists
