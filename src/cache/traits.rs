pub trait LRUCache<K, V> {
    fn put(&mut self, key: K, value: V);
    fn get(&mut self, key: &K) -> Option<&V>;
}
