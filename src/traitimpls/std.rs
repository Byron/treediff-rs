use traitdef::Value;

impl Value for String {
}

impl<T> Value for Vec<T>
    where
        T: PartialEq,
        Vec<T>: PartialEq<T>
{
}
