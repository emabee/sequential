//! Tiny library that provides a Sequence implementation.
//!
//! The crate has no dependency except the optional dependency to `serde`.

// only enables the `doc_cfg` feature when the `docsrs` configuration attribute is defined
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![forbid(unsafe_code)]

mod seq_num;
mod sequence;

pub use sequence::Sequence;
