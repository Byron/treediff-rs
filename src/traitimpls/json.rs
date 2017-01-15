use traitdef::Value;
use rustc_serialize::json::Json;

#[derive(Debug, Clone, PartialEq)]
pub enum JsonKey {
    String(String),
}

impl Value for Json {
    type Item = Json;
    type Key = JsonKey;
    fn items<'a>(&'a self) -> Option<Box<Iterator<Item = (Self::Key, &'a Self::Item)> + 'a>> {
        match *self {
            Json::String(_) | Json::U64(_) | Json::I64(_) | Json::F64(_) | Json::Boolean(_) |
            Json::Null => None,
            Json::Object(ref inner) => {
                Some(Box::new(inner.iter().map(|(s, v)| (JsonKey::String(s.clone()), v))))
            }
            _ => unimplemented!(),

        }
    }
}
