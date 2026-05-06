//! A distributed unique ID generator inspired by [Twitter's Snowflake].
//!
//! This is a Rust implementation of the original [fable/fableflake], which is written in Go.
//!
//! ## Quickstart
//!
//! Add the following to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! fableflake = "0.1"
//! ```
//!
//! Use the library like this:
//!
//! ```
//! use fableflake::Fableflake;
//!
//! let mut sf = Fableflake::new().unwrap();
//! let next_id = sf.next_id().unwrap();
//! println!("{}", next_id);
//! ```
//!
//! ## Concurrent use
//!
//! Fableflake is threadsafe. `clone` it before moving to another thread:
//! ```
//! use fableflake::Fableflake;
//! use std::thread;
//!
//! let sf = Fableflake::new().unwrap();
//!
//! let mut children = Vec::new();
//! for _ in 0..10 {
//!     let mut thread_sf = sf.clone();
//!     children.push(thread::spawn(move || {
//!         println!("{}", thread_sf.next_id().unwrap());
//!     }));
//! }
//!
//! for child in children {
//!     child.join().unwrap();
//! }
//! ```
//!
//! [fable/fableflake]: https://github.com/fable/fableflake
//! [Twitter's Snowflake]: https://blog.twitter.com/2010/announcing-snowflake
#![deny(warnings)]
#![deny(clippy::pedantic, clippy::unwrap_used)]
#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/fableflake/*")]

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
pub struct ReadmeDoctests;

mod builder;
mod error;
mod fableflake;
#[cfg(test)]
mod tests;

pub use crate::fableflake::*;
pub use builder::*;
pub use error::*;
