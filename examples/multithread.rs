use skv::KeyValueStore;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Writing from multiple threads");

    let kv_store = KeyValueStore::new("kv_store.db", "kv_index.db")?;

    let mut handles = vec![];

    for i in 0..10 {
        let kv_store = kv_store.clone();
        let handle =
            std::thread::spawn(move || kv_store.insert(format!("key{}", i), format!("value{}", i)));
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap()?;
    }

    println!("Reading from multiple threads");

    let mut handles = vec![];

    for i in 0..10 {
        let kv_store = kv_store.clone();
        let handle = std::thread::spawn(move || kv_store.get(&format!("key{}", i)));
        handles.push(handle);
    }

    for handle in handles {
        println!("{:?}", handle.join().unwrap()?);
    }

    Ok(())
}
