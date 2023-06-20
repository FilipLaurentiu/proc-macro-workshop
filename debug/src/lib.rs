use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(CustomDebug)]
pub fn derive(input: TokenStream) -> TokenStream {
    let derive = parse_macro_input!(input as DeriveInput);

    let struct_name = derive.ident;
    let struct_name_string = struct_name.to_string();

    let fields;

    if let syn::Data::Struct(data_struct) = derive.data {
        // if let syn::Fields::Named(named) = data_struct.fields {
        fields = data_struct.fields;
    } else {
        panic!("Support only structs")
    }

    let fields_ident = fields
        .iter()
        .filter_map(|field| field.ident.clone())
        .collect::<Vec<Ident>>();
    let fields_name = fields_ident
        .iter()
        .map(|ident| ident.to_string())
        .collect::<Vec<String>>();

    let res = quote! {
        impl std::fmt::Debug for #struct_name {
            fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                fmt.debug_struct(#struct_name_string)
                    #(.field(#fields_name, &self.#fields_ident))*
                    .finish()
            }
        }

    };

    res.into()
}
