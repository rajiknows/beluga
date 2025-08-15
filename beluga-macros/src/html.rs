use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn li(input: TokenStream) -> TokenStream {
    let content = parse_macro_input!(input as LitStr);
    let html = format!("<li>{}</li>", content.value());
    quote!(#html).into()
}

#[proc_macro]
pub fn a(input: TokenStream) -> TokenStream {
    let content = parse_macro_input!(input as LitStr);
    let html = format!("<a>{}</a>", content.value());
    quote!(#html).into()
}

#[proc_macro]
pub fn div(input: TokenStream) -> TokenStream {
    let content = parse_macro_input!(input as LitStr);
    let html = format!("<div>{}</div>", content.value());
    quote!(#html).into()
}

#[proc_macro]
pub fn h(input: TokenStream) -> TokenStream {
    let content = parse_macro_input!(input as LitStr);
    let html = format!("<h1>{}</h1>", content.value());
    quote!(#html).into()
}
