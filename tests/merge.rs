extern crate treediff;


#[cfg(feature = "with-rustc-serialize")]
mod merge {
    extern crate rustc_serialize;
    use self::rustc_serialize::json::{Object, Json};
    use treediff::{diff, Merger};

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
