extern crate treediff;


mod diff {
    use treediff::{diff, Delegate};
    #[derive(Debug, PartialEq)]
    enum ChangeType<'a, V: 'a> {
        Unchanged(&'a V),
    }
    #[derive(Default, Debug, PartialEq)]
    struct Recorder<'a, V: 'a> {
        calls: Vec<ChangeType<'a, V>>,
    }

    impl<'a, V> Delegate<'a, V> for Recorder<'a, V> {
        fn unchanged(&mut self, v: &'a V) {
            self.calls.push(ChangeType::Unchanged(v));
        }
    }

    #[test]
    fn scalar_modified() {
        let v = String::from("value");
        let v2 = String::from("value two");
        let mut d = Recorder::default();
        diff(&v, &v2, &mut d);
        assert_eq!(d.calls, vec![ChangeType::Unchanged(&v)]);
    }

    #[test]
    fn scalar_equal() {
        let v = String::from("value");
        let mut d = Recorder::default();
        diff(&v, &v, &mut d);
        assert_eq!(d.calls, vec![ChangeType::Unchanged(&v)]);
    }
}
