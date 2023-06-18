# skv

[![Crates.io][crates-badge]][crates-url]
[![Apache 2.0 licensed][apache2-badge]][apache2-url]

[crates-badge]: https://img.shields.io/crates/v/skv.svg

[crates-url]: https://crates.io/crates/skv

[apache2-badge]: https://img.shields.io/badge/license-mit-blue.svg

[apache2-url]: https://github.com/drbh/skv/blob/master/LICENSE


A simple key-value store written in Rust.

`skv` implements a basic thread-safe key-value store that persists data to disk. It maintains an index to quickly lookup data offsets in the storage file. Key-value pairs can be inserted into the store with the `insert` method and retrieved with the `get` method. The store's state can be loaded from disk using the `load` method, and a new store can be created with the `new` method.


#### Most basic usage

```rust
use skv::KeyValueStore;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Create a new key-value store
    let kv_store = KeyValueStore::new("kv_store.db", "kv_index.db")?;

    // Insert some key-value pairs
    kv_store.insert("key1".to_string(), "value1".to_string())?;
    kv_store.insert("key2".to_string(), "value2".to_string())?;

    // Retrieve some key-value pairs
    println!("{:?}", kv_store.get("key1"));
    println!("{:?}", kv_store.get("key2"));

    Ok(())
```
