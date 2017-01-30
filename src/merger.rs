use traitdef::Delegate;

pub struct Merger<K, V> {
    cursor: Vec<K>,
    inner: V,
}

fn appended<'b, K>(keys: &Vec<K>, k: Option<&'b K>) -> Vec<K>
    where K: Clone
{
    let mut keys = keys.clone();
    if let Some(k) = k {
        keys.push(k.clone());
    }
    keys
}

impl<'a, K, V> Delegate<'a, K, V> for Merger<K, V>
    where V: Mergeable<Key = K, Item = V>,
          K: Clone
{
    fn push<'b>(&mut self, k: &'b K) {
        self.cursor.push(k.clone());
    }
    fn pop(&mut self) {
        self.cursor.pop();
    }
    fn removed<'b>(&mut self, k: Option<&'b K>, _v: &'a V) {
        self.inner.remove(&appended(&self.cursor, k));
    }
    fn added<'b>(&mut self, k: Option<&'b K>, v: &'a V) {
        self.inner.set(&appended(&self.cursor, k), v, None);
    }
    fn unchanged<'b>(&mut self, k: Option<&'b K>, v: &'a V) {
        assert!(k.is_none());
        self.inner.set(&self.cursor, v, None)
    }
    fn modified<'b>(&mut self, k: Option<&'b K>, v1: &'a V, v2: &'a V) {
        self.inner.set(&appended(&self.cursor, k), v2, Some(v1));
    }
}

pub trait Mergeable {
    type Key;
    type Item;

    fn set(&mut self, keys: &[Self::Key], new: &Self::Item, previous: Option<&Self::Item>);
    fn remove(&mut self, keys: &[Self::Key]);
}

impl<K, V> Merger<K, V> {
    pub fn into_inner(self) -> V {
        self.inner
    }
}

impl<M> From<M> for Merger<M::Key, M>
    where M: Mergeable
{
    fn from(v: M) -> Self {
        Merger {
            inner: v,
            cursor: Vec::new(),
        }
    }
}
