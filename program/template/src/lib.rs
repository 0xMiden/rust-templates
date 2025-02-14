// Do not link against libstd (i.e. anything defined in `std::`)
#![no_std]

// However, we could still use some standard library types while
// remaining no-std compatible, if we uncommented the following lines:
//
// extern crate alloc;
// use alloc::vec::Vec;

// // Global allocator to use heap memory in no-std environment
#[global_allocator]
static ALLOC: BumpAlloc = miden::BumpAlloc::new();

// Required for no-std crates
#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

use miden::*;

// Pass up to 16 Felt inputs as entrypoint function parameters.
// The output is temporarely limited to 1 Felt value
//
// NOTE:
// The name of the entrypoint function is expected to be `entrypoint`. Do not remove the
// `#[no_mangle]` attribute, otherwise, the rustc will mangle the name and it'll not be recognized
// by the Miden compiler.
#[no_mangle]
pub fn entrypoint(a: Felt, b: Felt) -> Felt {
    a + b
}
