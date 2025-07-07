#![no_std]

extern crate alloc;

#[global_allocator]
static ALLOC: miden::BumpAlloc = miden::BumpAlloc::new();

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

mod bindings;

use bindings::exports::miden::{{crate_name}}::{{crate_name}}::Guest;
use miden::{component, Felt};

#[component]
struct MyAccountComponent;

bindings::export!(MyAccountComponent with_types_in bindings);

impl Guest for MyAccountComponent {
    fn add(a: Felt, b: Felt) -> Felt {
        a + b
    }
}
