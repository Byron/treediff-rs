use traitdef::{Value, Delegate};

pub fn diff<V, D>(l: &V, r: &V, d: &mut D)
where V: Value, D: Delegate<V> {
   match (l.items(), r.items()) {
       // two scalars, equal
       (None, None) if l == r => d.unchanged(&l),
       _ => unimplemented!()
   }
}