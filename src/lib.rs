//! **See what's different in arbitrary data structures**.
//!
//! The [main diff algorithm][diffalgo]
//! we implement here is less than a screen full of lines, yet it enables a vast amount of
//! applications.
//! It can work with all values implementing the `Value` trait.
//!
//! On top of it there is a [merge implementation][merge], which can be used to aggregate changes
//! into a target value. It works with `Value`s which implement the `Mergeable` trait.
//! [merge]: https://github.com/Byron/treediff-rs/blob/master/src/merge.rs
//! [diffalgo]: https://github.com/Byron/treediff-rs/blob/master/src/diff.rs
#[cfg(feature = "with-rustc-serialize")]
extern crate rustc_serialize;
#[cfg(feature = "with-serde-json")]
extern crate serde_json;


mod traitdef;
mod diff;
pub mod record;
pub mod merge;
pub mod value;

pub use traitdef::*;
pub use diff::*;
