
pub trait Value<Rhs = Self> where Self: PartialEq<Rhs> {
    fn is_scalar(&self) -> bool;
}