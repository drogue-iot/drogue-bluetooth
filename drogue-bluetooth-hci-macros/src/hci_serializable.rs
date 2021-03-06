use proc_macro2::TokenStream;

use syn::{DeriveInput, Attribute, Data, DataStruct, DataUnion, DataEnum};
use proc_macro::Ident;

pub fn hci_serializable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let ser = serialize(&input.data);

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl drogue_bluetooth_hci::ser::Serializable for #name {
            fn serialize<N:heapless::ArrayLength<u8>>(&self, o: &mut Vec<u8,N>) -> Result<(),()> {
                #ser
                Ok(())
            }
        }
    };

    // Hand the output tokens back to the compiler
    proc_macro::TokenStream::from(expanded)
}

fn serialize(data: &Data) -> TokenStream {
    match data {
        Data::Struct(s) => {
            serialize_struct(s)
        }
        Data::Enum(e) => {
            serialize_enum(e)
        }
        Data::Union(u) => {
            serialize_union(u)
        }
    }
}

fn serialize_struct(s: &DataStruct) -> TokenStream {
    let field: Vec<&syn::Ident> = s.fields.iter().map(|e| e.ident.as_ref().unwrap()).collect();
    quote! {
        #(
            self.#field.serialize(o)?;
        )*
    }
}

fn serialize_enum(s: &DataEnum) -> TokenStream {
    quote! {}
}

fn serialize_union(s: &DataUnion) -> TokenStream {
    quote! {}
}
