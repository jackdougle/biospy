// FeatureExtractor trait — central extensibility point for both rounds

use crate::kmer::KmerCounter;

/// Core trait: extract fixed-dim feature vector from a sequence
pub trait FeatureExtractor: Send + Sync {
    fn name(&self) -> &str;
    fn dim(&self) -> usize;
    fn extract(&self, seq: &[u8]) -> Vec<f64>;
}

/// k-mer frequency features
pub struct KmerFeatures {
    k: usize,
    top_n: usize,
}

impl KmerFeatures {
    pub fn new(k: usize, top_n: usize) -> Self {
        Self { k, top_n }
    }
}

impl FeatureExtractor for KmerFeatures {
    fn name(&self) -> &str {
        "kmer_freq"
    }

    fn dim(&self) -> usize {
        self.top_n
    }

    fn extract(&self, seq: &[u8]) -> Vec<f64> {
        let mut kc = KmerCounter::new(self.k);
        kc.add_seq(seq);
        let total = kc.total_count();
        let top = kc.top_n(self.top_n);
        let mut feats = Vec::with_capacity(self.top_n);
        for (_, count) in &top {
            feats.push(if total > 0 {
                *count as f64 / total as f64
            } else {
                0.0
            });
        }
        feats.resize(self.top_n, 0.0);
        feats
    }
}

/// Basic sequence statistics (len, gc content, complexity)
pub struct BasicFeatures;

impl FeatureExtractor for BasicFeatures {
    fn name(&self) -> &str {
        "basic"
    }

    fn dim(&self) -> usize {
        3
    }

    fn extract(&self, seq: &[u8]) -> Vec<f64> {
        let len = seq.len() as f64;
        let gc = seq
            .iter()
            .filter(|&&b| matches!(b, b'G' | b'g' | b'C' | b'c'))
            .count() as f64;
        let gc_frac = if len > 0.0 { gc / len } else { 0.0 };
        // linguistic complexity: unique 3-mers / possible 3-mers
        let complexity = if seq.len() >= 3 {
            let mut set = std::collections::HashSet::new();
            for w in seq.windows(3) {
                set.insert((w[0], w[1], w[2]));
            }
            set.len() as f64 / 64.0
        } else {
            0.0
        };
        vec![len, gc_frac, complexity]
    }
}

/// Pipeline: concatenate features from multiple extractors
pub struct Pipeline {
    extractors: Vec<Box<dyn FeatureExtractor>>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self {
            extractors: Vec::new(),
        }
    }

    pub fn add(mut self, ext: Box<dyn FeatureExtractor>) -> Self {
        self.extractors.push(ext);
        self
    }

    pub fn dim(&self) -> usize {
        self.extractors.iter().map(|e| e.dim()).sum()
    }

    pub fn extract(&self, seq: &[u8]) -> Vec<f64> {
        self.extractors.iter().flat_map(|e| e.extract(seq)).collect()
    }

    pub fn names(&self) -> Vec<&str> {
        self.extractors.iter().map(|e| e.name()).collect()
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kmer_features() {
        let ext = KmerFeatures::new(3, 10);
        let feats = ext.extract(b"ACGTACGTACGT");
        assert_eq!(feats.len(), 10);
    }

    #[test]
    fn basic_features() {
        let ext = BasicFeatures;
        let feats = ext.extract(b"ACGTACGT");
        assert_eq!(feats.len(), 3);
        assert!((feats[1] - 0.5).abs() < 1e-9); // 50% GC
    }

    #[test]
    fn pipeline() {
        let pipe = Pipeline::new()
            .add(Box::new(BasicFeatures))
            .add(Box::new(KmerFeatures::new(3, 5)));
        assert_eq!(pipe.dim(), 8);
        let feats = pipe.extract(b"ACGTACGT");
        assert_eq!(feats.len(), 8);
    }
}
