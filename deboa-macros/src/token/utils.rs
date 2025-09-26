use regex::Regex;
use crate::parser::utils::extract_path_params;
use proc_macro2::Span;
use syn::{parse_str, LitStr};
use quote::quote;
use proc_macro2::TokenStream;

pub fn extract_params_from_path(path: &LitStr) -> (TokenStream, LitStr) {
    let raw_path = path.value();
    let params = Regex::new(r"<(\w*:\&{0,1}\w*)>")
        .unwrap()
        .captures_iter(&raw_path)
        .map(|m| m.get(1).unwrap().as_str())
        .collect::<Vec<_>>();

    let api_params = params.clone().into_iter().fold(TokenStream::new(), |mut acc, param| {
        let pair = param.split(':').collect::<Vec<_>>();
        let param = parse_str::<syn::Ident>(pair[0]).unwrap();
        let param_type = parse_str::<syn::Type>(pair[1]).unwrap();
        acc.extend(quote! {
            #param: #param_type,
        });
        acc
    });

    let has_query = raw_path.contains("?");
    let new_path = if has_query {
        let parts = raw_path.split("?").collect::<Vec<_>>();
        let path = parts[0];
        let query = parts[1];

        let path_output = extract_path_params(path);

        let query_regex = Regex::new(r"<(\w*:\&{0,1}\w*)>").expect("Invalid query");
        let query_output = query_regex
            .captures_iter(query)
            .map(|m| {
                let pair = m.get(1).unwrap().as_str().split(':').collect::<Vec<_>>();
                format!("{}={{{}}}", pair[0], pair[0])
            })
            .collect::<Vec<_>>()
            .join("&");
        format!("{path_output}?{query_output}")
    } else {
        extract_path_params(&raw_path)
    };

    (api_params, LitStr::new(&new_path, Span::call_site()))
}
