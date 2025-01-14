use cache_lru::cache::{ Cache, LRUCache };
use criterion::{ black_box, criterion_group, criterion_main, Criterion };

fn cache_insertion_benchmark(c: &mut Criterion) {
    c.bench_function("insert 1000 items", |b| {
        b.iter(|| {
            let mut cache = Cache::new(black_box(1000));
            for i in 0..1000 {
                cache.put(i, i);
            }
        })
    });
}

fn cache_get_benchmark(c: &mut Criterion) {
    let mut cache = Cache::new(1000);
    for i in 0..1000 {
        cache.put(i, i);
    }

    c.bench_function("get existing items", |b| {
        b.iter(|| {
            for i in 0..1000 {
                black_box(cache.get(&i));
            }
        })
    });
}

fn cache_update_benchmark(c: &mut Criterion) {
    c.bench_function("update existing items", |b| {
        b.iter(|| {
            let mut cache = Cache::new(black_box(100));
            for i in 0..100 {
                cache.put(i, i);
                cache.put(i, i + 1);
            }
        })
    });
}

criterion_group!(benches, cache_insertion_benchmark, cache_get_benchmark, cache_update_benchmark);
criterion_main!(benches);
