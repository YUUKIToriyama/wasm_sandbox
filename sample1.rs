#![no_main]
#![no_std]
#![no_mangle]

pub extern fn add(a: i32, b: i32) -> i32 { a + b }

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> !{ loop {} }
