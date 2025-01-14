use std::io;

/// Trait définissant les opérations de persistance d'un cache
///
/// # Type Parameters
///
/// * `K` - Le type de la clé
/// * `V` - Le type de la valeur
pub trait Persistence<K, V> {
    /// Sauvegarde le contenu du cache dans un fichier
    ///
    /// # Arguments
    ///
    /// * `filename` - Le chemin du fichier de sauvegarde
    ///
    /// # Returns
    ///
    /// * `io::Result<()>` - Ok si la sauvegarde réussit, Err sinon
    fn save_to_file(&self, filename: &str) -> io::Result<()>;

    /// Charge le contenu d'un fichier dans le cache
    ///
    /// # Arguments
    ///
    /// * `filename` - Le chemin du fichier à charger
    ///
    /// # Returns
    ///
    /// * `io::Result<Vec<(K, V)>>` - Les paires clé-valeur chargées si succès
    fn load_from_file(filename: &str) -> io::Result<Vec<(K, V)>>;
}
