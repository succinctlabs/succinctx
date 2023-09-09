extern crate proc_macro;

mod constant;
mod init;
mod value;
mod variables;
mod witness;

use constant::constant;
use init::init;
use proc_macro2::Ident;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Generics, Meta, Type, Visibility,
    WherePredicate,
};
use value::value;
use variables::{from_variables, variables};
use witness::{get, set};

struct StructData {
    fields: Vec<(Option<Ident>, Type, Visibility)>,
}

#[proc_macro_derive(CircuitVariable, attributes(value_name, value_derive))]
pub fn derive_circuit_variable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let data = parse_struct_data(input.data);

    let mut value_ident = Ident::new(&format!("{}Value", name), name.span());
    let mut value_derive = vec![parse_quote!(Debug), parse_quote!(Clone)];

    for attr in &input.attrs {
        if attr.path().is_ident("value_name") {
            value_ident = attr.parse_args::<Ident>().unwrap();
        }
        if attr.path().is_ident("value_derive") {
            match attr.meta {
                Meta::Path(ref path) => value_derive.push(path.get_ident().unwrap().clone()),
                Meta::List(ref list) => {
                    list.parse_nested_meta(|meta| {
                        let ident = meta
                            .path
                            .get_ident()
                            .expect("Could not parse value_derive attribute");
                        value_derive.push(ident.clone());
                        Ok(())
                    })
                    .expect("Could not parse value_derive atrributes");
                }
                Meta::NameValue(_) => panic!("value_derive cannot be a named value"),
            }
        }
    }

    let mut generics = input.generics;
    make_where_clause(&data, &mut generics);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let (value_generics, value_expanded) = value(&value_ident, &value_derive, &data, &generics);
    let (_, value_ty_generics, _) = value_generics.split_for_impl();

    let init_expanded = init(&data);
    let constant_expanded = constant(&data);
    let variables_expanded = variables(&data);
    let from_variables_expanded = from_variables(&data);
    let set_exapaned = set(&data);
    let get_exapaned = get(&data);

    let expanded = quote! {

        #value_expanded

        impl #impl_generics CircuitVariable for #name #ty_generics #where_clause {

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

fn parse_struct_data(data: Data) -> StructData {
    match data {
        Data::Struct(data) => StructData {
            fields: data
                .fields
                .into_iter()
                .map(|f| {
                    let name = f.ident;
                    let ty = f.ty;
                    let vis = f.vis;

                    (name, ty, vis)
                })
                .collect(),
        },
        Data::Enum(_) => unimplemented!("enums not supported"),
        Data::Union(_) => unimplemented!("unions not supported"),
    }
}

fn make_where_clause(data: &StructData, generics: &mut Generics) {
    let circuit_var_recurse = data.fields.iter().map(|(_, ty, _)| -> WherePredicate {
        parse_quote! {
            #ty: CircuitVariable
        }
    });

    let where_clause = generics
        .where_clause
        .get_or_insert_with(|| parse_quote!(where));
    where_clause.predicates.extend(circuit_var_recurse);
}
