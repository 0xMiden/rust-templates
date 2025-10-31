#![no_std]

extern crate alloc;

use miden::{component, Felt};

#[component]
struct MyAccountComponent;

#[component]
impl MyAccountComponent {
    pub fn add(&self, a: Felt, b: Felt) -> Felt {
        a + b
    }
}
