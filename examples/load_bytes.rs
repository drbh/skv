use skv::KeyValueStore;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("tmp")?;

    let store_name = "kv_store1.db";
    let index_name = "kv_index1.db";

    let kv_store = KeyValueStore::<[u8; 4]>::new(
        format!("tmp/{}", store_name),
        format!("tmp/{}", index_name),
    )?;

    // store a new value
    kv_store.insert("key1".to_string(), [255, 255, 255, 255])?;

    // get the value
    println!("Inital {:?}", kv_store.get("key1"));

    drop(kv_store);

    // load the store and index files
    let kv_store = KeyValueStore::<[u8; 4]>::load(
        format!("tmp/{}", store_name),
        format!("tmp/{}", index_name),
    )?;

    // get the value
    println!("ReRead {:?}", kv_store.get("key1"));

    // remove the store and index files
    std::fs::remove_file(format!("tmp/{}", store_name))?;
    std::fs::remove_file(format!("tmp/{}", index_name))?;

    // remove the tmp directory
    std::fs::remove_dir("tmp")?;

    Ok(())
}
