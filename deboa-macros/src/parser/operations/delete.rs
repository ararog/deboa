use syn::{
    Ident, Token, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

use crate::parser::common::field::{NameStruct, PathStruct};

pub struct DeleteStruct {
    pub fields: Punctuated<DeleteFieldEnum, Token![,]>,
}

impl Parse for DeleteStruct {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let content;
        parenthesized!(content in input);
        Ok(DeleteStruct {
            fields: content.parse_terminated(DeleteFieldEnum::parse, Token![,])?,
        })
    }
}

#[allow(non_camel_case_types)]
pub enum DeleteFieldEnum {
    name(NameStruct),
    path(PathStruct),
}

impl Parse for DeleteFieldEnum {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            let ident = input.parse::<Ident>()?;
            match ident.to_string().as_str() {
                "name" => Ok(DeleteFieldEnum::name(NameStruct::parse(input)?)),
                "path" => Ok(DeleteFieldEnum::path(PathStruct::parse(input)?)),
                _ => Err(input.error(format!("expected one of name, path or req_body, found '{ident}'"))),
            }
        } else {
            Err(lookahead.error())
        }
    }
}
