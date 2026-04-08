"""Shared gradient-boosted classifier for detection and attribution."""

from __future__ import annotations

import numpy as np
from sklearn.ensemble import GradientBoostingClassifier


class Classifier:
    """Gradient-boosted classifier wrapper."""

    def __init__(self, n_estimators: int = 100, max_depth: int = 5):
        self.model = GradientBoostingClassifier(
            n_estimators=n_estimators, max_depth=max_depth,
        )
        self.fitted = False

    def fit(self, X: np.ndarray, y: np.ndarray) -> None:
        self.model.fit(X, y)
        self.fitted = True

    def predict(self, X: np.ndarray) -> np.ndarray:
        if not self.fitted:
            raise ValueError("model has not been fitted — call fit() first")
        return self.model.predict(X)

    def predict_proba(self, X: np.ndarray) -> np.ndarray:
        if not self.fitted:
            raise ValueError("model has not been fitted — call fit() first")
        return self.model.predict_proba(X)
