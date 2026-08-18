use std::path::{Path, PathBuf};
use sha2::{Sha256, Digest};
use hex;
use std::io::Error;
use std::fs;


pub struct CacheStore {
    base_dir: PathBuf
}

impl CacheStore{
    pub fn new(base_dir: &Path) -> Self {
        Self { 
            base_dir: base_dir.to_path_buf()
        }
    }

    pub fn store(&self, path_to_store: &Path, file_name: &str, data_to_store: &[u8]) -> Result<(), Error> {
        let dir_path = self.base_dir.join(path_to_store);
        fs::create_dir_all(&dir_path)?;
        let file_path = dir_path.join(file_name);
        fs::write(&file_path, data_to_store)?;
        Ok(())
    }

    pub fn read(&self, path_to_read: &Path) -> Result<Option<Vec<u8>>, Error> {
        let abs_path_to_read = self.base_dir.join(path_to_read);
        if abs_path_to_read.exists() {
            let data = fs::read(&abs_path_to_read)?;
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }

    pub fn delete(&self, path_to_delete: &Path) -> Result<(), Error> {
        let abs_path_to_delete = self.base_dir.join(path_to_delete);
        fs::remove_file(&abs_path_to_delete)?;
        Ok(())
    }

    pub fn is_file_exists(&self, file_path_to_search: &Path) -> Result<bool, Error> {
        Ok(self.base_dir.join(file_path_to_search).exists())
    }
}