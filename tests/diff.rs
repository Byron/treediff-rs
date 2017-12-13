extern crate treediff;

#[cfg(feature = "with-rustc-serialize")]
mod diff {
    extern crate rustc_serialize;
    use treediff::diff;
    use treediff::tools::Recorder;
    use treediff::tools::ChangeType::*;
    use treediff::value::Key;
    use self::rustc_serialize::json::Json;

    fn k(s: &'static str) -> Key {
        Key::String(s.into())
    }

    #[test]
    fn scalar_modified() {
        let v1: Json = r#""one""#.parse().unwrap();
        let v2: Json = r#""two""#.parse().unwrap();
        let mut d = Recorder::default();
        diff(&v1, &v2, &mut d);
        assert_eq!(d.calls, vec![Modified(vec![], &v1, &v2)]);
    }

    #[test]
    fn scalar_equal() {
        let v: Json = r#""one""#.parse().unwrap();
        let mut d = Recorder::default();
        diff(&v, &v, &mut d);
        assert_eq!(d.calls, vec![Unchanged(vec![], &v)]);
    }

    #[test]
    fn object_equal() {
        let v: Json = r#"{"one": 1}"#.parse().unwrap();
        let mut d = Recorder::default();
        diff(&v, &v, &mut d);
        assert_eq!(d.calls, vec![Unchanged(vec![], &v)]);
    }

    #[test]
    fn object_root_partially_different() {
        let v1: Json = r#"{"1": 1, "2": 2}"#.parse().unwrap();
        let v2: Json = r#"{"1": 1, "2": 3}"#.parse().unwrap();
        let mut d = Recorder::default();
        diff(&v1, &v2, &mut d);
        assert_eq!(
            d.calls,
            vec![
                Unchanged(vec![k("1")], v1.as_object().unwrap().get("1").unwrap()),
                Modified(
                    vec![k("2")],
                    v1.as_object().unwrap().get("2").unwrap(),
                    v2.as_object().unwrap().get("2").unwrap(),
                ),
            ]
        );
    }

    #[test]
    fn empty_object_added_key() {
        let v1: Json = r#"{}"#.parse().unwrap();
        let v2: Json = r#"{"1": 1}"#.parse().unwrap();
        let mut d = Recorder::default();
        diff(&v1, &v2, &mut d);
        assert_eq!(
            d.calls,
            vec![
                Added(vec![k("1")], v2.as_object().unwrap().get("1").unwrap()),
            ]
        );
    }

    #[test]
    fn empty_object_removed_key() {
        let v1: Json = r#"{"1": 1}"#.parse().unwrap();
        let v2: Json = r#"{}"#.parse().unwrap();
        let mut d = Recorder::default();
        diff(&v1, &v2, &mut d);
        assert_eq!(
            d.calls,
            vec![
                Removed(vec![k("1")], v1.as_object().unwrap().get("1").unwrap()),
            ]
        );
    }

    #[test]
    fn nested_object_added_key() {
        let v1: Json = r#"{"a": {}}"#.parse().unwrap();
        let v2: Json = r#"{"a": {"1": 1}}"#.parse().unwrap();
        let mut d = Recorder::default();
        diff(&v1, &v2, &mut d);
        assert_eq!(
            d.calls,
            vec![
                Added(vec![k("a"), k("1")], v2.find_path(&["a", "1"]).unwrap()),
            ]
        );
    }

    #[test]
    fn nested_object_removed_key() {
        let v1: Json = r#"{"a": {"1": 1}}"#.parse().unwrap();
        let v2: Json = r#"{"a": {}}"#.parse().unwrap();
        let mut d = Recorder::default();
        diff(&v1, &v2, &mut d);
        assert_eq!(
            d.calls,
            vec![
                Removed(vec![k("a"), k("1")], v1.find_path(&["a", "1"]).unwrap()),
            ]
        );
    }

    #[test]
    fn nested_object_modified_key() {
        let v1: Json = r#"{"a": {"1": 1}}"#.parse().unwrap();
        let v2: Json = r#"{"a": {"1": 2}}"#.parse().unwrap();
        let mut d = Recorder::default();
        diff(&v1, &v2, &mut d);
        assert_eq!(
            d.calls,
            vec![
                Modified(
                    vec![k("a"), k("1")],
                    v1.find_path(&["a", "1"]).unwrap(),
                    v2.find_path(&["a", "1"]).unwrap(),
                ),
            ]
        );
    }

    #[test]
    fn object_and_scalar_is_modified() {
        let v1: Json = r#"{"1": 1}"#.parse().unwrap();
        let v2: Json = r#""1""#.parse().unwrap();
        let mut d = Recorder::default();
        diff(&v1, &v2, &mut d);
        assert_eq!(d.calls, vec![Modified(vec![], &v1, &v2)]);
    }

    #[test]
    fn scalar_and_object_is_modified() {
        let v1: Json = r#""1""#.parse().unwrap();
        let v2: Json = r#"{"1": 1}"#.parse().unwrap();
        let mut d = Recorder::default();
        diff(&v1, &v2, &mut d);
        assert_eq!(d.calls, vec![Modified(vec![], &v1, &v2)]);
    }
}
