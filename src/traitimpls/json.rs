use traitdef::Value;
use rustc_serialize::json::Json;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum JsonKey {
    Index(usize),
    String(String),
}

impl Value for Json {
    type Item = Json;
    type Key = JsonKey;
    fn items<'a>(&'a self) -> Option<Box<Iterator<Item = (Self::Key, &'a Self::Item)> + 'a>> {
        match *self {
            Json::String(_) | Json::U64(_) | Json::I64(_) | Json::F64(_) | Json::Boolean(_) |
            Json::Null => None,
            Json::Array(ref inner) => {
                Some(Box::new(inner.iter().enumerate().map(|(i, v)| (JsonKey::Index(i), v))))
            }
            Json::Object(ref inner) => {
                Some(Box::new(inner.iter().map(|(s, v)| (JsonKey::String(s.clone()), v))))
            }
        }
    }
}
