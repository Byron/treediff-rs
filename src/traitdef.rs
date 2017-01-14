
pub trait Value: PartialEq<Self> {
    type Item;
    fn items(&self) -> Option<Vec<Self::Item>>;
}
