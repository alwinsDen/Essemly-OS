#![no_std]
#![no_main]

mod vga_buffer;

//we will not be using main function as the entrypoint
use core::panic::PanicInfo;

//this function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"hello world!";

// no_mangle prevents from function name prined as a crytic
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // let vga_buffer = 0xb8000 as *mut u8;
    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }
    vga_buffer::print_something();
    loop {}
}

//test commit
