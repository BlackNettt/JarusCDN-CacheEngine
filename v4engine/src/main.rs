use std::fs;
use std::path::{Path, PathBuf};

mod cache_engine;
mod metadata;
mod storage;

fn main() {
    println!("Phase 4 Cache engine !!");

    let base_dir = Path::new("/home/suraj-21245/JarusCDN/CacheEngine/cache");
    let _c_engine = cache_engine::CacheEngine::new(&base_dir);

    let key = String::from("https://jarusv4.jaruscdn.com/sample.html?a=5");
    let value = String::from("Hello, All this is my first cache engine");

 
    let retrieved_data = _c_engine.read(&key);
    match retrieved_data.unwrap() {
        Some(retrieved_data) => {
            println!("0 retrieve result: {:?}", String::from_utf8(retrieved_data).unwrap());
        },
        None => println!("Cache meta not found !! MISS !!!")
    }

    let store_result = _c_engine.store(&key, value.as_bytes());
    if store_result.is_err() {
        println!("Error: {:?}", store_result.err().unwrap());
    }

    let retrieved_data = _c_engine.read(&key);
    match retrieved_data.unwrap() {
        Some(retrieved_data) => {
            println!("1 retrieve result: {:?}", String::from_utf8(retrieved_data).unwrap());
        },
        None => println!("Cache meta not found !! MISS !!!")
    }

}
