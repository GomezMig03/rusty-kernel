#![no_std]
#![no_main]
use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buff = 0xb8000 as *mut u8; // Raw pointer with the VGA buffer address

    // Iterate the "Hello" word and write it in the VGA adresses with cyan color (0xb) 
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buff.offset(i as isize * 2) = byte;
            *vga_buff.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}