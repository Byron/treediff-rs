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
    let l = vec!["one"];
    let r = vec!["two"];
    assert_non_scalar(l, r);
}

#[cfg(feature = "with-std")]
#[test]
fn std_value_vec_int() {
    let l = vec![1];
    let r = vec![2];
    assert_non_scalar(l, r);
}
