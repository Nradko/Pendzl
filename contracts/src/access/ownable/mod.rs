// SPDX-License-Identifier: MIT
include!("ownable_error.rs");
include!("ownable_events.rs");
include!("ownable_trait.rs");

/// implementation of the traits
#[cfg(feature = "ownable_impl")]
pub mod implementation;
