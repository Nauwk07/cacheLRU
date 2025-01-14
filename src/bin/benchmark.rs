use cache_lru::cache::{ Cache, LRUCache };
use std::time::Instant;

fn main() {
    let mut cache = Cache::new(1000);
    let start = Instant::now();

    // Test d'insertion
    for i in 0..1000 {
        cache.put(i, i);
    }

    println!("Temps d'insertion : {:?}", start.elapsed());
}
