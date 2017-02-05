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

fn mk<'b, K>(c: &Vec<K>, k: Option<&'b K>) -> Vec<K>
    where K: Clone
{
    let mut c = c.clone();
    match k {
        Some(k) => {
            c.push(k.clone());
            c
        }
        None => c,
    }
}

impl<'a, K, V> Delegate<'a, K, V> for Recorder<'a, K, V>
    where K: Clone
{
    fn push<'b>(&mut self, k: &'b K) {
        self.cursor.push(k.clone())
    }
    fn pop(&mut self) {
        self.cursor.pop();
    }
    fn removed<'b>(&mut self, k: &'b K, v: &'a V) {
        self.calls.push(ChangeType::Removed(mk(&self.cursor, Some(k)), v));
    }
    fn added<'b>(&mut self, k: &'b K, v: &'a V) {
        self.calls.push(ChangeType::Added(mk(&self.cursor, Some(k)), v));
    }
    fn unchanged<'b>(&mut self, v: &'a V) {
        self.calls.push(ChangeType::Unchanged(self.cursor.clone(), v));
    }
    fn modified<'b>(&mut self, v1: &'a V, v2: &'a V) {
        self.calls.push(ChangeType::Modified(mk(&self.cursor, None), v1, v2));
    }
}
