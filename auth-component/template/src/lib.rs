#![no_std]

#[global_allocator]
static ALLOC: miden::BumpAlloc = miden::BumpAlloc::new();

#[cfg(not(test))]
#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

use miden::{component, Word};

#[component]
struct AuthComponent;

#[component]
impl AuthComponent {
    pub fn auth_procedure(&self, _arg: Word) {
        // If this procedure returns control the transcation authentication is considered succesfull.
        // In case of panic the transaction authentication considered as failed by the transaction kernel.
        todo!()
    }
}
