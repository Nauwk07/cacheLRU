use std::io;

pub trait Persistence<K, V> {
    fn save_to_file(&self, filename: &str) -> io::Result<()>;
    fn load_from_file(filename: &str) -> io::Result<Vec<(K, V)>>;
}
