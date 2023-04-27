use proc_macro::TokenStream;

use proc_macro2::{Literal, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_macro_input, parse_str, DeriveInput, Field};

#[proc_macro_derive(DisplayHelper, attributes(display))]
pub fn display_helper(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let struct_: syn::DataStruct = match ast.data {
        syn::Data::Struct(data) => data,
        _ => panic!("Usage of #[DisplayHelper] on a non-struct type"),
    };

    let fields = struct_
        .fields
        .iter()
        .flat_map(get_attr_name_values)
        .collect::<Vec<_>>();

    if fields.is_empty() {
        panic!("No fields to display, add #[display()] to some fields")
    }

    let field_format = fields
        .iter()
        .map(|field| format!("{:14}{{}}", field.0))
        .collect::<Vec<_>>()
        .join("\n");

    let field_params = fields
        .iter()
        .map(|field| format!("self.{}", field.1))
        .collect::<Vec<_>>()
        .join(", ");

    let type_name = parse_str::<TokenStream2>(ast.ident.to_string().as_str()).unwrap();
    let field_params_ts = parse_str::<TokenStream2>(field_params.as_str()).unwrap();

    let token_stream = quote!(
        impl Display for #type_name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                writeln!(f, #field_format, #field_params_ts)
            }
        }
    );

    TokenStream::from(token_stream)
}

#[proc_macro_derive(SelectDisplayHelper, attributes(select_value))]
pub fn select_display_helper(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let struct_: syn::DataStruct = match ast.data {
        syn::Data::Struct(data) => data,
        _ => panic!("Usage of #[SelectDisplayHelper] on a non-struct type"),
    };

    let field_names = struct_
        .fields
        .iter()
        .flat_map(|field| {
            field
                .attrs
                .iter()
                .map(|attr| {
                    if attr.path().is_ident("select_value") {
                        return match &field.ident {
                            None => String::new(),
                            Some(ident) => ident.to_string(),
                        };
                    }
                    String::new()
                })
                .filter(|name| !name.is_empty())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    if field_names.is_empty() {
        panic!("No #[select_value]")
    } else if field_names.len() > 1 {
        panic!("Multiple #[select_value]")
    }

    let type_name = parse_str::<TokenStream2>(ast.ident.to_string().as_str()).unwrap();
    let name = parse_str::<TokenStream2>(field_names.last().unwrap().as_str()).unwrap();

    let token_stream = quote!(
        impl SelectDisplay for #type_name {
            fn to_string(&self) -> String {
                format!("{}: {}", self.id, self.#name)
            }
        }
    );

    TokenStream::from(token_stream)
}

fn get_attr_name_values(field: &Field) -> Vec<(String, String)> {
    if let Some(ident) = &field.ident {
        field
            .attrs
            .iter()
            .map(|attr| {
                if attr.path().is_ident("display") {
                    if let Ok(name_ts) = attr.parse_args::<Literal>() {
                        return (
                            name_ts.to_string().replace('"', "") + ":",
                            ident.to_string(),
                        );
                    }
                }
                (String::new(), String::new())
            })
            .filter(|(name, ident)| !name.is_empty() && !ident.is_empty())
            .collect::<Vec<_>>()
    } else {
        panic!("No field ident")
    }
}
