use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, Token,
};

use crate::parser::operations::{
    delete::DeleteStruct, get::GetStruct, patch::PatchStruct, post::PostStruct, put::PutStruct,
};
pub struct BoraApi {
    pub operations: Punctuated<OperationEnum, Token![,]>,
}

impl Parse for BoraApi {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            let ident = input.parse::<Ident>()?;
            if ident != "api" {
                return Err(input.error(format!("expected 'api', found '{ident}'")));
            }
            let content;
            parenthesized!(content in input);
            Ok(BoraApi { operations: content.parse_terminated(OperationEnum::parse, Token![,])? })
        } else {
            Err(lookahead.error())
        }
    }
}

#[allow(non_camel_case_types)]
pub enum OperationEnum {
    get(GetStruct),
    post(PostStruct),
    put(PutStruct),
    delete(DeleteStruct),
    patch(PatchStruct),
}

const METHODS: [&str; 9] =
    ["get", "post", "put", "delete", "patch", "head", "options", "connect", "trace"];

impl Parse for OperationEnum {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            let ident = input.parse::<Ident>()?;
            match ident
                .to_string()
                .as_str()
            {
                "get" => Ok(OperationEnum::get(GetStruct::parse(input)?)),
                "post" => Ok(OperationEnum::post(PostStruct::parse(input)?)),
                "put" => Ok(OperationEnum::put(PutStruct::parse(input)?)),
                "delete" => Ok(OperationEnum::delete(DeleteStruct::parse(input)?)),
                "patch" => Ok(OperationEnum::patch(PatchStruct::parse(input)?)),
                _ => Err(input.error(format!("expected one of {METHODS:?}, found '{ident}'"))),
            }
        } else {
            Err(lookahead.error())
        }
    }
}
