pub trait Value: PartialEq<Self> {
    type Item;
    type Key;
    fn items<'a>(&'a self) -> Option<Box<Iterator<Item = (Self::Key, &'a Self::Item)> + 'a>>;
}

pub trait Delegate<'a, V> {
    fn removed(&mut self, _v: &'a V) {}
    fn added(&mut self, _v: &'a V) {}
    fn unchanged(&mut self, _v: &'a V) {}
    fn modified(&mut self, _v1: &'a V, _v2: &'a V) {}
}
