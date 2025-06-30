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

use bindings::exports::miden::base::note_script::Guest;
use bindings::miden::add_contract::add_contract::add;
use miden::{felt, Felt};

struct MyNote;

bindings::export!(MyNote with_types_in bindings);

impl Guest for MyNote {
    fn note_script() {
        let a = felt!(1);
        let b = felt!(2);
        add(a, b);
    }
}

