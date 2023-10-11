use proc_macro2::TokenStream;
use quote::quote;

use crate::StructData;

pub(crate) fn init_unsafe(data: &StructData) -> TokenStream {
    let recurse = data.fields.iter().map(|(name, ty, _)| {
        quote! {
            #name: <#ty as CircuitVariable>::init_unsafe(builder),
        }
    });
    quote! {
        Self {
            #(#recurse)*
        }
    }
}
