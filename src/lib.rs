use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{metadata, remove_file, rename, File, OpenOptions},
    io::{BufReader, Read, Seek, SeekFrom, Write},
    marker::PhantomData,
    path::Path,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex, RwLock,
    },
};

#[derive(Debug, Serialize, Deserialize)]
struct Index(HashMap<String, (u64, u64)>);

#[derive(Debug, Clone)]
// pub struct KeyValueStore<T: ToBytes + FromBytes> {
pub struct KeyValueStore<T: Serialize + for<'de> Deserialize<'de> + Send + Sync> {
    index: Arc<RwLock<Index>>,
    storage: Arc<Mutex<File>>,
    index_storage: Arc<RwLock<File>>,
    offset: Arc<AtomicU64>,
    paths: (String, String),
    phantom: PhantomData<T>,
}

impl<T: Serialize + for<'de> Deserialize<'de> + Send + Sync> KeyValueStore<T> {
    pub fn new<P: AsRef<Path>>(path: P, index_path: P) -> Result<Self> {
        let mut storage = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;

        let index_storage = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&index_path)?;

        let offset = storage.seek(SeekFrom::End(0))?;
        Ok(Self {
            index: Arc::new(RwLock::new(Index(HashMap::new()))),
            storage: Arc::new(Mutex::new(storage)),
            index_storage: Arc::new(RwLock::new(index_storage)),
            offset: Arc::new(AtomicU64::new(offset)),
            paths: (
                path.as_ref().to_str().unwrap().to_string(),
                index_path.as_ref().to_str().unwrap().to_string(),
            ),
            phantom: PhantomData,
        })
    }

    pub fn insert(&self, key: String, value: T) -> Result<()> {
        let value_bytes = bincode::serialize(&value)?;
        let len = value_bytes.len() as u64;

        let mut storage = self
            .storage
            .lock()
            .map_err(|_| anyhow!("Failed to acquire write lock on storage"))?;

        let mut index = self
            .index
            .write()
            .map_err(|_| anyhow!("Failed to acquire write lock on index"))?;

        let offset = self.offset.fetch_add(len, Ordering::SeqCst);

        storage.seek(SeekFrom::Start(offset))?;
        storage.write_all(&value_bytes)?;

        index.0.insert(key.clone(), (offset, len));

        let serialized_index = bincode::serialize(&index.0)?;
        let mut index_storage = self
            .index_storage
            .write()
            .map_err(|_| anyhow!("Failed to acquire write lock on index storage"))?;
        index_storage.set_len(0)?;
        index_storage.seek(SeekFrom::Start(0))?;
        index_storage.write_all(&serialized_index)?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<T>> {
        let index = self
            .index
            .read()
            .map_err(|_| anyhow!("Failed to acquire read lock on index"))?;
        if let Some(&(offset, len)) = index.0.get(key) {
            let storage = self
                .storage
                .lock()
                .map_err(|_| anyhow!("Failed to acquire lock on storage"))?;
            let mut reader = BufReader::new(&*storage);
            reader.seek(SeekFrom::Start(offset))?;
            let mut value_bytes = vec![0; len as usize];
            reader.read_exact(&mut value_bytes)?;
            let value = bincode::deserialize(&value_bytes)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    // removes the entry from the index, making it inaccessible even though
    // the actual data would remain in the storage file.
    pub fn delete(&self, key: &str) -> Result<()> {
        let mut index = self
            .index
            .write()
            .map_err(|_| anyhow!("Failed to acquire write lock on index"))?;

        if index.0.remove(key).is_none() {
            return Err(anyhow!("No entry found for key {}", key));
        }

        let serialized_index = bincode::serialize(&index.0)?;
        let mut index_storage = self
            .index_storage
            .write()
            .map_err(|_| anyhow!("Failed to acquire write lock on index storage"))?;
        index_storage.set_len(0)?;
        index_storage.seek(SeekFrom::Start(0))?;
        index_storage.write_all(&serialized_index)?;

        Ok(())
    }

    // force sync
    pub fn sync(&self) -> Result<()> {
        let storage = self
            .storage
            .lock()
            .map_err(|_| anyhow!("Failed to acquire write lock on storage"))?;
        storage.sync_all()?;
        let index_storage = self
            .index_storage
            .write()
            .map_err(|_| anyhow!("Failed to acquire write lock on index storage"))?;
        index_storage.sync_all()?;
        Ok(())
    }

    pub fn load<P: AsRef<Path>>(path: P, index_path: P) -> Result<Self> {
        let mut storage = OpenOptions::new().read(true).write(true).open(&path)?;
        let mut index_storage = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&index_path)?;
        let mut index_content = Vec::new();
        index_storage.read_to_end(&mut index_content)?;
        let index: Index = bincode::deserialize(&index_content)?;
        let offset = storage.seek(SeekFrom::End(0))?;
        Ok(Self {
            index: Arc::new(RwLock::new(index)),
            storage: Arc::new(Mutex::new(storage)),
            index_storage: Arc::new(RwLock::new(index_storage)),
            offset: Arc::new(AtomicU64::new(offset)),
            paths: (
                path.as_ref().to_str().unwrap().to_string(),
                index_path.as_ref().to_str().unwrap().to_string(),
            ),
            phantom: PhantomData,
        })
    }

    pub fn gc(&mut self) -> Result<()> {
        // Get the current paths
        let old_storage_path = self.paths.0.clone();
        let old_index_path = self.paths.1.clone();

        // Get the old sizes
        let old_storage_size = metadata(&old_storage_path)?.len();
        let old_index_size = metadata(&old_index_path)?.len();

        // Generate temporary file paths
        let new_storage_path = format!("{}_tmp", old_storage_path);
        let new_index_path = format!("{}_tmp", old_index_path);

        let new_kv = Self::new(&new_storage_path, &new_index_path)?;

        {
            let index = self
                .index
                .read()
                .map_err(|_| anyhow!("Failed to acquire read lock on index"))?;

            for (key, &(offset, len)) in index.0.iter() {
                let storage = self
                    .storage
                    .lock()
                    .map_err(|_| anyhow!("Failed to acquire lock on storage"))?;

                let mut reader = BufReader::new(&*storage);
                reader.seek(SeekFrom::Start(offset))?;
                let mut value_bytes = vec![0; len as usize];
                reader.read_exact(&mut value_bytes)?;

                let value: T = bincode::deserialize(&value_bytes)?;
                new_kv.insert(key.to_string(), value)?;
            }
        }

        // Now, you have the new KV filled with the old data
        // Replace the old KV files with the new ones

        remove_file(&old_storage_path)?;
        remove_file(&old_index_path)?;

        rename(&new_storage_path, &old_storage_path)?;
        rename(&new_index_path, &old_index_path)?;

        // Get the new sizes
        let new_storage_size = metadata(&old_storage_path)?.len();
        let new_index_size = metadata(&old_index_path)?.len();

        println!(
            "Storage size savings: {} bytes",
            old_storage_size as isize - new_storage_size as isize
        );
        println!(
            "Index size savings: {} bytes",
            old_index_size as isize - new_index_size as isize
        );

        // Reloading index and storage files
        *self = Self::load(&old_storage_path, &old_index_path)?;

        Ok(())
    }
}
