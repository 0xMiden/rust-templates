#![no_std]

extern crate alloc;

#[global_allocator]
static ALLOC: miden::BumpAlloc = miden::BumpAlloc::new();

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

use bindings::miden::add_contract::add_contract::add;
use miden::{felt, note_script, Felt, Word};

#[note_script]
fn run(_arg: Word) {
    let a = felt!(1);
    let b = felt!(2);
    add(a, b);
}
