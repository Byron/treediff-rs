extern crate treediff;

macro_rules! make_suite {
() => {
    use treediff::{diff, Merger, pick_new, pick_old, drop_removed};
    use std::borrow::Cow;

    fn make_object() -> Json {
        Json::Object(Object::new())
    }

    fn make_scalar() -> Json {
        Json::Null
    }

    #[test]
    fn unchanged_at_root_() {
        for s in &[r#"{"1": 1, "2": {"1" : 1}}"#, r#"-1"#, r#"1"#, r#""str""#] {
            for t in vec![make_object(), make_scalar()].into_iter() {
                let v = s.parse().unwrap();
                let mut m = Merger::from(t);
                diff(&v, &v, &mut m);
                assert_eq!(v, m.into_inner());
            }
        }
    }

    #[test]
    fn nested_object_in_array_modified() {
        let v1 = r#"{"a": [{"1": 1}]}"#.parse().unwrap();
        let v2 = r#"{"a": [{"1": 2}]}"#.parse().unwrap();
        let mut m = Merger::from(Json::clone(&v1));
        diff(&v1, &v2, &mut m);
        assert_eq!(v2, m.into_inner());
    }

    #[test]
    fn nested_object_in_array_removed() {
        let v1 = r#"{"a": [{"1": 2}]}"#.parse().unwrap();
        let v2 = r#"{"a": []}"#.parse().unwrap();
        let mut m = Merger::from(Json::clone(&v1));
        diff(&v1, &v2, &mut m);
        assert_eq!(v2, m.into_inner());
    }

    #[test]
    fn nested_object_in_array_added() {
        let v1 = r#"{"a": []}"#.parse().unwrap();
        let v2 = r#"{"a": [{"1": 2}]}"#.parse().unwrap();
        let mut m = Merger::from(Json::clone(&v1));
        diff(&v1, &v2, &mut m);
        assert_eq!(v2, m.into_inner());
    }

    #[test]
    fn modified_array_at_root() {
        let v1 = r#"[1]"#.parse().unwrap();
        let v2 = r#"[2]"#.parse().unwrap();
        let mut m = Merger::from(Json::clone(&v1));
        diff(&v1, &v2, &mut m);
        assert_eq!(v2, m.into_inner());
    }

    #[test]
    fn added_array_at_root_scalar_target() {
        let v1 = r#"[]"#.parse().unwrap();
        let v2 = r#"[1, 2]"#.parse().unwrap();
        let t: Json = r#"null"#.parse().unwrap();
        let mut m = Merger::from(t);
        diff(&v1, &v2, &mut m);
        assert_eq!(v2, m.into_inner());
    }

    #[test]
    fn modified_at_root_scalar_target() {
        let v1 = r#"{"1": 1, "a": {"2": 2}}"#.parse().unwrap();
        let v2 = r#"{"1": 1, "a": {"2": 1}}"#.parse().unwrap();
        let t: Json = r#"null"#.parse().unwrap();
        let mut m = Merger::from(t);
        diff(&v1, &v2, &mut m);
        assert_eq!(v2, m.into_inner());
    }
    #[test]
    fn modified_at_root_with_resolver() {
        let v1 = r#"{"1": 1}"#.parse().unwrap();
        let v2 = r#"{"1": 2}"#.parse().unwrap();
        let mut m = Merger::with_resolver(Json::clone(&v2), pick_old, drop_removed);
        diff(&v1, &v2, &mut m);
        assert_eq!(v1, m.into_inner());
    }

    #[test]
    fn modified_at_root() {
        let v1 = r#"{"1": 1}"#.parse().unwrap();
        let v2 = r#"{"1": 2}"#.parse().unwrap();
        let mut m = Merger::from(Json::clone(&v1));
        diff(&v1, &v2, &mut m);
        assert_eq!(v2, m.into_inner());
    }

    #[test]
    fn removed_at_root_with_resolver() {
        pub fn incr<'a, K, V: Clone>(_keys: &[K], removed: &'a V, _self: &mut V)
                -> Option<Cow<'a, V>>
        {
            Some(Cow::Borrowed(removed))
        }
        let v1 = r#"{"1": 1, "2": 2}"#.parse().unwrap();
        let v2 = r#"{"1": 1}"#.parse().unwrap();
        let mut m = Merger::with_resolver(Json::clone(&v1), pick_new, incr);
        diff(&v1, &v2, &mut m);
        assert_eq!(v1, m.into_inner());
    }

    #[test]
    fn removed_at_root() {
        let v1 = r#"{"1": 1, "2": 2}"#.parse().unwrap();
        let v2 = r#"{"1": 1}"#.parse().unwrap();
        let mut m = Merger::from(Json::clone(&v1));
        diff(&v1, &v2, &mut m);
        assert_eq!(v2, m.into_inner());
    }

    #[test]
    fn added_at_root() {
        let v1 = r#"{"1": 1}"#.parse().unwrap();
        let v2 = r#"{"1": 1, "2": 2}"#.parse().unwrap();
        let mut m = Merger::from(Json::clone(&v1));
        diff(&v1, &v2, &mut m);
        assert_eq!(v2, m.into_inner());
    }
}
}

#[cfg(feature = "with-rustc-serialize")]
mod serde_json {
    extern crate serde_json;
    use self::serde_json::{Map as Object, Value as Json};

    make_suite!();
}

#[cfg(feature = "with-rustc-serialize")]
mod rustc_serialize {
    extern crate rustc_serialize;
    use self::rustc_serialize::json::{Object, Json};

    make_suite!();
}
