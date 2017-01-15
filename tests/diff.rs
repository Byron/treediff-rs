extern crate treediff;


mod diff {
    use treediff::{diff, Delegate};
    #[derive(Debug, PartialEq)]
    enum ChangeType<V> {
        Unchanged(V),
    }
    #[derive(Default, Debug, PartialEq)]
    struct Recorder<V> {
        calls: Vec<ChangeType<V>>,
    }

    impl<V> Delegate<V> for Recorder<V>
        where V: Clone
    {
        fn unchanged(&mut self, v: &V) {
            self.calls.push(ChangeType::Unchanged(v.clone()));
        }
    }

    #[test]
    fn scalar_modified() {
        let v = String::from("value");
        let v2 = String::from("value two");
        let mut d = Recorder::default();
        diff(&v, &v2, &mut d);
        assert_eq!(d.calls, vec![ChangeType::Unchanged(v.clone())]);
    }

    #[test]
    fn scalar_equal() {
        let v = String::from("value");
        let mut d = Recorder::default();
        diff(&v, &v, &mut d);
        assert_eq!(d.calls, vec![ChangeType::Unchanged(v.clone())]);
    }
}
