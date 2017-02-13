use traitdef::Value;
use super::Key;
use serde_yaml::{Value as SerdeYaml, to_string};

impl Value for SerdeYaml {
    type Item = SerdeYaml;
    type Key = Key;
    fn items<'a>(&'a self) -> Option<Box<Iterator<Item = (Self::Key, &'a Self::Item)> + 'a>> {
        match *self {
            SerdeYaml::String(_) |
            SerdeYaml::I64(_) |
            SerdeYaml::F64(_) |
            SerdeYaml::Bool(_) |
            SerdeYaml::Null => None,
            SerdeYaml::Sequence(ref inner) => {
                Some(Box::new(inner.iter().enumerate().map(|(i, v)| (Key::Index(i), v))))
            }
            SerdeYaml::Mapping(ref inner) => {
                Some(Box::new(inner.iter().map(|(k, v)| {
                    (Key::String(to_string(k)
                             .expect("yaml value to serialize into yaml correctly")
                                     [4..]
                         .to_owned()),
                     v)
                })))
            }
        }
    }
}
