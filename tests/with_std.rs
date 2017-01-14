extern crate treediff;
use treediff::Value;

//fn use_val_borrowed<'a, V>(v: &'a V)
//    where V: Value<'a>
//{
//    assert!(v.items().is_none());
//    assert!(v.eq(&v));
//}
//
//fn use_val_owned<'a, V>(v: V)
//    where V: Value<'a>
//{
//    //    use_val_borrowed(&v);
//}
//
//fn assert_scalar<'a, V>(l: V, r: V)
//    where V: Value<'a> + 'a
//{
//    assert!(l != r);
//    //    use_val_borrowed(&l);
//    //    use_val_owned(l);
//}

fn assert_polymorphic<T>(l: &Value<T, Item=T>, r: &Value<T, Item=T>) {
    assert!(!l.equals(r));
}

#[cfg(feature = "with-std")]
#[test]
fn std_value_string() {
    let l = String::from("one");
    let r = String::from("two");

    assert!(l.items().is_none());
    assert!(l.equals(&l));

//    assert_scalar(l, r);
}

#[cfg(feature = "with-std")]
#[test]
fn std_value_vec_str() {
    let l = vec!["one"];
    let r = vec!["two"];

    assert!(l.items().is_some());
    assert!(l.eq(&l));

//    assert_scalar(l, r);
}

#[cfg(feature = "with-std")]
#[test]
fn std_value_vec_int() {
    let l = vec![1];
    let r = vec![2];

    assert!(l.items().is_some());
    assert!(l.eq(&l));

//    assert_scalar(l, r);
//    assert_polymorphic(&l, &r);
}
