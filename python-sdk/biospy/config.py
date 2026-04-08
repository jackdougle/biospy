"""Load TOML configuration."""

from __future__ import annotations

import tomllib
from pathlib import Path

DEFAULTS = {
    "kmer": {"k": 21},
    "sketch": {"n": 1000},
    "bloom": {"expected_items": 1_000_000, "fp_rate": 0.001},
    "detection": {"threshold": 0.5, "top_n_features": 50},
    "attribution": {"top_n_features": 50},
}

_CONFIG_DIR = Path(__file__).parent.parent.parent / "configs"


def load_config(name: str = "default") -> dict:
    """Load config from configs/<name>.toml, falling back to built-in defaults."""
    path = _CONFIG_DIR / f"{name}.toml"
    if path.exists():
        with open(path, "rb") as f:
            return tomllib.load(f)
    return dict(DEFAULTS)
