extern crate treediff;

macro_rules! make_suite {
    ($module:ident) => {
    mod $module {

    }
    };
}

#[cfg(feature = "with-rustc-serialize")]
mod rustc_json_value {
    extern crate rustc_serialize;
    use treediff::Value;
    use treediff::value::json::rustc_serialize::JsonKey;
    use self::rustc_serialize::json::Json;

    make_suite!(rustc_value);

    #[test]
    fn scalar_values() {
        for v in &["null", "true", "1.23", "-1234456", "1234456", "\"string\""] {
            let j: Json = v.parse().unwrap();
            assert!(j.items().is_none());
        }
    }

    #[test]
    fn array() {
        let j: Json = r#"[null, true]"#.parse().unwrap();
        assert_eq!(j.items().unwrap().collect::<Vec<_>>(),
                   vec![(JsonKey::Index(0), &Json::Null),
                        (JsonKey::Index(1), &Json::Boolean(true))]);
    }

    #[test]
    fn object() {
        let j: Json = r#"{"a": null, "b": true}"#.parse().unwrap();
        assert_eq!(j.items().unwrap().collect::<Vec<_>>(),
                   vec![(JsonKey::String("a".into()), &Json::Null),
                        (JsonKey::String("b".into()), &Json::Boolean(true))]);
    }

    #[test]
    fn empty_object() {
        let j: Json = r#"{}"#.parse().unwrap();
        assert_eq!(j.items().unwrap().collect::<Vec<_>>(), vec![]);
    }
}
