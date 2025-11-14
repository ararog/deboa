use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, Token,
};

use crate::parser::common::field::{FormatStruct, NameStruct, PathStruct, ResBodyStruct};

pub struct GetStruct {
    pub fields: Punctuated<GetFieldEnum, Token![,]>,
}

impl Parse for GetStruct {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let content;
        parenthesized!(content in input);
        let get = GetStruct { fields: content.parse_terminated(GetFieldEnum::parse, Token![,])? };

        let mut fields = get.fields.iter();
        let required_fields = vec!["name", "path", "res_body", "format"];
        let missing_fields = required_fields
            .into_iter()
            .filter(|field| {
                !fields.any(|f| match f {
                    GetFieldEnum::name(_) => *field == "name",
                    GetFieldEnum::path(_) => *field == "path",
                    GetFieldEnum::res_body(_) => *field == "res_body",
                    GetFieldEnum::format(_) => *field == "format",
                })
            })
            .collect::<Vec<_>>();

        if !missing_fields.is_empty() {
            return Err(input.error(format!("expected one of {missing_fields:?}")));
        }

        Ok(get)
    }
}

#[allow(non_camel_case_types)]
pub enum GetFieldEnum {
    name(NameStruct),
    path(PathStruct),
    res_body(Box<ResBodyStruct>),
    format(FormatStruct),
}

impl Parse for GetFieldEnum {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            let ident = input.parse::<Ident>()?;
            match ident
                .to_string()
                .as_str()
            {
                "name" => Ok(GetFieldEnum::name(NameStruct::parse(input)?)),
                "path" => Ok(GetFieldEnum::path(PathStruct::parse(input)?)),
                "res_body" => Ok(GetFieldEnum::res_body(Box::new(ResBodyStruct::parse(input)?))),
                "format" => Ok(GetFieldEnum::format(FormatStruct::parse(input)?)),
                _ => {
                    Err(input
                        .error(format!("expected one of name, path or res_body, found '{ident}'")))
                }
            }
        } else {
            Err(lookahead.error())
        }
    }
}
