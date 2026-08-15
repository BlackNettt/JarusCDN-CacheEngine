use std::path::{Path, PathBuf};
use std::collections::HashMap;
use sha2::{Sha256, Digest};
use hex;
use std::io::Error;
use std::fs;

struct CacheMeta {
    clocation : PathBuf,
    size: u64
}


struct CacheEngine {
    base_dir: PathBuf,
    meta: HashMap<String, CacheMeta>
}

impl CacheEngine {
    // Constructor
    pub fn new(base_dir: &Path) -> Self {
        Self { 
            base_dir: base_dir.to_path_buf(), 
            meta: HashMap::new() 
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
        
        self.meta.insert(cache_key.to_string(), 
                        CacheMeta{
                            clocation: absolute_cache_file_path.clone(),
                            size: fs::metadata(&absolute_cache_file_path).unwrap().len()
                        });
        Ok(())
    }

    pub fn exists(&self, cache_key: &str) -> bool{
        return self.meta.contains_key(cache_key);
    }

    pub fn read(&self, cache_key: &str) -> Result<Vec<u8>, Error> {
        let meta_data = self.meta.get(cache_key);
        if meta_data.is_none() {
            return Ok(Vec::new());
        } 

        let data = fs::read(&meta_data.unwrap().clocation)?;
        Ok(data)
    }

    pub fn purge(&mut self, cache_key: &str) -> Result<bool, Error> {
        let meta_data = self.meta.get(cache_key);
        if meta_data.is_none() {
            return Ok(false);
        }

        fs::remove_file(&meta_data.unwrap().clocation)?;
        self.meta.remove(cache_key);
        Ok(true)
    }


    // pub fn rebuild_cache_meta(&mut self) -> Result<(), Error> {
    //     let base_dir = &self.base_dir;
        
    //     // Iterate through the cache directory and rebuild the metadata
    //     for entry in fs::read_dir(base_dir)? {
    //         let entry = entry?;
    //         if entry.file_type()?.is_dir() {
    //             for sub_entry in fs::read_dir(entry.path())? {
    //                 let sub_entry = sub_entry?;
    //                 if sub_entry.file_type()?.is_dir() {
    //                     for file_entry in fs::read_dir(sub_entry.path())? {
    //                         let file_entry = file_entry?;
    //                         if file_entry.file_type()?.is_file() {
    //                             let file_path = file_entry.path();
    //                             let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();
    //                             let file_size = fs::metadata(&file_path)?.len();
    //                             let cache_key = format!("{}{}", entry.file_name().to_string_lossy(), file_name);
    //                             self.meta.insert(cache_key, CacheMeta { clocation: file_path, size: file_size });
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }

    //     Ok(())
    // }

    pub fn print_meta_data(&self) {
        for (_key, _val) in self.meta.iter() {
            println!("Key: {:?}, Value: {:?}", _key, _val.clocation);
        }
    }
    
}


fn main() {
    println!("Hello, Im from JarusCache Engine V2!");

    let base_dir = Path::new("/home/suraj-21245/JarusCDN/CacheEngine/cache");
    let mut cache = CacheEngine::new(base_dir);

    // println!("Rebuilding cache metadata");
    // let is_rebuild_success = cache.rebuild_cache_meta();
    // if is_rebuild_success.is_err() {
    //     println!("Error rebuilding cache metadata: {:?}", is_rebuild_success.err());
    // } else {
    //     println!("Cache metadata rebuilt successfully");
    // }

    cache.print_meta_data();


    let key = String::from("https://jarusv3.jaruscdn.com/sample.html1");
    let value = String::from("Hello, All this is my first cache engine");

    let is_cache_exists = cache.exists(&key);
    println!("1 Cache exists: {:?}", is_cache_exists);

    let store_result = cache.store(&key, value.as_bytes());

    let is_cache_exists = cache.exists(&key);
    println!("2 Cache exists: {:?}", is_cache_exists);

    let retrieved_data = cache.read(&key);
    println!("1 retrieve result: {:?}", String::from_utf8(retrieved_data.unwrap()).unwrap());

    let purge_cache_result = cache.purge(&key);
    println!("purge result: {:?}", purge_cache_result);

    let is_cache_exists = cache.exists(&key);
    println!("3 Cache exists: {:?}", is_cache_exists);

    let retrieved_data = cache.read(&key);
    println!("2 retrieve result: {:?}", String::from_utf8(retrieved_data.unwrap()).unwrap());
}

