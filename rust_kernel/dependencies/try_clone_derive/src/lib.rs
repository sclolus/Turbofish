extern crate proc_macro;
use syn::spanned::Spanned;
use syn::Data::{Enum, Struct};
use syn::GenericParam;
use syn::{parse_macro_input, DeriveInput, Fields};
extern crate proc_quote;
use proc_quote::quote;

#[proc_macro_derive(TryClone)]
pub fn derive_try_clone(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // dbg!(&input);
    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    let impl_generic = input.generics.clone();
    let generic = input.generics.params.into_iter().map(|x| match x {
        GenericParam::Type(type_param) => type_param.ident,
        _ => panic!("unhandled generic param"),
    });
    // dbg!(&impl_generic);
    let expanded = match input.data {
        Struct(datastruct) => {
            // dbg!(&datastruct);
            let all_names = match datastruct.fields {
                Fields::Named(fields) => {
                    let fields = fields.named.iter().map(|x| x.ident.clone());
                    quote!(Self {#( #fields: fallible_collections::TryClone::try_clone(&self.#fields)?, )*})
                }
                Fields::Unit => quote!(Self),
                Fields::Unnamed(fields) => {
                    let fields = (0..fields.unnamed.len()).map(|i| syn::Index::from(i));
                    quote!(Self(#(self.#fields.try_clone()?,)*))
                }
            };
            quote!(
                impl#impl_generic fallible_collections::TryClone for #name<#(#generic),*> {
                    fn try_clone(&self) -> core::result::Result<Self,alloc::collections::CollectionAllocErr> {
                        Ok(
                                #all_names
                        )
                    }
                }
            )
        }
        Enum(datastruct) => {
            // dbg!(&datastruct);
            let all_names = datastruct.variants.iter().map(|x| match &x.fields {
                Fields::Unit => {
                    let variant = x.ident.clone();
                    quote!(#name::#variant => #name::#variant,)
                }
                Fields::Unnamed(fields) => {
                    let fields = (0..fields.unnamed.len()).map(|i| {//.iter().enumerate().map(|(i, x)| {
                        // quote!(a#i)
                        let mut ident = [0];
                        syn::Ident::new(
                            ((('a' as u8) + i as u8) as char).encode_utf8(&mut ident),
                            x.span(),
                        )
                    });
                    let fields_clone = fields.clone();
                    let variant = x.ident.clone();
                    quote!(#name::#variant(#(#fields,)*) => #name::#variant(#(#fields_clone.try_clone()?,)*),)
                }
                Fields::Named(fields) => {
                    let fields = fields.named.iter().map(|x| x.ident.clone());;
                    let fields_clone = fields.clone();
                    let fields_clone2 = fields.clone();
                    let variant = x.ident.clone();
                    quote!(#name::#variant{#(#fields,)*} => #name::#variant{#(#fields_clone2: #fields_clone.try_clone()?,)*},)
                }
            });
            quote!(
                impl fallible_collections::TryClone for #name {
                    fn try_clone(&self) -> core::result::Result<Self,alloc::collections::CollectionAllocErr> {
                        Ok(
                            match self {
                                #( #all_names )*
                            }
                        )
                    }
                }
            )
        }
        _ => panic!("bad"),
    };

    // println!("{}", &expanded.to_string());

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}
