# Rust API Reference

## Modules

### `seq` -- 2-bit encoded sequences

- `Seq::from_ascii(bases: &[u8]) -> Seq` -- encode ASCII DNA into 2-bit. Ambiguous bases (N etc.) become A.
- `Seq::len() -> usize` -- number of bases
- `Seq::is_empty() -> bool`
- `Seq::base_at(i: usize) -> u8` -- ASCII base at position `i`. Panics if out of bounds.
- `Seq::rc() -> Seq` -- reverse complement
- `Seq::to_ascii() -> Vec<u8>` -- decode back to ASCII bytes
- `is_valid_base(b: u8) -> bool` -- true if `b` is A/C/G/T (case-insensitive)
- `count_ambiguous(seq: &[u8]) -> usize` -- count non-ACGT bases in a byte slice

### `io` -- streaming parsers

- `FastqReader<R: BufRead>` -- `Iterator<Item=Result<FastqRecord>>`
  - Validates `@` header prefix, `+` separator, and seq/qual length match
- `FastaReader<R: BufRead>` -- `Iterator<Item=Result<FastaRecord>>`
  - Handles multi-line sequences, validates `>` header prefix
- `FastqRecord { name: String, seq: Vec<u8>, qual: Vec<u8> }`
- `FastaRecord { name: String, seq: Vec<u8> }`
- `read_fastq_seqs(path: &str) -> io::Result<Vec<Vec<u8>>>` -- read all sequences from a FASTQ file
- `read_fasta_seqs(path: &str) -> io::Result<Vec<Vec<u8>>>` -- read all sequences from a FASTA file

### `kmer` -- k-mer counting

- `KmerCounter::new(k: usize) -> KmerCounter` -- create counter for k-mers of length `k`
- `KmerCounter::add_seq(seq: &[u8])` -- count all k-mers in `seq`
- `KmerCounter::top_n(n: usize) -> Vec<(u64, u32)>` -- top `n` k-mers by count, as (hash, count) pairs
- `KmerCounter::total_count() -> u64` -- total k-mers added (including duplicates)
- `KmerCounter::len() -> usize` -- number of distinct k-mers
- `KmerCounter::is_empty() -> bool`
- `KmerCounter::get(hash: u64) -> u32` -- count for a specific hash
- `KmerCounter::counts() -> &AHashMap<u64, u32>` -- all counts
- `canonical_hash(kmer: &[u8]) -> u64` -- min(hash(fwd), hash(rc)). Uses stack buffer for k <= 64, heap allocation for larger.

### `sketch` -- MinHash

- `MinHash::new(k: usize, n: usize) -> MinHash` -- sketch with k-mer size `k` and sketch size `n`
- `MinHash::add_seq(seq: &[u8])` -- add all k-mers from sequence
- `MinHash::jaccard(other: &MinHash) -> f64` -- estimated Jaccard similarity (0.0 to 1.0)
- `MinHash::merge(other: &MinHash)` -- merge another sketch into this one
- `MinHash::sketch() -> Vec<u64>` -- sorted sketch hash values
- `MinHash::len() -> usize` -- current sketch size
- `MinHash::is_empty() -> bool`

### `bloom` -- Bloom filter

- `BloomFilter::with_rate(expected: usize, fp_rate: f64) -> BloomFilter` -- auto-sized for `expected` items at target false-positive rate
- `BloomFilter::new(nbits: usize, nhash: usize) -> BloomFilter` -- manual sizing
- `BloomFilter::insert(item: &[u8])` -- add an item
- `BloomFilter::contains(item: &[u8]) -> bool` -- probabilistic membership test

### `feat` -- feature extraction

- `trait FeatureExtractor: Send + Sync`
  - `name() -> &str` -- extractor identifier
  - `dim() -> usize` -- output vector dimension
  - `extract(seq: &[u8]) -> Vec<f64>` -- compute features from raw sequence bytes
- `BasicFeatures` -- outputs `[length, gc_fraction, complexity]` (dim = 3)
- `KmerFeatures::new(k: usize, top_n: usize)` -- outputs top-N k-mer frequencies normalized by total count (dim = top_n)
- `Pipeline::new() -> Pipeline` -- empty pipeline
- `Pipeline::add(ext: Box<dyn FeatureExtractor>) -> Pipeline` -- builder: append an extractor
- `Pipeline::dim() -> usize` -- total output dimension (sum of all extractors)
- `Pipeline::extract(seq: &[u8]) -> Vec<f64>` -- concatenated feature vector
- `Pipeline::names() -> Vec<&str>` -- extractor names in order

### `py` -- pyo3 bindings

Exposed to Python as `biospy.biospy_core`:

- `PyKmerCounter(k: int)` -- k-mer counter class
  - `.add_seq(seq: bytes)`, `.top_n(n: int) -> list[tuple[int, int]]`, `.len() -> int`
- `PyMinHash(k: int, n: int)` -- MinHash sketch class
  - `.add_seq(seq: bytes)`, `.jaccard(other: PyMinHash) -> float`
- `count_fastq(path: str) -> int`
- `extract_features(seq: bytes, k: int, top_n: int) -> list[float]`
- `extract_features_batch(seqs: list[bytes], k: int, top_n: int) -> list[list[float]]`
- `read_fastq_seqs(path: str) -> list[bytes]`
- `read_fasta_seqs(path: str) -> list[bytes]`
- `count_ambiguous(seq: bytes) -> int`
