use sha2::{Digest, Sha256};
use hex;
use std::path::PathBuf;
use std::{fs, println};
use std::io::Error;


fn main(){
    println!("Vilkommen to V1 Engine!");

    let base_dir = String::from("/home/suraj-21245/JarusCDN/CacheEngine/cache");

    let key = String::from("https://jarus2.jaruscdn.com/sample.html");
    let value = String::from("Hello, All this is my first cache engine");


    let cache = JarusV1CacheEngine::new(base_dir);

    // 1. Check if the cache exists before storing
    let is_exists = cache.exists(&key);
    println!("1. exists before store: {:?}", is_exists);

    if is_exists.is_ok() {
        println!("Cache already exists for key: {}", key);
    } else {
        // 2. Store the cache
        let store_result = cache.store(&key, value.as_bytes());
        println!("2. store result: {:?}", store_result);

        // 3. Check if the cache exists after storing
        let is_exists_after_store = cache.exists(&key);
        println!("3. exists after store: {:?}", is_exists_after_store);

        // 4. Retrieve the cache
        let retrieve_result = cache.retrieve(&key);
        println!("4. retrieve result: {:?}", String::from_utf8(retrieve_result.unwrap()).unwrap());

        // 5. Delete the cache
        let delete_result = cache.delete(&key);
        println!("5. delete result: {:?}", delete_result);

        // 6. Check if the cache exists after deletion
        let is_exists_after_delete = cache.exists(&key);
        println!("6. exists after delete: {:?}", is_exists_after_delete);
    }  
}

struct JarusV1CacheEngine {
    base_dir: String,
}

impl JarusV1CacheEngine {
    pub fn new(base_dir: String) -> Self {
        Self { base_dir }
    }

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

    pub fn store(&self, cache_key: &str, cache_value: &[u8]) -> Result<(), Error> {
        let (cache_path_buf, cache_file_name) = self.build_cache_path(cache_key);
        fs::create_dir_all(&cache_path_buf)?;
        let absolute_cache_file_path = cache_path_buf.join(cache_file_name);
        fs::write(&absolute_cache_file_path, cache_value)?;
        Ok(())
    }

    pub fn retrieve(&self, cache_key: &str) -> Result<Vec<u8>, Error>{
        let (cache_path_buf, cache_file_name) = self.build_cache_path(cache_key);
        let absolute_cache_file_path = cache_path_buf.join(cache_file_name);

        let data = fs::read(absolute_cache_file_path)?;
        Ok(data)
    }

    pub fn exists(&self, cache_key: &str) -> Result<(), Error> {
        let (cache_path_buf, cache_file_name) = self.build_cache_path(cache_key);
        let absolute_cache_file_path = cache_path_buf.join(cache_file_name);

        fs::metadata(absolute_cache_file_path)?;
        Ok(())
    }

    pub fn delete(&self, cache_key: &str) -> Result<(), Error>{
        let (cache_path_buf, cache_file_name) = self.build_cache_path(cache_key);
        let absolute_cache_file_path = cache_path_buf.join(cache_file_name);

        fs::remove_file(absolute_cache_file_path)?;
        Ok(())
    }
}


fn store(cache_key: &str, value: &str) {

    let hashed_value = Sha256::digest(cache_key.as_bytes());

    let first_byte = hashed_value[0];
    let second_byte = hashed_value[1];
    let remaining_bytes = &hashed_value[2..];

    let first_level_dir = format!("{:02x}", first_byte);
    let second_level_dir = format!("{:02x}", second_byte);

    let file_name = hex::encode(remaining_bytes);

    println!("First Level Directory: {}", first_level_dir);
    println!("Second Level Directory: {}", second_level_dir);
    println!("File Name: {}", file_name);


    let BASE_DIR = String::from("/home/suraj-21245/JarusCDN/CacheEngine/cache");
    let new_directory_path = format!("{}/{}/{}", BASE_DIR, first_level_dir, second_level_dir);
    let file_path = format!("{}/{}", new_directory_path, file_name);


    fs::create_dir_all(&new_directory_path);
    fs::write(&file_path, value);

    println!("Saved cache file at: {}", file_path);

}


fn retrieve(key: &str) -> Vec<u8>{
        
    let hashed_value = Sha256::digest(key.as_bytes());
    let BASE_DIR = String::from("/home/suraj-21245/JarusCDN/CacheEngine/cache");
    let first_byte = hashed_value[0];
    let second_byte = hashed_value[1];
    let remaining_bytes = &hashed_value[2..];

    let first_level_dir = format!("{:02x}", first_byte);
    let second_level_dir = format!("{:02x}", second_byte);

    let file_name = hex::encode(remaining_bytes);

    let file_path = format!("{}/{}/{}/{}", BASE_DIR, first_level_dir, second_level_dir, file_name);

    let resultant = fs::read(file_path);
    return resultant.unwrap_or_default();
}
