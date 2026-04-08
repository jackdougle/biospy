// Integration test for IO module
use std::io::BufReader;

use biospy_core::io::{FastaReader, FastqReader};
use biospy_core::seq::count_ambiguous;

#[test]
fn fastq_roundtrip() {
    let data = b"@r1\nACGT\n+\nIIII\n@r2\nTGCA\n+\nIIII\n";
    let reader = FastqReader::new(BufReader::new(&data[..]));
    let recs: Vec<_> = reader.map(|r| r.unwrap()).collect();
    assert_eq!(recs.len(), 2);
    assert_eq!(recs[0].seq, b"ACGT");
    assert_eq!(recs[1].seq, b"TGCA");
}

#[test]
fn fastq_rejects_bad_separator() {
    let data = b"@r1\nACGT\nNOT_PLUS\nIIII\n";
    let reader = FastqReader::new(BufReader::new(&data[..]));
    let recs: Vec<_> = reader.collect();
    assert!(recs[0].is_err());
}

#[test]
fn fastq_rejects_length_mismatch() {
    let data = b"@r1\nACGT\n+\nII\n";
    let reader = FastqReader::new(BufReader::new(&data[..]));
    let recs: Vec<_> = reader.collect();
    assert!(recs[0].is_err());
}

#[test]
fn fasta_multiline() {
    let data = b">s1\nACGT\nTGCA\n>s2\nAAAA\n";
    let reader = FastaReader::new(BufReader::new(&data[..]));
    let recs: Vec<_> = reader.map(|r| r.unwrap()).collect();
    assert_eq!(recs.len(), 2);
    assert_eq!(recs[0].seq, b"ACGTTGCA");
}

#[test]
fn ambiguous_base_count() {
    assert_eq!(count_ambiguous(b"ACGTNNA"), 2);
    assert_eq!(count_ambiguous(b"ACGT"), 0);
    assert_eq!(count_ambiguous(b""), 0);
}
