/// A representation of all key types typical Value types will assume.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum Key {
    /// An array index
    Index(usize),
    /// A string index for mappings
    String(String),
}
