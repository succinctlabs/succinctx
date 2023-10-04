use proc_macro2::TokenStream;
use quote::quote;

use crate::StructData;

pub(crate) fn elements(data: &StructData) -> TokenStream {
    let recurse = data.fields.iter().map(|(name, ty, _)| {
        quote! {
            elements_vec.extend_from_slice(<#ty as CircuitVariable>::elements(&value.#name).as_slice());

        }
    });
    quote! {
        let mut elements_vec = vec![];

        #(#recurse)*

        elements_vec
    }
}

pub(crate) fn from_elements(data: &StructData) -> TokenStream {
    let value_recurse = data.fields.iter().map(|(name, ty, _)| {
        quote! {
            let cv_derive_imple_size = <#ty as CircuitVariable>::nb_elements();
            let #name = <#ty as CircuitVariable>::from_elements(&elements[cv_derive_impl_index..cv_derive_impl_index+cv_derive_imple_size]);
            cv_derive_impl_index += cv_derive_imple_size;
        }
    });

    let instant_recurse = data.fields.iter().map(|(name, _, _)| {
        quote! {
            #name,
        }
    });
    quote! {
        let mut cv_derive_impl_index = 0;
        #(#value_recurse)*

        Self {
            #(#instant_recurse)*
        }
    }
}

pub(crate) fn nb_elements(data: &StructData) -> TokenStream {
    let value_recurse = data.fields.iter().map(|(_, ty, _)| {
        quote! {
            res += <#ty as CircuitVariable>::nb_elements();
        }
    });

    quote! {
        let mut res = 0;
        #(#value_recurse)*

        res
    }
}
