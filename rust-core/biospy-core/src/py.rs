// pyo3 bindings — expose core types and functions to Python

use pyo3::prelude::*;

use crate::feat::{BasicFeatures, KmerFeatures, Pipeline};
use crate::io::{self, FastqReader};
use crate::kmer::KmerCounter;
use crate::seq;
use crate::sketch::MinHash;

#[pyclass]
pub struct PyKmerCounter {
    inner: KmerCounter,
}

#[pymethods]
impl PyKmerCounter {
    #[new]
    fn new(k: usize) -> Self {
        Self {
            inner: KmerCounter::new(k),
        }
    }

    fn add_seq(&mut self, seq: &[u8]) {
        self.inner.add_seq(seq);
    }

    fn top_n(&self, n: usize) -> Vec<(u64, u32)> {
        self.inner.top_n(n)
    }

    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[pyclass]
pub struct PyMinHash {
    inner: MinHash,
}

#[pymethods]
impl PyMinHash {
    #[new]
    fn new(k: usize, n: usize) -> Self {
        Self {
            inner: MinHash::new(k, n),
        }
    }

    fn add_seq(&mut self, seq: &[u8]) {
        self.inner.add_seq(seq);
    }

    fn jaccard(&self, other: &PyMinHash) -> f64 {
        self.inner.jaccard(&other.inner)
    }
}

/// Count records in a FASTQ file
#[pyfunction]
fn count_fastq(path: &str) -> PyResult<usize> {
    let file = std::fs::File::open(path)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
    let reader = FastqReader::new(std::io::BufReader::new(file));
    let mut count = 0;
    for rec in reader {
        rec.map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        count += 1;
    }
    Ok(count)
}

/// Extract features from a sequence using the default pipeline
#[pyfunction]
fn extract_features(seq: &[u8], k: usize, top_n: usize) -> Vec<f64> {
    let pipe = Pipeline::new()
        .add(Box::new(BasicFeatures))
        .add(Box::new(KmerFeatures::new(k, top_n)));
    pipe.extract(seq)
}

/// Read all sequences from a FASTQ file
#[pyfunction]
fn read_fastq_seqs(path: &str) -> PyResult<Vec<Vec<u8>>> {
    io::read_fastq_seqs(path)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))
}

/// Read all sequences from a FASTA file
#[pyfunction]
fn read_fasta_seqs(path: &str) -> PyResult<Vec<Vec<u8>>> {
    io::read_fasta_seqs(path)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))
}

/// Extract features from multiple sequences, return list of feature vectors
#[pyfunction]
fn extract_features_batch(seqs: Vec<Vec<u8>>, k: usize, top_n: usize) -> Vec<Vec<f64>> {
    let pipe = Pipeline::new()
        .add(Box::new(BasicFeatures))
        .add(Box::new(KmerFeatures::new(k, top_n)));
    seqs.iter().map(|s| pipe.extract(s)).collect()
}

/// Count ambiguous (non-ACGT) bases in a sequence
#[pyfunction]
fn count_ambiguous(seq_bytes: &[u8]) -> usize {
    seq::count_ambiguous(seq_bytes)
}

#[pymodule]
pub fn biospy_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyKmerCounter>()?;
    m.add_class::<PyMinHash>()?;
    m.add_function(wrap_pyfunction!(count_fastq, m)?)?;
    m.add_function(wrap_pyfunction!(extract_features, m)?)?;
    m.add_function(wrap_pyfunction!(extract_features_batch, m)?)?;
    m.add_function(wrap_pyfunction!(read_fastq_seqs, m)?)?;
    m.add_function(wrap_pyfunction!(read_fasta_seqs, m)?)?;
    m.add_function(wrap_pyfunction!(count_ambiguous, m)?)?;
    Ok(())
}
