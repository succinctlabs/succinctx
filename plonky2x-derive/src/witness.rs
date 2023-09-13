use proc_macro2::TokenStream;
use quote::quote;

use crate::StructData;

pub(crate) fn set(data: &StructData) -> TokenStream {
    let recurse = data.fields.iter().map(|(name, ty, _)| {
        quote! {
            <#ty as CircuitVariable>::set(&self.#name, witness, value.#name);
        }
    });
    quote! {
        #(#recurse)*
    }
}

pub(crate) fn get(data: &StructData) -> TokenStream {
    let recurse = data.fields.iter().map(|(name, ty, _)| {
        quote! {
            #name: <#ty as CircuitVariable>::get(&self.#name, witness),

        }
    });
    quote! {
        Self::ValueType::<F> {
            #(#recurse)*
        }
    }
}
