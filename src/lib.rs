#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![allow(clippy::module_inception)]

#[macro_use]
mod macros;

pub mod board;
mod parser;
pub mod piece;
