// Do not link against libstd (i.e. anything defined in `std::`)
#![no_std]

// However, we could still use some standard library types while
// remaining no-std compatible, if we uncommented the following lines:
//
// extern crate alloc;
// use alloc::vec::Vec;

// // Global allocator to use heap memory in no-std environment
// #[global_allocator]
// static ALLOC: BumpAlloc = miden::BumpAlloc::new();

// Required for no-std crates
#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Pass up to the 16 u32 inputs as entrypoint function parameters.
// The output is temporarely limited to 1 u32 value
#[no_mangle]
pub fn entrypoint(a: u32, b: u32) -> u32 {
    a + b
}
