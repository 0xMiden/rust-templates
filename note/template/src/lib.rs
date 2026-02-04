#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

use miden::{felt, note, Word};

use bindings::miden::add_contract::add_contract::add;
use bindings::Account;

#[note]
struct MyNote {}

#[note]
impl MyNote {
    #[note_script]
    pub fn run(self, _arg: Word, _account: &mut Account) {
        let a = felt!(1);
        let b = felt!(2);
        add(a, b);
    }
}
