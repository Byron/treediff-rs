extern crate treediff;


#[cfg(feature = "with-rustc-serialize")]
mod diff {
    extern crate rustc_serialize;
    use treediff::{diff, ChangeType, Recorder};
    use self::rustc_serialize::json::Json;

    #[test]
    fn scalar_modified() {
        let v1: Json = r#""one""#.parse().unwrap();
        let v2: Json = r#""two""#.parse().unwrap();
        let mut d = Recorder::default();
        diff(&v1, &v2, &mut d);
        assert_eq!(d.calls, vec![ChangeType::Modified(&v1, &v2)]);
    }

    #[test]
    fn scalar_equal() {
        let v: Json = r#""one""#.parse().unwrap();
        let mut d = Recorder::default();
        diff(&v, &v, &mut d);
        assert_eq!(d.calls, vec![ChangeType::Unchanged(&v)]);
    }

    #[test]
    fn object_equal() {
        let v: Json = r#"{"one": 1}"#.parse().unwrap();
        let mut d = Recorder::default();
        diff(&v, &v, &mut d);
        assert_eq!(d.calls, vec![ChangeType::Unchanged(&v)]);
    }

    #[test]
    fn object_root_partially_different() {
        let v1: Json = r#"{"one": 1, "two": 2}"#.parse().unwrap();
        let v2: Json = r#"{"one": 1, "two": 3}"#.parse().unwrap();
        let mut d = Recorder::default();
        diff(&v1, &v2, &mut d);
        assert_eq!(d.calls,
                   vec![ChangeType::Unchanged(v1.as_object().unwrap().get("one").unwrap()),
                        ChangeType::Modified(v1.as_object().unwrap().get("two").unwrap(),
                                             v2.as_object().unwrap().get("two").unwrap())]);
    }
}
