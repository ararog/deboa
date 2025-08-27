use syn::{
    Ident, LitStr, Token, Type, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
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
            Ok(BoraApi {
                operations: content.parse_terminated(OperationEnum::parse, Token![,])?,
            })
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

const METHODS: [&str; 9] = ["get", "post", "put", "delete", "patch", "head", "options", "connect", "trace"];

impl Parse for OperationEnum {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            let ident = input.parse::<Ident>()?;
            match ident.to_string().as_str() {
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

pub struct GetStruct {
    pub fields: Punctuated<GetFieldEnum, Token![,]>,
}

impl Parse for GetStruct {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let content;
        parenthesized!(content in input);
        Ok(GetStruct {
            fields: content.parse_terminated(GetFieldEnum::parse, Token![,])?,
        })
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
            match ident.to_string().as_str() {
                "name" => Ok(GetFieldEnum::name(NameStruct::parse(input)?)),
                "path" => Ok(GetFieldEnum::path(PathStruct::parse(input)?)),
                "res_body" => Ok(GetFieldEnum::res_body(Box::new(ResBodyStruct::parse(input)?))),
                "format" => Ok(GetFieldEnum::format(FormatStruct::parse(input)?)),
                _ => Err(input.error(format!("expected one of name, path or res_body, found '{ident}'"))),
            }
        } else {
            Err(lookahead.error())
        }
    }
}

pub struct PostStruct {
    pub fields: Punctuated<PostFieldEnum, Token![,]>,
}

impl Parse for PostStruct {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let content;
        parenthesized!(content in input);
        Ok(PostStruct {
            fields: content.parse_terminated(PostFieldEnum::parse, Token![,])?,
        })
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
                "req_body" => Ok(PostFieldEnum::req_body(Box::new(ReqBodyStruct::parse(input)?))),
                "res_body" => Ok(PostFieldEnum::res_body(Box::new(ResBodyStruct::parse(input)?))),
                "format" => Ok(PostFieldEnum::format(FormatStruct::parse(input)?)),
                _ => Err(input.error(format!("expected one of name, path or req_body, found '{ident}'"))),
            }
        } else {
            Err(lookahead.error())
        }
    }
}

pub struct PutStruct {
    pub fields: Punctuated<PutFieldEnum, Token![,]>,
}

impl Parse for PutStruct {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let content;
        parenthesized!(content in input);
        Ok(PutStruct {
            fields: content.parse_terminated(PutFieldEnum::parse, Token![,])?,
        })
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
            match ident.to_string().as_str() {
                "name" => Ok(PutFieldEnum::name(NameStruct::parse(input)?)),
                "path" => Ok(PutFieldEnum::path(PathStruct::parse(input)?)),
                "req_body" => Ok(PutFieldEnum::req_body(Box::new(ReqBodyStruct::parse(input)?))),
                "res_body" => Ok(PutFieldEnum::res_body(Box::new(ResBodyStruct::parse(input)?))),
                "format" => Ok(PutFieldEnum::format(FormatStruct::parse(input)?)),
                _ => Err(input.error(format!("expected one of name, path or req_body, found '{ident}'"))),
            }
        } else {
            Err(lookahead.error())
        }
    }
}

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

pub struct PatchStruct {
    pub fields: Punctuated<PatchFieldEnum, Token![,]>,
}

impl Parse for PatchStruct {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let content;
        parenthesized!(content in input);
        Ok(PatchStruct {
            fields: content.parse_terminated(PatchFieldEnum::parse, Token![,])?,
        })
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

pub struct NameStruct {
    _equal_token: Token![=],
    pub value: LitStr,
}

impl Parse for NameStruct {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(NameStruct {
            _equal_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

pub struct PathStruct {
    _equal_token: Token![=],
    pub value: LitStr,
}

impl Parse for PathStruct {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(PathStruct {
            _equal_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

pub struct ReqBodyStruct {
    _equal_token: Token![=],
    pub value: Type,
}

impl Parse for ReqBodyStruct {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(ReqBodyStruct {
            _equal_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

pub struct ResBodyStruct {
    _equal_token: Token![=],
    pub value: Type,
}

impl Parse for ResBodyStruct {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(ResBodyStruct {
            _equal_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

pub struct FormatStruct {
    _equal_token: Token![=],
    pub value: LitStr,
}

impl Parse for FormatStruct {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(FormatStruct {
            _equal_token: input.parse()?,
            value: input.parse()?,
        })
    }
}
