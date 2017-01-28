use traitdef::Delegate;

#[derive(Debug, PartialEq)]
pub enum ChangeType<'a, V: 'a> {
    Removed(&'a V),
    Added(&'a V),
    Unchanged(&'a V),
    Modified(&'a V, &'a V),
}

#[derive(Debug, PartialEq)]
pub struct Recorder<'a, V: 'a> {
    pub calls: Vec<ChangeType<'a, V>>,
}

impl<'a, V> Default for Recorder<'a, V> {
    fn default() -> Self {
        Recorder { calls: Vec::new() }
    }
}

impl<'a, V> Delegate<'a, V> for Recorder<'a, V> {
    fn removed(&mut self, v: &'a V) {
        self.calls.push(ChangeType::Removed(v));
    }
    fn added(&mut self, v: &'a V) {
        self.calls.push(ChangeType::Added(v));
    }
    fn unchanged(&mut self, v: &'a V) {
        self.calls.push(ChangeType::Unchanged(v));
    }
    fn modified(&mut self, v1: &'a V, v2: &'a V) {
        self.calls.push(ChangeType::Modified(v1, v2));
    }
}
