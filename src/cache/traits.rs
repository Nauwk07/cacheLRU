/// Trait définissant les opérations de base d'un cache LRU
///
/// # Type Parameters
///
/// * `K` - Le type de la clé
/// * `V` - Le type de la valeur
pub trait LRUCache<K, V> {
    /// Ajoute ou met à jour une paire clé-valeur dans le cache
    ///
    /// Si la clé existe déjà, la valeur est mise à jour.
    /// Si le cache est plein, l'élément le moins récemment utilisé est supprimé.
    ///
    /// # Arguments
    ///
    /// * `key` - La clé à ajouter
    /// * `value` - La valeur associée à la clé
    fn put(&mut self, key: K, value: V);

    /// Récupère une référence à la valeur associée à la clé
    ///
    /// Met à jour l'ordre LRU en déplaçant l'élément en tête.
    ///
    /// # Arguments
    ///
    /// * `key` - La clé à rechercher
    ///
    /// # Returns
    ///
    /// * `Some(&V)` - Une référence à la valeur si la clé existe
    /// * `None` - Si la clé n'existe pas
    fn get(&mut self, key: &K) -> Option<&V>;
}
