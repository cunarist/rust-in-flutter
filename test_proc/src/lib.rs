use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Serialize)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let gen = match input.data {
        Data::Struct(ref data) => {
            let fields: Vec<_> = match &data.fields {
                Fields::Named(fields_named) => fields_named.named.iter().collect(),
                Fields::Unnamed(fields_unnamed) => fields_unnamed.unnamed.iter().collect(),
                Fields::Unit => Vec::new(),
            };

            let field_serializations = fields.iter().enumerate().map(|(i, field)| {
                let name = &field.ident;
                if let Some(name) = name {
                    quote! { format!("{}: {}", stringify!(#name), self.#name.serialize()) }
                } else {
                    let index = syn::Index::from(i);
                    quote! { format!("{}: {}", stringify!(#index), self.#index.serialize()) }
                }
            });

            quote! {
                impl Serialize for #name {
                    fn serialize(&self) -> String {
                        let serialized_fields = vec![
                            #(#field_serializations),*
                        ].join(", ");
                        format!("{} {{ {} }}", stringify!(#name), serialized_fields)
                    }
                }
            }
        }
        Data::Enum(ref data) => {
            let variant_serializations = data.variants.iter().map(|variant| {
                let ident = &variant.ident;
                let fields = &variant.fields;
                match fields {
                    Fields::Named(_) => {
                        quote! {
                            Self::#ident {..} => format!("{}::{}", stringify!(#name), stringify!(#ident))
                        }
                    }
                    Fields::Unnamed(_) => {
                        quote! {
                            Self::#ident(..) => format!("{}::{}", stringify!(#name), stringify!(#ident))
                        }
                    }
                    Fields::Unit => {
                        quote! {
                            Self::#ident => format!("{}::{}", stringify!(#name), stringify!(#ident))
                        }
                    }
                }
            });

            quote! {
                impl Serialize for #name {
                    fn serialize(&self) -> String {
                        match self {
                            #(#variant_serializations),*
                        }
                    }
                }
            }
        }
        _ => unimplemented!(),
    };

    TokenStream::from(gen)
}
