use proc_macro2::TokenStream;
use quote::quote;
use syn::Data;

pub fn variables(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            let recurse = data.fields.iter().map(|f| {
                        let name = &f.ident;
                        let ty = &f.ty;
                        quote! {
                            vars_vec.extend_from_slice(<#ty as CircuitVariable>::variables(&self.#name).as_slice());

                        }
                    });
            quote! {
                let mut vars_vec = vec![];

                #(#recurse)*

                vars_vec
            }
        }
        Data::Enum(ref data) => {
            let recurse = data.variants.iter().enumerate().map(|(i, v)| {
                let name = &v.ident;
                quote! {
                    #name => {
                        let mut vars = vec![F::from_canonical_usize(#i as usize)];
                        vars.extend_from_slice(&<#name as CircuitVariable>::variables(self));
                        vars
                    },
                }
            });
            quote! {
                match self {
                    #(#recurse)*
                }
            }
        }
        Data::Union(_) => unimplemented!(),
    }
}

pub fn from_variables(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            let value_recurse = data.fields.iter().map(|f| {
                let name = &f.ident;
                let ty = &f.ty;
                quote! {
                    let size = <#ty as CircuitVariable>::nb_elements();
                    let #name = <#ty as CircuitVariable>::from_variables(&variables[index..index+size]);
                    index += size;
                }
            });

            let instant_recurse = data.fields.iter().map(|f| {
                let name = &f.ident;
                quote! {
                    #name,
                }
            });
            quote! {
                let mut index = 0;
                #(#value_recurse)*

                Self {
                    #(#instant_recurse)*
                }
            }
        }
        Data::Enum(_) => unimplemented!("enums not supported"),
        Data::Union(_) => unimplemented!("unions not supported"),
    }
}
