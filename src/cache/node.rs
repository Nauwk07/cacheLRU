#[derive(Debug)]
pub(crate) struct Node<K> {
    pub prev: Option<K>,
    pub next: Option<K>,
}
