"""Tests for origin attribution pipeline."""

import numpy as np

from biospy.attribution import attribute_origin
from biospy.models import Attributor


def test_attribute_no_model(sample_fastq):
    results = attribute_origin([sample_fastq])
    assert len(results) == 1
    r = results[0]
    assert r["origin"] == "unknown"
    assert r["confidence"] == 0.0


def test_attribute_multiple_files(sample_fastq):
    results = attribute_origin([sample_fastq, sample_fastq])
    assert len(results) == 2


def test_attribute_with_model(sample_fastq):
    """Smoke test: train a tiny model and run attribution."""
    attr = Attributor(n_estimators=5, max_depth=2)
    rng = np.random.default_rng(42)
    X = rng.random((30, 10))
    y = np.array(["lab_a"] * 10 + ["lab_b"] * 10 + ["lab_c"] * 10)
    attr.fit(X, y)
    results = attribute_origin([sample_fastq], model=attr, k=3, top_n=7)
    assert len(results) == 1
    r = results[0]
    assert isinstance(r["confidence"], float)
    assert 0.0 <= r["confidence"] <= 1.0
    assert r["origin"] in ("lab_a", "lab_b", "lab_c")
