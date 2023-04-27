use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, parse_str, DeriveInput, Field};

#[proc_macro_derive(DisplayHelper, attributes(display))]
pub fn print_attr(input: TokenStream) -> TokenStream {
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

    let x = quote!(
        impl Display for #type_name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                writeln!(f, #field_format, #field_params_ts)
            }
        }
    );

    TokenStream::from(x)
}

fn get_attr_name_values(field: &Field) -> Vec<(String, String)> {
    if let Some(ident) = &field.ident {
        field
            .attrs
            .iter()
            .map(|attr| {
                if attr.path().is_ident("display") {
                    if let Ok(name_ts) = attr.parse_args::<TokenStream2>() {
                        let n = format!("{}", name_ts);
                        let regexp = regex::Regex::new("name\\s*=\\s*\"(?P<name>.*)\"").unwrap();
                        let captures = regexp.captures(n.as_str()).expect("Captures");
                        let name = &captures["name"];
                        return (name.to_string() + ":", ident.to_string());
                    }
                }
                panic!("Missing field name")
            })
            .collect::<Vec<_>>()
    } else {
        panic!("Missing field name")
    }
}
