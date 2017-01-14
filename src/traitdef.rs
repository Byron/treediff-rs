
pub trait Value<Rhs = Self>
    where Self: PartialEq<Rhs>
{
    type Item;
    fn items(&self) -> Option<Box<Iterator<Item = Self::Item>>>;
}
