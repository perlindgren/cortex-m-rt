//! Startup code and minimal runtime for Cortex-M microcontrollers
//!
//! This crate contains all the required parts to build a `no_std` application (binary crate) that
//! targets a Cortex-M microcontroller.
//!
//! Features
// #![feature(uniform_paths)] // enable to use nice paths
#![deny(missing_docs)]
// #![deny(warnings)]
#![no_std]

// re-exports

// re-export `lib_klee_rt` thumb library as `cortex_m_rt`
#[cfg(feature = "klee-analysis")]
pub mod lib_klee_rt;
#[cfg(feature = "klee-analysis")]
pub use lib_klee_rt::*;

// re-export `lib_thumb_rt` thumb library as `cortex_m_rt`
#[cfg(not(feature = "klee-analysis"))]
pub mod lib_thumb_rt;
#[cfg(not(feature = "klee-analysis"))]
pub use lib_thumb_rt::*; // ugly path for now...
