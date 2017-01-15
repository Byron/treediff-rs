extern crate treediff;


mod diff {
    use treediff::{diff, ChangeType, Recorder};
    #[test]
    fn scalar_modified() {
        let v = String::from("value");
        let v2 = String::from("value two");
        let mut d = Recorder::default();
        diff(&v, &v2, &mut d);
        assert_eq!(d.calls, vec![ChangeType::Modified(&v, &v2)]);
    }

    #[test]
    fn scalar_equal() {
        let v = String::from("value");
        let mut d = Recorder::default();
        diff(&v, &v, &mut d);
        assert_eq!(d.calls, vec![ChangeType::Unchanged(&v)]);
    }
}
