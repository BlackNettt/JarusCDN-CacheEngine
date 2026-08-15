use rust_rocksdb::{DB, Error as RocksDBError, Options};


pub struct RocksDBStore {
    db: DB
}

impl RocksDBStore {
    pub fn new() -> Result<Self, RocksDBError> {
        let db = connect_to_rocksdb()?;
        Ok(RocksDBStore { db })
    }

    pub fn put(&self, key: &[u8], value: &[u8]) -> Result<(), RocksDBError> {
        self.db.put(key, value)?;
        Ok(())
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, RocksDBError> {
        let data = self.db.get(key)?;
        Ok(data)
    }

    pub fn delete(&self, key: &[u8]) -> Result<(), RocksDBError> {
        self.db.delete(key)?;
        Ok(())
    }

    pub fn exists(&self, key: &[u8]) -> Result<bool, RocksDBError> {
        let data = self.db.get(key)?;
        Ok(data.is_some())
    }
}

pub fn connect_to_rocksdb() -> Result<DB, RocksDBError> {
    let path = "/home/suraj-21245/JarusCDN/CacheEngine/rocksdb_meta";
    let mut opts = Options::default();
    opts.create_if_missing(true);
    
    let db = DB::open(&opts, path)?;
    Ok(db)
}