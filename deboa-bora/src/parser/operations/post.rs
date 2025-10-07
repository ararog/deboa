use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, Token,
};

use crate::parser::common::field::{
    FormatStruct, NameStruct, PathStruct, ReqBodyStruct, ResBodyStruct,
};

pub struct PostStruct {
    pub fields: Punctuated<PostFieldEnum, Token![,]>,
}

impl Parse for PostStruct {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let content;
        parenthesized!(content in input);

        let post = PostStruct {
            fields: content.parse_terminated(PostFieldEnum::parse, Token![,])?,
        };

        let mut fields = post.fields.iter();
        let required_fields = vec!["name", "path", "req_body", "format"];
        let missing_fields = required_fields
            .into_iter()
            .filter(|field| {
                !fields.any(|f| match f {
                    PostFieldEnum::name(_) => *field == "name",
                    PostFieldEnum::path(_) => *field == "path",
                    PostFieldEnum::res_body(_) => *field == "res_body",
                    PostFieldEnum::format(_) => *field == "format",
                    PostFieldEnum::req_body(_) => *field == "req_body",
                })
            })
            .collect::<Vec<_>>();

        if !missing_fields.is_empty() {
            return Err(input.error(format!("expected one of {missing_fields:?}")));
        }

        Ok(post)
    }
}

#[allow(non_camel_case_types)]
pub enum PostFieldEnum {
    name(NameStruct),
    path(PathStruct),
    req_body(Box<ReqBodyStruct>),
    res_body(Box<ResBodyStruct>),
    format(FormatStruct),
}

impl Parse for PostFieldEnum {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            let ident = input.parse::<Ident>()?;
            match ident.to_string().as_str() {
                "name" => Ok(PostFieldEnum::name(NameStruct::parse(input)?)),
                "path" => Ok(PostFieldEnum::path(PathStruct::parse(input)?)),
                "req_body" => Ok(PostFieldEnum::req_body(Box::new(ReqBodyStruct::parse(
                    input,
                )?))),
                "res_body" => Ok(PostFieldEnum::res_body(Box::new(ResBodyStruct::parse(
                    input,
                )?))),
                "format" => Ok(PostFieldEnum::format(FormatStruct::parse(input)?)),
                _ => Err(input.error(format!(
                    "expected one of name, path or req_body, found '{ident}'"
                ))),
            }
        } else {
            Err(lookahead.error())
        }
    }
}
