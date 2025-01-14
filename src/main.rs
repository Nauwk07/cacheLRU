use cache_lru::cache::{ Cache, LRUCache };

fn main() {
    // Créer un cache persistant avec une capacité de 3
    let mut cache = Cache::<String, i32>::new_persistent(3, "mon_cache.txt").unwrap();

    // Test du cache
    cache.put("A".to_string(), 1);
    cache.put("B".to_string(), 2);
    cache.put("C".to_string(), 3);

    println!("Valeur de A : {:?}", cache.get(&"A".to_string()));

    // Test de dépassement de capacité
    cache.put("D".to_string(), 4);
    println!("Valeur de A après dépassement : {:?}", cache.get(&"A".to_string()));
}
