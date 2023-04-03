#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)] //here crate::test_runner is the name of the function.
#![reexport_test_harness_main = "test_main"]
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

//exit Qemu function
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode{
    Success = 0x10,
    Failed = 0x11
}
pub fn exit_qemu(exit_code : QemuExitCode) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

static HELLO: &[u8] = b"hello world!";
#[cfg(test)] //panic = "abort" needs to be commented in toml for 'cargo test' to run.
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
//here are the added test cases.
//test case 1
#[test_case]
fn trivial_assertion(){
    print!("trivial assertion...");
    assert_eq!(1,1);
    println!("[Ok]");
}
#[test_case]
fn test_2(){
    print!("Test for equality...");
    assert_eq!(2,2);
    println!("[Passed]");
}

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

    //adding test function (only called with 'cargo test')
    #[cfg(test)]
    test_main();

    //finally using the println macro
    println!("hello world! {}","Alwin");
    panic!("This is a test panic!");

    loop {}
}
