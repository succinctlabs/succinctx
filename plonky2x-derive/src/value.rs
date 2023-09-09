use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{parse_quote, Generics};

use crate::StructData;

pub(crate) fn value(
    name: &Ident,
    value_derive: &[Ident],
    data: &StructData,
    generics: &Generics,
) -> (Generics, TokenStream) {
    // let namevalue = Ident::new(&format!("{}Value", name), name.span());
    let mut value_generics = generics.clone();
    value_generics.params.push(parse_quote!(F: RichField));

    let (_, _, where_clause) = value_generics.split_for_impl();

    let value_derive_recurs = value_derive
        .iter()
        .map(|d| {
            quote! {
                #d,
            }
        })
        .collect::<Vec<_>>();

    let value_derive_expanded = quote! {
        #[derive(#(#value_derive_recurs)*)]
    };

    let recurse = data.fields.iter().map(|(name, ty, vis)| {
        quote! {
            #vis #name: <#ty as CircuitVariable>::ValueType<F>,
        }
    });

    let value_expanded = quote! {
        #value_derive_expanded
        pub struct #name #value_generics #where_clause {
            #(#recurse)*
        }
    };
    (value_generics, value_expanded)
}
