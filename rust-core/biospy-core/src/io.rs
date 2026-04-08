// Streaming FASTQ/FASTA parser — never buffers more than one record

use std::io::BufRead;

#[derive(Clone, Debug)]
pub struct FastqRecord {
    pub name: String,
    pub seq: Vec<u8>,
    pub qual: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct FastaRecord {
    pub name: String,
    pub seq: Vec<u8>,
}

/// Streaming FASTQ reader, yields one record at a time
pub struct FastqReader<R: BufRead> {
    reader: R,
    buf: String,
}

impl<R: BufRead> FastqReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf: String::new(),
        }
    }
}

impl<R: BufRead> Iterator for FastqReader<R> {
    type Item = std::io::Result<FastqRecord>;

    fn next(&mut self) -> Option<Self::Item> {
        // header line (@name)
        self.buf.clear();
        match self.reader.read_line(&mut self.buf) {
            Ok(0) => return None,
            Err(e) => return Some(Err(e)),
            _ => {}
        }
        if !self.buf.starts_with('@') {
            return Some(Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "expected @",
            )));
        }
        let name = self.buf.trim_start_matches('@').trim().to_string();

        // seq line
        self.buf.clear();
        if let Err(e) = self.reader.read_line(&mut self.buf) {
            return Some(Err(e));
        }
        let seq = self.buf.trim().as_bytes().to_vec();

        // + separator line
        self.buf.clear();
        if let Err(e) = self.reader.read_line(&mut self.buf) {
            return Some(Err(e));
        }
        if !self.buf.starts_with('+') {
            return Some(Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "expected + separator",
            )));
        }

        // qual line
        self.buf.clear();
        if let Err(e) = self.reader.read_line(&mut self.buf) {
            return Some(Err(e));
        }
        let qual = self.buf.trim().as_bytes().to_vec();

        if seq.len() != qual.len() {
            return Some(Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "seq/qual length mismatch for {}: {} vs {}",
                    name,
                    seq.len(),
                    qual.len()
                ),
            )));
        }

        Some(Ok(FastqRecord { name, seq, qual }))
    }
}

/// Streaming FASTA reader
pub struct FastaReader<R: BufRead> {
    reader: R,
    buf: String,
    peeked_header: Option<String>,
}

impl<R: BufRead> FastaReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf: String::new(),
            peeked_header: None,
        }
    }
}

impl<R: BufRead> Iterator for FastaReader<R> {
    type Item = std::io::Result<FastaRecord>;

    fn next(&mut self) -> Option<Self::Item> {
        // get header
        let name = if let Some(h) = self.peeked_header.take() {
            h
        } else {
            self.buf.clear();
            match self.reader.read_line(&mut self.buf) {
                Ok(0) => return None,
                Err(e) => return Some(Err(e)),
                _ => {}
            }
            if !self.buf.starts_with('>') {
                return Some(Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "expected >",
                )));
            }
            self.buf.trim_start_matches('>').trim().to_string()
        };

        // accumulate seq lines until next header or EOF
        let mut seq = Vec::new();
        loop {
            self.buf.clear();
            match self.reader.read_line(&mut self.buf) {
                Ok(0) => break,
                Err(e) => return Some(Err(e)),
                _ => {}
            }
            if self.buf.starts_with('>') {
                self.peeked_header = Some(self.buf.trim_start_matches('>').trim().to_string());
                break;
            }
            seq.extend_from_slice(self.buf.trim().as_bytes());
        }

        Some(Ok(FastaRecord { name, seq }))
    }
}

/// Read all sequences from a FASTQ file.
pub fn read_fastq_seqs(path: &str) -> std::io::Result<Vec<Vec<u8>>> {
    let file = std::fs::File::open(path)?;
    let reader = FastqReader::new(std::io::BufReader::new(file));
    let mut seqs = Vec::new();
    for rec in reader {
        seqs.push(rec?.seq);
    }
    Ok(seqs)
}

/// Read all sequences from a FASTA file.
pub fn read_fasta_seqs(path: &str) -> std::io::Result<Vec<Vec<u8>>> {
    let file = std::fs::File::open(path)?;
    let reader = FastaReader::new(std::io::BufReader::new(file));
    let mut seqs = Vec::new();
    for rec in reader {
        seqs.push(rec?.seq);
    }
    Ok(seqs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn parse_fastq() {
        let data = b"@read1\nACGT\n+\nIIII\n@read2\nTGCA\n+\nIIII\n";
        let reader = FastqReader::new(BufReader::new(&data[..]));
        let recs: Vec<_> = reader.map(|r| r.unwrap()).collect();
        assert_eq!(recs.len(), 2);
        assert_eq!(recs[0].name, "read1");
        assert_eq!(recs[0].seq, b"ACGT");
    }

    #[test]
    fn parse_fasta() {
        let data = b">seq1\nACGT\nTGCA\n>seq2\nAAAA\n";
        let reader = FastaReader::new(BufReader::new(&data[..]));
        let recs: Vec<_> = reader.map(|r| r.unwrap()).collect();
        assert_eq!(recs.len(), 2);
        assert_eq!(recs[0].seq, b"ACGTTGCA");
        assert_eq!(recs[1].seq, b"AAAA");
    }
}
