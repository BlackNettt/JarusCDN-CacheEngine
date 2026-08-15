
use std::fs;
use std::path::{Path, PathBuf};

mod cache_engine;
use cache_engine::CacheEngine;
mod metadata;
mod storage;


fn main() {
    println!("Hello, Im from JarusCache Engine V3!");

    let base_dir = Path::new("/home/suraj-21245/JarusCDN/CacheEngine/cache");
    let mut cache = CacheEngine::new(base_dir);


    let key = String::from("https://jarusv3.jaruscdn.com/sample.html?a=1");
    let value = String::from("Hello, All this is my first cache engine");

    let is_cache_exists = cache.exists(&key);
    println!("1 Cache exists: {:?}", is_cache_exists);

    let store_result = cache.store(&key, value.as_bytes());

    let is_cache_exists = cache.exists(&key);
    println!("2 Cache exists: {:?}", is_cache_exists);

    let purge_cache_result = cache.purge(&key);
    println!("purge result: {:?}", purge_cache_result);


    let retrieved_data = cache.read(&key);
    match retrieved_data.unwrap() {
        Some(retrieved_data) => {
            println!("1 retrieve result: {:?}", String::from_utf8(retrieved_data).unwrap());
        },
        None => println!("Cache meta not found !! MISS !!!")
    }

    let is_cache_exists = cache.exists(&key);
    println!("3 Cache exists: {:?}", is_cache_exists);

    let retrieved_data = cache.read(&key);
    match retrieved_data.unwrap() {
        Some(retrieved_data) => {
            println!("1 retrieve result: {:?}", String::from_utf8(retrieved_data).unwrap());
        },
        None => println!("Cache meta not found !! MISS !!!")
    }
}
