use traitdef::Value;

impl Value for String {
    fn items(&self) -> Option<Box<Iterator<Item = Self>>> {
        None
    }
}

impl<T> Value<T> for Vec<T>
    where T: PartialEq + Clone,
          Vec<T>: PartialEq<T>
{
    fn items<'a>(&'a self) -> Option<Box<Iterator<Item = T> + 'a>> {
        Some(Box::new(self.iter().cloned()))
    }
}
