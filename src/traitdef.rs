
pub trait Value<T = Self>: PartialEq<Self> {
    fn items<'a>(&'a self) -> Option<Box<Iterator<Item = T> + 'a>>;
}
