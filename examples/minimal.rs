//! Minimal `cortex-m-rt` based program

#![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m_rt as rt;

extern crate klee;
extern crate panic_abort;

use klee::ksymbol;
use rt::{entry, exception, pre_init, ExceptionFrame};

// the program entry point
#[entry]
fn main() {
    let mut s = (0, 0);
    ksymbol!(&mut s, "s");
    let mut _sum = 0;
    if s.0 == 0 {
        if s.1 == 0 {
            _sum += 1;
        }
    }
}

// #[exception]
// fn SysTick() {
//     let mut s = (0, 0);
//     ksymbol!(&mut s, "s");
//     let mut _sum = 0;
//     if s.0 == 0 {
//         if s.1 == 0 {
//             _sum += 1;
//         }
//     }
// }

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) {
    let mut _sum = 0;
    if ef.r0 == 0 {
        if ef.r1 == 0 {
            _sum += 1;
        }
    }
}

#[exception]
fn DefaultHandler(irqn: i16) {
    let mut _sum = 0;
    if irqn == 0 {
        _sum += 1;
    }
    if irqn == 1 {
        _sum += 2;
    }
}

#[pre_init]
unsafe fn before_main() {
    let mut s = (0, 0);
    ksymbol!(&mut s, "s");
    let mut _sum = 0;
    if s.0 == 0 {
        if s.1 == 0 {
            _sum += 1;
        }
    }
}
