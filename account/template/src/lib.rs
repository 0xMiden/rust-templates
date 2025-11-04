#![no_std]

extern crate alloc;

#[global_allocator]
static ALLOC: miden::BumpAlloc = miden::BumpAlloc::new();

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

use miden::{component, Felt};

#[component]
struct MyAccountComponent;

#[component]
impl MyAccountComponent {
    pub fn add(&self, a: Felt, b: Felt) -> Felt {
        a + b
    }
}
