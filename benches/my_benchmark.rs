use criterion::{criterion_group, criterion_main, Criterion};
use rand::distributions::Alphanumeric;
use rand::Rng;
use skv::KeyValueStore;
use std::iter;

// Performance Tests
fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let n = 10;
    let path = "/tmp/storage";
    let index_path = "/tmp/index";

    let store = KeyValueStore::new(path, index_path).unwrap();

    // Sequential Writes
    c.bench_function("sequential writes", |b| {
        b.iter(|| {
            for i in 0..n {
                let key = format!("key{}", i);
                let value = format!("value{}", i);
                store.insert(key, value).unwrap();
            }
        })
    });

    // Sequential Reads
    c.bench_function("sequential reads", |b| {
        b.iter(|| {
            for i in 0..n {
                let key = format!("key{}", i);
                store.get(&key).unwrap();
            }
        })
    });

    // Random Writes
    c.bench_function("random writes", |b| {
        b.iter(|| {
            for _ in 0..n {
                let key: String = iter::repeat(())
                    .map(|_| rng.sample(Alphanumeric))
                    .map(char::from)
                    .take(30)
                    .collect();
                let value: String = iter::repeat(())
                    .map(|_| rng.sample(Alphanumeric))
                    .map(char::from)
                    .take(30)
                    .collect();
                store.insert(key, value).unwrap();
            }
        })
    });

    // Random Reads
    c.bench_function("random reads", |b| {
        b.iter(|| {
            for _ in 0..n {
                let key: String = iter::repeat(())
                    .map(|_| rng.sample(Alphanumeric))
                    .map(char::from)
                    .take(30)
                    .collect();
                store.get(&key).unwrap();
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches,);
