use traitdef::{Value, Delegate};

pub fn diff<'a, V, D>(l: &'a V, r: &'a V, d: &mut D)
    where V: Value,
          D: Delegate<'a, V>
{
    match (l.items(), r.items()) {
        // two scalars, equal
        (None, None) if l == r => d.unchanged(l),
        // two scalars, different
        (None, None) => d.modified(l, r),
        _ => unimplemented!(),
    }
}
