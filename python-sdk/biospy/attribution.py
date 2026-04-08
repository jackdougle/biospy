"""Round 2: attribution of engineered pathogens via design signatures."""

from __future__ import annotations

from pathlib import Path

import numpy as np

from biospy.core import extract_features_batch, read_fastq_seqs
from biospy.utils import resolve_paths


def attribute_origin(
    fastq_paths: list[str | Path],
    model=None,
    k: int = 21,
    top_n: int = 50,
) -> list[dict]:
    """Attribute origin/lab for sequences in FASTQ files.

    Returns list of dicts with keys: path, origin, confidence.
    """
    resolved = resolve_paths(fastq_paths)
    results = []
    for path in resolved:
        seqs = read_fastq_seqs(str(path))
        origin = "unknown"
        confidence = 0.0
        if model is not None:
            if hasattr(model, "fitted") and not model.fitted:
                raise ValueError("model has not been fitted")
            feats = extract_features_batch(seqs, k=k, top_n=top_n)
            if feats:
                preds = model.predict(np.array(feats))
                probas = model.predict_proba(np.array(feats))
                # majority vote across reads
                unique, counts = np.unique(preds, return_counts=True)
                origin = str(unique[np.argmax(counts)])
                confidence = float(np.mean(np.max(probas, axis=1)))
        results.append({
            "path": str(path),
            "origin": origin,
            "confidence": confidence,
        })
    return results
