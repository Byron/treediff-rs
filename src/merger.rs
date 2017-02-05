use traitdef::Delegate;
use std::borrow::Cow;

pub struct Merger<K, V, C, R> {
    cursor: Vec<K>,
    inner: V,
    resolve_conflict: C,
    handle_removal: R,
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

impl<'a, K, V, C, R> Delegate<'a, K, V> for Merger<K, V, C, R>
    where V: Mergeable<Key = K, Item = V> + Clone + 'a,
          K: Clone,
          C: Fn(Cow<'a, V>, Cow<'a, V>) -> Cow<'a, V>,
          R: Fn(&[K], Cow<'a, V>) -> Option<Cow<'a, V>>
{
    fn push<'b>(&mut self, k: &'b K) {
        self.cursor.push(k.clone());
    }
    fn pop(&mut self) {
        self.cursor.pop();
    }
    fn removed<'b>(&mut self, k: Option<&'b K>, v: &'a V) {
        let keys = appended(&self.cursor, k);
        match (self.handle_removal)(&keys, Cow::Borrowed(v)) {
            Some(nv) => self.inner.set(&keys, &nv),
            None => self.inner.remove(&keys),
        }
    }
    fn added<'b>(&mut self, k: Option<&'b K>, v: &'a V) {
        self.inner.set(&appended(&self.cursor, k), v);
    }
    fn unchanged<'b>(&mut self, v: &'a V) {
        self.inner.set(&self.cursor, v)
    }
    fn modified<'b>(&mut self, k: Option<&'b K>, prev: &'a V, new: &'a V) {
        let v = (self.resolve_conflict)(Cow::Borrowed(prev), Cow::Borrowed(new));
        self.inner.set(&appended(&self.cursor, k), &v);
    }
}

pub trait Mergeable {
    type Key;
    type Item;

    fn set(&mut self, keys: &[Self::Key], new: &Self::Item);
    fn remove(&mut self, keys: &[Self::Key]);
}

impl<K, V, C, R> Merger<K, V, C, R> {
    pub fn into_inner(self) -> V {
        self.inner
    }
}

pub fn pick_new<'a, V: Clone>(_old: Cow<'a, V>, new: Cow<'a, V>) -> Cow<'a, V> {
    new
}

pub fn pick_old<'a, V: Clone>(old: Cow<'a, V>, _new: Cow<'a, V>) -> Cow<'a, V> {
    old
}

pub fn drop_removed<'a, K, V: Clone>(_keys: &[K], _removed: Cow<'a, V>) -> Option<Cow<'a, V>> {
    None
}


impl<'a, V, C, R> Merger<V::Key, V, C, R>
    where V: Mergeable + 'a + Clone,
          C: Fn(Cow<'a, V>, Cow<'a, V>) -> Cow<'a, V>,
          R: Fn(&[V::Key], Cow<'a, V>) -> Option<Cow<'a, V>>
{
    pub fn with_resolver(v: V, c: C, r: R) -> Self {
        Merger {
            inner: v,
            cursor: Vec::new(),
            resolve_conflict: c,
            handle_removal: r,
        }
    }
}

impl<'a, V> From<V>
    for Merger<V::Key,
                                   V,
                                   fn(Cow<'a, V>, Cow<'a, V>) -> Cow<'a, V>,
                                   fn(&[V::Key], Cow<'a, V>) -> Option<Cow<'a, V>>>
    where V: Mergeable + 'a + Clone
{
    fn from(v: V) -> Self {
        Merger {
            inner: v,
            cursor: Vec::new(),
            resolve_conflict: pick_new::<V> as fn(Cow<'a, V>, Cow<'a, V>) -> Cow<'a, V>,
            handle_removal: drop_removed::<V::Key, V> as
                            fn(&[V::Key], Cow<'a, V>) -> Option<Cow<'a, V>>,
        }
    }
}
