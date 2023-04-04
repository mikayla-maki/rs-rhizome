#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_debug_implementations, rust_2018_idioms)]
#![deny(unreachable_pub, private_in_public)]

//! rhizome

pub(crate) mod col;
pub(crate) mod col_val;
pub(crate) mod id;
pub(crate) mod interner;
pub(crate) mod lattice;
pub(crate) mod logic;
pub(crate) mod ram;
pub(crate) mod relation;
pub(crate) mod schema;
pub(crate) mod timestamp;
pub(crate) mod value;
pub(crate) mod var;

pub mod error;
pub mod fact;
pub mod pretty;
pub mod runtime;
pub mod storage;
pub mod types;

pub(crate) use logic::build;
pub use logic::ProgramBuilder;

/// Test utilities.
#[cfg(any(test, feature = "test_utils"))]
#[cfg_attr(docsrs, doc(cfg(feature = "test_utils")))]
pub mod test_utils;
