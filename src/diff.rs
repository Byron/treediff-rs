use traitdef::{Value, Delegate};
use std::cmp::Ordering;
use std::collections::BTreeSet;

pub fn diff<'a, V, D>(l: &'a V, r: &'a V, d: &mut D)
    where V: Value,
          <V as Value>::Key: Ord,
          D: Delegate<'a, V>
{
    match (l.items(), r.items()) {
        // two scalars, equal
        (None, None) if l == r => d.unchanged(l),
        // two scalars, different
        (None, None) => d.modified(l, r),
        (Some(_), Some(_)) if l == r => d.unchanged(l),
        (Some(li), Some(ri)) => {
            let mut sl: BTreeSet<OrdByKey<_, _>> = BTreeSet::new();
            sl.extend(li.map(Into::into));
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
