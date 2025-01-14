use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Display;
use std::str::FromStr;
use std::fs::{ File, OpenOptions };
use std::io::{ self, BufRead, BufReader, Write };
use super::node::Node;
use super::LRUCache;

/// Cache LRU (Least Recently Used) avec support optionnel de persistance
///
/// # Type Parameters
///
/// * `K` - Le type de la clé, doit implémenter `Eq + Hash + Clone + Display + FromStr`
/// * `V` - Le type de la valeur, doit implémenter `Display + FromStr`
///
/// # Examples
///
/// ```
/// use cache_lru::cache::{Cache, LRUCache};
///
/// // Création d'un cache simple
/// let mut cache = Cache::<String, i32>::new(2);
/// cache.put("A".to_string(), 1);
/// cache.put("B".to_string(), 2);
///
/// assert_eq!(cache.get(&"A".to_string()), Some(&1));
///
/// // Le cache supprime automatiquement les éléments les moins utilisés
/// cache.put("C".to_string(), 3);
/// assert_eq!(cache.get(&"B".to_string()), None);
/// ```
#[derive(Debug)]
pub struct Cache<K: Eq + Hash + Clone + Display + FromStr, V: Display + FromStr> {
    capacity: usize,
    map: HashMap<K, (V, Node<K>)>,
    head: Option<K>,
    tail: Option<K>,
    filename: Option<String>,
}

impl<K: Eq + Hash + Clone + Display + FromStr, V: Display + FromStr> Cache<K, V> {
    /// Crée un nouveau cache avec une capacité donnée
    ///
    /// # Arguments
    ///
    /// * `capacity` - Nombre maximum d'éléments dans le cache
    ///
    /// # Examples
    ///
    /// ```
    /// use cache_lru::cache::Cache;
    /// let cache = Cache::<String, i32>::new(3);
    /// ```
    pub fn new(capacity: usize) -> Self {
        Cache {
            map: HashMap::new(),
            head: None,
            tail: None,
            capacity,
            filename: None,
        }
    }

    /// Crée un nouveau cache persistant avec une capacité donnée
    ///
    /// Le cache sera automatiquement sauvegardé dans le fichier spécifié
    /// après chaque modification.
    ///
    /// # Arguments
    ///
    /// * `capacity` - Nombre maximum d'éléments dans le cache
    /// * `filename` - Chemin du fichier de persistance
    ///
    /// # Returns
    ///
    /// * `io::Result<Self>` - Le cache créé ou une erreur
    ///
    /// # Examples
    ///
    /// ```
    /// use cache_lru::cache::Cache;
    /// let cache = Cache::<String, i32>::new_persistent(3, "cache.txt").unwrap();
    /// ```
    pub fn new_persistent(capacity: usize, filename: &str) -> io::Result<Self> {
        let mut cache = Cache {
            capacity,
            map: HashMap::new(),
            head: None,
            tail: None,
            filename: Some(filename.to_string()),
        };

        // Charger les données existantes si le fichier existe
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

    /// Sauvegarde le contenu du cache dans un fichier
    ///
    /// # Arguments
    ///
    /// * `filename` - Chemin du fichier de sauvegarde
    ///
    /// # Returns
    ///
    /// * `io::Result<()>` - Ok si la sauvegarde réussit, Err sinon
    pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(filename)?;

        for (key, (value, _)) in &self.map {
            writeln!(file, "{}\t{}", key, value)?;
        }

        Ok(())
    }

    /// Supprime un nœud de la liste chaînée
    fn remove_node(&mut self, key: &K) {
        if let Some((_, node)) = self.map.get(key) {
            let prev = node.prev.clone();
            let next = node.next.clone();

            match (prev.clone(), next.clone()) {
                (Some(prev_key), Some(next_key)) => {
                    // Mise à jour des liens pour le nœud précédent
                    if let Some((_, node)) = self.map.get_mut(&prev_key) {
                        node.next = Some(next_key.clone());
                    }
                    // Mise à jour des liens pour le nœud suivant
                    if let Some((_, node)) = self.map.get_mut(&next_key) {
                        node.prev = Some(prev_key);
                    }
                }
                (None, Some(next_key)) => {
                    // Le nœud était en tête
                    self.head = Some(next_key.clone());
                    if let Some((_, node)) = self.map.get_mut(&next_key) {
                        node.prev = None;
                    }
                }
                (Some(prev_key), None) => {
                    // Le nœud était en queue
                    self.tail = Some(prev_key.clone());
                    if let Some((_, node)) = self.map.get_mut(&prev_key) {
                        node.next = None;
                    }
                }
                (None, None) => {
                    // Le nœud était seul
                    self.head = None;
                    self.tail = None;
                }
            }
        }
    }

    /// Supprime le nœud en queue de liste
    fn remove_tail(&mut self) {
        if let Some(tail_key) = self.tail.clone() {
            self.remove_node(&tail_key);
            self.map.remove(&tail_key);
        }
    }

    /// Ajoute un nœud en tête de liste
    fn add_to_head(&mut self, key: K) {
        if let Some(old_head) = self.head.clone() {
            if let Some((_, node)) = self.map.get_mut(&old_head) {
                node.prev = Some(key.clone());
            }
        } else {
            self.tail = Some(key.clone());
        }

        if let Some((_, node)) = self.map.get_mut(&key) {
            node.next = self.head.clone();
            node.prev = None;
        }

        self.head = Some(key);
    }

    /// Déplace un nœud existant en tête de liste
    fn move_to_head(&mut self, key: &K) {
        if self.head.as_ref() != Some(key) {
            let key_clone = key.clone();
            self.remove_node(key);
            self.add_to_head(key_clone);
        }
    }
}

impl<K: Eq + Hash + Clone + Display + FromStr, V: Display + FromStr> LRUCache<K, V>
for Cache<K, V> {
    fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.map.remove(&key);
            self.remove_node(&key);
        } else if self.map.len() == self.capacity {
            self.remove_tail();
        }

        self.map.insert(key.clone(), (
            value,
            Node {
                prev: None,
                next: None,
            },
        ));
        self.add_to_head(key);

        // Sauvegarder après chaque modification si persistent
        if let Some(filename) = &self.filename {
            let _ = self.save_to_file(filename);
        }
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            self.move_to_head(key);
            if let Some((value, _)) = self.map.get(key) {
                return Some(value);
            }
        }
        None
    }
}
