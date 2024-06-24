// Do not link against libstd (i.e. anything defined in `std::`)
#![no_std]

// However, we could still use some standard library types while
// remaining no-std compatible, if we uncommented the following lines:
//
// extern crate alloc;
// use alloc::vec::Vec;

// Global allocator to use heap memory in no-std environment
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Required for no-std crates
#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

use miden_sdk::*;

// Marking the function no_mangle ensures that it is exported
// from the compiled binary as `fib`, otherwise it would have
// a mangled name that has no stable form.
//
// You can specify a different name from the library than the
// name in the source code using the `#[export_name = "foo"]`
// attribute, which will make the function callable as `foo`
// externally (in this example)
#[no_mangle]
pub fn fib(n: u32) -> Felt {
    let mut a = felt!(0);
    let mut b = felt!(1);
    for _ in 0..n {
        let c = a + b;
        a = b;
        b = c;
    }
    a
}
