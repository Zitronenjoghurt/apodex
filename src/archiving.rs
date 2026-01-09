use std::collections::HashSet;
use std::path::Path;

pub mod html;

#[derive(Debug, thiserror::Error)]
pub enum ArchiveError {
    #[error("Codec error: {0}")]
    Codec(#[from] bitcode::Error),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}

#[derive(Debug)]
pub struct Archive<E: ArchiveEntry> {
    entries: Vec<E>,
}

pub trait ArchiveEntry: bitcode::Encode + for<'a> bitcode::Decode<'a> {
    fn naive_date(&self) -> Option<chrono::NaiveDate>;
}

impl<E: ArchiveEntry> Default for Archive<E> {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl<E: ArchiveEntry> Archive<E> {
    pub fn new(entries: Vec<E>) -> Self {
        Self { entries }
    }

    pub fn entries(&self) -> &[E] {
        &self.entries
    }

    pub fn push(&mut self, entry: E) {
        self.entries.push(entry);
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn encode(&self) -> Vec<u8> {
        bitcode::encode(&self.entries)
    }

    pub fn decode(data: &[u8]) -> Result<Self, ArchiveError> {
        Ok(Self {
            entries: bitcode::decode(data)?,
        })
    }

    pub fn compress(&self) -> Vec<u8> {
        zstd::encode_all(self.encode().as_slice(), 2).unwrap()
    }

    pub fn decompress(data: &[u8]) -> Result<Self, ArchiveError> {
        Self::decode(&zstd::decode_all(data)?)
    }

    pub fn save(&self, path: &Path) -> Result<(), std::io::Error> {
        std::fs::write(path, self.compress())
    }

    pub fn load(path: &Path) -> Result<Self, ArchiveError> {
        Self::decompress(&std::fs::read(path)?)
    }

    pub fn all_dates(&self) -> HashSet<chrono::NaiveDate> {
        self.entries.iter().filter_map(|e| e.naive_date()).collect()
    }
}
