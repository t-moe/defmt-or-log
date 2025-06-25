#![no_std]
#![macro_use]
#![allow(unused_macros)]

pub mod macros;
mod traits;
pub use traits::*;

pub use defmt_or_log_macros::*;

pub mod wrappers;
