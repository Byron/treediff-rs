use traitdef::Delegate;

pub struct Merger<K, V, R> {
    cursor: Vec<K>,
    inner: V,
    resolve: R,
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

impl<'a, K, V, R> Delegate<'a, K, V> for Merger<K, V, R>
    where V: Mergeable<Key = K, Item = V> + 'a,
          K: Clone,
          R: Fn(&'a V, &'a V) -> &'a V
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
    fn unchanged<'b>(&mut self, v: &'a V) {
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

impl<K, V, R> Merger<K, V, R> {
    pub fn into_inner(self) -> V {
        self.inner
    }
}

fn pick_new_value<'a, V>(prev: &'a V, new: &'a V) -> &'a V {
    new
}

impl<'a, M> From<M> for Merger<M::Key, M, fn(&'a M, &'a M) -> &'a M>
    where M: Mergeable + 'a
{
    fn from(v: M) -> Self {
        Merger {
            inner: v,
            cursor: Vec::new(),
            resolve: pick_new_value::<M> as fn(&'a M, &'a M) -> &'a M,
        }
    }
}
