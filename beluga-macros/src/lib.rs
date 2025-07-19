//use std::fs;

use proc_macro::TokenStream;
use quote::*;

#[proc_macro]
pub fn render(_input: TokenStream) -> TokenStream {
    //let html = fs::read_to_string("templates/index.html").unwrap();

    let final_html = "<> hello guys this is form macro</>";

    quote! {
        #final_html
    }
    .into()
}
