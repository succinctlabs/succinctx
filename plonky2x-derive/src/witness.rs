use proc_macro2::TokenStream;
use quote::quote;
use syn::Data;

pub fn set(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            let recurse = data.fields.iter().map(|f| {
                let name = &f.ident;
                let ty = &f.ty;
                quote! {
                    <#ty as CircuitVariable>::set(&self.#name, witness, value.#name);
                }
            });
            quote! {
                #(#recurse)*
            }
        }
        Data::Enum(_) => unimplemented!("enums not supported"),
        Data::Union(_) => unimplemented!("unions not supported"),
    }
}

pub fn get(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            let recurse = data.fields.iter().map(|f| {
                let name = &f.ident;
                let ty = &f.ty;
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
        Data::Enum(_) => unimplemented!("enums not supported"),
        Data::Union(_) => unimplemented!("unions not supported"),
    }
}
