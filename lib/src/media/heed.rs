use crate::date::ApodDate;
use crate::media::{MediaCache, MediaEntry, MediaType};
use heed::byteorder::NativeEndian;
use heed::types::{Bytes, I32};
use heed::EnvOpenOptions;
use std::error::Error;
use std::path::Path;

pub struct HeedMediaCache {
    env: heed::Env,
    db: heed::Database<I32<NativeEndian>, Bytes>,
}

impl HeedMediaCache {
    pub fn new(
        name: impl AsRef<str>,
        dir: impl AsRef<Path>,
        max_size_mb: usize,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        std::fs::create_dir_all(&dir)?;

        let env = unsafe {
            EnvOpenOptions::new()
                .map_size(max_size_mb * 1024 * 1024)
                .max_dbs(1)
                .open(dir)?
        };

        let db = {
            let mut txn = env.write_txn()?;
            let db = env.create_database(&mut txn, Some(name.as_ref()))?;
            txn.commit()?;
            db
        };

        Ok(Self { env, db })
    }
}

impl MediaCache for HeedMediaCache {
    fn store(
        &mut self,
        date: ApodDate,
        data: &[u8],
        media_type: MediaType,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let key = date.days();

        let entry = MediaEntry {
            media_type,
            data: data.to_vec(),
        };
        let entry_bytes = bitcode::encode(&entry);

        let mut txn = self.env.write_txn()?;
        self.db.put(&mut txn, &key, entry_bytes.as_slice())?;
        txn.commit()?;
        Ok(())
    }

    fn get(&self, date: ApodDate) -> Result<Option<MediaEntry>, Box<dyn Error + Send + Sync>> {
        let key = date.days();
        let txn = self.env.read_txn()?;
        let Some(entry_bytes) = self.db.get(&txn, &key)? else {
            return Ok(None);
        };
        let entry: MediaEntry = bitcode::decode(entry_bytes)?;
        Ok(Some(entry))
    }
}
