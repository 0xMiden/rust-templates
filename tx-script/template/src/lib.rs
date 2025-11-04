#![no_std]

extern crate alloc;

use bindings::miden::add_contract::add_contract::add;
use miden::{felt, tx_script, Felt, Word};

#[tx_script]
fn run(_arg: Word) {
    // Transaction script logic here
    // This is just an example - replace with your actual transaction logic

    let a = felt!(1);
    let b = felt!(2);
    let _result = add(a, b);

    // You can also create notes, move assets, etc.
    // See the Miden SDK documentation for available transaction operations
}
