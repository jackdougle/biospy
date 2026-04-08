"""Tests for pathogen detection pipeline."""

import numpy as np

from biospy.detection import detect_pathogens
from biospy.models import Detector


def test_detect_no_model(sample_fastq):
    results = detect_pathogens([sample_fastq])
    assert len(results) == 1
    r = results[0]
    assert r["n_reads"] == 4
    assert r["detected"] is False
    assert r["confidence"] == 0.0


def test_detect_multiple_files(sample_fastq):
    results = detect_pathogens([sample_fastq, sample_fastq])
    assert len(results) == 2
    assert all(r["n_reads"] == 4 for r in results)


def test_detect_with_model(sample_fastq):
    """Smoke test: train a tiny model and run detection."""
    det = Detector(n_estimators=5, max_depth=2)
    # fake training data: 10 features (3 basic + 7 kmer)
    rng = np.random.default_rng(42)
    X = rng.random((20, 10))
    y = np.array([0] * 10 + [1] * 10)
    det.fit(X, y)
    results = detect_pathogens([sample_fastq], model=det, k=3, top_n=7)
    assert len(results) == 1
    r = results[0]
    assert isinstance(r["confidence"], float)
    assert 0.0 <= r["confidence"] <= 1.0
    assert isinstance(r["detected"], bool)
