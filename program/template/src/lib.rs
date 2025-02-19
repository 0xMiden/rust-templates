// Do not link against libstd (i.e. anything defined in `std::`)
#![no_std]

// However, we could still use some standard library types while
// remaining no-std compatible, if we uncommented the following lines:
//
// extern crate alloc;
// use alloc::vec::Vec;

// // Global allocator to use heap memory in no-std environment
// #[global_allocator]
// static ALLOC: miden::BumpAlloc = miden::BumpAlloc::new();

// Required for no-std crates
#[cfg(not(test))]
#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Pass up to 16 i32 inputs as entrypoint function parameters.
// The output is temporarely limited to 1 i32 value
//
// NOTE:
// The name of the entrypoint function is expected to be `entrypoint`. Do not remove the
// `#[no_mangle]` attribute, otherwise, the rustc will mangle the name and it'll not be recognized
// by the Miden compiler.
#[no_mangle]
pub fn entrypoint(a: i32, b: i32) -> i32 {
    a + b
}
