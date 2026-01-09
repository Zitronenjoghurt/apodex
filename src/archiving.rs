use std::path::Path;

pub mod html;

#[derive(Debug)]
pub struct Archive<E: bitcode::Encode + for<'a> bitcode::Decode<'a>> {
    entries: Vec<E>,
}

impl<E: bitcode::Encode + for<'a> bitcode::Decode<'a>> Default for Archive<E> {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl<E: bitcode::Encode + for<'a> bitcode::Decode<'a>> Archive<E> {
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

    pub fn compress(&self) -> Vec<u8> {
        zstd::encode_all(self.encode().as_slice(), 2).unwrap()
    }

    pub fn save(&self, path: &Path) -> Result<(), std::io::Error> {
        std::fs::write(path, self.compress())
    }
}
