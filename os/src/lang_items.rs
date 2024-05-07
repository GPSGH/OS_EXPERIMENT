//! The panic handler

use core::panic::PanicInfo;
use crate::sbi::shutdown;

// print the error message and shutdown
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Hello Panic Handler!");
    if let Some(location) = info.location() {
        println!(
            "[kernel] Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("[kernel] Panicked: {}", info.message().unwrap());
    }
    shutdown(true)
}