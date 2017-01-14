use traitdef::Value;

impl Value for String {
    type Item = Self;
    fn items(&self) -> Option<Vec<Self::Item>> {
        None
    }
}

impl<T> Value for Vec<T>
    where T: PartialEq + Clone
{
    type Item = T;
    fn items(&self) -> Option<Vec<T>> {
        Some(self.iter().cloned().collect())
    }
}
