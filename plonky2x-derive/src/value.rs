use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{parse_quote, Data, Generics};

pub fn value(name: &Ident, data: &Data, generics: &Generics) -> (Ident, Generics, TokenStream) {
    let namevalue = Ident::new(&format!("{}Value", name), name.span());
    let mut value_generics = generics.clone();
    value_generics.params.push(parse_quote!(F: RichField));
    let value_expanded = match *data {
        Data::Struct(ref data) => {
            let recurse = data.fields.iter().map(|f| {
                let name = &f.ident;
                let ty = &f.ty;
                let vis = &f.vis;
                quote! {
                    #vis #name: <#ty as CircuitVariable>::ValueType<F>,
                }
            });
            quote! {
                #[derive(Debug, Clone)]
                pub struct #namevalue #value_generics {
                    #(#recurse)*
                }
            }
        }
        Data::Enum(_) => unimplemented!("enums not supported"),
        Data::Union(_) => unimplemented!("unions not supported"),
    };
    (namevalue, value_generics, value_expanded)
}
