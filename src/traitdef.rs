
pub trait Value: PartialEq<Self> {
    type Item;
    type Key;
    fn items(&self) -> Option<Vec<(Self::Key, Self::Item)>>;
}
