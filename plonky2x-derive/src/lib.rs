extern crate proc_macro;

mod constant;
mod init;
mod value;
mod variables;
mod witness;

use constant::constant;
use init::init;
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput, GenericParam, Generics};
use value::value;
use variables::{from_variables, variables};
use witness::{get, set};

#[proc_macro_derive(CircuitVariable)]
pub fn derive_circuit_variable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let data = input.data;

    let (value_ident, value_generics, value_expanded) = value(&name, &data, &generics);
    let (_, value_ty_generics, _) = value_generics.split_for_impl();

    let init_expanded = init(&data);
    let constant_expanded = constant(&data);
    let variables_expanded = variables(&data);
    let from_variables_expanded = from_variables(&data);
    let set_exapaned = set(&data);
    let get_exapaned = get(&data);

    let expanded = quote! {

        #value_expanded

        impl #impl_generics CircuitVariable for #name #ty_generics where #where_clause {

            type ValueType<F: RichField> = #value_ident #value_ty_generics;

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

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(CircuitVariable));
        }
    }
    generics
}
