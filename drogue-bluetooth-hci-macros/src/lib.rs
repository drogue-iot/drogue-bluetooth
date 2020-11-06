#![allow(dead_code)]
#![allow(unused_variables)]

extern crate proc_macro;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

mod hci_command;
mod hci_serializable;
mod hci_deserializable;

#[proc_macro_derive(HCISerializable)]
pub fn hci_serializable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    hci_serializable::hci_serializable(input)
}

#[proc_macro_derive(HCIDeserializable)]
pub fn hci_deserializable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    hci_deserializable::hci_deserializable(input)
}

