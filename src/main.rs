#![no_std]
#![no_main]

mod vga_buffer;

use core::fmt::Write;
//we will not be using main function as the entrypoint
use core::panic::PanicInfo;

//this function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("{}",info);
    loop{}
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
    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    // write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}",42,1.337).unwrap();

    //finally using the println macro
    println!("hello world! {}","Alwin");
    panic!("This is a test panic!");
    loop {}
}

//test commit
