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
    
}
```

#### Generic input

```rust
use skv::KeyValueStore;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Create a new key-value store with a type that can be serialized
    // as a byte array. In this case, we use a fixed size array.
    let kv_store = KeyValueStore::<[u8;2]>::new("kv_store.db", "kv_index.db")?;

    // Insert some key-value pairs
    kv_store.insert("key1".to_string(), [0, 1])?;
    kv_store.insert("key2".to_string(), [2, 3])?;

    // Retrieve some key-value pairs
    println!("{:?}", kv_store.get("key1"));
    println!("{:?}", kv_store.get("key2"));

    Ok(())

}
```

### Benchmarks

**TLDR;** `skv` is pretty fast. Read take ~18µs and writes take ~750µs 🏎️

Please run `cargo bench` to run the benchmarks on your machine to get a better idea of how `skv` performs. The following benchmarks were run on a 2019 MBP, they may not be representative of your machine.

| Benchmark         | Lower     | Estimate  | Upper     |
| ----------------- | --------- | --------- | --------- |
| random reads      | 17.544 µs | 17.774 µs | 18.035 µs |
| random writes     | 710.12 µs | 756.05 µs | 804.26 µs |
| sequential reads  | 17.864 µs | 18.160 µs | 18.497 µs |
| sequential writes | 698.29 µs | 746.16 µs | 800.71 µs |

### Limitations

Below are some of the limitations of `skv`. These limitations may be addressed in future versions, or may require a more complex implementation. `skv` is intended stay exremely simple and easy to use and may not fit your use case.

- benches are test are on relatively small data sets <1M key-value pairs
- only supports `String` key-value pairs
- index is not optimized for memory usage and lookup is O(n)
- persisted data is not compressed or optimized for storage
