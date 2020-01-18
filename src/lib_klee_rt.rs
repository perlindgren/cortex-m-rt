//! klee-rt
extern crate cortex_m_rt_macros as macros;
extern crate klee_sys;
use self::klee_sys::{klee_abort, klee_assume, klee_make_symbolic};

// re-export macros
pub use self::macros::{entry, exception, pre_init};

use ExceptionFrame;

// do not provide `main` if compiled for RTFM
#[cfg(not(feature = "rtpro"))]
#[no_mangle]
unsafe fn main() {
    extern "Rust" {
        fn __pre_init();
        fn main_klee() -> !;
        fn DefaultHandler(_irqn: i16) -> !;
        fn NonMaskableInt() -> !;
        fn HardFault(ef: &ExceptionFrame) -> !;
        fn MemoryManagement();
        fn BusFault();
        fn UsageFault();
        fn SecureFault();
        fn SVCall();
        fn DebugMonitor();
        fn PendSV();
        fn SysTick();
    }
    // exceptions
    let mut exception: u8 = 0;
    klee_make_symbolic!(&mut exception, "EXCEPTION");
    klee_assume(exception <= 12);
    match exception {
        0 => __pre_init(),
        1 => main_klee(),
        2 => {
            let mut irqn: i16 = 0;
            klee_make_symbolic!(&mut irqn, "IRQN");
            DefaultHandler(irqn)
        }
        3 => NonMaskableInt(),
        4 => {
            let mut ef = ExceptionFrame {
                r0: 0,   // (General purpose) Register 0
                r1: 0,   // (General purpose) Register 1
                r2: 0,   // (General purpose) Register 2
                r3: 0,   // (General purpose) Register 3
                r12: 0,  // (General purpose) Register 12
                lr: 0,   // Link Register
                pc: 0,   // Program Counter
                xpsr: 0, // Program Status Register
            };
            klee_make_symbolic!(&mut ef, "EXCEPTION_FRAME");
            HardFault(&ef)
        }
        5 => MemoryManagement(),
        6 => BusFault(),
        7 => UsageFault(),
        8 => SecureFault(),
        9 => SVCall(),
        10 => DebugMonitor(),
        11 => PendSV(),
        12 => SysTick(),
        _ => unreachable!(),
    }
    #[cfg(feature = "klee-replay")]
    bkpt!();
}

pub use self::macros::interrupt;
