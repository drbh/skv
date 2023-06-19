use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::distributions::Alphanumeric;
use rand::Rng;
use skv::KeyValueStore;
use std::iter;

// Performance Tests
fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Basic Benchmark");
    group.sample_size(50);
    group.measurement_time(std::time::Duration::new(20, 0));

    let mut rng = rand::thread_rng();
    let n = 10;
    let path = "/tmp/storage";
    let index_path = "/tmp/index";

    let store = KeyValueStore::new(path, index_path).unwrap();

    let sequential_keys_values: Vec<(String, String)> = (0..n)
        .map(|i| (format!("key{}", i), format!("value{}", i)))
        .collect();

    let random_keys_values: Vec<(String, String)> = (0..n)
        .map(|_| {
            (
                iter::repeat(())
                    .map(|_| rng.sample(Alphanumeric))
                    .map(char::from)
                    .take(30)
                    .collect(),
                iter::repeat(())
                    .map(|_| rng.sample(Alphanumeric))
                    .map(char::from)
                    .take(30)
                    .collect(),
            )
        })
        .collect();

    // Sequential Writes
    group.bench_function("sequential writes", |b| {
        b.iter(|| {
            for (key, value) in &sequential_keys_values {
                store
                    .insert(black_box(key.to_string()), black_box(value.to_string()))
                    .unwrap();
            }
        })
    });

    // Sequential Reads
    group.bench_function("sequential reads", |b| {
        b.iter(|| {
            for (key, _) in &sequential_keys_values {
                store.get(black_box(key)).unwrap();
            }
        })
    });

    // Random Writes
    group.bench_function("random writes", |b| {
        b.iter(|| {
            for (key, value) in &random_keys_values {
                store
                    .insert(black_box(key.to_string()), black_box(value.to_string()))
                    .unwrap();
            }
        })
    });

    // Random Reads
    group.bench_function("random reads", |b| {
        b.iter(|| {
            for (key, _) in &random_keys_values {
                store.get(black_box(key)).unwrap();
            }
        })
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
