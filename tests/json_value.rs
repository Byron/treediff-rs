extern crate treediff;

#[cfg(feature = "with-rustc-serialize")]
mod rustc_json_value {
    extern crate rustc_serialize;
    use treediff::Value;
    use self::rustc_serialize::json::Json;

    #[test]
    fn scalar_value_string() {
        let j: Json = r#""value""#.parse().unwrap();
        assert!(j.items().is_none());
    }

    #[test]
    fn empty_object() {
        let j: Json = r#"{}"#.parse().unwrap();
        assert_eq!(j.items().unwrap().collect::<Vec<_>>(), vec![]);
    }
}
