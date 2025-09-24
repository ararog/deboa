use syn::{
    Ident, Token, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

use crate::parser::common::field::{FormatStruct, NameStruct, PathStruct, ReqBodyStruct, ResBodyStruct};

pub struct PatchStruct {
    pub fields: Punctuated<PatchFieldEnum, Token![,]>,
}

impl Parse for PatchStruct {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let content;
        parenthesized!(content in input);

        let patch = PatchStruct {
            fields: content.parse_terminated(PatchFieldEnum::parse, Token![,])?,
        };

        let mut fields = patch.fields.iter();
        let required_fields = vec!["name", "path", "req_body", "format"];
        let missing_fields = required_fields
            .into_iter()
            .filter(|field| {
                !fields.any(|f| match f {
                    PatchFieldEnum::name(_) => *field == "name",
                    PatchFieldEnum::path(_) => *field == "path",
                    PatchFieldEnum::res_body(_) => *field == "res_body",
                    PatchFieldEnum::format(_) => *field == "format",
                    PatchFieldEnum::req_body(_) => *field == "req_body",
                })
            })
            .collect::<Vec<_>>();

        if !missing_fields.is_empty() {
            return Err(input.error(format!("expected one of {missing_fields:?}")));
        }

        Ok(patch)
    }
}

#[allow(non_camel_case_types)]
pub enum PatchFieldEnum {
    name(NameStruct),
    path(PathStruct),
    req_body(Box<ReqBodyStruct>),
    res_body(Box<ResBodyStruct>),
    format(FormatStruct),
}

impl Parse for PatchFieldEnum {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            let ident = input.parse::<Ident>()?;
            match ident.to_string().as_str() {
                "name" => Ok(PatchFieldEnum::name(NameStruct::parse(input)?)),
                "path" => Ok(PatchFieldEnum::path(PathStruct::parse(input)?)),
                "req_body" => Ok(PatchFieldEnum::req_body(Box::new(ReqBodyStruct::parse(input)?))),
                "res_body" => Ok(PatchFieldEnum::res_body(Box::new(ResBodyStruct::parse(input)?))),
                "format" => Ok(PatchFieldEnum::format(FormatStruct::parse(input)?)),
                _ => Err(input.error(format!("expected one of name, path or req_body, found '{ident}'"))),
            }
        } else {
            Err(lookahead.error())
        }
    }
}
