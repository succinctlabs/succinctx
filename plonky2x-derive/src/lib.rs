extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, GenericParam, parse_quote, Generics};

mod value;

#[proc_macro_derive(CircuitVariable)]
pub fn derive_circuit_variable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    
    let expanded = quote! {
        impl #impl_generics CircuitVariable for #name #ty_generics #where_clause {

            type ValueType<F: RichField> = F;

            fn init<L: PlonkParameters<D>, const D: usize>(builder: &mut CircuitBuilder<L, D>) -> Self {

            }

            fn constant<L: PlonkParameters<D>, const D: usize>(
                builder: &mut CircuitBuilder<L, D>,
                value: Self::ValueType<L::Field>,
            ) -> Self {

            }

            fn variables(&self) -> Vec<Variable> {

            }

            fn from_variables(variables: &[Variable]) -> Self {

            }

            fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {

            }

            fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {

            }
        }
    };

    TokenStream::from(expanded)
}



fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(CircuitVariable));
        }
    }
    generics
}