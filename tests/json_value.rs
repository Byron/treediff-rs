extern crate treediff;

#[cfg(feature = "with-rustc-serialize")]
mod rustc_json_value {
    extern crate rustc_serialize;
    use treediff::Value;
    use self::rustc_serialize::json::Json;

    #[test]
    fn empty_object() {
        let j: Json = r#"{}"#.parse().unwrap();
        assert!(j == j);
        assert!(j.items().is_some());
    }
}
