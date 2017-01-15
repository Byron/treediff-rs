
pub trait Value: PartialEq<Self> {
    type Item;
    type Key;
    fn items<'a>(&'a self) -> Option<Box<Iterator<Item=(Self::Key, &'a Self::Item)> + 'a>>;
}
