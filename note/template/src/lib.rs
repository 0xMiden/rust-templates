#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

use bindings::miden::add_contract::add_contract::add;
use miden::{felt, note_script, Felt, Word};

#[note_script]
fn run(_arg: Word) {
    let a = felt!(1);
    let b = felt!(2);
    add(a, b);
}
