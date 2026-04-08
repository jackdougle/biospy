"""Round 1: pathogen detection in environmental samples."""

from __future__ import annotations

from pathlib import Path

import numpy as np

from biospy.core import extract_features_batch, read_fastq_seqs
from biospy.utils import resolve_paths


def detect_pathogens(
    fastq_paths: list[str | Path],
    model=None,
    threshold: float = 0.5,
    k: int = 21,
    top_n: int = 50,
) -> list[dict]:
    """Detect pathogens across one or more FASTQ files.

    Returns list of dicts with keys: path, n_reads, detected, confidence.
    """
    resolved = resolve_paths(fastq_paths)
    results = []
    for path in resolved:
        seqs = read_fastq_seqs(str(path))
        n_reads = len(seqs)
        confidence = 0.0
        if model is not None:
            if hasattr(model, "fitted") and not model.fitted:
                raise ValueError("model has not been fitted")
            feats = extract_features_batch(seqs, k=k, top_n=top_n)
            if feats:
                probas = model.predict_proba(np.array(feats))
                # per-read positive probability, average across reads
                confidence = float(np.mean(probas[:, 1]))
        results.append({
            "path": str(path),
            "n_reads": n_reads,
            "detected": confidence >= threshold,
            "confidence": confidence,
        })
    return results
