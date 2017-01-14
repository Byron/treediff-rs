use traitdef::Value;

impl Value for String {
    type Item = Self;
    type Key = Self;
    fn items(&self) -> Option<Vec<(Self::Key, Self::Item)>> {
        None
    }
}

impl<T> Value for Vec<T>
    where T: PartialEq + Clone
{
    type Item = T;
    type Key = usize;
    fn items(&self) -> Option<Vec<(usize, T)>> {
        Some(self.iter().cloned().enumerate().collect())
    }
}
