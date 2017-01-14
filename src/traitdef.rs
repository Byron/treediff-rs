
pub trait Value<Rhs = Self> {
    fn is_scalar(&self) -> bool;
    fn eq(&self, rhs: &Rhs) -> bool;
}