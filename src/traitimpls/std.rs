use traitdef::Value;

impl<'a> Value<'a> for String {
    type Item = Self;
    fn items(&self) -> Option<Box<Iterator<Item = Self>>> {
        None
    }
}

impl<'a, T> Value<'a> for Vec<T>
    where T: PartialEq + 'a
{
    type Item = &'a T;
    fn items(&'a self) -> Option<Box<Iterator<Item = Self::Item> + 'a>> {
        Some(Box::new(self.iter()))
    }
}
