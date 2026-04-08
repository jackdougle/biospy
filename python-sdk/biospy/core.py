"""Thin wrappers around Rust bindings."""

from __future__ import annotations

from collections import Counter

try:
    from biospy.biospy_core import count_fastq as _count_fastq
    from biospy.biospy_core import extract_features as _extract_features
    from biospy.biospy_core import extract_features_batch as _extract_features_batch
    from biospy.biospy_core import read_fastq_seqs as _read_fastq_seqs
    from biospy.biospy_core import read_fasta_seqs as _read_fasta_seqs
    from biospy.biospy_core import count_ambiguous as _count_ambiguous
    from biospy.biospy_core import PyKmerCounter, PyMinHash

    HAS_NATIVE = True
except ImportError:
    HAS_NATIVE = False


def count_fastq(path: str) -> int:
    """Count records in a FASTQ file."""
    if HAS_NATIVE:
        return _count_fastq(path)
    # pure-python fallback: parse 4-line FASTQ structure
    count = 0
    with open(path) as f:
        while True:
            header = f.readline()
            if not header:
                break
            if not header.startswith("@"):
                raise ValueError(f"expected @ header, got: {header[:40]}")
            f.readline()  # seq
            f.readline()  # +
            f.readline()  # qual
            count += 1
    return count


def read_fastq_seqs(path: str) -> list[bytes]:
    """Read all sequences from a FASTQ file."""
    if HAS_NATIVE:
        return _read_fastq_seqs(path)
    seqs = []
    with open(path, "rb") as f:
        while True:
            header = f.readline()
            if not header:
                break
            seq = f.readline().strip()
            f.readline()  # +
            f.readline()  # qual
            seqs.append(seq)
    return seqs


def read_fasta_seqs(path: str) -> list[bytes]:
    """Read all sequences from a FASTA file."""
    if HAS_NATIVE:
        return _read_fasta_seqs(path)
    seqs = []
    cur: list[bytes] = []
    with open(path, "rb") as f:
        for line in f:
            line = line.strip()
            if line.startswith(b">"):
                if cur:
                    seqs.append(b"".join(cur))
                    cur = []
            else:
                cur.append(line)
    if cur:
        seqs.append(b"".join(cur))
    return seqs


def extract_features(seq: bytes, k: int = 21, top_n: int = 50) -> list[float]:
    """Extract feature vector from a DNA sequence."""
    if HAS_NATIVE:
        return _extract_features(seq, k, top_n)
    return _py_extract_features(seq, k, top_n)


def extract_features_batch(
    seqs: list[bytes], k: int = 21, top_n: int = 50,
) -> list[list[float]]:
    """Extract features from multiple sequences."""
    if HAS_NATIVE:
        return _extract_features_batch(seqs, k, top_n)
    return [_py_extract_features(s, k, top_n) for s in seqs]


def _py_extract_features(seq: bytes, k: int, top_n: int) -> list[float]:
    """Pure-python fallback mirroring BasicFeatures + KmerFeatures."""
    n = len(seq)
    # basic features: length, gc fraction, complexity
    gc = sum(1 for b in seq if b in (ord("G"), ord("g"), ord("C"), ord("c")))
    gc_frac = gc / n if n > 0 else 0.0

    if n >= 3:
        trigrams = set()
        for i in range(n - 2):
            trigrams.add(seq[i : i + 3])
        complexity = len(trigrams) / 64.0
    else:
        complexity = 0.0

    # kmer frequency features
    if n >= k:
        counts: Counter[bytes] = Counter()
        upper = seq.upper()
        for i in range(n - k + 1):
            kmer = upper[i : i + k]
            # canonical: min of fwd and rc
            rc = bytes(
                {65: 84, 84: 65, 67: 71, 71: 67}.get(b, 78)
                for b in reversed(kmer)
            )
            counts[min(kmer, rc)] += 1
        total = sum(counts.values())
        top = counts.most_common(top_n)
        kmer_feats = [c / total for _, c in top]
    else:
        kmer_feats = []

    kmer_feats.extend([0.0] * (top_n - len(kmer_feats)))
    return [float(n), gc_frac, complexity] + kmer_feats
