use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock};

#[derive(Debug, Serialize, Deserialize)]
struct Index(HashMap<String, (u64, u64)>);

#[derive(Debug, Clone)]
pub struct KeyValueStore {
    index: Arc<RwLock<Index>>,
    storage: Arc<Mutex<File>>,
    index_storage: Arc<RwLock<File>>,
    offset: Arc<AtomicU64>,
}

impl KeyValueStore {
    pub fn new<P: AsRef<Path>>(path: P, index_path: P) -> Result<Self> {
        let mut storage = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        let index_storage = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(index_path)?;

        let offset = storage.seek(SeekFrom::End(0))?;
        Ok(Self {
            index: Arc::new(RwLock::new(Index(HashMap::new()))),
            storage: Arc::new(Mutex::new(storage)),
            index_storage: Arc::new(RwLock::new(index_storage)),
            offset: Arc::new(AtomicU64::new(offset)),
        })
    }

    pub fn insert(&self, key: String, value: String) -> Result<()> {
        let index_clone = Arc::clone(&self.index);
        let storage_clone = Arc::clone(&self.storage);
        let index_storage_clone = Arc::clone(&self.index_storage);

        {
            let mut storage = storage_clone
                .lock()
                .map_err(|_| anyhow!("Failed to acquire write lock on storage"))?;

            let mut index = index_clone
                .write()
                .map_err(|_| anyhow!("Failed to acquire write lock on index"))?;

            let len = value.len() as u64;

            // Fetch and add in a single atomic operation.
            let offset = self
                .offset
                .fetch_add((value.len() + 1) as u64, Ordering::SeqCst);

            storage.seek(SeekFrom::Start(offset))?;
            storage.write_all(value.as_bytes())?;
            storage.write_all(b"\n")?; // Write delimiter
            storage.sync_all()?;

            index.0.insert(key.clone(), (offset, len)); // Change her

            let mut index_storage = index_storage_clone
                .write()
                .map_err(|_| anyhow!("Failed to acquire write lock on index storage"))?;
            index_storage.set_len(0)?;
            index_storage.write_all(&serde_json::to_vec(&*index)?)?;
        }

        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<String>> {
        let index = self
            .index
            .read()
            .map_err(|_| anyhow!("Failed to acquire read lock on index"))?;
        if let Some(&(offset, len)) = index.0.get(key) {
            let storage = self
                .storage
                .lock() // Use lock() instead of read() because we need exclusive access.
                .map_err(|_| anyhow!("Failed to acquire lock on storage"))?;
            let mut reader = BufReader::new(&*storage);
            reader.seek(SeekFrom::Start(offset))?;
            let mut value = vec![0; len as usize];
            reader.read_exact(&mut value)?;
            Ok(Some(String::from_utf8(value)?))
        } else {
            Ok(None)
        }
    }

    pub fn load<P: AsRef<Path>>(path: P, index_path: P) -> Result<Self> {
        let mut storage = OpenOptions::new().read(true).write(true).open(path)?;
        let mut index_storage = OpenOptions::new().read(true).write(true).open(index_path)?;
        let mut index_content = Vec::new();
        index_storage.read_to_end(&mut index_content)?;
        let index: Index = serde_json::from_slice(&index_content)?;
        let offset = storage.seek(SeekFrom::End(0))?;
        Ok(Self {
            index: Arc::new(RwLock::new(index)),
            storage: Arc::new(Mutex::new(storage)),
            index_storage: Arc::new(RwLock::new(index_storage)),
            offset: Arc::new(AtomicU64::new(offset)),
        })
    }
}
