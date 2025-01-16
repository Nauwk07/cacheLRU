use cache_lru::cache::{ Cache, LRUCache };

#[test]
fn test_basic_operations() {
    let mut cache = Cache::<String, i32>::new(2);
    cache.put("a".to_string(), 1);
    cache.put("b".to_string(), 2);
    assert_eq!(cache.get(&"a".to_string()), Some(&1));
    cache.put("c".to_string(), 3);
    assert_eq!(cache.get(&"b".to_string()), None);
    assert_eq!(cache.get(&"a".to_string()), Some(&1));
    assert_eq!(cache.get(&"c".to_string()), Some(&3));
}

#[test]
fn test_capacity() {
    let mut cache = Cache::<String, i32>::new(2);
    cache.put("A".to_string(), 1);
    cache.put("B".to_string(), 2);
    cache.put("C".to_string(), 3);
    assert_eq!(cache.get(&"A".to_string()), None);
    assert_eq!(cache.get(&"B".to_string()), Some(&2));
    assert_eq!(cache.get(&"C".to_string()), Some(&3));
}

#[test]
fn test_update_existing() {
    let mut cache = Cache::<String, i32>::new(2);
    cache.put("A".to_string(), 1);
    cache.put("B".to_string(), 2);
    cache.put("A".to_string(), 3);
    assert_eq!(cache.get(&"A".to_string()), Some(&3));
    assert_eq!(cache.get(&"B".to_string()), Some(&2));
}

#[test]
fn test_persistence() {
    let filename = "test_cache.txt";
    {
        let mut cache = Cache::<String, i32>::new_persistent(2, filename).unwrap();
        cache.put("test".to_string(), 123);
        cache.put("test2".to_string(), 456);
        cache.save_to_file(filename).unwrap();
    }

    {
        let mut cache = Cache::<String, i32>::new_persistent(2, filename).unwrap();
        assert_eq!(cache.get(&"test".to_string()), Some(&123));
        assert_eq!(cache.get(&"test2".to_string()), Some(&456));
    }

    std::fs::remove_file(filename).unwrap();
}
