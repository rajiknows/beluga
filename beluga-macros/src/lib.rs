use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};
mod html;

#[proc_macro]
pub fn beluga(input: TokenStream) -> TokenStream {
    // Parse the input as a string literal (e.g., "src/index.beluga")
    let path_literal = parse_macro_input!(input as LitStr);
    let file_path = path_literal.value();

    // Read the file content at compile time
    let content =
        std::fs::read_to_string(&file_path).expect(&format!("Failed to read file: {}", file_path));

    let parsed_bag = parse_html(content);
    let rust_code = parsed_bag.rust_code;
    parsed_bag.put(dynamic_content);

    let final_html = parsed_bag.html();

    // put this  into the dist/
    // as it was directory wise in src/
    //

    // Use `quote` to generate Rust code that creates a string
    let expanded_code = quote! {
        #content
    };

    // Return the generated code as a TokenStream
    expanded_code.into()
}
