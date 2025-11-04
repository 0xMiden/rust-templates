#![no_std]

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
