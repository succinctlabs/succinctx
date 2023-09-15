use proc_macro2::TokenStream;
use quote::quote;

use crate::StructData;

pub(crate) fn assert_is_valid(data: &StructData) -> TokenStream {
    let recurse = data.fields.iter().map(|(name, ty, _)| {
        quote! {
            <#ty as CircuitVariable>::assert_is_valid(&self.#name, builder);
        }
    });
    quote! {
        #(#recurse)*
    }
}
