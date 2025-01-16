use std::collections::{ HashMap, VecDeque };
use std::hash::Hash;
use std::fmt::Display;
use std::str::FromStr;
use std::fs::{ File, OpenOptions };
use std::io::{ self, BufRead, BufReader, Write };
use super::LRUCache;

#[derive(Debug)]
pub struct Cache<K: Eq + Hash + Clone + Display + FromStr, V: Display + FromStr> {
    capacity: usize,
    map: HashMap<K, V>,
    order: VecDeque<K>,
    filename: Option<String>,
}

impl<K: Eq + Hash + Clone + Display + FromStr, V: Display + FromStr> Cache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Cache {
            capacity,
            map: HashMap::new(),
            order: VecDeque::with_capacity(capacity),
            filename: None,
        }
    }

    pub fn new_persistent(capacity: usize, filename: &str) -> io::Result<Self> {
        let mut cache = Cache {
            capacity,
            map: HashMap::new(),
            order: VecDeque::with_capacity(capacity),
            filename: Some(filename.to_string()),
        };

        if let Ok(file) = File::open(filename) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line?;
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() == 2 {
                    if let (Ok(key), Ok(value)) = (K::from_str(parts[0]), V::from_str(parts[1])) {
                        cache.put(key, value);
                    }
                }
            }
        }

        Ok(cache)
    }

    pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(filename)?;

        for key in &self.order {
            if let Some(value) = self.map.get(key) {
                writeln!(file, "{}\t{}", key, value)?;
            }
        }

        Ok(())
    }
}

impl<K: Eq + Hash + Clone + Display + FromStr, V: Display + FromStr> LRUCache<K, V>
for Cache<K, V> {
    fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.order.retain(|k| k != &key);
        } else if self.map.len() == self.capacity {
            if let Some(old_key) = self.order.pop_front() {
                self.map.remove(&old_key);
            }
        }

        self.map.insert(key.clone(), value);
        self.order.push_back(key);

        if let Some(filename) = &self.filename {
            let _ = self.save_to_file(filename);
        }
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            self.order.retain(|k| k != key);
            self.order.push_back(key.clone());
            self.map.get(key)
        } else {
            None
        }
    }
}
