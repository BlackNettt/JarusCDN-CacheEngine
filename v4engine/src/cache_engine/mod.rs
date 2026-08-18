use std::path::{Path, PathBuf};
use sha2::{Sha256, Digest};
use hex;
use std::io::Error as IOError;
use oxicode::{Decode, Encode};
use crate::storage::CacheStore;
use crate::metadata::RocksDBStore;
use std::fs;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

#[derive(Encode, Decode)]
struct CacheMeta {
    clocation : PathBuf,
    size: u64,
    expired_at: u64,
}

pub struct CacheEngine {
    cache_store: CacheStore,
    meta_store: RocksDBStore
}

impl CacheEngine {
    pub fn new(base_dir: &Path) -> Self {
        Self {
            cache_store: CacheStore::new(base_dir),
            meta_store: RocksDBStore::new().unwrap()
        }
    }

    pub fn build_cache_path(&self, cache_key: &str) -> (PathBuf, String){
        let hashed_cache_key = Sha256::digest(cache_key.as_bytes());

        let first_level_dir = format!("{:02x}", hashed_cache_key[0]);
        let second_level_dir = format!("{:02x}", hashed_cache_key[1]);

        let cache_file_name = hex::encode(&hashed_cache_key[2..]);
        let cache_file_path_buf = PathBuf::from(&first_level_dir).join(&second_level_dir);
        return (cache_file_path_buf, cache_file_name);
    }

    pub fn store(&self, cache_key: &str, cache_value: &[u8]) -> Result<(), IOError> {
        let (path_to_store, file_name) = self.build_cache_path(cache_key);
        let file_store_res = self.cache_store.store(&path_to_store.as_path(), &file_name, cache_value);

        if file_store_res.is_err() {
            return Err(IOError::other(format!("Failed to store cache value for key: {}", cache_key)));
        }

        let absolute_cache_file_path = path_to_store.join(file_name);
        let cache_meta_data = CacheMeta {
                                clocation: absolute_cache_file_path.clone(),
                                size: cache_value.len() as u64,
                                expired_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 12 
                            };

        let serialized_meta = oxicode::encode_to_vec(&cache_meta_data)
            .map_err(|e| IOError::other(format!("encode cache metadata failed: {e}")))?;

        self.meta_store
            .put(cache_key.as_bytes(), &serialized_meta)
            .map_err(|e| IOError::other(format!("rocksdb put failed: {e}")))?;
        Ok(())
    }

    pub fn read(&self, cache_key: &str) -> Result<Option<Vec<u8>>, IOError> {
        let meta_data_option: Option<Vec<u8>> = self
            .meta_store
            .get(cache_key.as_bytes())
            .map_err(|e| IOError::other(format!("rocksdb get failed: {e}")))?;

        match meta_data_option {
            Some(meta_data) => {
                let (cache_meta, _): (CacheMeta, usize) = oxicode::decode_from_slice(&meta_data)
                    .map_err(|e| IOError::other(format!("decode cache metadata failed: {e}")))?;
                

                if cache_meta.expired_at < SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() {
                    // Cache has expired, delete the cache file and metadata
                    self.cache_store.delete(&cache_meta.clocation)?;
                    self.meta_store.delete(cache_key.as_bytes()).map_err(|e| IOError::other(format!("rocksdb delete failed: {e}")))?;
                    return Ok(None);
                }

                
                let cached_data_result = self.cache_store.read(&cache_meta.clocation)?;
                Ok(cached_data_result)
            },
            None => Ok(None)
        }
    }

    pub fn purge(&self, cache_key: &str) -> Result<bool, IOError> {
        let meta_data_option: Option<Vec<u8>> = self
            .meta_store
            .get(cache_key.as_bytes())
            .map_err(|e| IOError::other(format!("rocksdb get failed: {e}")))?;

        match meta_data_option {
            Some(meta_data) => {
                let (cache_meta, _): (CacheMeta, usize) = oxicode::decode_from_slice(&meta_data)
                    .map_err(|e| IOError::other(format!("decode cache metadata failed: {e}")))?;
                // remove the cached file and metadata from rocksdb
                self.cache_store.delete(&cache_meta.clocation)?;
                self.meta_store.delete(cache_key.as_bytes()).map_err(|e| IOError::other(format!("rocksdb delete failed: {e}")))?;
                Ok(true)
            },
            None => Ok(false)
        }
    }
}