use syn::{
    Ident, LitStr, Token, parenthesized,
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
}

const METHODS: [&str; 9] = [
    "get", "post", "put", "delete", "patch", "head", "options", "connect", "trace",
];

impl Parse for OperationEnum {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            let ident = input.parse::<Ident>()?;
            match ident.to_string().as_str() {
                "get" => Ok(OperationEnum::get(GetStruct::parse(input)?)),
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
    target(TargetStruct),
}

impl Parse for GetFieldEnum {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            let ident = input.parse::<Ident>()?;
            match ident.to_string().as_str() {
                "name" => Ok(GetFieldEnum::name(NameStruct::parse(input)?)),
                "path" => Ok(GetFieldEnum::path(PathStruct::parse(input)?)),
                "target" => Ok(GetFieldEnum::target(TargetStruct::parse(input)?)),
                _ => Err(input.error(format!(
                    "expected one of name, path or target, found '{ident}'"
                ))),
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

pub struct TargetStruct {
    _equal_token: Token![=],
    pub value: Ident,
}

impl Parse for TargetStruct {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(TargetStruct {
            _equal_token: input.parse()?,
            value: input.parse()?,
        })
    }
}
