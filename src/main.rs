#![no_std]
#![no_main]

//we will not be using main function as the entrypoint
use core::panic::PanicInfo;

//this function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// no_mangle prevents from function name prined as a crytic
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

//test commit
