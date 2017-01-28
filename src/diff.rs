use traitdef::{Value, Delegate};
use std::cmp::Ordering;
use std::collections::BTreeSet;

pub fn diff<'a, V, D>(l: &'a V, r: &'a V, d: &mut D)
    where V: Value<Item = V>,
          V::Key: Ord,
          D: Delegate<'a, V>
{
    match (l.items(), r.items()) {
        // two scalars, equal
        (None, None) if l == r => d.unchanged(l),
        // two scalars, different
        (None, None) => d.modified(l, r),
        // two objects, equal
        (Some(_), Some(_)) if l == r => d.unchanged(l),
        // two objects, different
        (Some(li), Some(ri)) => {
            let mut sl: BTreeSet<OrdByKey<_, _>> = BTreeSet::new();
            sl.extend(li.map(Into::into));
            let mut sr: BTreeSet<OrdByKey<_, _>> = BTreeSet::new();
            sr.extend(ri.map(Into::into));
            for k in sr.intersection(&sl) {
                let v1 = sl.get(k).expect("intersection to work");
                let v2 = sr.get(k).expect("intersection to work");
                if v1.1 == v2.1 {
                    d.unchanged(v1.1);
                } else {
                    d.modified(v1.1, v2.1);
                }
            }
            for k in sr.difference(&sl) {
                d.added(sr.get(k).expect("difference to work").1);
            }
            for k in sl.difference(&sr) {
                d.removed(sl.get(k).expect("difference to work").1);
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
