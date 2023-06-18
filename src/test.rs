#[cfg(test)]
mod tests {
    use super::*;
    use criterion::{criterion_group, criterion_main, Criterion};
    use proptest::prelude::*;
    use rand::distributions::Alphanumeric;
    use rand::Rng;
    use std::iter;

    // Prop Tests
    proptest! {
        #[test]
        fn test_insert_get_any_string(key in "[a-zA-Z0-9]{1,64}", value in "[a-zA-Z0-9\\S]{0,128}") { // max 65536
            let path = "/tmp/storage_prop";
            let index_path = "/tmp/index_prop";
            let store = KeyValueStore::new(path, index_path).unwrap();
            store.insert(key.clone(), value.clone()).unwrap();
            let fetched_value = store.get(&key).unwrap();
            assert_eq!(fetched_value, Some(value));
        }
    }
}
