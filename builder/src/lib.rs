use proc_macro::{TokenStream};
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};
use syn::Data::Struct;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let builder_name = format_ident!("{}Builder", name);
    let fields;

    match input.data {
        Struct(data_struct) => {
            fields = data_struct.fields;
        }
        _ => panic!("Support only struct")
    }

    let fields_idents = fields.iter().filter_map(|field| {
        if field.ident.is_some() {
            Some(&field.ident)
        } else {
            None
        }
    }).collect::<Vec<_>>();
    let fields_types = fields.iter().map(|field| &field.ty).collect::<Vec<_>>();


    let res = quote! {
        pub struct #builder_name {
                 #(#fields_idents : std::option::Option::<#fields_types>,)*
        }

        impl #builder_name {
            #(fn #fields_idents(&mut self, ty: #fields_types) -> &mut Self  {
                self.#fields_idents = Some(ty);
                self
            })*

            pub fn build(&mut self) -> std::result::Result<#name, std::boxed::Box<dyn std::error::Error>> {
                Ok(#name {
                     #(#fields_idents : self.#fields_idents.clone().expect("Field not set"),)*
                })
            }
        }


        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#fields_idents: std::option::Option::None,)*
                }
            }
        }
    };

    res.into()
}
