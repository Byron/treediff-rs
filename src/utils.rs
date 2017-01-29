use traitdef::Delegate;

#[derive(Debug, PartialEq)]
pub enum ChangeType<'a, K, V: 'a> {
    Removed(Vec<K>, &'a V),
    Added(Vec<K>, &'a V),
    Unchanged(Vec<K>, &'a V),
    Modified(Vec<K>, &'a V, &'a V),
}

#[derive(Debug, PartialEq)]
pub struct Recorder<'a, K, V: 'a> {
    pub cursor: Vec<K>,
    pub calls: Vec<ChangeType<'a, K, V>>,
}

impl<'a, K, V> Default for Recorder<'a, K, V> {
    fn default() -> Self {
        Recorder {
            cursor: Vec::new(),
            calls: Vec::new(),
        }
    }
}

fn mk<K>(c: &Vec<K>, k: Option<K>) -> Vec<K>
    where K: Clone
{
    let mut c = c.clone();
    match k {
        Some(k) => {
            c.push(k);
            c
        }
        None => c,
    }
}

impl<'a, K, V> Delegate<'a, K, V> for Recorder<'a, K, V>
    where K: Clone
{
    fn push(&mut self, k: K) {
        self.cursor.push(k)
    }
    fn pop(&mut self) {
        self.cursor.pop();
    }
    fn removed(&mut self, k: Option<K>, v: &'a V) {
        self.calls.push(ChangeType::Removed(mk(&self.cursor, k), v));
    }
    fn added(&mut self, k: Option<K>, v: &'a V) {
        self.calls.push(ChangeType::Added(mk(&self.cursor, k), v));
    }
    fn unchanged(&mut self, k: Option<K>, v: &'a V) {
        self.calls.push(ChangeType::Unchanged(mk(&self.cursor, k), v));
    }
    fn modified(&mut self, k: Option<K>, v1: &'a V, v2: &'a V) {
        self.calls.push(ChangeType::Modified(mk(&self.cursor, k), v1, v2));
    }
}
