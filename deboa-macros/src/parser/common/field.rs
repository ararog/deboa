use syn::{
    LitStr, Token, Type,
    parse::{Parse, ParseStream},
};

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct FormatStruct {
    _equal_token: Token![=],
    pub value: LitStr,
}

fn avaliable_formats() -> Vec<String> {
    let mut valid_formats = Vec::<String>::with_capacity(3);

    #[cfg(feature = "json")]
    valid_formats.push("json".to_string());
    #[cfg(feature = "xml")]
    valid_formats.push("xml".to_string());
    #[cfg(feature = "msgpack")]
    valid_formats.push("msgpack".to_string());

    valid_formats
}

fn is_valid_format(format: &String) -> bool {
    avaliable_formats().contains(format)
}

impl Parse for FormatStruct {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let format = FormatStruct {
            _equal_token: input.parse()?,
            value: input.parse()?,
        };

        let format_name = format.value.value();
        if !is_valid_format(&format_name) {
            return Err(input.error(format!("expected one of {}, found '{format_name}'", avaliable_formats().join(", "))));
        }

        Ok(format)
    }
}
