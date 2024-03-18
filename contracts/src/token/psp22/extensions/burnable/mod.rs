// SPDX-License-Identifier: MIT
include!("burnable_trait.rs");

#[cfg(feature = "psp22_burnable_impl")]
mod implementation;

#[cfg(feature = "psp22_burnable_impl")]
pub use implementation::*;
