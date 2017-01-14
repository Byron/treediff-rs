
pub trait Value<'a>: PartialEq<Self> {
    type Item;
    fn items(&'a self) -> Option<Box<Iterator<Item = Self::Item> + 'a>>;
}
