extern crate treediff;
use treediff::Value;

fn use_val_borrowed<V>(v: &V)
    where V: Value
{
//    assert!(v.items().is_none());
    assert!(v == v);
}

fn use_val_owned<V>(v: V)
    where V: Value
{
    use_val_borrowed(&v);
}

fn assert_scalar<V>(l: V, r: V)
    where V: Value
{
    assert!(l != r);
    use_val_borrowed(&l);
    use_val_owned(l);
}

#[cfg(feature = "with-std")]
#[test]
fn std_value_string() {
    let s = String::from("one");
    let r = String::from("two");
    assert_scalar(s, r);
}
