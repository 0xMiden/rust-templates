#![no_std]

#[cfg(not(test))]
#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub fn entrypoint() -> i32 {
    // TODO: Implement your program logic here
    0
}