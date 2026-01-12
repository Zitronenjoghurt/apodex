use crate::date::ApodDate;

#[cfg(feature = "heed-media-cache")]
pub mod heed;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[repr(u8)]
pub enum MediaType {
    ImagePNG = 0,
}

#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub struct MediaEntry {
    pub media_type: MediaType,
    pub data: Vec<u8>,
}

pub trait MediaCache {
    fn store(
        &mut self,
        date: ApodDate,
        data: &[u8],
        media_type: MediaType,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    fn get(
        &self,
        date: ApodDate,
    ) -> Result<Option<MediaEntry>, Box<dyn std::error::Error + Send + Sync>>;
}
