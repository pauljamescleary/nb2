// alias for the test macro
#[cfg(test)]
extern crate self as nb2;

// make sure macros are defined before other mods
mod macros;

mod core_map;
mod dpdk;
mod ffi;
mod mempool_map;
pub mod net;
mod runtime;
pub mod settings;
mod testil;

pub use crate::dpdk::Mbuf;
pub use crate::runtime::Runtime;
pub use nb2_macros::test;

use failure::Error;
use std::result;

/// A type alias of `std:result::Result` for convenience.
pub type Result<T> = result::Result<T, Error>;
