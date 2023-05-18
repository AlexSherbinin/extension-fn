use syn::{
    parse::Parse, punctuated::Punctuated, token::Plus, Generics, Token, Type,
    TypeParamBound,
};

#[public]
struct Args {
    kind: ArgKind,
    generics: Option<Generics>,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let generics = if input.peek(Token![<]) {
            Some(input.parse()?)
        } else {
            None
        };

        Ok(Self { generics, kind: input.parse()? })
    }
}

pub enum ArgKind {
    Type(Type),
    TraitBound(Punctuated<TypeParamBound, Plus>),
}

impl Parse for ArgKind {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.parse::<Option<Token![trait]>>()?.is_some() {
            Ok(Self::TraitBound(Punctuated::parse_terminated(input)?))
        } else {
            Ok(Self::Type(input.parse()?))
        }
    }
}
