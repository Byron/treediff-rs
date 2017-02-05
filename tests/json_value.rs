extern crate treediff;

macro_rules! make_suite {
($null:expr, $bool:expr) => {
    use treediff::Value;
    use treediff::value::json::JsonKey;

    fn make(v: &str) -> Json {
        v.parse().unwrap()
    }

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
        assert_eq!(j.items().unwrap().collect::<Vec<_>>(),
                   vec![(JsonKey::Index(0), &$null),
                        (JsonKey::Index(1), &$bool(true))]);
    }

    #[test]
    fn object() {
        let j = make(r#"{"a": null, "b": true}"#);
        assert_eq!(j.items().unwrap().collect::<Vec<_>>(),
                   vec![(JsonKey::String("a".into()), &$null),
                        (JsonKey::String("b".into()), &$bool(true))]);
    }

    #[test]
    fn empty_object() {
        let j = make(r#"{}"#);
        assert_eq!(j.items().unwrap().collect::<Vec<_>>(), vec![]);
    }
};
}


#[cfg(feature = "with-serde-json")]
mod serde_json {
    extern crate serde_json;
    use self::serde_json::Value as Json;

    make_suite!(Json::Null, Json::Bool);
}

#[cfg(feature = "with-rustc-serialize")]
mod rustc_serialize {
    extern crate rustc_serialize;
    use self::rustc_serialize::json::Json;

    make_suite!(Json::Null, Json::Boolean);
}
