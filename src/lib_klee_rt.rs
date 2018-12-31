//! klee-rt

extern crate cortex_m_rt_macros as macros;

pub use self::macros::{entry, exception, pre_init};

// main implementation

#[no_mangle]
fn main() {
    // let mut s: u32 = 0;
    // ksymbol!(&mut s, "s");
    // if s == 0 {
    //     panic!();
    // }
}