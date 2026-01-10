use chrono::NaiveDate;
use std::collections::HashMap;
use std::path::Path;

pub mod html;

#[derive(Debug, thiserror::Error)]
pub enum ArchiveError {
    #[error("Codec error: {0}")]
    Codec(#[from] bitcode::Error),
    #[error("Invalid date")]
    InvalidDate,
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}

#[derive(Debug)]
pub struct Archive<E: ArchiveEntry> {
    entries: HashMap<NaiveDate, E>,
}

pub trait ArchiveEntry: bitcode::Encode + for<'a> bitcode::Decode<'a> + Clone {
    fn naive_date(&self) -> Option<NaiveDate>;
}

impl<E: ArchiveEntry> Default for Archive<E> {
    fn default() -> Self {
        Self::new(HashMap::new())
    }
}

impl<E: ArchiveEntry> Archive<E> {
    pub fn new(entries: HashMap<NaiveDate, E>) -> Self {
        Self { entries }
    }

    pub fn push(&mut self, entry: E) {
        let Some(date) = entry.naive_date() else {
            return;
        };
        self.entries.insert(date, entry);
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
        let entries: Vec<E> = self.entries.values().cloned().collect();
        bitcode::encode(&entries)
    }

    pub fn decode(data: &[u8]) -> Result<Self, ArchiveError> {
        let entries: Vec<E> = bitcode::decode(data)?;
        entries.try_into()
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

    pub fn has_date(&self, date: NaiveDate) -> bool {
        self.entries.contains_key(&date)
    }

    pub fn get(&self, date: NaiveDate) -> Option<&E> {
        self.entries.get(&date)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&NaiveDate, &E)> {
        self.entries.iter()
    }
}

impl<E: ArchiveEntry> TryFrom<Vec<E>> for Archive<E> {
    type Error = ArchiveError;

    fn try_from(value: Vec<E>) -> Result<Self, Self::Error> {
        let entries = value
            .into_iter()
            .map(|e| {
                e.naive_date()
                    .ok_or(ArchiveError::InvalidDate)
                    .map(|date| (date, e))
            })
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(Self::new(entries))
    }
}
