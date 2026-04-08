# Rust API Reference

## Modules

### `seq` — 2-bit encoded sequences
- `Seq::from_ascii(&[u8]) -> Seq`
- `Seq::len() -> usize`
- `Seq::base_at(usize) -> u8`
- `Seq::rc() -> Seq`
- `Seq::to_ascii() -> Vec<u8>`

### `io` — streaming parsers
- `FastqReader<R: BufRead>` — `Iterator<Item=Result<FastqRecord>>`
- `FastaReader<R: BufRead>` — `Iterator<Item=Result<FastaRecord>>`
- `FastqRecord { name, seq, qual }`
- `FastaRecord { name, seq }`

### `kmer` — k-mer counting
- `KmerCounter::new(k) -> KmerCounter`
- `KmerCounter::add_seq(&[u8])`
- `KmerCounter::top_n(n) -> Vec<(u64, u32)>`
- `canonical_hash(&[u8]) -> u64`

### `sketch` — MinHash
- `MinHash::new(k, n) -> MinHash`
- `MinHash::add_seq(&[u8])`
- `MinHash::jaccard(&MinHash) -> f64`
- `MinHash::merge(&MinHash)`

### `bloom` — Bloom filter
- `BloomFilter::with_rate(expected, fp_rate) -> BloomFilter`
- `BloomFilter::insert(&[u8])`
- `BloomFilter::contains(&[u8]) -> bool`

### `feat` — feature extraction
- `trait FeatureExtractor { name(), dim(), extract(&[u8]) -> Vec<f64> }`
- `KmerFeatures`, `BasicFeatures` — built-in extractors
- `Pipeline` — concatenates multiple extractors
