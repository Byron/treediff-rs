use traitdef::Value;
use rustc_serialize::json::Json;

impl Value for Json {
    type Item = Json;
    type Key = String;
    fn items<'a>(&'a self) -> Option<Box<Iterator<Item = (Self::Key, &'a Self::Item)> + 'a>> {
        match *self {
            Json::Object(ref inner) => Some(Box::new(inner.iter().map(|(s, v)| (s.clone(), v)))),
            _ => unimplemented!(),

        }
    }
}
