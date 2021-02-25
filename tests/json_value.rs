extern crate treediff;

macro_rules! make_suite {
    ($bool:expr) => {
        use treediff::value::Key;
        use treediff::Value;

        #[test]
        fn scalar_values() {
            for v in &["null", "true", "1.23", "-1234456", "1234456", "\"string\""] {
                let j = make(v);
                assert!(j.items().is_none());
            }
        }

        #[test]
        fn array() {
            let j = make(r#"[null, true]"#);
            assert_eq!(
                j.items().unwrap().collect::<Vec<_>>(),
                vec![
                    (Key::Index(0), &ValueType::Null),
                    (Key::Index(1), &$bool(true))
                ]
            );
        }

        #[test]
        fn object() {
            let j = make(r#"{"a": null, "b": true}"#);
            assert_eq!(
                j.items().unwrap().collect::<Vec<_>>(),
                vec![
                    (Key::String("a".into()), &ValueType::Null),
                    (Key::String("b".into()), &$bool(true))
                ]
            );
        }

        #[test]
        fn empty_object() {
            let j = make(r#"{}"#);
            assert_eq!(j.items().unwrap().collect::<Vec<_>>(), vec![]);
        }
    };
}

#[cfg(feature = "with-yaml-rust")]
mod yaml_rust {
    extern crate yaml_rust;
    use self::yaml_rust::{Yaml as ValueType, YamlLoader};

    fn make(v: &str) -> ValueType {
        YamlLoader::load_from_str(v).unwrap().pop().unwrap()
    }

    make_suite!(ValueType::Boolean);
}

#[cfg(feature = "with-serde-yaml")]
mod serde_yaml {
    extern crate serde_yaml;
    use self::serde_yaml::{from_str, Value as ValueType};

    fn make(v: &str) -> ValueType {
        from_str(v).unwrap()
    }

    use treediff::value::Key;
    use treediff::Value;

    #[test]
    fn scalar_values() {
        for v in &["null", "true", "1.23", "-1234456", "1234456", "\"string\""] {
            let j = make(v);
            assert!(j.items().is_none());
        }
    }

    #[test]
    fn array() {
        let j = make(r#"[null, true]"#);
        assert_eq!(
            j.items().unwrap().collect::<Vec<_>>(),
            vec![
                (Key::Index(0), &ValueType::Null),
                (Key::Index(1), &ValueType::Bool(true))
            ]
        );
    }

    #[test]
    fn object() {
        let j = make(r#"{"a": null, "b": true}"#);
        assert_eq!(
            j.items().unwrap().collect::<Vec<_>>(),
            vec![
                (Key::String("a\n".into()), &ValueType::Null),
                (Key::String("b\n".into()), &ValueType::Bool(true))
            ]
        );
    }

    #[test]
    fn empty_object() {
        let j = make(r#"{}"#);
        assert_eq!(j.items().unwrap().collect::<Vec<_>>(), vec![]);
    }
}

#[cfg(feature = "with-serde-json")]
mod serde_json {
    extern crate serde_json;
    use self::serde_json::Value as ValueType;

    fn make(v: &str) -> ValueType {
        v.parse().unwrap()
    }

    make_suite!(ValueType::Bool);
}

#[cfg(feature = "with-rustc-serialize")]
mod rustc_serialize {
    extern crate rustc_serialize;
    use self::rustc_serialize::json::Json as ValueType;

    fn make(v: &str) -> ValueType {
        v.parse().unwrap()
    }

    make_suite!(ValueType::Boolean);
}
