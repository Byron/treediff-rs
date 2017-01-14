use traitdef::Value;

impl Value for String {
    type Item = Self;
    fn items(&self) -> Option<Box<Iterator<Item = Self>>> {
        None
    }
}

impl<T> Value for Vec<T>
    where T: PartialEq + Value + Clone,
          Vec<T>: PartialEq<T>
{
    type Item = T;
    fn items(&self) -> Option<Box<Iterator<Item = Self::Item>>> {
        Some(Box::new(self.iter().cloned()))
    }
}
