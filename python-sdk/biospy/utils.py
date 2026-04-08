"""Shared helpers."""

from __future__ import annotations

from pathlib import Path


def resolve_paths(paths: list[str | Path]) -> list[Path]:
    """Resolve and validate input paths."""
    resolved = []
    for p in paths:
        p = Path(p).resolve()
        if not p.exists():
            raise FileNotFoundError(f"not found: {p}")
        resolved.append(p)
    return resolved


def chunk_list(lst: list, n: int) -> list[list]:
    """Split list into chunks of size n."""
    return [lst[i : i + n] for i in range(0, len(lst), n)]
