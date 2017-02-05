pub trait Value: PartialEq<Self> {
    type Item;
    type Key;
    fn items<'a>(&'a self) -> Option<Box<Iterator<Item = (Self::Key, &'a Self::Item)> + 'a>>;
}

pub trait Delegate<'a, K, V> {
    fn push<'b>(&mut self, _k: &'b K) {}
    fn pop(&mut self) {}
    fn removed<'b>(&mut self, _k: Option<&'b K>, _v: &'a V) {}
    fn added<'b>(&mut self, _k: Option<&'b K>, _v: &'a V) {}
    fn unchanged<'b>(&mut self, _v: &'a V) {}
    fn modified<'b>(&mut self, _k: Option<&'b K>, _v1: &'a V, _v2: &'a V) {}
}
