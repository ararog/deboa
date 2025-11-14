use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, Token,
};

use crate::parser::common::field::{
    FormatStruct, NameStruct, PathStruct, ReqBodyStruct, ResBodyStruct,
};

pub struct PutStruct {
    pub fields: Punctuated<PutFieldEnum, Token![,]>,
}

impl Parse for PutStruct {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let content;
        parenthesized!(content in input);

        let put = PutStruct { fields: content.parse_terminated(PutFieldEnum::parse, Token![,])? };

        let mut fields = put.fields.iter();
        let required_fields = vec!["name", "path", "req_body", "format"];
        let missing_fields = required_fields
            .into_iter()
            .filter(|field| {
                !fields.any(|f| match f {
                    PutFieldEnum::name(_) => *field == "name",
                    PutFieldEnum::path(_) => *field == "path",
                    PutFieldEnum::res_body(_) => *field == "res_body",
                    PutFieldEnum::format(_) => *field == "format",
                    PutFieldEnum::req_body(_) => *field == "req_body",
                })
            })
            .collect::<Vec<_>>();

        if !missing_fields.is_empty() {
            return Err(input.error(format!("expected one of {missing_fields:?}")));
        }

        Ok(put)
    }
}

#[allow(non_camel_case_types)]
pub enum PutFieldEnum {
    name(NameStruct),
    path(PathStruct),
    req_body(Box<ReqBodyStruct>),
    res_body(Box<ResBodyStruct>),
    format(FormatStruct),
}

impl Parse for PutFieldEnum {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            let ident = input.parse::<Ident>()?;
            match ident
                .to_string()
                .as_str()
            {
                "name" => Ok(PutFieldEnum::name(NameStruct::parse(input)?)),
                "path" => Ok(PutFieldEnum::path(PathStruct::parse(input)?)),
                "req_body" => Ok(PutFieldEnum::req_body(Box::new(ReqBodyStruct::parse(input)?))),
                "res_body" => Ok(PutFieldEnum::res_body(Box::new(ResBodyStruct::parse(input)?))),
                "format" => Ok(PutFieldEnum::format(FormatStruct::parse(input)?)),
                _ => {
                    Err(input
                        .error(format!("expected one of name, path or req_body, found '{ident}'")))
                }
            }
        } else {
            Err(lookahead.error())
        }
    }
}
