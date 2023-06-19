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

### Benchmarks

**TLDR;** `skv` is pretty fast. Read take ~15µs and writes take ~330µs 🏎️

Please run `cargo bench` to run the benchmarks on your machine to get a better idea of how `skv` performs. The following benchmarks were run on a 2019 MBP, they may not be representative of your machine.

| Benchmark         | Lower     | Estimate  | Upper     |
| ----------------- | --------- | --------- | --------- |
| random reads      | 15.478 µs | 15.579 µs | 15.689 µs |
| random writes     | 332.72 µs | 336.56 µs | 341.08 µs |
| sequential reads  | 15.305 µs | 15.405 µs | 15.517 µs |
| sequential writes | 324.78 µs | 331.69 µs | 339.40 µs |

### Limitations

Below are some of the limitations of `skv`. These limitations may be addressed in future versions, or may require a more complex implementation. `skv` is intended stay exremely simple and easy to use and may not fit your use case.

- benches are test are on relatively small data sets <1M key-value pairs
- only supports `String` key-value pairs
- index is not optimized for memory usage and lookup is O(n)
- persisted data is not compressed or optimized for storage
