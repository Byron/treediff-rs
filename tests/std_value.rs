extern crate treediff;
mod std_value {
    use treediff::Value;
    use std::collections::HashMap;

    fn assert_non_scalar<V>(l: V)
        where V: Value
    {
        assert!(l.items().is_some());
    }

    fn assert_scalar<V>(l: V)
        where V: Value
    {
        assert!(l.items().is_none());
    }

    #[test]
    fn string() {
        let l = String::from("one");
        assert!(l.items().is_none());
        assert_scalar(l);
    }

    #[test]
    fn vec_str() {
        let l = vec!["one", "two"];
        assert_eq!(l.items().unwrap().map(|(i, &s)| (i, s)).collect::<Vec<_>>(),
                   vec![(0, "one"), (1, "two")]);
        assert_non_scalar(l);
    }

    #[test]
    fn vec_string() {
        let l = vec![String::from("one")];
        assert_eq!(l.items().unwrap().collect::<Vec<_>>(),
                   vec![(0, &String::from("one"))]);
        assert_non_scalar(l);
    }

    #[test]
    fn hashmap() {
        let mut l = HashMap::new();
        l.insert("hello", 123);
        l.insert("there", 42);

        let sorted = {
            let mut s = l.items().unwrap().collect::<Vec<_>>();
            s.sort_by_key(|v| v.0);
            s
        };
        assert_eq!(sorted, vec![("hello", &123), ("there", &42)]);
    }
}
