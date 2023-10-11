use proc_macro2::TokenStream;
use quote::quote;

use crate::StructData;

pub(crate) fn constant(data: &StructData) -> TokenStream {
    let recurse = data.fields.iter().map(|(name, ty, _)| {
        quote! {
            #name: <#ty as CircuitVariable>::constant(builder, value.#name),
        }
    });
    quote! {
        Self {
            #(#recurse)*
        }
    }
}
