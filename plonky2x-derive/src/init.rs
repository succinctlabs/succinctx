use proc_macro2::TokenStream;
use quote::quote;

use crate::StructData;

pub(crate) fn init(data: &StructData) -> TokenStream {
    let recurse = data.fields.iter().map(|(name, ty, _)| {
        quote! {
            #name: <#ty as CircuitVariable>::init(builder),
        }
    });
    quote! {
        Self {
            #(#recurse)*
        }
    }
}
