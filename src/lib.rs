//! No boilerplate code for extension function definitions.

use case::CaseExt;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    parse::Parse,
    parse_macro_input,
    punctuated::Punctuated,
    token:: Plus,
    Attribute, Ident, ItemFn, TypeParamBound, Visibility, Token,
};

enum Args {
    Ident(Ident),
    Generics(Punctuated<TypeParamBound, Plus>),
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.parse::<Option<Token![trait]>>()?.is_some() {
            Ok(Self::Generics(Punctuated::parse_terminated(input)?))
        } else {
            Ok(Self::Ident(input.parse()?))
        }
    }
}

/// Extension function macro. Example usage:
/// ```rust
/// use extension_fn::extension_fn;
/// 
/// // Also you can change pub to anything e.g. pub(crate) to make this function public only for current crate.
/// #[extension_fn(str)]
/// pub fn count_numbers(&self) -> u32 {
///     self.chars().fold(0, |count, char| {
///        if char.is_numeric() {
///         count + 1
///        } else {
///           count
///        }
///    })
/// }
/// ```
/// Example with async function(you must add async-trait to your dependencies`):
/// ```rust
/// use extension_fn::extension_fn;
/// use std::net::TcpStream;
/// 
/// #[extension_fn(TcpStream)]
/// pub async fn do_something() {}
/// ```
/// Example for trait bound:
/// ```rust
/// use extension_fn::extension_fn;
/// 
/// #[extension_fn(trait AsRef<str>)]
/// fn print(&self) {
///     println!("{}", self.as_ref());
/// }
/// ```
#[proc_macro_attribute]
pub fn extension_fn(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = parse_macro_input!(args as Args);
    let fn_definition = parse_macro_input!(input as ItemFn);
    let trait_ident = Ident::new(
        &fn_definition.sig.ident.to_string().to_camel(),
        Span::mixed_site(),
    );
    let mod_trait_sealed_ident= Ident::new(
        &("__seal_".to_string() + &fn_definition.sig.ident.to_string().to_camel_lowercase()),
        Span::mixed_site()
    );
    let trait_definition = def_trait(&fn_definition, trait_ident.clone(), mod_trait_sealed_ident.clone());
    let trait_impl = match args {
        Args::Ident(ident) => impl_trait(ident, &fn_definition, trait_ident, mod_trait_sealed_ident),
        Args::Generics(generics) => impl_trait_for_generics(generics, &fn_definition, trait_ident, mod_trait_sealed_ident)
    };
    quote! {
        #trait_definition
        #trait_impl
    }
    .into()
}

fn attrs_tokenstream(attrs: Vec<Attribute>) -> TokenStream {
    let mut attrs_tokenstream = TokenStream::new();
    for attr in attrs {
        attrs_tokenstream.extend(quote! {
            #attr
        });
    }
    attrs_tokenstream
}

fn impl_trait(target: Ident, fn_definition: &ItemFn, trait_ident: Ident, mod_trait_sealed_ident: Ident) -> TokenStream {
    let fn_definition = {
        let mut fn_definition = fn_definition.clone();
        fn_definition.vis = Visibility::Inherited;

        fn_definition
    };

    let trait_implementation = quote! {
        impl #trait_ident for #target {
                #fn_definition
        }

        impl #mod_trait_sealed_ident::Sealed for #target {}
    };

    if fn_definition.sig.asyncness.is_some() {
        return quote! {
            #[async_trait::async_trait]
            #trait_implementation
        };
    } 

    trait_implementation
    
}

fn impl_trait_for_generics(target: Punctuated<TypeParamBound, Plus>, fn_definition: &ItemFn, trait_ident: Ident, mod_trait_sealed_ident: Ident) -> TokenStream {
    let fn_definition = {
        let mut fn_definition = fn_definition.clone();
        fn_definition.vis = Visibility::Inherited;

        fn_definition
    };

    let trait_implementation = quote! {
        impl<T> #trait_ident for T where T: #target {
            #fn_definition
        }
        impl<T> #mod_trait_sealed_ident::Sealed for T where T: #target {}
    };

    if fn_definition.sig.asyncness.is_some() {
        return quote! {
            #[async_trait::async_trait]
            #trait_implementation
        };
    }

    trait_implementation
}


fn def_trait(fn_definition: &ItemFn, trait_ident: Ident, mod_trait_sealed_ident: Ident) -> TokenStream {
    let fn_attrs = attrs_tokenstream(fn_definition.attrs.clone());
    let fn_sig = fn_definition.sig.clone();
    let visibility = fn_definition.vis.clone();

    let trait_definition = quote! {
        #visibility trait #trait_ident: #mod_trait_sealed_ident::Sealed {
            #fn_attrs #fn_sig;
        }
        
        mod #mod_trait_sealed_ident {
            pub trait Sealed {}
        }
    };

    if fn_sig.asyncness.is_some() {
        return quote! {
            #[async_trait::async_trait]
            #trait_definition
        };
    } 

    trait_definition
}
