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
#![feature(asm)]
// #![cfg(feature = "klee-replay", feature(asm)] // set the asm feature for klee-replay
#![cfg(feature = "klee-replay")]
#[macro_use]
extern crate cortex_m;

// Common data structures
/// Registers stacked (pushed into the stack) during an exception
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExceptionFrame {
    /// (General purpose) Register 0
    pub r0: u32,

    /// (General purpose) Register 1
    pub r1: u32,

    /// (General purpose) Register 2
    pub r2: u32,

    /// (General purpose) Register 3
    pub r3: u32,

    /// (General purpose) Register 12
    pub r12: u32,

    /// Linker Register
    pub lr: u32,

    /// Program Counter
    pub pc: u32,

    /// Program Status Register
    pub xpsr: u32,
}
// re-exports

// re-export `lib_klee_rt` thumb library as `cortex_m_rt`
#[cfg(any(feature = "klee-analysis", feature = "klee-replay"))]
pub mod lib_klee_rt;
#[cfg(any(feature = "klee-analysis", feature = "klee-replay"))]
pub use self::lib_klee_rt::*;

// re-export `lib_thumb_rt` thumb library as `cortex_m_rt`
#[cfg(all(not(feature = "klee-analysis"), feature = "klee-replay"))]
pub mod lib_thumb_rt;
#[cfg(all(not(feature = "klee-analysis"), feature = "klee-replay"))]
pub use self::lib_thumb_rt::*; // ugly path for now...

// pub use lib_thumb_rt::*; // self can be dropped when uniform paths are stabilized
