// MinHash sketching for sequence similarity

use std::collections::BinaryHeap;

use crate::kmer::canonical_hash;

/// MinHash sketch for estimating Jaccard similarity.
/// Uses a max-heap + set for O(m log n) insertion over m k-mers.
pub struct MinHash {
    k: usize,
    n: usize,
    heap: BinaryHeap<u64>,
    seen: ahash::AHashSet<u64>,
}

impl MinHash {
    pub fn new(k: usize, n: usize) -> Self {
        Self {
            k,
            n,
            heap: BinaryHeap::with_capacity(n + 1),
            seen: ahash::AHashSet::new(),
        }
    }

    /// Add all k-mers from seq to sketch
    pub fn add_seq(&mut self, seq: &[u8]) {
        if seq.len() < self.k {
            return;
        }
        for window in seq.windows(self.k) {
            let h = canonical_hash(window);
            self.insert_hash(h);
        }
    }

    fn insert_hash(&mut self, h: u64) {
        if self.seen.contains(&h) {
            return;
        }
        if self.heap.len() < self.n {
            self.heap.push(h);
            self.seen.insert(h);
        } else if let Some(&max) = self.heap.peek() {
            if h < max {
                self.seen.remove(&max);
                self.heap.pop();
                self.heap.push(h);
                self.seen.insert(h);
            }
        }
    }

    /// Get sorted sketch values (heap → sorted vec).
    fn sorted(&self) -> Vec<u64> {
        let mut v: Vec<u64> = self.heap.iter().copied().collect();
        v.sort_unstable();
        v
    }

    /// Jaccard similarity estimate between two sketches
    pub fn jaccard(&self, other: &MinHash) -> f64 {
        let a = self.sorted();
        let b = other.sorted();
        if a.is_empty() || b.is_empty() {
            return 0.0;
        }
        let mut i = 0;
        let mut j = 0;
        let mut common = 0u64;
        let mut total = 0u64;
        while i < a.len() && j < b.len() {
            match a[i].cmp(&b[j]) {
                std::cmp::Ordering::Equal => {
                    common += 1;
                    total += 1;
                    i += 1;
                    j += 1;
                }
                std::cmp::Ordering::Less => {
                    total += 1;
                    i += 1;
                }
                std::cmp::Ordering::Greater => {
                    total += 1;
                    j += 1;
                }
            }
        }
        total += (a.len() - i + b.len() - j) as u64;
        if total == 0 {
            0.0
        } else {
            common as f64 / total as f64
        }
    }

    /// Merge another sketch into this one
    pub fn merge(&mut self, other: &MinHash) {
        for &h in other.heap.iter() {
            self.insert_hash(h);
        }
    }

    pub fn sketch(&self) -> Vec<u64> {
        self.sorted()
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn self_jaccard() {
        let mut s = MinHash::new(3, 100);
        s.add_seq(b"ACGTACGTACGTACGT");
        assert!((s.jaccard(&s) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn disjoint_jaccard() {
        let mut a = MinHash::new(3, 100);
        let mut b = MinHash::new(3, 100);
        a.add_seq(b"AAAAAAAAAA");
        b.add_seq(b"CCCCCCCCCC");
        assert!(a.jaccard(&b) < 0.5);
    }

    #[test]
    fn merge() {
        let mut a = MinHash::new(3, 100);
        let mut b = MinHash::new(3, 100);
        a.add_seq(b"ACGTACGT");
        b.add_seq(b"TGCATGCA");
        let pre = a.len();
        a.merge(&b);
        assert!(a.len() >= pre);
    }
}
