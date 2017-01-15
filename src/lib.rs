//! See what's different in arbitrary data structures.
#[cfg(feature = "with-rustc-serialize")]
extern crate rustc_serialize;


mod traitdef;
mod traitimpls;
mod diff;

pub use traitdef::*;
pub use diff::*;
