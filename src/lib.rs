#![cfg_attr(all(not(test), not(feature = "std")), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub use macros::*;

// This mod MUST go first, so that the others see its macros.
mod macros;