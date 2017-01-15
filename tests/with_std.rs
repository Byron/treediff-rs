extern crate treediff;
use treediff::Value;

fn assert_any<V>(l: &V, r: &V)
    where V: Value
{
    assert!(l != r);
}

fn assert_non_scalar<V>(l: V, r: V)
    where V: Value
{
    assert_any(&l, &r);
    assert!(l.items().is_some());
}

fn assert_scalar<V>(l: V, r: V)
    where V: Value
{
    assert_any(&l, &r);
    assert!(l.items().is_none());
}

#[cfg(feature = "with-std")]
#[test]
fn std_value_string() {
    let l = String::from("one");
    let r = String::from("two");

    assert!(l.items().is_none());
    assert!(l == l);

    assert_scalar(l, r);
}

#[cfg(feature = "with-std")]
#[test]
fn std_value_vec_str() {
    let l = vec!["one", "two"];
    let r = vec!["two"];
    assert_eq!(l.items().unwrap().map(|(i, s)|(i, *s)).collect::<Vec<_>>(), vec![(0, "one"), (1, "two")]);
    assert_non_scalar(l, r);
}

#[cfg(feature = "with-std")]
#[test]
fn std_value_vec_string() {
    let l = vec![String::from("one")];
    let r = vec![String::from("two")];
    assert_eq!(l.items().unwrap().collect::<Vec<_>>(), vec![(0, &String::from("one"))]);
    assert_non_scalar(l, r);
}
