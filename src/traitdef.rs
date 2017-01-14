
pub trait Value<'a, Rhs = Self> where Rhs: PartialEq<Self> {
    type Item;
    fn equals(&self, rhs: &Rhs) -> bool {
        *rhs == *self
    }
    fn items(&'a self) -> Option<Box<Iterator<Item = Self::Item> + 'a>>;
}
