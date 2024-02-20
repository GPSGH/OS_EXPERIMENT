
#![no_std] //! no need for rust std
#![no_main]
#![feature(panic_info_message)]
#![deny(missing_docs)]
#![deny(warnings)]

mod std_panic_handler;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));
