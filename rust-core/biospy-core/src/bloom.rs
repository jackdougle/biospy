// Bloom filter for fast set membership queries

use std::hash::Hash;

pub struct BloomFilter {
    bits: Vec<u8>,
    nbits: usize,
    nhash: usize,
}

impl BloomFilter {
    /// Create with optimal sizing for expected items and false-positive rate
    pub fn with_rate(expected: usize, fp_rate: f64) -> Self {
        let nbits = optimal_bits(expected, fp_rate);
        let nhash = optimal_hashes(nbits, expected);
        Self {
            bits: vec![0u8; (nbits + 7) / 8],
            nbits,
            nhash,
        }
    }

    pub fn new(nbits: usize, nhash: usize) -> Self {
        Self {
            bits: vec![0u8; (nbits + 7) / 8],
            nbits,
            nhash,
        }
    }

    pub fn insert(&mut self, item: &[u8]) {
        for i in 0..self.nhash {
            let pos = self.hash_pos(item, i);
            self.bits[pos / 8] |= 1 << (pos % 8);
        }
    }

    pub fn contains(&self, item: &[u8]) -> bool {
        (0..self.nhash).all(|i| {
            let pos = self.hash_pos(item, i);
            self.bits[pos / 8] & (1 << (pos % 8)) != 0
        })
    }

    /// Double-hashing: h_i(x) = (h1(x) + i * h2(x)) % nbits
    fn hash_pos(&self, item: &[u8], i: usize) -> usize {
        let (h1, h2) = self.two_hashes(item);
        (h1.wrapping_add((i as u64).wrapping_mul(h2)) % self.nbits as u64) as usize
    }

    fn two_hashes(&self, item: &[u8]) -> (u64, u64) {
        use std::hash::{BuildHasher, Hasher};
        static S1: std::sync::LazyLock<ahash::RandomState> =
            std::sync::LazyLock::new(|| ahash::RandomState::with_seeds(0x517c, 0x6c62, 0x0bb4, 0x2f3b));
        static S2: std::sync::LazyLock<ahash::RandomState> =
            std::sync::LazyLock::new(|| ahash::RandomState::with_seeds(0xa1b2, 0xc3d4, 0xe5f6, 0x7890));
        let mut h1 = S1.build_hasher();
        item.hash(&mut h1);
        let mut h2 = S2.build_hasher();
        item.hash(&mut h2);
        (h1.finish(), h2.finish())
    }
}

fn optimal_bits(n: usize, p: f64) -> usize {
    let ln2 = std::f64::consts::LN_2;
    (-(n as f64) * p.ln() / (ln2 * ln2)).ceil() as usize
}

fn optimal_hashes(m: usize, n: usize) -> usize {
    let k = (m as f64 / n as f64) * std::f64::consts::LN_2;
    k.ceil().max(1.0) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_contains() {
        let mut bf = BloomFilter::with_rate(1000, 0.01);
        bf.insert(b"ACGT");
        assert!(bf.contains(b"ACGT"));
    }

    #[test]
    fn probably_absent() {
        let bf = BloomFilter::with_rate(1000, 0.01);
        // empty filter should not contain anything
        assert!(!bf.contains(b"ACGT"));
    }
}
