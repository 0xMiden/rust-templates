#![no_std]

#[global_allocator]
static ALLOC: miden::BumpAlloc = miden::BumpAlloc::new();

#[cfg(not(test))]
#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

use bindings::exports::miden::base::authentication_component::Guest;
use miden::{component, Word};

#[component]
struct AuthComponent;

impl Guest for AuthComponent {
    fn auth_procedure(_arg: Word) {
        // If this procedure returns control the transcation authentication is considered succesfull.
        // In case of panic the transaction authentication considered as failed by the transaction kernel.
        todo!()
    }
}
