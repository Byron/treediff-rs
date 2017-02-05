use traitdef::Delegate;
use std::borrow::Cow;

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
    where V: Mergeable<Key = K, Item = V> + Clone + 'a,
          K: Clone,
          R: Fn(Cow<'a, V>, Cow<'a, V>) -> Cow<'a, V>
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
        self.inner.set(&appended(&self.cursor, k), v);
    }
    fn unchanged<'b>(&mut self, v: &'a V) {
        self.inner.set(&self.cursor, v)
    }
    fn modified<'b>(&mut self, k: Option<&'b K>, prev: &'a V, new: &'a V) {
        let v = (self.resolve)(Cow::Borrowed(prev), Cow::Borrowed(new));
        self.inner.set(&appended(&self.cursor, k), &v);
    }
}

pub trait Mergeable {
    type Key;
    type Item;

    fn set(&mut self, keys: &[Self::Key], new: &Self::Item);
    fn remove(&mut self, keys: &[Self::Key]);
}

impl<K, V, R> Merger<K, V, R> {
    pub fn into_inner(self) -> V {
        self.inner
    }
}

fn pick_new_value<'a, V: Clone>(_old: Cow<'a, V>, new: Cow<'a, V>) -> Cow<'a, V> {
    new
}


impl<'a, M> From<M> for Merger<M::Key, M, fn(Cow<'a, M>, Cow<'a, M>) -> Cow<'a, M>>
    where M: Mergeable + 'a + Clone
{
    fn from(v: M) -> Self {
        Merger {
            inner: v,
            cursor: Vec::new(),
            resolve: pick_new_value::<M> as fn(Cow<'a, M>, Cow<'a, M>) -> Cow<'a, M>,
        }
    }
}
