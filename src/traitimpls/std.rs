use traitdef::Value;

impl Value for String {
    type Item = Self;
    type Key = Self;
    fn items<'a>(&'a self) -> Option<Box<Iterator<Item = (Self::Key, &'a Self::Item)> + 'a>> {
        None
    }
}

impl<T> Value for Vec<T>
    where T: PartialEq
{
    type Item = T;
    type Key = usize;
    fn items<'a>(&'a self) -> Option<Box<Iterator<Item = (Self::Key, &'a Self::Item)> + 'a>> {
        Some(Box::new(self.iter().enumerate()))
    }
}
