#![no_std]
#![feature(alloc_error_handler)]

#[cfg(not(test))]
#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(not(test))]
#[alloc_error_handler]
fn my_alloc_error(_info: core::alloc::Layout) -> ! {
    loop {}
}

#[no_mangle]
pub fn entrypoint() -> i32 {
    // TODO: Implement your program logic here
    0
}
