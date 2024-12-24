//! Tiny library that provides with [`Sequence`] a configurable number generator.

// only enables the `doc_cfg` feature when the `docsrs` configuration attribute is defined
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![forbid(unsafe_code)]

mod seq_num;
mod sequence;

pub use sequence::Sequence;
