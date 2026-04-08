"""Shared fixtures for Python tests."""

from pathlib import Path

import pytest

FIXTURES = Path(__file__).parent.parent / "fixtures"


@pytest.fixture
def sample_fastq():
    return str(FIXTURES / "sample.fastq")


@pytest.fixture
def sample_fasta():
    return str(FIXTURES / "sample.fasta")
