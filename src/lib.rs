#![warn(missing_docs)]
#![doc=include_str!("../README.md")]

use proc_macro2::TokenStream;
use quote::quote;
use syn::Index;
/// from text "msg: &str, is_true: bool", return a TokenStream whith content that declares variables with the value of a tuple with the name and value of the variable corresponding to the text.
// for example
///```
///use quote_tool_params::prepare_values_from_params;
///use proc_macro2::TokenStream;
///let params1 = "msg: &str";
///let expected_result1: Vec<TokenStream> = vec!["let msg = params;".parse().unwrap()];
///let params2 = "msg: &str, is_true: bool";
///let expected_result2: Vec<TokenStream> = vec!["let msg = params.0;".parse().unwrap(), "let is_true = params.1;".parse().unwrap()];
///assert_eq!(prepare_values_from_params(params1, "params")[0].to_string(), expected_result1[0].to_string());
///assert_eq!(prepare_values_from_params(params2, "params")[0].to_string(), expected_result2[0].to_string());
///assert_eq!(prepare_values_from_params(params2, "params")[1].to_string(), expected_result2[1].to_string());
///```
pub fn prepare_values_from_params(params: &str, name_tuple: &str) -> Vec<TokenStream> {
    let mut token = Vec::new();
    let params_tuple_name: TokenStream = name_tuple.parse().unwrap();
    let params_name = get_from_params(params, true);
    let split = params_name.split(',');
    let unit = split.clone().collect::<Vec<&str>>().len() <= 1;
    for (index, name) in split.enumerate() {
        if !name.is_empty() {
            let name: TokenStream = name.parse().unwrap();
            let index = Index::from(index);
            if unit {
                token.push(quote! {
                    let #name = #params_tuple_name;
                })
            } else {
                token.push(quote! {
                    let #name = #params_tuple_name.#index;
                })
            }
        }
    }
    token
}

/// get a string of name of params or type of params dilimeted by comma.
/// true for getting the names and false to get the types.
/// example:
///```
///use quote_tool_params::get_from_params;
///let params = "msg: &str, is_true: bool";
///assert_eq!("msg, is_true", get_from_params(params, true));
///assert_eq!("&str, bool", get_from_params(params, false));
///```
pub fn get_from_params(params: &str, name_true_type_false: bool) -> String {
    let mut literal = Vec::new();
    for p in params.split(", ") {
        // p is name_param: type
        if let Some(p) = p.split_once(": ") {
            if name_true_type_false {
                literal.push(p.0.trim())
            } else {
                literal.push(p.1.trim())
            }
        }
    }
    literal.join(", ")
}
