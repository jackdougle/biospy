// 2-bit encoded DNA sequence (A=0, C=1, G=2, T=3)

#[derive(Clone, Debug)]
pub struct Seq {
    data: Vec<u8>,
    len: usize,
}

impl Seq {
    /// Encode ASCII bases into 2-bit representation.
    /// Ambiguous bases (N etc.) are encoded as A — use `count_ambiguous` to check first.
    pub fn from_ascii(bases: &[u8]) -> Self {
        let n = bases.len();
        let nbytes = (n + 3) / 4;
        let mut data = vec![0u8; nbytes];
        for (i, &b) in bases.iter().enumerate() {
            let enc = encode_base(b);
            let byte_idx = i / 4;
            let bit_off = (i % 4) * 2;
            data[byte_idx] |= enc << bit_off;
        }
        Self { data, len: n }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Get base at position as ASCII
    pub fn base_at(&self, i: usize) -> u8 {
        assert!(i < self.len);
        let enc = (self.data[i / 4] >> ((i % 4) * 2)) & 0b11;
        decode_base(enc)
    }

    /// Reverse complement
    pub fn rc(&self) -> Self {
        let mut bases = Vec::with_capacity(self.len);
        for i in (0..self.len).rev() {
            bases.push(complement(self.base_at(i)));
        }
        Self::from_ascii(&bases)
    }

    /// Decode back to ASCII
    pub fn to_ascii(&self) -> Vec<u8> {
        (0..self.len).map(|i| self.base_at(i)).collect()
    }
}

/// Encode a base, returning None for ambiguous/invalid bases.
fn try_encode_base(b: u8) -> Option<u8> {
    match b {
        b'A' | b'a' => Some(0),
        b'C' | b'c' => Some(1),
        b'G' | b'g' => Some(2),
        b'T' | b't' => Some(3),
        _ => None,
    }
}

fn encode_base(b: u8) -> u8 {
    try_encode_base(b).unwrap_or(0)
}

/// True if the base is a valid DNA character (ACGT, case-insensitive).
pub fn is_valid_base(b: u8) -> bool {
    try_encode_base(b).is_some()
}

/// Count ambiguous (non-ACGT) bases in a sequence.
pub fn count_ambiguous(seq: &[u8]) -> usize {
    seq.iter().filter(|&&b| !is_valid_base(b)).count()
}

fn decode_base(enc: u8) -> u8 {
    match enc {
        0 => b'A',
        1 => b'C',
        2 => b'G',
        3 => b'T',
        _ => unreachable!(),
    }
}

fn complement(b: u8) -> u8 {
    match b {
        b'A' => b'T',
        b'T' => b'A',
        b'C' => b'G',
        b'G' => b'C',
        _ => b'N',
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let seq = Seq::from_ascii(b"ACGTACGT");
        assert_eq!(seq.len(), 8);
        assert_eq!(seq.to_ascii(), b"ACGTACGT");
    }

    #[test]
    fn reverse_complement() {
        let seq = Seq::from_ascii(b"ACGT");
        let rc = seq.rc();
        assert_eq!(rc.to_ascii(), b"ACGT"); // palindrome
    }

    #[test]
    fn base_at() {
        let seq = Seq::from_ascii(b"TACG");
        assert_eq!(seq.base_at(0), b'T');
        assert_eq!(seq.base_at(1), b'A');
        assert_eq!(seq.base_at(2), b'C');
        assert_eq!(seq.base_at(3), b'G');
    }
}
