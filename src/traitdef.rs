
pub trait Value<T=Self>: PartialEq<Self>
{
    fn items(&self) -> Option<Box<Iterator<Item=T>>>;
}
