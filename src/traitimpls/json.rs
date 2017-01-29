use traitdef::Value;
use rustc_serialize::json::{Object, Json};
use merger::Mergeable;
use std::mem;

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

impl Mergeable for Json {
    type Key = JsonKey;
    type Item = Json;

    fn set(&mut self, keys: &[Self::Key], v: &Self::Item) {
        if keys.len() == 0 {
            *self = v.clone();
        } else {
            let mut c = self;
            let object_or_value = |index| {
                if index == keys.len() - 1 {
                    v.clone()
                } else {
                    Json::Object(Object::new())
                }
            };
            for (i, k) in keys.iter().enumerate() {
                c = match *k {
                    JsonKey::String(ref k) => {
                        match {
                            c
                        } {
                            &mut Json::Object(ref mut obj) => {
                                obj.entry(k.clone()).or_insert_with(|| object_or_value(i))
                            }
                            c @ &mut Json::String(_) |
                            c @ &mut Json::F64(_) |
                            c @ &mut Json::Boolean(_) |
                            c @ &mut Json::Null |
                            c @ &mut Json::U64(_) |
                            c @ &mut Json::Array(_) |
                            c @ &mut Json::I64(_) => {
                                mem::replace(c,
                                             Json::Object({
                                                 let mut o = Object::new();
                                                 o.insert(k.clone(), object_or_value(i));
                                                 o
                                             }));
                                c
                            }
                        }
                    }
                    _ => panic!("handle JsonKey::Index"),
                }
            }
        }
    }
}
