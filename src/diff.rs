use traitdef::{Value, Delegate};
use std::cmp::Ordering;
use std::collections::BTreeSet;

#[derive(PartialEq)]
enum Values<'a, V1: 'a, V2: 'a> {
    V1(&'a V1),
    V2(&'a V2),
}

impl<'b, V1, V2> Value for Values<'b, V1, V2>
    where V1: Value<Key = V2::Key, Item = V2::Item>,
          V2: Value
{
    type Key = V1::Key;
    type Item = V1::Item;
    fn items<'a>(&'a self) -> Option<Box<Iterator<Item = (Self::Key, &'a Self::Item)> + 'a>> {
        match *self {
            Values::V1(ref v) => v.items(),
            Values::V2(ref v) => v.items(),
        }
    }
}

pub fn diff<'a, V, D, T>(l: &'a V, r: &'a V, d: &'a mut D)
    where V: Value,
          <V as Value>::Key: Ord,
          <V as Value>::Item: Value,
          D: Delegate<'a, Values<'a, V, <V as Value>::Item>>
{
    match (l.items(), r.items()) {
        // two scalars, equal
        (None, None) if l == r => {
            let v = Values::V1(l);
            d.unchanged(&v)
        }
        // two scalars, different
        (None, None) => d.modified(&Values::V1(l), &Values::V1(r)),
        // two objects, equal
        (Some(_), Some(_)) if l == r => d.unchanged(&Values::V1(l)),
        // two objects, different
        (Some(li), Some(ri)) => {
            let mut sl: BTreeSet<OrdByKey<_, _>> = BTreeSet::new();
            sl.extend(li.map(Into::into));
            let mut sr: BTreeSet<OrdByKey<_, _>> = BTreeSet::new();
            sr.extend(ri.map(Into::into));
            for k in sr.union(&sl) {
                let v1 = sl.get(k).expect("union to work");
                let v2 = sr.get(k).expect("union to work");
                if v1.1 == v2.1 {
                    //                    d.unchanged(v1.1);
                }
            }
        }
        _ => unimplemented!(),
    }
}

struct OrdByKey<'a, K, V: 'a>(pub K, pub &'a V);

impl<'a, K, V> From<(K, &'a V)> for OrdByKey<'a, K, V> {
    fn from(src: (K, &'a V)) -> Self {
        OrdByKey(src.0, src.1)
    }
}

impl<'a, K, V> Eq for OrdByKey<'a, K, V> where K: Eq + PartialOrd {}

impl<'a, K, V> PartialEq for OrdByKey<'a, K, V>
    where K: PartialOrd
{
    fn eq(&self, other: &OrdByKey<'a, K, V>) -> bool {
        self.0.eq(&other.0)
    }
}

impl<'a, K, V> PartialOrd for OrdByKey<'a, K, V>
    where K: PartialOrd
{
    fn partial_cmp(&self, other: &OrdByKey<'a, K, V>) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<'a, K, V> Ord for OrdByKey<'a, K, V>
    where K: Ord
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}
