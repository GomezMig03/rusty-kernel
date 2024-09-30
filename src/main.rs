#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rusty_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rusty_kernel::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    println!("Testing a really really really really really really really really really really really really really really long line");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))] 
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rusty_kernel::test_panic_handler(info)
}