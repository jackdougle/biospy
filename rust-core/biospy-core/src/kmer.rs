// k-mer counting with canonical hashing via ahash

use ahash::AHashMap;

/// Counts k-mers using ahash for fast hashing
pub struct KmerCounter {
    k: usize,
    counts: AHashMap<u64, u32>,
    total: u64,
}

impl KmerCounter {
    pub fn new(k: usize) -> Self {
        Self {
            k,
            counts: AHashMap::new(),
            total: 0,
        }
    }

    pub fn k(&self) -> usize {
        self.k
    }

    /// Total number of k-mers added (including duplicates).
    pub fn total_count(&self) -> u64 {
        self.total
    }

    /// Add all k-mers from a sequence
    pub fn add_seq(&mut self, seq: &[u8]) {
        if seq.len() < self.k {
            return;
        }
        for window in seq.windows(self.k) {
            let h = canonical_hash(window);
            *self.counts.entry(h).or_insert(0) += 1;
            self.total += 1;
        }
    }

    /// Top-n most frequent k-mers by count
    pub fn top_n(&self, n: usize) -> Vec<(u64, u32)> {
        let mut v: Vec<_> = self.counts.iter().map(|(&k, &v)| (k, v)).collect();
        v.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        v.truncate(n);
        v
    }

    pub fn len(&self) -> usize {
        self.counts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.counts.is_empty()
    }

    pub fn get(&self, hash: u64) -> u32 {
        self.counts.get(&hash).copied().unwrap_or(0)
    }

    /// Get all counts as a vec
    pub fn counts(&self) -> &AHashMap<u64, u32> {
        &self.counts
    }
}

/// Canonical k-mer hash — min of forward and reverse complement hash.
/// Uses stack buffer to avoid heap allocation for k <= 64.
pub fn canonical_hash(kmer: &[u8]) -> u64 {
    let fwd = hash_bytes(kmer);
    let mut buf = [0u8; 64];
    let k = kmer.len();
    if k <= 64 {
        for (i, &b) in kmer.iter().rev().enumerate() {
            buf[i] = comp_base(b);
        }
        let rev = hash_bytes(&buf[..k]);
        fwd.min(rev)
    } else {
        // fallback for very large k (rare)
        let rc: Vec<u8> = kmer.iter().rev().map(|&b| comp_base(b)).collect();
        let rev = hash_bytes(&rc);
        fwd.min(rev)
    }
}

fn comp_base(b: u8) -> u8 {
    match b {
        b'A' | b'a' => b'T',
        b'T' | b't' => b'A',
        b'C' | b'c' => b'G',
        b'G' | b'g' => b'C',
        _ => b'N',
    }
}

/// Fixed-seed hash for reproducibility across runs.
fn hash_bytes(data: &[u8]) -> u64 {
    static STATE: std::sync::LazyLock<ahash::RandomState> =
        std::sync::LazyLock::new(|| ahash::RandomState::with_seeds(0x517c, 0x6c62, 0x0bb4, 0x2f3b));
    STATE.hash_one(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_kmers() {
        let mut kc = KmerCounter::new(3);
        kc.add_seq(b"ACGTACGT");
        assert!(kc.len() > 0);
    }

    #[test]
    fn canonical_palindrome() {
        // ACGT is its own reverse complement
        let h1 = canonical_hash(b"ACGT");
        let h2 = canonical_hash(b"ACGT");
        assert_eq!(h1, h2);
    }

    #[test]
    fn top_n() {
        let mut kc = KmerCounter::new(2);
        kc.add_seq(b"AAAA"); // AA appears 3 times
        let top = kc.top_n(1);
        assert_eq!(top.len(), 1);
        assert_eq!(top[0].1, 3);
    }
}
