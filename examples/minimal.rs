//! Minimal `cortex-m-rt` based program

#![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m_rt as rt;
#[macro_use]
extern crate klee;
extern crate panic_abort;

use rt::{entry, exception};

// the program entry point
#[entry]
fn main() -> ! {
    panic!();
}

// #[no_mangle]
// fn main() {
//     let mut s: u32 = 0;
//     ksymbol!(&mut s, "s");
//     if s == 0 {
//         panic!();
//     }
// }

#[exception]
fn DefaultHandler(_irqn: i16) {
    panic!();
}
