extern crate proc_macro;

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, GenericParam, Generics};

#[proc_macro_derive(CircuitVariable)]
pub fn derive_circuit_variable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let data = input.data;

    let (value_ident, value_expanded) = value(&name, &data, &generics);

    let init_expanded = init(&data);
    let constant_expanded = constant(&data);
    let variables_expanded = variables(&data);
    let from_variables_expanded = from_variables(&data);
    let set_exapaned = set(&data);
    let get_exapaned = get(&data);

    let expanded = quote! {

        #value_expanded

        impl #impl_generics CircuitVariable for #name #ty_generics #where_clause {

            type ValueType<F: RichField> = #value_ident<F>;

            fn init<L: PlonkParameters<D>, const D: usize>(builder: &mut CircuitBuilder<L, D>) -> Self {
                #init_expanded
            }

            fn constant<L: PlonkParameters<D>, const D: usize>(
                builder: &mut CircuitBuilder<L, D>,
                value: Self::ValueType<L::Field>,
            ) -> Self {
                #constant_expanded
            }

            fn variables(&self) -> Vec<Variable> {
                #variables_expanded
            }

            fn from_variables(variables: &[Variable]) -> Self {
                #from_variables_expanded
            }

            fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
                #get_exapaned
            }

            fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
                #set_exapaned
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn init(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            let recurse = data.fields.iter().map(|f| {
                let name = &f.ident;
                let ty = &f.ty;
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
        Data::Enum(_) => unimplemented!("enums not supported"),
        Data::Union(_) => unimplemented!("unions not supported"),
    }
}

fn constant(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            let recurse = data.fields.iter().map(|f| {
                let name = &f.ident;
                let ty = &f.ty;
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
        Data::Enum(_) => unimplemented!("enums not supported"),
        Data::Union(_) => unimplemented!("unions not supported"),
    }
}

fn variables(data: &Data) -> TokenStream {
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

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(CircuitVariable));
        }
    }
    generics
}

fn value(name: &Ident, data: &Data, _generics: &Generics) -> (Ident, TokenStream) {
    let namevalue = Ident::new(&format!("{}Value", name), name.span());
    // let mut value_generics = generics.clone();
    // value_generics.params.push(parse_quote!(F: RichField));
    let value_expanded = match *data {
        Data::Struct(ref data) => {
            let recurse = data.fields.iter().map(|f| {
                let name = &f.ident;
                let ty = &f.ty;
                // let visibility = &f.vis;
                quote! {
                    pub #name: <#ty as CircuitVariable>::ValueType<F>,
                }
            });
            quote! {
                #[derive(Debug, Clone)]
                pub struct #namevalue<F: RichField> {
                    #(#recurse)*
                }
            }
        }
        Data::Enum(_) => unimplemented!("enums not supported"),
        Data::Union(_) => unimplemented!("unions not supported"),
    };
    (namevalue, value_expanded)
}

fn from_variables(data: &Data) -> TokenStream {
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

fn set(data: &Data) -> TokenStream {
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

fn get(data: &Data) -> TokenStream {
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
