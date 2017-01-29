//! See what's different in arbitrary data structures.
#[cfg(feature = "with-rustc-serialize")]
extern crate rustc_serialize;


mod traitdef;
mod traitimpls;
mod diff;
mod recorder;
mod merger;

pub use traitdef::*;
pub use traitimpls::*;
pub use diff::*;
pub use recorder::*;
pub use merger::*;
