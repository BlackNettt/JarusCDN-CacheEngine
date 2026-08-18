use std::path::{Path, PathBuf};
use std::collections::HashMap;
use sha2::{Sha256, Digest};
use hex;
use std::io::Error;
use std::fs;
use oxicode::{Decode, Encode};

use crate::metadata::RocksDBStore;

#[derive(Encode, Decode)]
struct CacheMeta {
    clocation : PathBuf,
    size: u64
}


pub struct CacheEngine {
    base_dir: PathBuf,
    meta: RocksDBStore
}

impl CacheEngine {
    // Constructor
    pub fn new(base_dir: &Path) -> Self {
        Self { 
            base_dir: base_dir.to_path_buf(), 
            meta: RocksDBStore::new().unwrap()
        }
    }

    // Construct Build Path
    pub fn build_cache_path(&self, cache_key: &str) -> (PathBuf, String){
        
        let hashed_cache_key = Sha256::digest(cache_key.as_bytes());

        let first_level_dir = format!("{:02x}", hashed_cache_key[0]);
        let second_level_dir = format!("{:02x}", hashed_cache_key[1]);

        let cache_file_name = hex::encode(&hashed_cache_key[2..]);
        let cache_file_path_buf = PathBuf::from(&self.base_dir)
                                                .join(&first_level_dir)
                                                .join(&second_level_dir);
        return (cache_file_path_buf, cache_file_name);
    }

    // Store Cache value with metadata
    pub fn store(&mut self, cache_key: &str, cache_value: &[u8]) -> Result<(), Error> {
        let (cache_path_buf, cache_file_name) = self.build_cache_path(cache_key);
        fs::create_dir_all(&cache_path_buf)?;

        let absolute_cache_file_path = cache_path_buf.join(cache_file_name);
        fs::write(&absolute_cache_file_path, cache_value)?;
        
        let cache_meta_data = CacheMeta {
                                clocation: absolute_cache_file_path.clone(),
                                size: fs::metadata(&absolute_cache_file_path)?.len()
                            };
        let serialized_meta = oxicode::encode_to_vec(&cache_meta_data)
            .map_err(|e| Error::other(format!("encode cache metadata failed: {e}")))?;

        self.meta
            .put(cache_key.as_bytes(), &serialized_meta)
            .map_err(|e| Error::other(format!("rocksdb put failed: {e}")))?;
        Ok(())
    }

    pub fn exists(&self, cache_key: &str) -> bool{
        self.meta.exists(cache_key.as_bytes()).unwrap_or(false)
    }

    pub fn read(&self, cache_key: &str) -> Result<Option<Vec<u8>>, Error> {
        let meta_data = self
            .meta
            .get(cache_key.as_bytes())
            .map_err(|e| Error::other(format!("rocksdb get failed: {e}")))?;


        match meta_data {
            Some(meta_data) => {
                let (meta_data, _):(CacheMeta, usize) = oxicode::decode_from_slice(&meta_data)
                                                        .map_err(|e| Error::other(format!("decode cache metadata failed: {e}")))?;
                
                let data = fs::read(&meta_data.clocation)?;
                Ok(Some(data))
            },
            None => Ok(None)
        }
    }

    pub fn purge(&mut self, cache_key: &str) -> Result<bool, Error> {
        let meta_data = self.meta.get(cache_key.as_bytes()).map_err(|e| Error::other(format!("rocksdb get failed: {e}")))?;
        if meta_data.is_none() {
            return Ok(false);
        }

        let meta_bytes = meta_data.unwrap();
        let (meta_data, _): (CacheMeta, usize) = oxicode::decode_from_slice(&meta_bytes)
            .map_err(|e| Error::other(format!("decode cache metadata failed: {e}")))?;

        fs::remove_file(&meta_data.clocation)?;
        self.meta.delete(cache_key.as_bytes()).map_err(|e| Error::other(format!("rocksdb delete failed: {e}")))?;
        Ok(true)
    }

}

