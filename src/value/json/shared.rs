#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum JsonKey {
    Index(usize),
    String(String),
}
