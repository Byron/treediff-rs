extern crate treediff;

macro_rules! make_suite {
    ($make:ident, $index:tt, $null:expr, $bool:expr) => {
        #[test]
        fn scalar_values() {
            for v in &["null", "true", "1.23", "-1234456", "1234456", "\"string\""] {
                let j = $make(v);
                assert!(j.items().is_none());
            }
        }

        #[test]
        fn array() {
            let j = $make(r#"[null, true]"#);
            assert_eq!(j.items().unwrap().collect::<Vec<_>>(),
                       vec![($index::Index(0), &$null),
                            ($index::Index(1), &$bool(true))]);
        }

        #[test]
        fn object() {
            let j = $make(r#"{"a": null, "b": true}"#);
            assert_eq!(j.items().unwrap().collect::<Vec<_>>(),
                       vec![($index::String("a".into()), &$null),
                            ($index::String("b".into()), &$bool(true))]);
        }

        #[test]
        fn empty_object() {
            let j = $make(r#"{}"#);
            assert_eq!(j.items().unwrap().collect::<Vec<_>>(), vec![]);
        }
    };
}


#[cfg(feature = "with-rustc-serialize")]
mod rustc_serialize {
    extern crate rustc_serialize;
    use treediff::value::json::rustc_serialize::JsonKey;
    use self::rustc_serialize::json::Json;
    use treediff::Value;

    fn make_json(v: &str) -> Json {
        v.parse().unwrap()
    }

    make_suite!(make_json, JsonKey, Json::Null, Json::Boolean);


}
