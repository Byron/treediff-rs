/// Represents a scalar value or an associative array
pub trait Value: PartialEq<Self> {
    /// The Value type itself
    type Item;
    /// The Key type used to find Values in a mapping
    type Key;
    /// Returns `None` if this is a scalar value, and an iterator yielding (Key, Value) pairs
    /// otherwise. It is entirely possible for it to yield no values though.
    fn items<'a>(&'a self) -> Option<Box<Iterator<Item = (Self::Key, &'a Self::Item)> + 'a>>;
}

/// The delegate receiving callbacks by the `diff` algorithm.
///
/// # Type Parameters
/// * `K` is the Key's type
/// * `V` is the Value's type
///
/// Methods will be called if...
pub trait Delegate<'a, K, V> {
    /// ... we recurse into the `Value` at the given `Key`
    ///
    /// Delegates should memoize the current Key path to be able to compute
    /// the full Key path when needed.
    fn push<'b>(&mut self, _k: &'b K) {}
    /// ... we have processed all items and leave the object previously `push`ed.
    fn pop(&mut self) {}
    /// ... the Value `v` at the given Key `k` should be removed.
    fn removed<'b>(&mut self, _k: &'b K, _v: &'a V) {}
    fn added<'b>(&mut self, _k: &'b K, _v: &'a V) {}
    fn unchanged<'b>(&mut self, _v: &'a V) {}
    fn modified<'b>(&mut self, _v1: &'a V, _v2: &'a V) {}
}
