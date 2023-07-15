use skv::KeyValueStore;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("tmp")?;

    let store_name = "kv_store1.db";
    let index_name = "kv_index1.db";

    let kv_store =
        KeyValueStore::<String>::new(format!("tmp/{}", store_name), format!("tmp/{}", index_name))?;

    // store a new value
    kv_store.insert("key1".to_string(), "one".to_string())?;
    kv_store.insert("key2".to_string(), "two".to_string())?;
    kv_store.insert("key3".to_string(), "three".to_string())?;
    kv_store.insert("key4".to_string(), "four".to_string())?;

    // get the value
    println!("Inital\t{}", kv_store.get("key2").unwrap().unwrap());

    drop(kv_store);

    // load the store and index files
    let mut kv_store = KeyValueStore::<String>::load(
        format!("tmp/{}", store_name),
        format!("tmp/{}", index_name),
    )?;

    // get the value
    println!("ReRead\t{}", kv_store.get("key2").unwrap().unwrap());
    
    kv_store.delete("key1")?;
    kv_store.delete("key2")?;
    kv_store.delete("key3")?;
    
    println!("IsGone\t{}", kv_store.get("key2").unwrap().is_none());


    kv_store.gc()?;

    // remove the store and index files
    std::fs::remove_file(format!("tmp/{}", store_name))?;
    std::fs::remove_file(format!("tmp/{}", index_name))?;

    // remove the tmp directory
    std::fs::remove_dir("tmp")?;

    Ok(())
}
