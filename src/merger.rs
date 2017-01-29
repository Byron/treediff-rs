use traitdef::Delegate;

pub struct Merger {}

impl<'a, K, V> Delegate<'a, K, V> for Merger {
    fn push<'b>(&mut self, k: &'b K) {}
    fn pop(&mut self) {}
    fn removed<'b>(&mut self, k: Option<&'b K>, v: &'a V) {}
    fn added<'b>(&mut self, k: Option<&'b K>, v: &'a V) {}
    fn unchanged<'b>(&mut self, k: Option<&'b K>, v: &'a V) {}
    fn modified<'b>(&mut self, k: Option<&'b K>, v1: &'a V, v2: &'a V) {}
}

pub trait Mergeable {}

impl<M> From<M> for Merger
    where M: Mergeable
{
    fn from(v: M) -> Self {
        Merger {}
    }
}
