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
    pub calls: Vec<ChangeType<'a, K, V>>,
}

impl<'a, K, V> Default for Recorder<'a, K, V> {
    fn default() -> Self {
        Recorder { calls: Vec::new() }
    }
}

fn mk<K>(_k: Option<K>) -> Vec<K> {
    Vec::new()
}

impl<'a, K, V> Delegate<'a, K, V> for Recorder<'a, K, V> {
    fn removed(&mut self, k: Option<K>, v: &'a V) {
        self.calls.push(ChangeType::Removed(mk(k), v));
    }
    fn added(&mut self, k: Option<K>, v: &'a V) {
        self.calls.push(ChangeType::Added(mk(k), v));
    }
    fn unchanged(&mut self, k: Option<K>, v: &'a V) {
        self.calls.push(ChangeType::Unchanged(mk(k), v));
    }
    fn modified(&mut self, k: Option<K>, v1: &'a V, v2: &'a V) {
        self.calls.push(ChangeType::Modified(mk(k), v1, v2));
    }
}
