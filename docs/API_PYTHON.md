# Python API Reference

## Core (`biospy.core`)

```python
count_fastq(path: str) -> int
extract_features(seq: bytes, k: int = 21, top_n: int = 50) -> list[float]
```

## Detection (`biospy.detection`)

```python
detect_pathogens(
    fastq_paths: list[str],
    model=None,
    threshold: float = 0.5,
) -> list[dict]
# Returns: [{"path": str, "n_reads": int, "detected": bool, "confidence": float}]
```

## Attribution (`biospy.attribution`)

```python
attribute_origin(
    fastq_paths: list[str],
    model=None,
) -> list[dict]
# Returns: [{"path": str, "origin": str, "confidence": float}]
```

## Models

```python
from biospy.models import Detector, Attributor

det = Detector(n_estimators=100, max_depth=5)
det.fit(X_train, y_train)
det.predict(X_test)

attr = Attributor(n_estimators=100, max_depth=5)
attr.fit(X_train, y_train)
attr.predict(X_test)
```
