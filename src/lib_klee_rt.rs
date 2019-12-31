//! klee-rt
extern crate cortex_m_rt_macros as macros;
// extern crate klee_sys;
use klee_sys::{klee_abort, klee_make_symbolic};

pub use self::macros::{entry, exception, pre_init};

// Generetate tests for
// user_main (generated from [entry])
//

#[no_mangle]
fn main() {
    extern "Rust" {
        fn __pre_init();
        fn user_main();

        fn DefaultHandler(irq_nr: i16);
        fn NonMaskableInt();
        fn HardFault(ef: &ExceptionFrame);
        fn MemoryManagement();
        fn BusFault();
        fn UsageFault();
        fn SecureFault();
        fn SVCall();
        fn DebugMonitor();
        fn PendSV();
        fn SysTick();
    }

    let mut root = 0;
    klee_make_symbolic!(&mut root, "ROOT");
    unsafe {
        match root {
            0 => __pre_init(),
            1 => user_main(),
            2 => {
                let mut irqn: i16 = 0;
                klee_make_symbolic!(&mut irqn, "IRQN");
                DefaultHandler(irqn)
            }
            3 => NonMaskableInt(),
            4 => {
                let mut ef: ExceptionFrame = core::mem::uninitialized();
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
            _ => klee_abort!(),
        }
    }
}

///
pub fn exit() -> ! {
    loop {}
}

/* Exceptions */
#[doc(hidden)]
pub enum Exception {
    NonMaskableInt,
    MemoryManagement,
    BusFault,
    UsageFault,
    SecureFault,
    SVCall,
    DebugMonitor,
    PendSV,
    SysTick,
}

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
