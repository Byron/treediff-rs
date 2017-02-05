extern crate treediff;

macro_rules! make_suite {
($null:expr, $bool:expr) => {
}
}

#[cfg(feature = "with-rustc-serialize")]
mod merge {
    extern crate rustc_serialize;
    use self::rustc_serialize::json::{Object, Json};
    use treediff::{diff, Merger};

    //    make_suite!();

    #[test]
    fn unchanged_at_root() {
        for s in &[r#"{"1": 1, "2": {"1" : 1}}"#, r#"-1"#, r#"1"#, r#""str""#] {
            for t in vec![Json::Object(Object::new()), Json::U64(12)].into_iter() {
                let v: Json = s.parse().unwrap();
                let mut m = Merger::from(t);
                diff(&v, &v, &mut m);
                assert_eq!(v, m.into_inner());
            }
        }
    }

    #[test]
    fn nested_object_in_array_modified() {
        let v1: Json = r#"{"a": [{"1": 1}]}"#.parse().unwrap();
        let v2: Json = r#"{"a": [{"1": 2}]}"#.parse().unwrap();
        let mut m = Merger::from(v1.clone());
        diff(&v1, &v2, &mut m);
        assert_eq!(v2, m.into_inner());
    }

    #[test]
    fn nested_object_in_array_removed() {
        let v1: Json = r#"{"a": [{"1": 2}]}"#.parse().unwrap();
        let v2: Json = r#"{"a": []}"#.parse().unwrap();
        let mut m = Merger::from(v1.clone());
        diff(&v1, &v2, &mut m);
        assert_eq!(v2, m.into_inner());
    }

    #[test]
    fn nested_object_in_array_added() {
        let v1: Json = r#"{"a": []}"#.parse().unwrap();
        let v2: Json = r#"{"a": [{"1": 2}]}"#.parse().unwrap();
        let mut m = Merger::from(v1.clone());
        diff(&v1, &v2, &mut m);
        assert_eq!(v2, m.into_inner());
    }

    #[test]
    fn modified_array_at_root() {
        let v1: Json = r#"[1]"#.parse().unwrap();
        let v2: Json = r#"[2]"#.parse().unwrap();
        let mut m = Merger::from(v1.clone());
        diff(&v1, &v2, &mut m);
        assert_eq!(v2, m.into_inner());
    }

    #[test]
    fn added_array_at_root_scalar_target() {
        let v1: Json = r#"[]"#.parse().unwrap();
        let v2: Json = r#"[1, 2]"#.parse().unwrap();
        let t: Json = r#"null"#.parse().unwrap();
        let mut m = Merger::from(t);
        diff(&v1, &v2, &mut m);
        assert_eq!(v2, m.into_inner());
    }

    #[test]
    fn modified_at_root_scalar_target() {
        let v1: Json = r#"{"1": 1, "a": {"2": 2}}"#.parse().unwrap();
        let v2: Json = r#"{"1": 1, "a": {"2": 1}}"#.parse().unwrap();
        let t: Json = r#"null"#.parse().unwrap();
        let mut m = Merger::from(t);
        diff(&v1, &v2, &mut m);
        assert_eq!(v2, m.into_inner());
    }

    #[test]
    fn modified_at_root() {
        let v1: Json = r#"{"1": 1}"#.parse().unwrap();
        let v2: Json = r#"{"1": 2}"#.parse().unwrap();
        let mut m = Merger::from(v1.clone());
        diff(&v1, &v2, &mut m);
        assert_eq!(v2, m.into_inner());
    }

    #[test]
    fn removed_at_root() {
        let v1: Json = r#"{"1": 1, "2": 2}"#.parse().unwrap();
        let v2: Json = r#"{"1": 1}"#.parse().unwrap();
        let mut m = Merger::from(v1.clone());
        diff(&v1, &v2, &mut m);
        assert_eq!(v2, m.into_inner());
    }

    #[test]
    fn added_at_root() {
        let v1: Json = r#"{"1": 1}"#.parse().unwrap();
        let v2: Json = r#"{"1": 1, "2": 2}"#.parse().unwrap();
        let mut m = Merger::from(v1.clone());
        diff(&v1, &v2, &mut m);
        assert_eq!(v2, m.into_inner());
    }
}
