use skv::KeyValueStore;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kv_store = KeyValueStore::new("kv_store.db", "kv_index.db")?;
    kv_store.insert("key1".to_string(), "value1".to_string())?;
    kv_store.insert("key2".to_string(), "value2".to_string())?;
    println!("{:?}", kv_store.get("key1"));
    println!("{:?}", kv_store.get("key2"));
    Ok(())
}