use case::CaseExt;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{ItemFn, Visibility, Attribute, Generics, Type, punctuated::Punctuated, TypeParamBound, token::Plus, parse2};

use crate::args::{Args, ArgKind};

pub(crate) struct Builder {
    trait_ident: Ident,
    sealed_trait_mod: Ident
}

impl Builder {
    pub fn new(function_ident: &Ident) -> Self {
        Self { 
            trait_ident: Ident::new(&function_ident.to_string().to_camel(), Span::mixed_site()),
            sealed_trait_mod: Ident::new(&format!("__seal_{}", function_ident.to_string().to_camel()), Span::mixed_site())
        }
    }
    pub fn build(self, mut fn_implementation: ItemFn, args: Args) -> TokenStream {
        let generics = args.generics.unwrap_or_default();
        fn_implementation.vis = Visibility::Inherited;

        let (trait_definition, trait_implementation) = match args.kind {
            ArgKind::Type(ty) => (self.define_trait(&fn_implementation, generics.clone()), self.impl_type(fn_implementation, ty, generics)),
            ArgKind::TraitBound(bound) => {
                let mut generics = generics;
                generics.params.push(parse2(quote!(T: #bound)).unwrap());
                (self.define_trait(&fn_implementation, generics.clone()), self.impl_trait_bound(fn_implementation, bound, generics))
            },
        };

        quote! {
            #trait_definition
            #trait_implementation
        }
    }

    fn impl_trait_bound(&self, fn_implementation: ItemFn, _bound: Punctuated<TypeParamBound, Plus>, generics: Generics) -> TokenStream {
        let _attrs = attrs_tokenstream(&fn_implementation.attrs);
        let trait_ident = &self.trait_ident;
        let sealed_trait_mod = &self.sealed_trait_mod;
        let _signature = &fn_implementation.sig;

        let (impl_generics, ty_generics, _) = generics.split_for_impl();


        add_async_trait(&fn_implementation, quote! {
            impl #impl_generics #trait_ident #ty_generics for T {
                #fn_implementation
            }
            impl #impl_generics #sealed_trait_mod::Sealed for T {} 
        })
    }

    fn impl_type(&self, fn_implementation: ItemFn, impl_type: Type, generics: Generics) -> TokenStream {
        let trait_ident = &self.trait_ident;
        let sealed_trait_mod = &self.sealed_trait_mod;

        let (impl_generics, ty_generics, _) = generics.split_for_impl();

        add_async_trait(&fn_implementation, quote! {
            impl #impl_generics #trait_ident #ty_generics for #impl_type {
                #fn_implementation
            }
            impl #impl_generics #sealed_trait_mod::Sealed for #impl_type {}
        })
    }

    fn define_trait(&self, fn_implementation: &ItemFn, generics: Generics) -> TokenStream {
        let attrs = attrs_tokenstream(&fn_implementation.attrs);
        let visibility = &fn_implementation.vis;
        let signature = &fn_implementation.sig;

        let (_, ty_generics, _) = generics.split_for_impl();

        let trait_ident = &self.trait_ident;
        let sealed_trait_mod = &self.sealed_trait_mod;

        add_async_trait(fn_implementation, quote! {
            #visibility trait #trait_ident #ty_generics: #sealed_trait_mod::Sealed {
                #attrs #signature;
            }
            mod #sealed_trait_mod {
                pub trait Sealed {}
            }
        })
    }
}

fn add_async_trait(fn_implementation: &ItemFn, token_stream: TokenStream) -> TokenStream {
    if fn_implementation.sig.asyncness.is_some() {
        quote!(#[async_trait::async_trait] #token_stream)
    } else {
        token_stream
    }
}

fn attrs_tokenstream(attrs: &[Attribute]) -> TokenStream {
    TokenStream::from_iter(attrs.iter().map(|attr| attr.into_token_stream()))
}