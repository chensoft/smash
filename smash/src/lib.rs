#![doc = include_str!("../../README.md")]
// #![warn(missing_docs)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::let_underscore_future)]

#[macro_use] extern crate serde;
#[macro_use] extern crate thiserror;
#[macro_use] extern crate async_trait;

pub mod actor;
pub mod error;
pub mod linkq;
pub mod redix;

pub use actor::{Actor, Event};