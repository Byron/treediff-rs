use traitdef::Value;
use super::JsonKey;
use serde_json::{Map, Value as Json};
use merge::Mergeable;
use std::mem;

impl Value for Json {
    type Item = Json;
    type Key = JsonKey;
    fn items<'a>(&'a self) -> Option<Box<Iterator<Item = (Self::Key, &'a Self::Item)> + 'a>> {
        match *self {
            Json::String(_) | Json::Number(_) | Json::Bool(_) | Json::Null => None,
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
            let last_key_index = keys.len() - 1;
            let object_or_value = |index| if index == last_key_index {
                v.clone()
            } else {
                Json::Object(Map::new())
            };
            fn runup_array_or_value<'a>(array: &'a mut Vec<Json>,
                                        target_index: usize,
                                        key_index: usize,
                                        last_key_index: usize,
                                        v: &Json)
                                        -> &'a mut Json {
                for _ in array.len()..target_index {
                    array.push(Json::Null);
                }
                let value = if key_index == last_key_index {
                    v.clone()
                } else {
                    Json::Null
                };
                if target_index == array.len() {
                    array.push(value);
                } else {
                    array[target_index] = value;
                }
                &mut array[target_index]
            };
            for (i, k) in keys.iter().enumerate() {
                c = match *k {
                    JsonKey::String(ref k) => {
                        match {
                            c
                        } {
                            &mut Json::Object(ref mut obj) => {
                                if obj.contains_key(k) {
                                    let obj = obj.get_mut(k).expect("map to work");
                                    if i == last_key_index {
                                        *obj = v.clone();
                                        return;
                                    }
                                    obj
                                } else {
                                    obj.insert(k.clone(), object_or_value(i));
                                    obj.get_mut(k).expect("map to work")
                                }
                            }
                            c @ &mut Json::String(_) |
                            c @ &mut Json::Number(_) |
                            c @ &mut Json::Bool(_) |
                            c @ &mut Json::Null |
                            c @ &mut Json::Array(_) => {
                                mem::replace(c,
                                             Json::Object({
                                                 let mut o = Map::new();
                                                 o.insert(k.clone(), object_or_value(i));
                                                 o
                                             }));
                                if i == last_key_index {
                                    return;
                                }
                                match c {
                                    &mut Json::Object(ref mut obj) => {
                                        obj.get_mut(k).expect("previous insertion")
                                    }
                                    _ => unreachable!(),
                                }
                            }
                        }
                    }
                    JsonKey::Index(idx) => {
                        match {
                            c
                        } {
                            &mut Json::Array(ref mut a) => {
                                runup_array_or_value(a, idx, i, last_key_index, v)
                            }
                            c @ &mut Json::String(_) |
                            c @ &mut Json::Number(_) |
                            c @ &mut Json::Bool(_) |
                            c @ &mut Json::Null |
                            c @ &mut Json::Object(_) => {
                                let mut a = Vec::new();
                                runup_array_or_value(&mut a, idx, i, last_key_index, v);
                                mem::replace(c, Json::Array(a));
                                if i == last_key_index {
                                    return;
                                }
                                match c {
                                    &mut Json::Array(ref mut a) => {
                                        a.get_mut(idx).expect("previous insertion")
                                    }
                                    _ => unreachable!(),
                                }
                            }
                        }
                    }
                }
            }
        }
    }


    fn remove(&mut self, keys: &[Self::Key]) {
        let mut c = self;
        let last_key_index = keys.len() - 1;
        for (i, k) in keys.iter().enumerate() {
            c = match *k {
                JsonKey::String(ref k) => {
                    match {
                        c
                    } {
                        &mut Json::Object(ref mut obj) => {
                            if i == last_key_index {
                                obj.remove(k);
                                return;
                            } else {
                                match obj.get_mut(k) {
                                    Some(json) => json,
                                    None => return,
                                }
                            }
                        }
                        _ => return,
                    }
                }
                JsonKey::Index(idx) => {
                    match {
                        c
                    } {
                        &mut Json::Array(ref mut a) => {
                            if i == last_key_index {
                                a.remove(idx);
                                return;
                            } else {
                                match a.get_mut(idx) {
                                    Some(json) => json,
                                    None => return,
                                }
                            }
                        }
                        _ => return,
                    }
                }
            }
        }
    }
}
