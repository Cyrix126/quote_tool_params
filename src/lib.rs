#![warn(missing_docs)]
#![doc=include_str!("../README.md")]

use proc_macro2::TokenStream;
use quote::quote;
/// from text "msg: &str, is_true: bool", return a TokenStream whith content that declares variables with the value of a tuple with the name and value of the variable corresponding to the text.
// for example
///```
///use quote_tool_params::prepare_values_from_params;
///use proc_macro2::TokenStream;
///let params1 = "msg: &str";
///let expected_result1: TokenStream = "let msg = params;".parse().unwrap();
///let params2 = "msg: &str, is_true: bool";
///let expected_result2: TokenStream = "let (msg, is_true) = params;".parse().unwrap();
///assert_eq!(prepare_values_from_params(params1, "params").to_string(), expected_result1.to_string());
///assert_eq!(prepare_values_from_params(params2, "params").to_string(), expected_result2.to_string());
///```
pub fn prepare_values_from_params(params: &str, name_tuple: &str) -> TokenStream {
    let params_tuple_name: TokenStream = name_tuple.parse().unwrap();
    let params_name: TokenStream = get_from_params(params, true).parse().unwrap();
    if params.is_empty() {
        TokenStream::new()
    } else if params.contains(",") {
        quote! {
        let (#params_name) = #params_tuple_name;
        }
    } else {
        quote! {
        let #params_name = #params_tuple_name;
        }
    }
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

/// from the parameters, get the borrowing type of the tuple.
/// if the tuple has element(s) with mutable borrows, it will return &mut as TokenStream.
/// If the tuple has immutable borrows, it will return & as TokenStream.
/// If the tuple has owned elements, it will return an empty TokenStream.
/// example:
///```
///use quote_tool_params::get_borrower;
///let params_immutable = "msg: &str, is_true: bool";
///let params_mutable = "msg: &mut String, is_true: bool";
///let params_owned = "msg: String, is_true: bool";
///assert_eq!("&".to_string(), get_borrower(params_immutable).to_string());
///assert_eq!("& mut".to_string(), get_borrower(params_mutable).to_string());
///assert_eq!("".to_string(), get_borrower(params_owned).to_string());
///```
pub fn get_borrower(params: &str) -> TokenStream {
    if params.contains("&mut") {
        return "&mut".parse().unwrap();
    } else if params.contains("&") {
        return "&".parse().unwrap();
    } else {
        "".parse().unwrap()
    }
}
