use skv::KeyValueStore;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kv_store = KeyValueStore::load("kv_store.db", "kv_index.db")?;
    println!("{:?}", kv_store.get("key1"));
    println!("{:?}", kv_store.get("key2"));
    Ok(())
}