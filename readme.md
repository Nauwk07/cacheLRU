# Cache LRU (Least Recently Used)

Une implémentation en Rust d'un cache LRU (Least Recently Used) avec support optionnel de persistance sur disque.

## Caractéristiques

- Cache générique supportant différents types de clés et valeurs
- Implémentation du pattern LRU (suppression des éléments les moins récemment utilisés)
- Support optionnel de persistance sur disque
- Performance optimisée pour les opérations de base

## Installation

Ajoutez cette dépendance à votre `Cargo.toml` :

```toml
[dependencies]
cache-lru = "0.1.0"
```

## Utilisation

### Cache Simple

```rust
use cache_lru::cache::{Cache, LRUCache};

// Création d'un cache avec une capacité de 3 éléments
let mut cache = Cache::<String, i32>::new(3);

// Ajout d'éléments
cache.put("A".to_string(), 1);
cache.put("B".to_string(), 2);

// Récupération d'une valeur
if let Some(value) = cache.get(&"A".to_string()) {
    println!("Valeur de A : {}", value);
}
```

### Cache Persistant

```rust
use cache_lru::cache::{Cache, LRUCache};

// Création d'un cache persistant
let mut cache = Cache::<String, i32>::new_persistent(3, "mon_cache.txt").unwrap();

// Les données sont automatiquement sauvegardées après chaque modification
cache.put("X".to_string(), 10);
cache.put("Y".to_string(), 20);

// Les données seront rechargées à la prochaine initialisation du cache
```

## Fonctionnement

Le cache implémente une stratégie LRU (Least Recently Used) qui :

- Maintient un ordre d'utilisation des éléments
- Supprime automatiquement les éléments les moins récemment utilisés lorsque la capacité est atteinte
- Met à jour l'ordre lors de chaque accès aux éléments

## API

### Trait LRUCache

```rust
pub trait LRUCache<K, V> {
    fn put(&mut self, key: K, value: V);
    fn get(&mut self, key: &K) -> Option<&V>;
}
```

### Struct Cache

La structure principale requiert que les types `K` et `V` implémentent les traits nécessaires pour la sérialisation et la comparaison :

- `Eq`
- `Hash`
- `Clone`
- `Display`
- `FromStr`

## Licence

MIT
