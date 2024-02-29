
#![no_std] //! no need for rust std
#![no_main]
#![deny(missing_docs)]
#![deny(warnings)]
#![feature(panic_info_message)]

use core::arch::global_asm;

#[macro_use]
mod std_console;
mod std_panic_handler;
mod std_logging;
mod sbi;

global_asm!(include_str!("entry.asm"));

/// clear bss segment before the initialize of os
pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        // clear every Byte (a)
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}
/// the rust entry-point of os
#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    std_logging::init();
    panic!("shutdown machine!")
}
