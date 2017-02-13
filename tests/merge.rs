extern crate treediff;

macro_rules! make_suite {
() => {
    use treediff::diff;
    use treediff::tools::{MutableFilter, Merger};
    use std::borrow::Cow;

    fn make_object() -> ValueType {
        make(r#"{}"#)
    }

    fn make_scalar() -> ValueType {
        make(r#"null"#)
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
        let v1 = make(r#"{"a": [{"1": 1}]}"#);
        let v2 = make(r#"{"a": [{"1": 2}]}"#);
        let mut m = Merger::from(ValueType::clone(&v1));
        diff(&v1, &v2, &mut m);
        assert_eq!(v2, m.into_inner());
    }

    #[test]
    fn nested_object_in_array_removed() {
        let v1 = make(r#"{"a": [{"1": 2}]}"#);
        let v2 = make(r#"{"a": []}"#);
        let mut m = Merger::from(ValueType::clone(&v1));
        diff(&v1, &v2, &mut m);
        assert_eq!(v2, m.into_inner());
    }

    #[test]
    fn nested_object_in_array_added() {
        let v1 = make(r#"{"a": []}"#);
        let v2 = make(r#"{"a": [{"1": 2}]}"#);
        let mut m = Merger::from(ValueType::clone(&v1));
        diff(&v1, &v2, &mut m);
        assert_eq!(v2, m.into_inner());
    }

    #[test]
    fn modified_array_at_root() {
        let v1 = make(r#"[1]"#);
        let v2 = make(r#"[2]"#);
        let mut m = Merger::from(ValueType::clone(&v1));
        diff(&v1, &v2, &mut m);
        assert_eq!(v2, m.into_inner());
    }

    #[test]
    fn added_array_at_root_scalar_target() {
        let v1 = make(r#"[]"#);
        let v2 = make(r#"[1, 2]"#);
        let t: ValueType = make(r#"null"#);
        let mut m = Merger::from(t);
        diff(&v1, &v2, &mut m);
        assert_eq!(v2, m.into_inner());
    }

    #[test]
    fn modified_at_root_scalar_target() {
        let v1 = make(r#"{"1": 1, "a": {"2": 2}}"#);
        let v2 = make(r#"{"1": 1, "a": {"2": 1}}"#);
        let t: ValueType = make(r#"null"#);
        let mut m = Merger::from(t);
        diff(&v1, &v2, &mut m);
        assert_eq!(v2, m.into_inner());
    }

    #[test]
    fn modified_at_root_with_owned_filter_pick_none() {
        let v1 = make(r#"{"1": 1}"#);
        let v2 = make(r#"{"1": 2}"#);
        let v3: ValueType = make(r#"{}"#);
        struct Filter;
        impl MutableFilter for Filter {
            fn resolve_conflict<'a, K, V: Clone>(&mut self, _keys: &[K], _old: &'a V, _new: &'a V,
                                                 _self: &mut V)
                                                 -> Option<Cow<'a, V>> {
                None
            }
        }
        let mut m = Merger::with_filter(ValueType::clone(&v2), Filter);
        diff(&v1, &v2, &mut m);
        assert_eq!(v3, m.into_inner());
    }

    #[test]
    fn modified_at_root() {
        let v1 = make(r#"{"1": 1}"#);
        let v2 = make(r#"{"1": 2}"#);
        let mut m = Merger::from(ValueType::clone(&v1));
        diff(&v1, &v2, &mut m);
        assert_eq!(v2, m.into_inner());
    }

    #[test]
    fn removed_at_root_with_borrowed_filter() {
        struct Filter;
        impl MutableFilter for Filter {
            fn resolve_removal<'a, K, V: Clone>(&mut self,
                                               _keys: &[K],
                                               removed: &'a V,
                                               _self: &mut V)
                                               -> Option<Cow<'a, V>> {
                Some(Cow::Borrowed(removed))
            }
        }
        let v1 = make(r#"{"1": 1, "2": 2}"#);
        let v2 = make(r#"{"1": 1}"#);
        let mut f = Filter;
        let mut m = Merger::<_, _, _, Filter>::with_filter(ValueType::clone(&v1), &mut f);
        diff(&v1, &v2, &mut m);
        assert_eq!(v1, m.into_inner());
    }

    #[test]
    fn removed_at_root() {
        let v1 = make(r#"{"1": 1, "2": 2}"#);
        let v2 = make(r#"{"1": 1}"#);
        let mut m = Merger::from(ValueType::clone(&v1));
        diff(&v1, &v2, &mut m);
        assert_eq!(v2, m.into_inner());
    }

    #[test]
    fn added_at_root() {
        let v1 = make(r#"{"1": 1}"#);
        let v2 = make(r#"{"1": 1, "2": 2}"#);
        let mut m = Merger::from(ValueType::clone(&v1));
        diff(&v1, &v2, &mut m);
        assert_eq!(v2, m.into_inner());
    }
}
}

#[cfg(feature = "with-serde-json")]
mod serde_json {
    extern crate serde_json;
    use self::serde_json::Value as ValueType;

    fn make(v: &str) -> ValueType {
        v.parse().unwrap()
    }

    make_suite!();
}

#[cfg(feature = "with-rustc-serialize")]
mod rustc_serialize {
    extern crate rustc_serialize;
    use self::rustc_serialize::json::Json as ValueType;

    fn make(v: &str) -> ValueType {
        v.parse().unwrap()
    }

    make_suite!();
}
