use proc_macro2::TokenStream;

use syn::{DeriveInput, Attribute, Data, DataStruct, DataUnion, DataEnum};
use proc_macro::Ident;

pub fn hci_deserializable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let deser = deserialize(&name, &input.data);

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl drogue_bluetooth_hci::ser::Deserializable<#name> for #name {
            fn parse(i: &[u8]) -> nom::IResult<&[u8],#name> {
                #deser
            }
        }
    };

    // Hand the output tokens back to the compiler
    proc_macro::TokenStream::from(expanded)
}

fn deserialize(name: &syn::Ident, data: &Data) -> TokenStream {
    match data {
        Data::Struct(s) => {
            deserialize_struct(name, s)
        }
        Data::Enum(e) => {
            deserialize_enum(name, e)
        }
        Data::Union(u) => {
            deserialize_union(name, u)
        }
    }
}

fn deserialize_struct(name: &syn::Ident, s: &DataStruct) -> TokenStream {
    let parsers = s.fields.iter().map(|f| {
        let name = &f.ident.as_ref().unwrap();
        let ty = &f.ty;

        quote!{
            let (i, #name) = <#ty as drogue_bluetooth_hci::ser::Deserializable<_>>::parse(i)?;
        }
    });

    let fields = s.fields.iter().map(|f| {
        let name = &f.ident.as_ref().unwrap();
        quote! {
            #name,
        }
    });

    quote! {
        #(
            #parsers
        )*

        Ok(
            (i,
                Self {
                    #(
                        #fields
                    )*
                }
            )
        )
    }
}

fn deserialize_enum(name: &syn::Ident, s: &DataEnum) -> TokenStream {
    quote! {}
}

fn deserialize_union(name: &syn::Ident, s: &DataUnion) -> TokenStream {
    quote! {}
}
