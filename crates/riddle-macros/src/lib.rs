extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(CloneHandle, attributes(self_handle))]
pub fn derive_clone_handle(t: TokenStream) -> TokenStream {
    let input = parse_macro_input!(t as DeriveInput);
    let struct_name = input.ident;

    let handle_field = match input.data {
        syn::Data::Struct(ds) => {
            let handle_fields: Vec<syn::Field> = ds
                .fields
                .into_iter()
                .filter(|f| {
                    f.attrs
                        .iter()
                        .filter(|attr| attr.path.is_ident("self_handle"))
                        .count()
                        > 0
                })
                .collect();

            if handle_fields.len() == 1 {
                Some(handle_fields[0].clone())
            } else {
                None
            }
        }
        _ => None,
    };

    let output = if let Some(field) = handle_field {
        let handle_ident = field.ident.unwrap();
        quote! {
            impl riddle_common::CloneHandle for #struct_name {
                type Handle = std::sync::Arc<#struct_name>;
                type WeakHandle = std::sync::Weak<#struct_name>;

                #[inline]
                fn clone_handle(&self) -> Option<<Self as riddle_common::CloneHandle>::Handle> {
                    std::sync::Weak::upgrade(&self.#handle_ident)
                }

                #[inline]
                fn clone_weak_handle(&self) -> <Self as riddle_common::CloneHandle>::WeakHandle {
                    self.#handle_ident.clone()
                }
            }
        }
    } else {
        quote! {}
    };

    proc_macro::TokenStream::from(output)
}
