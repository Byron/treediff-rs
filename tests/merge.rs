extern crate treediff;


#[cfg(feature = "with-rustc-serialize")]
mod merge {
    extern crate rustc_serialize;
    use self::rustc_serialize::json::{Object, Json};
    use treediff::{diff, Merger};

    #[test]
    fn unchanged_at_root() {
        let v: Json = r#"{"1": 1}"#.parse().unwrap();
        let mut m = Merger::from(Json::Object(Object::new()));
        diff(&v, &v, &mut m);
    }
}
